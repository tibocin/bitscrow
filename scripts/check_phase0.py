# File: scripts/check_phase0.py
# Purpose: Offline Phase 0 gates: matrix rows, fixture presence, secret and locator patterns.
# Related: docs/fork_capability_matrix.json, docs/fixtures/, .github/workflows/ci.yml
# Tags: #phase0 #ci #qa
#
# Why: QA forbade live-fork CI. This script never opens a socket.
# Side effects: none. Exit 1 on failure.

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "fork_capability_matrix.json"
FIXTURES = ROOT / "docs" / "fixtures"
ALLOWED = {"cited", "unknown"}
REQUIRED_IDS = {
    "rpc-endpoints",
    "rpc-transport",
    "network-id",
    "tx-format",
    "address-key-format",
    "script-capabilities",
    "sighash",
    "multisig",
    "timelocks",
    "partial-sign",
    "reorg",
    "test-regtest",
    "bitscrow-digest",
    "fork-consensus-hash",
}
SECRET_PATTERNS = [
    re.compile(r"BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY"),
    re.compile(r"\bxprv[a-zA-Z0-9]{20,}"),
    re.compile(r"rpcpassword\s*=\s*(?!op://)\S+"),
    re.compile(r"BITSCROW_RPC_PASSWORD=(?!op://)\S+"),
]
# Live v3 onions and RFC1918 *hosts* (CIDR like 192.168.0.0/16 is allowed).
ONION_V3 = re.compile(r"\b[a-z2-7]{56}\.onion\b", re.IGNORECASE)
RFC1918_HOST = re.compile(
    r"\b(?:10\.\d{1,3}\.\d{1,3}\.\d{1,3}"
    r"|192\.168\.\d{1,3}\.\d{1,3}"
    r"|172\.(?:1[6-9]|2\d|3[0-1])\.\d{1,3}\.\d{1,3})\b"
)
# Only a real prefix length after '/' is CIDR; "/rpc" or "/16/rpc" are paths.
CIDR_SUFFIX = re.compile(r"^/(?:3[0-2]|[12][0-9]|[0-9])(?!/|\d)")
RPC_URL_ASSIGN = re.compile(
    r"(?m)^[ \t]*(?:export[ \t]+)?BITSCROW_RPC_URL[ \t]*=[ \t]*(\S+)"
)
PLACEHOLDER_HOST = re.compile(r"^<[^>]+>(?:\.onion)?$")
PLACEHOLDER_PORT = re.compile(r"^<[^>]+>$")
SKIP_SUFFIX = {".png", ".jpg", ".woff", ".woff2"}


def is_cidr_suffix(text: str, host_end: int) -> bool:
    """True only for /0../32 at end of token — not a URL path segment."""
    return CIDR_SUFFIX.match(text[host_end:]) is not None


def is_allowed_rpc_url(value: str) -> bool:
    """Placeholders and op:// refs only — never a live host."""
    if value.startswith("op://"):
        return True
    parsed = urlparse(value)
    if parsed.scheme not in ("http", "https") or not parsed.netloc:
        return False
    if ":" in parsed.netloc:
        host, port = parsed.netloc.rsplit(":", 1)
    else:
        host, port = parsed.netloc, None
    if not PLACEHOLDER_HOST.match(host):
        return False
    if port is not None and not PLACEHOLDER_PORT.match(port):
        return False
    return True


def fail(msg: str) -> None:
    print(f"FAIL: {msg}", file=sys.stderr)
    raise SystemExit(1)


def check_matrix() -> None:
    data = json.loads(MATRIX.read_text(encoding="utf-8"))
    if data.get("ci_must_not_call_node") is not True:
        fail("ci_must_not_call_node must be true")
    if data.get("digest_alg_bitscrow") != "blake2b-256":
        fail("digest_alg_bitscrow must be blake2b-256")
    rows = data.get("rows") or []
    ids = {row.get("id") for row in rows}
    missing = REQUIRED_IDS - ids
    if missing:
        fail(f"missing matrix ids: {sorted(missing)}")
    for row in rows:
        status = row.get("status")
        if status not in ALLOWED:
            fail(f"row {row.get('id')}: status {status!r} not in {ALLOWED}")
        if status == "cited" and not row.get("source"):
            fail(f"row {row.get('id')}: cited without source")
        if status == "unknown" and row.get("value") not in (None, "", "unknown"):
            fail(f"row {row.get('id')}: unknown row must not invent a value")


def check_fixtures() -> None:
    for name in (
        "valid_v0.1.yaml",
        "invalid_missing_required.yaml",
        "invalid_dirty.yaml",
        "README.md",
    ):
        path = FIXTURES / name
        if not path.is_file():
            fail(f"missing fixture {path}")
    valid = (FIXTURES / "valid_v0.1.yaml").read_text(encoding="utf-8")
    if "version:" not in valid:
        fail("valid fixture missing version")
    if "blake2b-256" not in valid:
        fail("valid fixture must declare blake2b-256")
    missing = (FIXTURES / "invalid_missing_required.yaml").read_text(encoding="utf-8")
    if re.search(r"(?m)^\s*version:", missing):
        fail("invalid_missing_required.yaml must omit protocol.version")
    dirty = (FIXTURES / "invalid_dirty.yaml").read_text(encoding="utf-8")
    if "<<" not in dirty:
        fail("invalid_dirty.yaml must include a YAML merge key")


def is_gitignored(path: Path) -> bool:
    """Operator .env may hold live locators; CI only cares what git would publish."""
    rel = path.relative_to(ROOT)
    result = subprocess.run(
        ["git", "check-ignore", "-q", "--", str(rel)],
        cwd=ROOT,
        check=False,
    )
    return result.returncode == 0


def check_secrets() -> None:
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(part in {".git", "target"} for part in path.parts):
            continue
        if path.suffix.lower() in SKIP_SUFFIX:
            continue
        if is_gitignored(path):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        if path.name != "check_phase0.py":
            for pat in SECRET_PATTERNS:
                if pat.search(text):
                    fail(f"secret-like pattern {pat.pattern} in {path.relative_to(ROOT)}")
        check_text_locators(path, text)


def check_text_locators(path: Path, text: str) -> None:
    """Fail closed if a live node locator lands in git."""
    rel = path.relative_to(ROOT)
    if ONION_V3.search(text):
        fail(f"live v3 onion locator in {rel}")
    for match in RFC1918_HOST.finditer(text):
        if is_cidr_suffix(text, match.end()):
            continue
        fail(f"RFC1918 host locator in {rel}")
    for match in RPC_URL_ASSIGN.finditer(text):
        value = match.group(1)
        if is_allowed_rpc_url(value):
            continue
        fail(f"live BITSCROW_RPC_URL host in {rel}")


def _self_test() -> None:
    """Inline gates for CIDR vs path and public RPC assignments."""
    lan = ".".join(["192", "168", "1", "20"])
    assert not is_cidr_suffix(f"{lan}/rpc", len(lan))
    cidr_host = ".".join(["10", "0", "0", "0"])
    assert is_cidr_suffix(f"{cidr_host}/16", len(cidr_host))
    path_host = ".".join(["10", "0", "0", "1"])
    assert not is_cidr_suffix(f"{path_host}/16/rpc", len(path_host))
    assert is_allowed_rpc_url("https://<lan-host>:<rpc-port>")
    assert is_allowed_rpc_url("https://<rpc-onion>.onion")
    assert is_allowed_rpc_url("op://vault/item/url")
    assert not is_allowed_rpc_url("https://rpc." + "example.com")
    assert not is_allowed_rpc_url("https://rpc." + "example.com/<project>")
    assert not is_allowed_rpc_url("https://node.example.com:<rpc-port>")
    assert RPC_URL_ASSIGN.search("export BITSCROW_RPC_URL=https://rpc.example.com")


def main() -> int:
    _self_test()
    check_matrix()
    check_fixtures()
    check_secrets()
    print("phase0 checks ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

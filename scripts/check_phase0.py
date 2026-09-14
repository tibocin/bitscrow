# File: scripts/check_phase0.py
# Purpose: Offline Phase 0 gates: matrix rows, fixture presence, secret patterns.
# Related: docs/fork_capability_matrix.json, docs/fixtures/, .github/workflows/ci.yml
# Tags: #phase0 #ci #qa
#
# Why: QA forbade live-fork CI. This script never opens a socket.
# Side effects: none. Exit 1 on failure.

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

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
SKIP_SUFFIX = {".png", ".jpg", ".woff", ".woff2"}


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


def check_secrets() -> None:
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(part in {".git", "target"} for part in path.parts):
            continue
        if path.name == "check_phase0.py":
            continue
        if path.suffix.lower() in SKIP_SUFFIX:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for pat in SECRET_PATTERNS:
            if pat.search(text):
                fail(f"secret-like pattern {pat.pattern} in {path.relative_to(ROOT)}")


def main() -> int:
    check_matrix()
    check_fixtures()
    check_secrets()
    print("phase0 checks ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

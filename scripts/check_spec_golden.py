# File: scripts/check_spec_golden.py
# Purpose: Hash the Python ContractV01 mirror and compare the committed golden hex.
# Related: scripts/spec_model.py, docs/fixtures/valid_v0.1.digest.blake2b-256.hex
# Tags: #spec #golden #crosscheck #phase1
#
# Why: CI should fail if the fixture, the model, or the hex drift apart.
# Side effects: none. Exit 1 on mismatch. No sockets.

"""Second-model check for docs/fixtures/valid_v0.1.yaml."""

from __future__ import annotations

import hashlib
from pathlib import Path

import rfc8785
import yaml

from spec_contract import contract_from_yaml
from spec_model import fail

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "docs" / "fixtures" / "valid_v0.1.yaml"
GOLDEN = ROOT / "docs" / "fixtures" / "valid_v0.1.digest.blake2b-256.hex"


def digest_hex(document: dict) -> str:
    """Unkeyed blake2b-256 over RFC 8785 bytes."""
    canonical = rfc8785.dumps(document)
    return hashlib.blake2b(canonical, digest_size=32).hexdigest()


def main() -> None:
    loaded = yaml.safe_load(FIXTURE.read_text(encoding="utf-8"))
    got = digest_hex(contract_from_yaml(loaded))
    expected = GOLDEN.read_text(encoding="utf-8").strip()
    if got != expected:
        fail(f"golden mismatch got={got} expected={expected}")
    print("spec golden cross-check ok")


if __name__ == "__main__":
    main()

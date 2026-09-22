# File: scripts/spec_model.py
# Purpose: Python mirror of ContractV01 for the valid v0.1 fixture only.
# Related: scripts/check_spec_golden.py, crates/bitscrow-spec/src/model.rs
# Tags: #spec #model #crosscheck
#
# Why: a second reader must name the same fields the Rust struct serializes.
# Side effects: none. fail() exits the process.

"""Field-exact YAML → JSON object. Opaque subtrees stay JSON values."""

from __future__ import annotations

import sys

MAX_SATS = (1 << 53) - 1


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def mapping(node: object, label: str, keys: set[str]) -> dict:
    if not isinstance(node, dict):
        fail(f"{label} must be a mapping")
    if "<<" in node:
        fail(f"{label} contains a YAML merge key")
    extra = set(node) - keys
    missing = keys - set(node)
    if extra or missing:
        fail(f"{label} keys extra={sorted(extra)} missing={sorted(missing)}")
    return node


def take(node: dict, label: str, fields: dict) -> dict:
    body = mapping(node, label, set(fields))
    return {name: conv(body[name], f"{label}.{name}") for name, conv in fields.items()}


def rows(node: object, label: str, fields: dict) -> list:
    if not isinstance(node, list):
        fail(f"{label} must be a list")
    return [take(item, f"{label}[{i}]", fields) for i, item in enumerate(node)]


def json_value(node: object, label: str) -> object:
    if isinstance(node, dict):
        if "<<" in node:
            fail(f"{label} contains a YAML merge key")
        return {str(key): json_value(val, f"{label}.{key}") for key, val in node.items()}
    if isinstance(node, list):
        return [json_value(item, f"{label}[]") for item in node]
    if node is None or isinstance(node, (str, bool)):
        return node
    if isinstance(node, int):
        return u64(node, label)
    fail(f"{label} is not JSON-safe ({type(node).__name__})")
    return None


def u64(node: object, label: str) -> int:
    if isinstance(node, bool) or not isinstance(node, int):
        fail(f"{label} must be an integer")
    if node < 0 or node > MAX_SATS:
        fail(f"{label} exceeds 2^53-1")
    return node


def text(node: object, label: str) -> str:
    if not isinstance(node, str) or node == "":
        fail(f"{label} must be a non-empty string")
    return node


def opt_text(node: object, label: str) -> str | None:
    if node is None:
        return None
    return text(node, label)


def flag(node: object, label: str) -> bool:
    if not isinstance(node, bool):
        fail(f"{label} must be a bool")
    return node


def strings(node: object, label: str) -> list:
    if not isinstance(node, list) or not all(isinstance(item, str) and item for item in node):
        fail(f"{label} must be a list of strings")
    return node


def amount(node: object, label: str) -> int | str:
    if isinstance(node, str):
        return text(node, label)
    return u64(node, label)


def null_secret(node: object, label: str) -> None:
    if node is not None:
        fail(f"{label} must be null")
    return None

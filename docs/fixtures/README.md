<!--
File: docs/fixtures/README.md
Purpose: Offline authoring stubs for Phase 0/1. Never executed against the fork node.
Related: valid_v0.1.yaml, invalid_missing_required.yaml, invalid_dirty.yaml
Tags: #fixtures #phase0 #qa
-->

# Authoring fixtures

Stubs only. CI and `bitscrow-spec` tests must not open a socket.

| File | Role |
|------|------|
| `valid_v0.1.yaml` | Filled marketplace example. Placeholders, not keys. |
| `invalid_missing_required.yaml` | Drops `protocol.version`. |
| `invalid_dirty.yaml` | YAML merge key — parser must reject. |

`docs/contract_template.yaml` stays the commented authoring sketch. Do not hash it as-is.

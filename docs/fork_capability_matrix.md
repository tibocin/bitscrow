<!--
File: docs/fork_capability_matrix.md
Purpose: Phase 0 Blake2b-fork audit: cited facts vs explicit unknowns.
Related: fork_capability_matrix.json, product_roadmap.md, system_architecture.md
Tags: #phase0 #fork #rpc #bitscrow
-->

# Phase 0 — fork capability matrix

Machine-readable rows: [`fork_capability_matrix.json`](fork_capability_matrix.json).
CI (`scripts/check_phase0.py`) rejects a row that is neither `cited` nor `unknown`.

**work_id:** `tkt:bitscrow-p0-matrix`. **Do not** call the node from CI.
**Do not** assume Bitcoin Core Taproot, PSBT, or script semantics.

## Operator answers (2026-09-14)

| Question | Answer | How we treat it |
|----------|--------|-----------------|
| Node location | Tor onion (see JSON) or LAN `https://192.168.4.177:57713` | Cited endpoints. Not a network magic number. |
| Contract-state hash | “use blake2b” | BITSCROW `digest_alg` is `blake2b-256` (unkeyed, 32 bytes). Fork tx/block hash still `unknown`. |
| Existing CI/regtest fixture | no | Offline fixtures only. |

## Unauthenticated probe (LAN, 2026-09-14T07:09:37Z)

- GET `/` → `JSONRPC server handles only POST requests`
- HTTP POST → `307` to HTTPS
- HTTPS POST without credentials → `401` + `WWW-Authenticate: Basic realm="jsonrpc"`
- TLS 1.3; cert `CN=localhost, O=Start9, OU=StartOS` (StartOS local CA)

RPC username/password stay in env. They are not in git. Remaining fork rows stay `unknown` until an authenticated `help` / `getnetworkinfo` is run **by a human**, not by CI.

## Reading a cell

- `cited` — source is this probe, this repo, or the operator message in the DEVAO chalkboard.
- `unknown` — must not be filled with Bitcoin Core lore.

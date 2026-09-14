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
Live RPC hosts are locators: placeholders in git, real URLs in `.env` / 1Password.

## Operator answers (2026-09-14)

| Question | Answer | How we treat it |
|----------|--------|-----------------|
| Node location | Operator LAN or Tor (placeholders in JSON) | Locator class. Live hosts stay gitignored / vault-only. |
| Contract-state hash | “use blake2b” | BITSCROW `digest_alg` is `blake2b-256` (unkeyed, 32 bytes). Fork **block** hash is the same primitive, different construction (see JSON). |
| Existing CI/regtest fixture | no | Offline fixtures only. |

## Unauthenticated probe (operator LAN host redacted, 2026-09-14T07:09:37Z)

- GET `/` → `JSONRPC server handles only POST requests`
- HTTP POST → `307` to HTTPS
- HTTPS POST without credentials → `401` + `WWW-Authenticate: Basic realm="jsonrpc"`
- TLS 1.3; cert `CN=localhost, O=Start9, OU=StartOS` (StartOS local CA)

RPC username/password stay in env. They are not in git. Remaining fork rows stay `unknown` until an authenticated `help` / `getnetworkinfo` is run **by a human**, not by CI.

## Fork hash variant (public Knots 29.4.1, 2026-09-14)

Do not say “the fork uses blake2b” and stop. The Aug/Sep 2026 chain (first BLAKE2b block **961640**, 30 August 2026) uses:

- **Txid / wtxid:** SHA256d, unchanged (`HashWriter::GetHash` on the transaction).
- **Block hash from 961640:** unkeyed **BLAKE2b-256** (`blake2b_nokey`, `outlen=32`). Two passes. The ASIC sees an **80-byte** work header, not a single hash of the **164-byte** header-v2 blob. Commitments into that work use tagged SHA256 (`Bitcoin block header 1`, etc.).
- **Not:** blake2s, blake2b-512, keyed BLAKE2, or Sia `personal` parameter.
- BITSCROW `digest_alg=blake2b-256` is the **same primitive**, a **different message** (canonical contract state).

Sources: `bitcoinknots/bitcoin` tag `v29.4.1.knots20260508` `src/primitives/block.cpp`, `src/crypto/blake2b.h`; Knots PR 385 (independent recompute of 961640); [bitcoin-blake2b.org/developers](https://bitcoin-blake2b.org/developers).

## Reading a cell

- `cited` — source is this probe, this repo, or the operator message in the DEVAO chalkboard.
- `unknown` — must not be filled with Bitcoin Core lore.

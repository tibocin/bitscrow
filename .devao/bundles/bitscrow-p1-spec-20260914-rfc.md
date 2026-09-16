<!--
File: .devao/bundles/bitscrow-p1-spec-20260914-rfc.md
Purpose: Architecture RFC for bitscrow-spec Phase 1 (planning artifact).
Related: bitscrow docs/system_architecture.md, contract_template.yaml
Tags: #rfc #bitscrow #phase1 #spec
-->

# RFC — bitscrow-spec (Phase 1)

Session: `bitscrow-p1-spec-20260914`. Pack: bitscrow. Digi off. `target_system=git`.
No crate ChangeIntent until human go.

## Problem

Phase 1 needs one reversible library that turns YAML authoring into a hashable
machine document. Raw YAML hashing is forbidden. Phase 0 already cites
contract-state digest as unkeyed **blake2b-256** (32 bytes; same primitive as
fork PoW, different message). `canon_profile: bitscrow-jcs-v0` is named and
not implemented. Fork script/PSBT/Taproot stay `unknown`.

## Options considered

**Crate scope**

1. `bitscrow-spec` only — YAML → typed v0.1 → canonicalize → blake2b-256.
2. Spec + `bitscrow-state` now — premature; `when` clauses are opaque and chain
   rows are unknown.
3. Full architecture §12 workspace — too wide; reject.

**Canonicalization**

1. `bitscrow-jcs-v0` = RFC 8785 JCS over UTF-8 JSON of the validated typed model.
2. RFC 8949 deterministic CBOR — would rename the cited profile.
3. YAML text/c14n — non-deterministic; forbidden.

## Chosen

Crate-1 + canon-1.

```text
YAML 1.2 → strict parse (reject merge keys / unknown fields)
  → ContractV01 (addresses/pubkeys opaque; no private-key types)
  → bitscrow-jcs-v0 normalize → RFC 8785 JCS UTF-8
  → unkeyed blake2b-256 (outlen=32)
```

## Why this wins

Matches the named profile and cited digest, one crate that is easy to delete,
fail-closed on dirty YAML, and leaves chain/script/state reversible. CBOR
forks the template name. Raw YAML is not a digest.

## API boundary

| | |
|--|--|
| In | UTF-8 YAML 1.2; optional already-typed `ContractV01` |
| Out | `ContractV01`; JCS `CanonicalBytes`; `Digest32` (`[u8; 32]`); typed `SpecError` |
| Surface | `parse_yaml` → `canonicalize(BitscrowJcsV0)` → `digest(Blake2b256)` |
| Reject | merge keys, missing required, wrong `digest_alg`/`canon_profile`, sats outside `[0, 2^53-1]`, non-null `plaintext_secret`, any socket/RPC |
| Not this crate | HEAD, transitions, RPC, tx/script/PSBT, oracles, evidence blobs, portal, private keys |

Do not hash `docs/contract_template.yaml`. Fixtures: `valid_v0.1.yaml` stable
hash; `invalid_missing_required.yaml` and `invalid_dirty.yaml` fail.

## Deferred

§12 workspace; `bitscrow-state` / `bitscrow-chain` / tx / signer / oracle /
evidence / portal; `tkt:bitscrow-p0-auth-rpc`; network-id; outcome interpreter;
production funds.

## Security zones

Public spec yes. Opaque strings (addresses, pubkeys, `when`) uninterpreted.
Signer-sovereign keys never. Locator-private onion/LAN RPC never (placeholders
only). Chain adapter never.

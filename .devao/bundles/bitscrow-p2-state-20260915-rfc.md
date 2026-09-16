<!--
File: .devao/bundles/bitscrow-p2-state-20260915-rfc.md
Purpose: Architecture RFC for bitscrow-state Phase 2 (planning artifact).
Related: docs/system_architecture.md §4, crates/bitscrow-spec, bitscrow-p1-spec RFC
Tags: #rfc #bitscrow #phase2 #state
-->

# RFC — bitscrow-state (Phase 2)

Session: `bitscrow-p2-state-20260915`. Pack: bitscrow. Digi off. `target_system=git`.
No crate ChangeIntent until human go. Precondition: land `tkt:bitscrow-spec-crate`.

## Problem

Phase 2 needs an offline lifecycle FSM that advances contract operational state
with deterministic HEAD hashes and predecessor links, without a chain adapter,
keys, or live RPC. Phase 1 already provides `ContractV01` + `bitscrow-jcs-v0` +
unkeyed blake2b-256. Architecture §4 names happy-path and exceptional statuses.
`outcome.when` and fork script/PSBT remain opaque / unknown.

## Options considered

**Crate scope**

1. New `bitscrow-state` depending on `bitscrow-spec` — pure FSM + hash HEAD.
2. Fold FSM into `bitscrow-spec` — mixes authoring validation with runtime.
3. Start `bitscrow-chain` stubs for FUNDING/ACTIVE — invents fork semantics; reject.

**Hashing**

1. Hash a dedicated `StateRevision` record via existing JCS + blake2b-256.
2. Re-hash entire `ContractV01` each transition — conflates genesis terms with HEAD.
3. Hash raw YAML / ad-hoc JSON — non-deterministic; forbidden.

**Funding without chain**

1. Stub events (`FundingStubObserved`, `ActivationStubGranted`, …) with opaque refs.
2. Defer FUNDING/ACTIVE/SETTLING until Phase 3 — leaves half of §4 untested.
3. Fake PSBT/script validation — assumes Core semantics on Blake2b fork; reject.

## Chosen

Crate-1 + hash-1 + stub-1.

```text
ContractV01 (immutable genesis terms)
  + TransitionEvent (typed) + caller Clock
  → apply guards (fail closed)
  → TransitionRecord (prev_hash, event, actor, evidence refs, ts, optional chain_hint)
  → StateRevision { status, contract_id, genesis_digest, head fields… }
  → bitscrow-jcs-v0 → unkeyed blake2b-256 → Digest32 HEAD
```

Happy path (§4):
`DRAFT → PROPOSED → ACCEPTED → FUNDING → ACTIVE → VERIFYING → SETTLING → CLOSED`

Exceptional: `DISPUTED`, `ORACLE_FALLBACK`, `EXPIRED`, `RECOVERY`, `FAILED_SAFE`.
Oracle / evidence values stay fixture enums; never HTTP. `when` never evaluated.

## Why this wins

Matches roadmap Phase 2, reuses Phase 1 canon/digest, keeps chain reversible,
exercises full §4 offline via stubs, and fails closed without inventing payouts
or fork PSBT. Folding into spec couples authoring to runtime. Deferring funding
edges leaves the matrix hollow.

## API boundary

| | |
|--|--|
| In | `ContractV01`; `TransitionEvent`; `now: DateTime`; optional expected prev `Digest32` |
| Out | `StateHead` / status; `TransitionRecord`; `Digest32`; typed `StateError` |
| Surface | `genesis` → `apply` → `allowed_edges` → `verify_chain` |
| Reject | illegal edge; wrong prev hash; unknown actor; missing required evidence id; clock rewind; inventing allocations |
| Opaque | `when`, addresses, pubkeys, outpoints, oracle payloads, `chain_hint`, commitment plaintext |
| Never | RPC, tx/PSBT/script, private keys, portal, evaluating `when` as code |

## Deferred

`bitscrow-chain` / tx / signer; live oracles; evidence blobs; portal; Digi;
production funds; second-impl golden cross-check (`story:state-hash-crosscheck`);
full `RECOVERY` quorum protocol (v0.1 = stub-entry-only per e18).

## Locked decisions (e17–e19)

- Events: hybrid typed enum, lifecycle-generic names; Actor enum.
- Recovery: stub-entry-only; reject fund-moving stubs.
- Second-impl hash: deferred to story.

## Security zones

Public state machine yes. Signer-sovereign keys never. Locator-private RPC never
(placeholders only). Chain adapter never this phase.

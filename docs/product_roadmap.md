# BITSCROW Product Development Roadmap

## Phase 0 — Fork Capability Audit

Before implementing Bitcoin-specific assumptions, document the Bitcoin Blake2b fork's node RPC, transaction format, address/key formats, script capabilities, signature hashing, multisig support, timelocks, partial-signing support, reorg behavior, and test/regtest environment.

Deliverable: a compatibility matrix and Rust test vectors.

Status (2026-09-14, `tkt:bitscrow-p0-matrix`): matrix lives in `docs/fork_capability_matrix.md`. LAN/Tor RPC exist (hosts redacted). BITSCROW digest and fork PoW are both unkeyed blake2b-256 (different messages). Most other chain semantics remain `unknown`. No CI/regtest fixture.

## Phase 1 — Open Contract Specification

Define BITSCROW v0.1 with canonical serialization, schema validation, participants, roles, contributions, refund destinations, outcomes, expiration, chain anchors, oracle policies, evidence hashes, signatures, and state references.

YAML is the authoring format; canonical machine hashing should use a deterministic representation rather than hashing arbitrary YAML text.

## Phase 2 — Rust State Machine

Implement `bitscrow-spec` and `bitscrow-state` crates. No real funds. Exhaustively test valid/invalid transitions, expiration, fallback behavior, and deterministic state hashes.

## Phase 3 — Chain + Transaction Prototype

Implement the Rust chain adapter against the Blake2b fork test environment. Construct funding and settlement transactions, verify outpoints, calculate fees, and implement the safest supported partial-signing/multisig mechanism.

## Phase 4 — Marketplace MVP

Build the portal for a two-party physical-goods contract. Support account creation, contract proposal/acceptance, per-party funding, shipment evidence, delivery verification, buyer receipt confirmation, collateral return, settlement, expiration, and final closure.

## Phase 5 — Human Agreement + Evidence

Generate a human-readable agreement after acceptance. Include participant signatures, canonical contract hash, chain anchor, portal link, and downloadable archive. Add encrypted evidence uploads with content hashes.

## Phase 6 — Oracle Framework

Create typed oracle adapters with authenticated observations, confidence/status metadata, retry rules, secondary sources, pre-approved fallback workflows, arbitration hooks, and complete-failure outcomes.

## Phase 7 — Agent-Assisted Builder

Allow parties to negotiate jointly or privately with an AI agent. The agent converts conversation and uploaded documents into proposed structured terms. Participants must review and sign the canonical contract before funding.

Agents may propose logic; they may not invent settlement authority after activation.

## Phase 8 — Portable State + Recovery

Define genesis and HEAD records, predecessor-linked state history, QR recovery payloads, protocol/interpreter version pinning, and optional content-addressed replication.

## Phase 9 — Decentralized Execution

Package deterministic workers as reproducible containers. Explore IPFS-compatible content addressing and Akash execution. Participants can fund compute/storage budgets in sats through the portal while infrastructure adapters handle external provider payment mechanics.

The protocol must remain executable without Akash or any single storage provider.

## Phase 10 — General Contract Modules

Generalize outcomes and roles beyond buyer/seller. Add reviewed templates for freelance milestones, traditional escrow, wagers, insurance-like triggers, and multi-party agreements.

Each new template receives threat modeling and adversarial state-machine tests.

## Phase 11 — Developer API and SDK

Publish versioned APIs and Rust SDKs for contract creation, validation, state observation, evidence submission, oracle integration, transaction construction, and settlement status.

## Phase 12 — Hardening

Independent cryptographic/security review, fuzzing, property-based testing, fork/reorg simulations, key-loss recovery testing, oracle manipulation testing, dependency auditing, reproducible builds, operational runbooks, and staged limits before production funds.

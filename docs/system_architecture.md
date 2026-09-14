# BITSCROW System Architecture

## 1. Purpose

BITSCROW is a protocol and reference application for programmable escrow and conditional settlement on the Bitcoin Blake2b fork. It separates **human agreement**, **machine-readable state**, **verification**, and **fund settlement** so each layer can evolve without giving an AI agent unilateral custody of funds.

The reference implementation is Rust.

## 2. Design Principles

1. **Deterministic settlement** — agents may help negotiate and interpret evidence, but spend authorization must resolve through explicit protocol rules.
2. **UTXO-first** — model funding and settlement as identifiable inputs and outputs.
3. **No hidden custody assumptions** — every contract declares keys, signing thresholds, refund destinations, timeouts, and recovery paths.
4. **Fail closed** — unavailable oracles and infrastructure must transition to declared fallback states, never improvise a payout.
5. **Portable specification** — contract state is serializable, hashable, versioned, and independently verifiable.
6. **Privacy by design** — use fresh settlement/refund addresses and keep documents/evidence off-chain where possible.
7. **Protocol/application separation** — the open BITSCROW state machine is distinct from the hosted portal.

## 3. Major Components

### Portal

Provides accounts, contract creation, funding status, required actions, signatures, evidence submission, notifications, and downloadable human-readable agreements.

### Builder Agent

Facilitates negotiation and converts agreed terms into a candidate BITSCROW contract. It may parse documents and media, but generated terms require participant approval before activation.

### Protocol Engine

A Rust library that validates contracts, canonicalizes state, calculates hashes, enforces state transitions, evaluates expiration/fallback rules, and produces settlement intents.

### Chain Adapter

Connects the protocol engine to the Bitcoin Blake2b fork node/RPC layer. Chain-specific transaction serialization, address rules, sighash behavior, script capabilities, block validation, and fee estimation belong behind this boundary.

Do not assume Bitcoin Core Taproot/PSBT semantics are identical on the Blake2b fork. Compatibility must be verified against the fork implementation before relying on a feature.

### Transaction/Signing Engine

Constructs unsigned or partially signed transactions according to the fork's supported transaction format. Private keys should not be exposed to agents. Signing occurs through a narrowly scoped signer interface.

### Oracle Service

Observes external facts such as shipment delivery, milestone approval, event outcomes, or data feeds. Each oracle definition includes validation rules, availability policy, and ordered fallbacks.

### Evidence Store

Stores encrypted documents/media off-chain. The contract records content hashes, metadata, submitter signatures, and optional content-addressed references. Hashes prove which evidence was considered without placing private material on-chain.

### State Store

Maintains the operational HEAD state and append-only transition history. Each state revision references its predecessor so history can be independently checked.

### Execution Workers

Optional isolated Rust workers/containers evaluate workflows, oracle adapters, or evidence-processing tasks. Later deployments may use decentralized compute such as Akash, but correctness must not depend on a particular hosting provider.

## 4. Contract Lifecycle

`DRAFT -> PROPOSED -> ACCEPTED -> FUNDING -> ACTIVE -> VERIFYING -> SETTLING -> CLOSED`

Exceptional states include:

`DISPUTED`, `ORACLE_FALLBACK`, `EXPIRED`, `RECOVERY`, and `FAILED_SAFE`.

Every transition must identify the previous state hash, event, actor/oracle, evidence references, resulting state, timestamp, and relevant chain anchor.

## 5. Chain Anchoring

Contract activation and important state checkpoints can record:

- network/fork identifier
- block height
- block hash
- optional transaction IDs/outpoints

Block hashes must be validated against the configured node and handled safely across reorganizations. A block anchor is evidence of chain context, not by itself proof of wall-clock time.

## 6. Funding and Settlement

Each participant can declare:

- required contribution
- collateral contribution
- settlement destination
- refund address
- public key/signing identity

A contract may define multiple payout outcomes rather than hard-coding buyer/seller semantics. Settlement is expressed as allocations over contract-controlled value.

Example outcomes include successful completion, participant default, mutual cancellation, oracle-confirmed event, expiration, arbitration, and catastrophic oracle failure.

## 7. Closure

A contract is CLOSED only after its selected terminal outcome has been executed or otherwise cryptographically acknowledged.

For the marketplace MVP, successful closure can require:

1. seller shipment evidence;
2. delivery verification;
3. buyer receipt confirmation via authenticated action or precommitted secret;
4. seller payment;
5. return of buyer over-collateralization;
6. return of seller collateral as specified;
7. final state hash and settlement transaction reference.

Secrets should be represented by cryptographic commitments/hashes, not plaintext passwords in the contract.

## 8. Oracle Failure Model

Every oracle condition should declare an ordered policy:

1. primary oracle;
2. secondary oracle(s);
3. pre-approved deterministic query/search instructions where appropriate;
4. participant confirmation or portal-assisted review;
5. optional arbitrator/quorum;
6. expiration/final failure outcome.

An AI search result should not automatically become authoritative simply because the primary oracle failed. Any fallback capable of moving funds must be explicitly agreed to in the contract.

## 9. Human-Readable Agreement

After acceptance, the Builder produces a signed agreement containing the human terms, contract ID, protocol version, contract-state hash, chain anchor, participant signing identities, portal URI, and optionally a QR recovery payload.

The document is evidence of agreement; the canonical machine-readable contract determines protocol execution. The document should explicitly identify the canonical contract hash.

## 10. HEAD and Recovery

The original agreement cannot contain a mutable content hash that magically points to future HEAD state. Instead it should contain a stable contract identifier and immutable genesis-state hash plus a resolver mechanism.

A resolver can return the current signed HEAD record:

`contract_id -> head_state_hash`

Every HEAD record links backward to its predecessor. Future versions may replicate these records across content-addressed storage, multiple relays, or decentralized infrastructure.

## 11. Security Boundaries

- AI agents do not receive raw private keys.
- Signing services accept constrained transaction/signing requests.
- Contract parsing uses strict schemas and canonical serialization.
- External documents are untrusted input.
- Oracle responses are authenticated and replay-resistant where possible.
- State transitions are idempotent.
- Settlement construction is reproducible from canonical state.
- Production activation requires fork-specific transaction and cryptographic test vectors.

## 12. Initial Rust Workspace

A likely workspace decomposition:

```text
crates/
  bitscrow-spec/
  bitscrow-state/
  bitscrow-chain/
  bitscrow-tx/
  bitscrow-signer/
  bitscrow-oracle/
  bitscrow-evidence/
  bitscrow-api/
apps/
  portal-api/
  worker/
docs/
examples/
```

The first engineering milestone should run entirely on the fork's test environment/regtest equivalent with simulated oracles and no production funds.

# BITSCROW

BITSCROW is an agent-assisted escrow and conditional settlement protocol targeting the **Bitcoin Blake2b fork**.

The project is designed around a simple idea: parties should be able to define an agreement in human terms, commit funds and collateral, satisfy verifiable conditions, and settle according to deterministic rules without requiring a traditional escrow officer to manually administer every step.

The reference implementation is written in **Rust**.

## Product Vision

BITSCROW combines human-readable agreements with machine-readable contract state, UTXO-based settlement, partial transaction signing workflows, per-party refund addresses, over-collateralized incentives, oracle verification with explicit fallback paths, cryptographic evidence references, expiration, failure handling, versioned state, and portable recovery metadata.

The protocol should be general enough to support marketplace escrow, freelance milestones, wagers, insurance-like conditional payouts, traditional escrow, multi-party agreements, and agent-mediated negotiation.

## MVP

The first use case is a two-party physical-goods transaction:

1. Buyer and seller agree on price, collateral, verification, expiration, and failure rules.
2. Both parties fund the contract.
3. Seller provides shipment tracking information.
4. An agreed oracle or verification method observes delivery.
5. Buyer may explicitly confirm receipt using a secret or authenticated portal action.
6. Settlement pays the seller and returns the appropriate collateral to each party.
7. Expiration and oracle-failure branches protect both parties when the happy path cannot complete.

## Protocol Documents

- `docs/system_architecture.md`
- `docs/product_roadmap.md`
- `docs/contract_template.yaml`
- `docs/fork_capability_matrix.md` — Phase 0 cited vs unknown
- `docs/fixtures/` — offline authoring stubs (never hit the node)

## Implementation Direction

The core protocol and settlement engine should be implemented in Rust with deterministic behavior, explicit state transitions, strong typing, testability, fault tolerance, and conservative key handling.

The specification should remain open and portable so independent implementations can validate compatible BITSCROW contracts.

## Build with DEVAO

BITSCROW is built through the sibling **DEVAO** shop (council + agent-os), not as a one-off chat.

1. Open `devao` and `bitscrow` in the same Cursor workspace.
2. Say **“Use the BITSCROW pack.”**
3. Paste the prompt in [`docs/devao-kickoff.md`](docs/devao-kickoff.md) and run `/run-council`
   with `--target-root` pointed at this checkout.
4. Confirm the plan before expanding past the current milestone.

Council runtime lives in this repo (`.devao/` gitignored; tracked briefs under
`docs/devao/sessions/`). Contracts stay in the sibling DEVAO shop.

Agent contract: [`AGENTS.md`](AGENTS.md).

## Status

Phase 0 audit is in-tree (`tkt:bitscrow-p0-matrix`). Phase 1 `bitscrow-spec`
(`tkt:bitscrow-spec-crate`) parses offline fixtures, applies `bitscrow-jcs-v0`
(RFC 8785), and digests with unkeyed **blake2b-256** (32 bytes). Phase 2
`bitscrow-state` (`tkt:bitscrow-state-crate`) is an offline lifecycle FSM with
stub funding/settle events and deterministic state hashes (no chain adapter).
Fork **block** hash from height 961640 is the same primitive over Knots header-v2
work (different message). Network id and script/tx semantics stay **unknown**
until authenticated RPC. CI does not call the node. Do not use with production
funds.

```bash
cargo test -p bitscrow-spec
cargo test -p bitscrow-state
uv run python scripts/check_phase0.py
```


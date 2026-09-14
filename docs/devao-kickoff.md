<!--
File: docs/devao-kickoff.md
Purpose: Paste-ready prompt to start BITSCROW with the DEVAO council.
Related: ../AGENTS.md, ../README.md, sibling devao/packs/bitscrow/KICKOFF.md
Tags: #devao #kickoff #council #bitscrow
-->

# Start BITSCROW with DEVAO

Canonical copy of this prompt lives in the sibling DEVAO pack:
`packs/bitscrow/KICKOFF.md`. Keep the two in sync.

Copy the block into Cursor chat with **both** repos open. Then `/run-council`.

```text
Use the BITSCROW pack. Point at packs/bitscrow/AGENTS.md.
Do not use the Dossierium pack. Digi is off. target_system=git.

I want to build BITSCROW with DEVAO. Run /run-council first.
Do not implement yet. Stop after a consolidated plan and wait for my go.

Seed:
  session_id: bitscrow-kickoff-20260914
  --pack bitscrow
  --goal "Plan BITSCROW Phase 0+1: Blake2b fork capability matrix + v0.1 spec; no production funds"

Product (do not invent beyond these docs):
  - bitscrow/README.md
  - bitscrow/docs/system_architecture.md
  - bitscrow/docs/product_roadmap.md
  - bitscrow/docs/contract_template.yaml

Facts already in those docs:
  - Agent-assisted escrow / conditional settlement on the Bitcoin Blake2b fork
  - Rust reference implementation; YAML authoring; canonical hashing not raw YAML
  - MVP later: two-party physical-goods marketplace escrow
  - Repo today is design-only (no crates, no portal, no chain adapter)
  - Agents never receive private keys; fail closed; UTXO-first
  - Do not assume Bitcoin Core Taproot/PSBT/script semantics match this fork
  - Privacy-first locators: never paste live onion/LAN RPC hosts (placeholders / op:// only)

First slice to plan (pick one, recommend safest that still moves spec):
  A. Phase 0 only — fork capability matrix + Rust test-vector stubs, no crates
  B. Phase 0 + Phase 1 — audit notes plus bitscrow-spec crate (parse YAML, canonicalize, hash)
  C. Full workspace skeleton (all crates in architecture §12) — too wide; reject unless I expand scope

Non-goals for this council pass:
  - Portal UI, accounts, notifications
  - Real/mainnet funds, production keys, hosted oracles
  - Phase 3+ chain adapter, PSBT-like signing, Akash, IPFS
  - Dossierium / Digi / Marketing OS assumptions

Council must produce:
  - Product: MVP in/out, acceptance, non-goals
  - Architecture: short RFC (spec crate boundaries, canonicalization, hash algorithm choice as an open risk if unaudited)
  - DevOps/Security: no secrets or live node locators in git; signer isolation; env = test/regtest only
  - QA: how we will test spec validation and state hashes without a live fork
  - work_ids (tkt:/story:) for the first implementation ticket after I say go
  - PromptEnvelope paths for the next owners
  - Chalkboard handoffs with model_class (think for RFC, build only after my go)

Open questions to record, not guess:
  1. Node RPC transport (LAN vs Tor) — record with placeholders, never a live host. Network identifier still unknown until a human probe.
  2. Which hash (blake2b-256 vs other) is consensus-canonical for contract state?
  3. Is there an existing regtest/test fixture we can point CI at?

Tracking: chalkboard session_id above; catalog work_id tkt:bitscrow-pack for this pack; later implementation tickets live in bitscrow PRs with the same hierarchy once the plan exists.
```

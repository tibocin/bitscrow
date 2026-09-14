<!--
File: AGENTS.md
Purpose: How DEVAO agents should treat this BITSCROW product repo.
Related: README.md, docs/devao-kickoff.md, sibling devao packs/bitscrow/
Tags: #agents #bitscrow #devao
-->

# BITSCROW — agent contract

This is the **product repo**. The shop is the sibling **`devao`** standards spine.

## Start here

1. Open `devao` and `bitscrow` in the same Cursor workspace.
2. Tell the agent: **“Use the BITSCROW pack.”**
3. Paste `docs/devao-kickoff.md` (same text as `devao/packs/bitscrow/KICKOFF.md`).
4. Run `/run-council`. Do not scaffold crates until the human confirms the plan.

Canonical pack: `devao/packs/bitscrow/AGENTS.md`. Product facts live in this repo’s
`README.md` and `docs/`. Digi and Dossierium stay off.

## Hard rules

- Rust reference implementation. No production funds.
- Agents never receive private keys.
- Do not assume Bitcoin Core Taproot/PSBT semantics on the Blake2b fork.
- First slice is Phase 0 (fork audit) and Phase 1 (v0.1 spec), not the portal.

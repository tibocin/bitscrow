<!-- File: docs/devao/sessions/bitscrow-p1-spec-20260914.md -->
<!-- Purpose: Tracked council brief for session bitscrow-p1-spec-20260914. -->
<!-- Related: .devao/chalkboard (gitignored), SessionBoard -->
<!-- Tags: #devao #session #brief -->

# Session `bitscrow-p1-spec-20260914`

- **Target:** `bitscrow`
- **Goal:** Ship BITSCROW Phase 1 v0.1 bitscrow-spec after Phase 0 matrix; no production funds
- **Current owner:** human
- **Pack:** `bitscrow`
- **work_id refs:** epic:bitscrow-p1-spec, ms:bitscrow-spec, story:v0.1-canonical-spec, tkt:bitscrow-spec-crate

## Acceptance

- tkt:bitscrow-spec-crate CP1–CP7: **PASS** (QA e33/e34). Evidence: `cargo test -p bitscrow-spec --offline`, `uv run python scripts/check_phase0.py`.
- Verifier: **pass with follow-ups** (e37): commit/PR by human; catalog seed; golden/second-impl deferred to story.

## Decisions

- HUMAN GO e27; JCS pin e28; crate + CI job `spec` landed; Digi off; `target_system=git`.

## Next

- goal=human commit + PR naming work_id=tkt:bitscrow-spec-crate; decisions=verifier pass_with_followups; artifacts=crates/bitscrow-spec; open_risks=catalog unseeded; next_owner=human; model_class=ops

## Blockers

- None.

<!-- File: docs/devao/sessions/bitscrow-spec-crate-20260921.md -->
<!-- Purpose: Tracked council brief for session bitscrow-spec-crate-20260921. -->
<!-- Related: .devao/chalkboard (gitignored), SessionBoard -->
<!-- Tags: #devao #session #brief -->

# Session `bitscrow-spec-crate-20260921`

- **Target:** `bitscrow`
- **Goal:** Plan remaining work on tkt:bitscrow-spec-crate after the crate landed on main
- **Current owner:** human
- **Pack:** `None`
- **work_id refs:** tkt:bitscrow-spec-crate

## Acceptance

- Golden pin: digest_yaml(docs/fixtures/valid_v0.1.yaml) hex equals docs/fixtures/valid_v0.1.digest.blake2b-256.hex (64 lowercase hex). CP2-CP7 stay green. cargo test -p bitscrow-spec --offline. Fixture YAML unchanged unless a human re-baselines.

## Decisions

- goal=plan remaining tkt:bitscrow-spec-crate; decisions=crate already on main (PR #3) with CP1-CP7; remaining slice is golden digest pin plus catalog question; no portal, funds, keys, or chain adapter; Digi off; target_system=git; open_risks=Jev advisory revise, catalog unseeded, golden hex not committed; next_owner=product_manager; model_class=think
- goal=golden hex pin only; decisions=sibling fixture not rust-only const; independent recompute before commit; second-impl and catalog seed deferred; Digi off; target_system=git; open_risks=single-impl pin can freeze a bug; Jev advisory revise; next_owner=human; model_class=think

## Next

- goal=PR for golden pin and Python cross-check on tkt:bitscrow-spec-crate; decisions=hex fixture plus scripts/check_spec_golden.py; open_risks=catalog unseeded, shared schema later; next_owner=human; model_class=ops

## Blockers

- None.

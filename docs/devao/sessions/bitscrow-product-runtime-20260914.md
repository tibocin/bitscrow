<!-- File: docs/devao/sessions/bitscrow-product-runtime-20260914.md -->
<!-- Purpose: Tracked council brief for session bitscrow-product-runtime-20260914. -->
<!-- Related: .devao/chalkboard (gitignored), SessionBoard -->
<!-- Tags: #devao #session #brief -->

# Session `bitscrow-product-runtime-20260914`

- **Target:** `bitscrow`
- **Goal:** Prove DEVAO dual-root runtime on BITSCROW: seed --target-root, SessionBoard, tracked brief; no crates
- **Current owner:** human
- **Pack:** `bitscrow`
- **work_id refs:** tkt:split-runtime-roots, story:product-runtime

## Acceptance

- seed --target-root writes bitscrow .devao chalkboard+graph+brief; active_pack=bitscrow; live JSON gitignored; agent-graph.json and docs/devao/sessions/*.md tracked; board HTML now/done/next; loopback only. Spine pytest 42 passed.

## Decisions

- Adopt dual-root on BITSCROW: spine=devao contracts; target=bitscrow checkout runtime. Copy product-runtime gitignore.snippet. Do not vendor packages/agent-os. No spec crate this session. work_ids: tkt:split-runtime-roots story:product-runtime.

## Next

- goal=human reviews commits on both branches; decisions=dual-root adopted; p1 session home=bitscrow; artifacts=docs/devao/sessions/ .devao/bundles/bitscrow-p1-spec-20260914-rfc.md; open_risks=shop p1 chalkboard stale; next_owner=human; model_class=think

## Blockers

- None.

<!-- File: docs/devao/sessions/bitscrow-p1-spec-20260914.md -->
<!-- Purpose: Tracked council brief for session bitscrow-p1-spec-20260914. -->
<!-- Related: .devao/chalkboard (gitignored), SessionBoard -->
<!-- Tags: #devao #session #brief -->

# Session `bitscrow-p1-spec-20260914`

- **Target:** `bitscrow`
- **Goal:** Plan BITSCROW Phase 1 v0.1 spec crate after Phase 0 matrix; no production funds
- **Current owner:** human
- **Pack:** `bitscrow`
- **work_id refs:** tkt:bitscrow-p0-auth-rpc, epic:bitscrow-p1-spec, ms:bitscrow-spec, story:v0, tkt:bitscrow-spec-crate, tkt:bitscrow-p0-matrix, epic:bitscrow-onboard

## Acceptance

- tkt:bitscrow-spec-crate: valid_v0.1.yaml yields a 32-byte unkeyed blake2b-256 digest; twice identical. Semantically equal YAML that differs only in insignificant whitespace/key order matches after canonicalize. invalid_missing_required.yaml (no protocol.version) rejected. invalid_dirty.yaml (YAML merge key) rejected fail-closed. Zero sockets. No live locators in crate/tests/docs from this ticket. story:v0.1-canonical-spec: independent implementations hash the same marketplace_escrow document the same way.
- Critical path tkt:bitscrow-spec-crate after human go: CP1 valid_v0.1.yaml Digest32 32-byte unkeyed blake2b-256; CP2 twice identical; CP3 semantic whitespace/key-order equal after canonicalize; CP4 invalid_missing_required.yaml rejected; CP5 invalid_dirty.yaml merge key rejected fail-closed; CP6 zero sockets / CI no node; CP7 no live locators. Evidence: cargo test --offline + uv run python scripts/check_phase0.py. Story-level second impl hash is not this tkt.

## Decisions

- Recommend kickoff option B minus shipped Phase 0 matrix: plan bitscrow-spec only (YAML parse, canonicalize, blake2b-256). Reject C (full §12 skeleton). Remaining Phase 0 unknowns need a human authenticated RPC (tkt:bitscrow-p0-auth-rpc), not this crate. Digi off. target_system=git. No implementation until human go. work_ids: epic:bitscrow-p1-spec, ms:bitscrow-spec, story:v0.1-canonical-spec, tkt:bitscrow-spec-crate (first impl). Constraints: no keys, no live locators, no portal/oracles/chain adapter/production funds.
- Product confirms work_ids: epic:bitscrow-p1-spec / ms:bitscrow-spec / story:v0.1-canonical-spec / tkt:bitscrow-spec-crate. Do not nest under epic:bitscrow-onboard. tkt:bitscrow-p0-matrix shipped. tkt:bitscrow-p0-auth-rpc is a human follow-up, out of this milestone.
- MVP in: parse v0.1 YAML, strict schema, bitscrow-jcs-v0 canonical bytes, unkeyed blake2b-256, offline fixtures only, CI no sockets. MVP out: portal, oracles, chain/tx, bitscrow-state, §12 skeleton, funds, auth RPC, hashing contract_template.yaml, keys, Digi, live locators, crates before human go.
- Non-goals: portal; live oracles; chain adapter/PSBT/Taproot lore; bitscrow-state and other §12 crates; production funds; tkt:bitscrow-p0-auth-rpc; hashing raw YAML; agents holding keys; Digi/Dossierium; live locators; implementing crates before human go.
- Architecture chosen: one bitscrow-spec crate; bitscrow-jcs-v0 = RFC 8785 JCS over UTF-8 JSON of typed ContractV01; unkeyed blake2b-256 outlen=32. Reject §12 skeleton, CBOR rename, raw YAML hash. RFC: .devao/bundles/bitscrow-p1-spec-20260914-rfc.md. No ChangeIntent.
- Env progression for this slice: local/offline + keyless GitHub Actions only. No VM, staging, prod, AWS, or Compose. Workstation op run for RPC is deferred (tkt:bitscrow-p0-auth-rpc). CI must not load .env. Placeholders only: https://<lan-host>:<rpc-port> and https://<rpc-onion>.onion.
- CI after human go: keep job phase0 (uv run python scripts/check_phase0.py). Add job spec: cargo test -p bitscrow-spec on ubuntu-latest with contents:read, no Actions secrets, no BITSCROW_RPC_*, no cargo publish/deploy. Both jobs required. Crate.io fetch allowed; tests must open zero sockets.
- Secrets/IAM: bitscrow-spec needs none. Never git/CI: RPC password, cookies, PEM/keys, WIF/xprv/mnemonic, live locators. Allowed: .env.example placeholders and .env.op.example op://Personal/BITSCROW RPC/{url,username,password}. Inject later with op run only. Cloud Agents and GHA stay keyless.
- Security checkpoints: no private-key types in ContractV01; fixtures throwaway opaque strings; non-null plaintext_secret rejects; locator scan stays; crate rejects socket/RPC; ci_must_not_call_node remains true; Digi off; target_system=git only after human go. No node from CI.
- This council pass: GO to wait-for-human. No test impl, no ChangeIntent, no crate, no node. No-go for publishing a golden hex until JCS pin.

## Next

- goal=human go still required before tkt:bitscrow-spec-crate; decisions=option B minus shipped matrix; runtime home=bitscrow .devao; artifacts=docs/devao/sessions/bitscrow-p1-spec-20260914.md .devao/bundles/bitscrow-p1-spec-20260914-rfc.md; open_risks=JCS unpinned; catalog unseeded; shop copy stale; next_owner=human; model_class=think

## Blockers

- None.

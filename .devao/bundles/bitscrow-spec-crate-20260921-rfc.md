# RFC: Golden digest for valid_v0.1.yaml

Scope: offline pin for bitscrow-jcs-v0 then unkeyed blake2b-256.
work_id: tkt:bitscrow-spec-crate
Session: bitscrow-spec-crate-20260921

## Problem

bitscrow-spec proves valid_v0.1.yaml digests to 32 bytes and is stable across key order. It does not pin an exact hex. A second implementation cannot cross-check, and a hex produced only by the same code path can freeze a bug.

## Options

1. Rust-only const in crates/bitscrow-spec/tests/acceptance.rs. Fast. A later second impl must scrape Rust.
2. Sibling fixture docs/fixtures/valid_v0.1.digest.blake2b-256.hex. Both Rust and a later impl read the same file.

## Chosen

Option 2. The test includes that file and asserts digest_yaml hex equality. The file body is one line of 64 lowercase hex characters.

Why: the later cross-check reads files. Review can reject a bad hex without reopening the crate. Do not commit a hex that only this crate produced; recompute independently before land.

## Deferred

A schema that generates both models, goldens for invalid fixtures, catalog seed. The Python checker for this one fixture is `scripts/check_spec_golden.py`.

## Security zone

Offline, unkeyed, no secrets, no live locators, no sockets.

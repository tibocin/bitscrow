// File: crates/bitscrow-spec/tests/acceptance.rs
// Purpose: CP1–CP7 acceptance for tkt:bitscrow-spec-crate (offline fixtures).
// Related: docs/fixtures/, docs/devao/sessions/bitscrow-p1-spec-20260914.md
// Tags: #test #acceptance #qa
//
// Why: map chalkboard critical path to cargo evidence. Zero sockets.

use bitscrow_spec::{canonicalize, digest, digest_yaml, parse_yaml, SpecError};

const VALID: &str = include_str!("../../../docs/fixtures/valid_v0.1.yaml");
const MISSING: &str =
    include_str!("../../../docs/fixtures/invalid_missing_required.yaml");
const DIRTY: &str = include_str!("../../../docs/fixtures/invalid_dirty.yaml");

/// CP1: valid_v0.1.yaml → Digest32 length 32 (unkeyed blake2b-256).
#[test]
fn cp1_valid_yields_digest32() {
    let d = digest_yaml(VALID).expect("valid fixture");
    assert_eq!(d.0.len(), 32);
}

/// CP2: hashing twice is identical.
#[test]
fn cp2_digest_stable_twice() {
    let a = digest_yaml(VALID).unwrap();
    let b = digest_yaml(VALID).unwrap();
    assert_eq!(a, b);
}

/// CP3: insignificant whitespace / key-order changes still match after canonicalize.
#[test]
fn cp3_semantic_whitespace_key_order() {
    let reordered = r#"
protocol:
  digest_alg: "blake2b-256"
  canon_profile: "bitscrow-jcs-v0"
  name: "bitscrow"
  version: "0.1.0"
  network: "bitcoin-blake2b"
  contract_type: "marketplace_escrow"
  contract_id: "01jtest00000000000000000000"
metadata:
  description: "Offline QA fixture. Not a live contract."
  title: "fixture physical-goods escrow"
  expires_at: "2026-10-14T00:00:00Z"
  created_at: "2026-09-14T00:00:00Z"
chain_anchor:
  confirmations_required: 6
  block_hash: "00"
  block_height: 0
participants:
  - collateral_sats: 50000
    required_contribution_sats: 250000
    refund_address: "addr_a_refund"
    settlement_address: "addr_a_settle"
    signing_public_key: "02aaa"
    role: "buyer"
    id: "party_a"
  - id: "party_b"
    role: "seller"
    signing_public_key: "02bbb"
    settlement_address: "addr_b_settle"
    refund_address: "addr_b_refund"
    required_contribution_sats: 50000
    collateral_sats: 50000
funding:
  outpoints: []
  funding_policy:
    activation: "all_required_contributions_confirmed"
  required_total_sats: 350000
  currency: "sats"
evidence:
  items:
    - media_type: null
      storage_reference: null
      content_hash: null
      allowed_submitters: ["party_b"]
      required: true
      id: "shipment_record"
commitments:
  - plaintext_secret: null
    commitment: "aa"
    algorithm: "blake2b-256"
    owner: "party_a"
    id: "buyer_receipt_secret"
oracles:
  - id: "delivery"
    subject: "shipment_delivery"
    primary:
      adapter: "fixture-adapter"
      parameters:
        tracking_reference: "fixture-track"
    fallback:
      ordered_steps:
        - type: "participant_confirmation"
          required_participants: ["party_a"]
    total_failure:
      outcome: "oracle_failure_expiry"
outcomes:
  - id: "successful_completion"
    when:
      all:
        - "oracle.delivery.status == delivered"
        - "commitment.buyer_receipt_secret == revealed_and_valid"
    allocations:
      - participant: "party_b"
        amount: "sale_price"
        destination: "settlement_address"
      - participant: "party_a"
        amount: "buyer_remaining_balance"
        destination: "refund_address"
    terminal: true
  - id: "oracle_failure_expiry"
    when:
      all:
        - "contract.expired == true"
        - "oracle.delivery.exhausted == true"
    allocations:
      - participant: "party_a"
        amount: 300000
        destination: "refund_address"
      - participant: "party_b"
        amount: 50000
        destination: "refund_address"
    terminal: true
state:
  status: "draft"
  state_version: 0
  previous_state_hash: null
  genesis_state_hash: null
head_resolution:
  resolver_uri: "bitscrow://01jtest00000000000000000000/head"
  resolver_scheme: "bitscrow"
execution:
  storage_budget_sats: 0
  compute_budget_sats: 0
  interpreter:
    artifact_digest: null
    protocol_version: "0.1.0"
  reference_implementation: "rust"
signatures:
  - signature: null
    participant: "party_a"
  - participant: "party_b"
    signature: null
"#;
    let a = digest_yaml(VALID).unwrap();
    let b = digest_yaml(reordered).unwrap();
    assert_eq!(a, b);
    // Also prove canonicalize bytes match.
    let ca = canonicalize(&parse_yaml(VALID).unwrap()).unwrap();
    let cb = canonicalize(&parse_yaml(reordered).unwrap()).unwrap();
    assert_eq!(ca, cb);
    assert_eq!(digest(&ca), a);
}

/// CP4: missing protocol.version rejected.
#[test]
fn cp4_missing_required_rejected() {
    let err = parse_yaml(MISSING).expect_err("must reject");
    match err {
        SpecError::YamlParse(_) | SpecError::Validation(_) => {}
        other => panic!("unexpected error: {other:?}"),
    }
}

/// CP5: YAML merge key rejected fail-closed.
#[test]
fn cp5_dirty_merge_rejected() {
    assert_eq!(parse_yaml(DIRTY), Err(SpecError::YamlMergeKey));
}

/// CP6/CP7: this test binary opens zero sockets; fixtures use placeholders only.
#[test]
fn cp6_cp7_offline_placeholders() {
    assert!(!VALID.contains(".onion"));
    assert!(!VALID.contains("192.168."));
    assert!(!VALID.contains("10."));
    let _ = digest_yaml(VALID).unwrap();
}

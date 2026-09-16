// File: crates/bitscrow-state/tests/acceptance_happy.rs
// Purpose: CP1–CP6 happy-path + hash acceptance (offline).
// Related: common/mod.rs, tkt:bitscrow-state-crate|hash
// Tags: #test #acceptance #qa
//
// Why: map Product CPs to cargo evidence. Zero sockets.

mod common;

use bitscrow_state::{apply, genesis, StateError, Status, TransitionEvent};
use common::{contract, happy_closed, party_a, step};

#[test]
fn cp1_legal_advance_illegal_reject() {
    let c = contract();
    let h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    assert_eq!(h.status, Status::Draft);
    let ok = apply(
        &c,
        &h,
        &TransitionEvent::Propose,
        &party_a(),
        "2026-09-15T00:01:00Z",
        Some(&h.digest),
    )
    .unwrap();
    assert_eq!(ok.status, Status::Proposed);
    let err = apply(
        &c,
        &h,
        &TransitionEvent::Close {
            outcome_id: "successful_completion".into(),
        },
        &party_a(),
        "2026-09-15T00:01:00Z",
        Some(&h.digest),
    )
    .unwrap_err();
    assert!(matches!(err, StateError::IllegalTransition { .. }));
    assert_eq!(h.status, Status::Draft);
}

#[test]
fn cp2_stub_walk_draft_to_closed() {
    let h = happy_closed(&contract());
    assert_eq!(h.status, Status::Closed);
    assert!(h.previous_digest.is_some());
}

#[test]
fn cp3_transition_records_prev_event_actor() {
    let c = contract();
    let h0 = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let h1 = step(&c, &h0, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    assert_eq!(
        h1.last_record.prev_digest_hex.as_deref(),
        Some(h0.digest.to_hex().as_str())
    );
    assert_eq!(h1.last_record.event_kind, "propose");
    assert_eq!(h1.last_record.actor, party_a());
}

#[test]
fn cp4_digest_stable_twice() {
    let c = contract();
    let h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let a = apply(
        &c,
        &h,
        &TransitionEvent::Propose,
        &party_a(),
        "2026-09-15T00:01:00Z",
        Some(&h.digest),
    )
    .unwrap();
    let b = apply(
        &c,
        &h,
        &TransitionEvent::Propose,
        &party_a(),
        "2026-09-15T00:01:00Z",
        Some(&h.digest),
    )
    .unwrap();
    assert_eq!(a.digest, b.digest);
}

#[test]
fn cp5_genesis_no_pred_later_links() {
    let c = contract();
    let h0 = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    assert!(h0.previous_digest.is_none());
    let h1 = step(&c, &h0, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    assert_eq!(h1.previous_digest, Some(h0.digest));
}

#[test]
fn cp6_distinct_events_distinct_hashes() {
    let c = contract();
    let h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let p = step(&c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    let a = step(&c, &p, TransitionEvent::Accept, party_a(), "2026-09-15T00:02:00Z");
    assert_ne!(p.digest, a.digest);
    assert_ne!(h.digest, p.digest);
}

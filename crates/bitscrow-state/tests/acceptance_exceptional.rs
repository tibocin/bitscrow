// File: crates/bitscrow-state/tests/acceptance_exceptional.rs
// Purpose: CP7–CP10 exceptional + recovery acceptance (offline).
// Related: common/mod.rs, tkt:bitscrow-state-matrix
// Tags: #test #acceptance #qa
//
// Why: expiration, oracle fallback, FAILED_SAFE, recovery fund-move deny.

mod common;

use bitscrow_state::{apply, genesis, Actor, StateError, Status, TransitionEvent};
use common::{contract, happy_closed, party_a, step};

#[test]
fn cp7_illegal_matrix_samples() {
    let c = contract();
    let closed = happy_closed(&c);
    let err = apply(
        &c,
        &closed,
        &TransitionEvent::Propose,
        &party_a(),
        "2026-09-15T01:00:00Z",
        Some(&closed.digest),
    )
    .unwrap_err();
    assert!(matches!(err, StateError::IllegalTransition { .. }));

    let mut h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    h = step(&c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    let err = apply(
        &c,
        &h,
        &TransitionEvent::EnterFunding,
        &party_a(),
        "2026-09-15T00:02:00Z",
        Some(&h.digest),
    )
    .unwrap_err();
    assert!(matches!(err, StateError::IllegalTransition { .. }));
}

#[test]
fn cp8_expiration_to_expired() {
    let c = contract();
    let mut h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    h = step(&c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    let exp = apply(
        &c,
        &h,
        &TransitionEvent::Expire,
        &Actor::Clock,
        "2026-10-15T00:00:00Z",
        Some(&h.digest),
    )
    .unwrap();
    assert_eq!(exp.status, Status::Expired);
}

#[test]
fn cp9_oracle_fallback_failed_safe_idempotent() {
    let c = contract();
    let mut h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let stub = Actor::TestStub;
    h = step(&c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    h = step(&c, &h, TransitionEvent::Accept, party_a(), "2026-09-15T00:02:00Z");
    h = step(&c, &h, TransitionEvent::EnterFunding, party_a(), "2026-09-15T00:03:00Z");
    h = step(
        &c,
        &h,
        TransitionEvent::FundingStubObserved {
            opaque_ref: "f".into(),
        },
        stub.clone(),
        "2026-09-15T00:04:00Z",
    );
    h = step(
        &c,
        &h,
        TransitionEvent::ActivationStubGranted {
            opaque_ref: "a".into(),
        },
        stub,
        "2026-09-15T00:05:00Z",
    );
    h = step(&c, &h, TransitionEvent::BeginVerifying, party_a(), "2026-09-15T00:06:00Z");
    let oracle = Actor::Oracle {
        id: "delivery".into(),
    };
    h = step(
        &c,
        &h,
        TransitionEvent::EnterOracleFallback {
            step: "secondary".into(),
        },
        oracle.clone(),
        "2026-09-15T00:07:00Z",
    );
    assert_eq!(h.status, Status::OracleFallback);
    let failed = step(
        &c,
        &h,
        TransitionEvent::OracleExhausted,
        oracle.clone(),
        "2026-09-15T00:08:00Z",
    );
    assert_eq!(failed.status, Status::FailedSafe);
    let again = apply(
        &c,
        &failed,
        &TransitionEvent::OracleExhausted,
        &oracle,
        "2026-09-15T00:08:00Z",
        Some(&failed.digest),
    )
    .unwrap();
    assert_eq!(again.digest, failed.digest);
    assert_eq!(again.version, failed.version);
}

#[test]
fn cp10_recovery_rejects_fund_move() {
    let c = contract();
    let mut h = genesis(&c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let stub = Actor::TestStub;
    h = step(&c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    h = step(&c, &h, TransitionEvent::Accept, party_a(), "2026-09-15T00:02:00Z");
    h = step(&c, &h, TransitionEvent::EnterFunding, party_a(), "2026-09-15T00:03:00Z");
    h = step(
        &c,
        &h,
        TransitionEvent::FundingStubObserved {
            opaque_ref: "f".into(),
        },
        stub.clone(),
        "2026-09-15T00:04:00Z",
    );
    h = step(
        &c,
        &h,
        TransitionEvent::ActivationStubGranted {
            opaque_ref: "a".into(),
        },
        stub.clone(),
        "2026-09-15T00:05:00Z",
    );
    h = step(&c, &h, TransitionEvent::RaiseDispute, party_a(), "2026-09-15T00:06:00Z");
    h = step(&c, &h, TransitionEvent::EnterRecovery, party_a(), "2026-09-15T00:07:00Z");
    assert_eq!(h.status, Status::Recovery);
    let err = apply(
        &c,
        &h,
        &TransitionEvent::SettleStubAcknowledged {
            opaque_ref: "nope".into(),
        },
        &stub,
        "2026-09-15T00:08:00Z",
        Some(&h.digest),
    )
    .unwrap_err();
    assert_eq!(err, StateError::RecoveryFundMove);
}

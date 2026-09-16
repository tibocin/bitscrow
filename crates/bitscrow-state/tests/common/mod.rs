// File: crates/bitscrow-state/tests/common/mod.rs
// Purpose: Shared fixture helpers for Phase 2 acceptance tests.
// Related: acceptance_happy.rs, acceptance_exceptional.rs
// Tags: #test #helpers
//
// Why: keep each acceptance file under the line budget.
// Side effects: none.

use bitscrow_spec::parse_yaml;
use bitscrow_state::{apply, genesis, Actor, StateHead, TransitionEvent};

pub const VALID: &str = include_str!("../../../../docs/fixtures/valid_v0.1.yaml");

pub fn contract() -> bitscrow_spec::ContractV01 {
    parse_yaml(VALID).expect("valid fixture")
}

pub fn party_a() -> Actor {
    Actor::Participant {
        id: "party_a".into(),
    }
}

pub fn step(
    c: &bitscrow_spec::ContractV01,
    head: &StateHead,
    event: TransitionEvent,
    actor: Actor,
    now: &str,
) -> StateHead {
    apply(c, head, &event, &actor, now, Some(&head.digest)).expect("step")
}

/// Stub walk DRAFT…CLOSED.
pub fn happy_closed(c: &bitscrow_spec::ContractV01) -> StateHead {
    let mut h = genesis(c, "2026-09-15T00:00:00Z", &party_a()).unwrap();
    let stub = Actor::TestStub;
    h = step(c, &h, TransitionEvent::Propose, party_a(), "2026-09-15T00:01:00Z");
    h = step(c, &h, TransitionEvent::Accept, party_a(), "2026-09-15T00:02:00Z");
    h = step(c, &h, TransitionEvent::EnterFunding, party_a(), "2026-09-15T00:03:00Z");
    h = step(
        c,
        &h,
        TransitionEvent::FundingStubObserved {
            opaque_ref: "fund-1".into(),
        },
        stub.clone(),
        "2026-09-15T00:04:00Z",
    );
    h = step(
        c,
        &h,
        TransitionEvent::ActivationStubGranted {
            opaque_ref: "act-1".into(),
        },
        stub.clone(),
        "2026-09-15T00:05:00Z",
    );
    h = step(c, &h, TransitionEvent::BeginVerifying, party_a(), "2026-09-15T00:06:00Z");
    h = step(
        c,
        &h,
        TransitionEvent::SubmitEvidence {
            evidence_id: "shipment_record".into(),
        },
        party_a(),
        "2026-09-15T00:07:00Z",
    );
    h = step(
        c,
        &h,
        TransitionEvent::SettleStubAcknowledged {
            opaque_ref: "settle-1".into(),
        },
        stub,
        "2026-09-15T00:08:00Z",
    );
    step(
        c,
        &h,
        TransitionEvent::Close {
            outcome_id: "successful_completion".into(),
        },
        party_a(),
        "2026-09-15T00:09:00Z",
    )
}

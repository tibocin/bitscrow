// File: crates/bitscrow-state/src/machine.rs
// Purpose: Fail-closed apply/genesis for offline lifecycle FSM.
// Related: transition.rs, hash.rs, revision.rs
// Tags: #fsm #machine
//
// Why: tracer for §4 edges without chain/RPC; stub funding/settle only.
// Side effects: none (pure functions over StateHead).

use bitscrow_spec::{canonicalize, digest, ContractV01, Digest32};

use crate::actor::Actor;
use crate::error::StateError;
use crate::event::TransitionEvent;
use crate::hash::digest_revision;
use crate::revision::{StateHead, StateRevision, TransitionRecord};
use crate::status::Status;
use crate::transition;

/// Create genesis HEAD at Draft from a validated contract + clock.
pub fn genesis(
    contract: &ContractV01,
    now: &str,
    actor: &Actor,
) -> Result<StateHead, StateError> {
    validate_actor(contract, actor)?;
    let genesis_digest = contract_digest(contract)?;
    let record = TransitionRecord {
        prev_digest_hex: None,
        event_kind: "genesis".into(),
        actor: actor.clone(),
        evidence_refs: vec![],
        timestamp: now.into(),
        chain_hint: None,
        outcome_id: None,
    };
    let revision = StateRevision {
        contract_id: contract.protocol.contract_id.clone(),
        genesis_digest_hex: genesis_digest.to_hex(),
        previous_digest_hex: None,
        version: 0,
        status: Status::Draft,
        event_kind: record.event_kind.clone(),
        actor: actor.clone(),
        evidence_refs: vec![],
        timestamp: now.into(),
        chain_hint: None,
        outcome_id: None,
        resume_status: None,
    };
    let d = digest_revision(&revision)?;
    Ok(StateHead {
        contract_id: contract.protocol.contract_id.clone(),
        genesis_digest,
        digest: d,
        previous_digest: None,
        version: 0,
        status: Status::Draft,
        updated_at: now.into(),
        last_record: record,
        resume_status: None,
    })
}

/// Apply one event. Idempotent if same event_kind+actor+ts on same HEAD.
pub fn apply(
    contract: &ContractV01,
    head: &StateHead,
    event: &TransitionEvent,
    actor: &Actor,
    now: &str,
    expected_prev: Option<&Digest32>,
) -> Result<StateHead, StateError> {
    if let Some(exp) = expected_prev {
        if exp != &head.digest {
            return Err(StateError::PrevMismatch);
        }
    }
    if now < head.updated_at.as_str() {
        return Err(StateError::ClockRewind);
    }
    if head.last_record.event_kind == event.kind_name()
        && &head.last_record.actor == actor
        && head.last_record.timestamp == now
    {
        return Ok(head.clone());
    }

    validate_actor(contract, actor)?;
    validate_event_refs(contract, event)?;

    if head.status == Status::Recovery && event.is_fund_moving_stub() {
        return Err(StateError::RecoveryFundMove);
    }

    let (next_status, evidence_refs, outcome_id, resume) =
        transition::resolve(contract, head, event, actor)?;

    let prev_hex = Some(head.digest.to_hex());
    let record = TransitionRecord {
        prev_digest_hex: prev_hex.clone(),
        event_kind: event.kind_name().into(),
        actor: actor.clone(),
        evidence_refs: evidence_refs.clone(),
        timestamp: now.into(),
        chain_hint: None,
        outcome_id: outcome_id.clone(),
    };
    let version = head.version + 1;
    let revision = StateRevision {
        contract_id: head.contract_id.clone(),
        genesis_digest_hex: head.genesis_digest.to_hex(),
        previous_digest_hex: prev_hex,
        version,
        status: next_status,
        event_kind: record.event_kind.clone(),
        actor: actor.clone(),
        evidence_refs,
        timestamp: now.into(),
        chain_hint: None,
        outcome_id,
        resume_status: resume,
    };
    let d = digest_revision(&revision)?;
    Ok(StateHead {
        contract_id: head.contract_id.clone(),
        genesis_digest: head.genesis_digest,
        digest: d,
        previous_digest: Some(head.digest),
        version,
        status: next_status,
        updated_at: now.into(),
        last_record: record,
        resume_status: resume,
    })
}

fn contract_digest(contract: &ContractV01) -> Result<Digest32, StateError> {
    let c = canonicalize(contract).map_err(|e| StateError::Digest(e.to_string()))?;
    Ok(digest(&c))
}

fn validate_actor(contract: &ContractV01, actor: &Actor) -> Result<(), StateError> {
    match actor {
        Actor::Participant { id } => {
            if !contract.participants.iter().any(|p| p.id == *id) {
                return Err(StateError::UnknownParticipant(id.clone()));
            }
        }
        Actor::Oracle { id } => {
            if !contract.oracles.iter().any(|o| o.id == *id) {
                return Err(StateError::UnknownOracle(id.clone()));
            }
        }
        Actor::Clock | Actor::TestStub => {}
    }
    Ok(())
}

fn validate_event_refs(
    contract: &ContractV01,
    event: &TransitionEvent,
) -> Result<(), StateError> {
    match event {
        TransitionEvent::SubmitEvidence { evidence_id } => {
            if !contract.evidence.items.iter().any(|e| e.id == *evidence_id) {
                return Err(StateError::UnknownEvidence(evidence_id.clone()));
            }
        }
        TransitionEvent::Close { outcome_id } => {
            if !contract.outcomes.iter().any(|o| o.id == *outcome_id) {
                return Err(StateError::UnknownOutcome(outcome_id.clone()));
            }
        }
        _ => {}
    }
    Ok(())
}

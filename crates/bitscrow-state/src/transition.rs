// File: crates/bitscrow-state/src/transition.rs
// Purpose: Edge table — (status, event) → next status (fail closed).
// Related: machine.rs, event.rs, status.rs
// Tags: #transition #matrix
//
// Why: keep apply() thin; table is easy to delete or widen per ticket.
// Side effects: none.

use bitscrow_spec::ContractV01;

use crate::actor::Actor;
use crate::error::StateError;
use crate::event::TransitionEvent;
use crate::revision::StateHead;
use crate::status::Status;

pub(crate) type Next = (Status, Vec<String>, Option<String>, Option<Status>);

pub(crate) fn resolve(
    _contract: &ContractV01,
    head: &StateHead,
    event: &TransitionEvent,
    actor: &Actor,
) -> Result<Next, StateError> {
    use Status::*;
    use TransitionEvent::*;

    match event {
        Expire => {
            if !matches!(actor, Actor::Clock) {
                return Err(StateError::ActorDenied(actor.label()));
            }
        }
        FundingStubObserved { .. }
        | ActivationStubGranted { .. }
        | SettleStubAcknowledged { .. } => {
            if !matches!(actor, Actor::TestStub) {
                return Err(StateError::ActorDenied(actor.label()));
            }
        }
        EnterOracleFallback { .. } | OracleExhausted => {
            if !matches!(actor, Actor::Oracle { .. } | Actor::TestStub) {
                return Err(StateError::ActorDenied(actor.label()));
            }
        }
        _ => {}
    }

    let from = head.status;
    match (from, event) {
        (Draft, Propose) => Ok((Proposed, vec![], None, None)),
        (Proposed, Accept) => Ok((Accepted, vec![], None, None)),
        (Accepted, EnterFunding) => Ok((Funding, vec![], None, None)),
        (Funding, FundingStubObserved { .. }) => Ok((Funding, vec![], None, None)),
        (Funding, ActivationStubGranted { .. }) => Ok((Active, vec![], None, None)),
        (Active, BeginVerifying) => Ok((Verifying, vec![], None, None)),
        (Verifying, SubmitEvidence { evidence_id }) => {
            Ok((Verifying, vec![evidence_id.clone()], None, None))
        }
        (Verifying, SettleStubAcknowledged { .. }) => Ok((Settling, vec![], None, None)),
        (Settling, Close { outcome_id }) => {
            Ok((Closed, vec![], Some(outcome_id.clone()), None))
        }
        (Active | Verifying, RaiseDispute) => Ok((Disputed, vec![], None, None)),
        (Verifying, EnterOracleFallback { .. }) => {
            Ok((OracleFallback, vec![], None, None))
        }
        (OracleFallback, OracleExhausted) => Ok((FailedSafe, vec![], None, None)),
        (s, Expire) if !s.is_terminal() && s != Expired => {
            Ok((Expired, vec![], None, None))
        }
        (Disputed | Expired | OracleFallback, EnterRecovery) => {
            Ok((Recovery, vec![], None, Some(from)))
        }
        (Recovery, ExitRecovery) => {
            let resume = head.resume_status.unwrap_or(Disputed);
            Ok((resume, vec![], None, None))
        }
        (Recovery, FailSafe) => Ok((FailedSafe, vec![], None, None)),
        (Disputed | Expired | OracleFallback, FailSafe) => {
            Ok((FailedSafe, vec![], None, None))
        }
        _ => Err(StateError::IllegalTransition {
            from: from.as_str().into(),
            event: event.kind_name().into(),
        }),
    }
}

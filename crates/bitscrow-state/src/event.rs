// File: crates/bitscrow-state/src/event.rs
// Purpose: Lifecycle-generic TransitionEvent vocabulary (OQ1 / e17).
// Related: machine.rs, actor.rs
// Tags: #event #fsm
//
// Why: typed edges for the matrix; stub/oracle payloads stay opaque strings.
// Side effects: none.

use serde::{Deserialize, Serialize};

/// Typed control events. Opaque refs are not interpreted as chain/script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TransitionEvent {
    Propose,
    Accept,
    EnterFunding,
    FundingStubObserved { opaque_ref: String },
    ActivationStubGranted { opaque_ref: String },
    BeginVerifying,
    SubmitEvidence { evidence_id: String },
    SettleStubAcknowledged { opaque_ref: String },
    Close { outcome_id: String },
    RaiseDispute,
    EnterOracleFallback { step: String },
    OracleExhausted,
    Expire,
    EnterRecovery,
    ExitRecovery,
    FailSafe,
}

impl TransitionEvent {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Propose => "propose",
            Self::Accept => "accept",
            Self::EnterFunding => "enter_funding",
            Self::FundingStubObserved { .. } => "funding_stub_observed",
            Self::ActivationStubGranted { .. } => "activation_stub_granted",
            Self::BeginVerifying => "begin_verifying",
            Self::SubmitEvidence { .. } => "submit_evidence",
            Self::SettleStubAcknowledged { .. } => "settle_stub_acknowledged",
            Self::Close { .. } => "close",
            Self::RaiseDispute => "raise_dispute",
            Self::EnterOracleFallback { .. } => "enter_oracle_fallback",
            Self::OracleExhausted => "oracle_exhausted",
            Self::Expire => "expire",
            Self::EnterRecovery => "enter_recovery",
            Self::ExitRecovery => "exit_recovery",
            Self::FailSafe => "fail_safe",
        }
    }

    /// Evidence, outcome, and chain-hint fields stored on each revision for hashing.
    pub fn record_payload(&self) -> (Vec<String>, Option<String>, Option<String>) {
        match self {
            Self::SubmitEvidence { evidence_id } => (vec![evidence_id.clone()], None, None),
            Self::Close { outcome_id } => (vec![], Some(outcome_id.clone()), None),
            Self::FundingStubObserved { opaque_ref }
            | Self::ActivationStubGranted { opaque_ref }
            | Self::SettleStubAcknowledged { opaque_ref } => (vec![], None, Some(opaque_ref.clone())),
            Self::EnterOracleFallback { step } => (vec![], None, Some(step.clone())),
            _ => (vec![], None, None),
        }
    }

    /// Stub funding/settle/activation — rejected while in Recovery.
    pub fn is_fund_moving_stub(&self) -> bool {
        matches!(
            self,
            Self::FundingStubObserved { .. }
                | Self::ActivationStubGranted { .. }
                | Self::SettleStubAcknowledged { .. }
                | Self::Close { .. }
                | Self::EnterFunding
        )
    }
}

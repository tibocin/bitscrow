// File: crates/bitscrow-state/src/error.rs
// Purpose: Typed fail-closed errors for illegal transitions and hash failures.
// Related: machine.rs, hash.rs
// Tags: #error #fail-closed
//
// Why: callers must never soft-succeed an illegal edge or invented payout.
// Side effects: none.

use thiserror::Error;

/// Fail-closed state-machine / digest errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum StateError {
    #[error("illegal transition from {from:?} via {event}")]
    IllegalTransition { from: String, event: String },
    #[error("actor not allowed for this event: {0}")]
    ActorDenied(String),
    #[error("unknown participant id: {0}")]
    UnknownParticipant(String),
    #[error("unknown oracle id: {0}")]
    UnknownOracle(String),
    #[error("unknown evidence id: {0}")]
    UnknownEvidence(String),
    #[error("unknown outcome id: {0}")]
    UnknownOutcome(String),
    #[error("previous digest mismatch")]
    PrevMismatch,
    #[error("clock rewind rejected")]
    ClockRewind,
    #[error("fund-moving event rejected in recovery")]
    RecoveryFundMove,
    #[error("canonicalize/digest: {0}")]
    Digest(String),
}

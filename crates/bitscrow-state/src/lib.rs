// File: crates/bitscrow-state/src/lib.rs
// Purpose: Public API for offline bitscrow-state (Phase 2).
// Related: machine.rs, event.rs, status.rs
// Tags: #api #bitscrow-state #phase2
//
// Why: reversible FSM crate; no sockets, keys, RPC, or chain adapter.
// Side effects: none.

//! BITSCROW v0.1 offline state machine.
//!
//! Pipeline: [`ContractV01`] + [`TransitionEvent`] → fail-closed guards →
//! [`StateRevision`] → bitscrow-jcs-v0 → blake2b-256 [`Digest32`].

mod actor;
mod error;
mod event;
mod hash;
mod machine;
mod revision;
mod status;
mod transition;

pub use actor::Actor;
pub use error::StateError;
pub use event::TransitionEvent;
pub use hash::digest_revision;
pub use machine::{apply, genesis};
pub use revision::{StateHead, StateRevision, TransitionRecord};
pub use status::Status;
pub use bitscrow_spec::Digest32;

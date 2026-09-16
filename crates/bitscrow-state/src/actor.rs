// File: crates/bitscrow-state/src/actor.rs
// Purpose: Closed actor enum — never carries private keys.
// Related: event.rs, machine.rs
// Tags: #actor #authz
//
// Why: transition records need an actor without inventing signing material.
// Side effects: none.

use serde::{Deserialize, Serialize};

/// Who submitted a transition. Keys stay out of this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Actor {
    Participant { id: String },
    Oracle { id: String },
    Clock,
    TestStub,
}

impl Actor {
    pub fn label(&self) -> String {
        match self {
            Actor::Participant { id } => format!("participant:{id}"),
            Actor::Oracle { id } => format!("oracle:{id}"),
            Actor::Clock => "clock".into(),
            Actor::TestStub => "test_stub".into(),
        }
    }
}

// File: crates/bitscrow-state/src/status.rs
// Purpose: Lifecycle + exceptional statuses from architecture §4.
// Related: machine.rs, revision.rs
// Tags: #status #lifecycle
//
// Why: stringly status in ContractV01 authoring; runtime uses a closed enum.
// Side effects: none.

use serde::{Deserialize, Serialize};

/// Operational contract status (happy path + exceptional).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Draft,
    Proposed,
    Accepted,
    Funding,
    Active,
    Verifying,
    Settling,
    Closed,
    Disputed,
    OracleFallback,
    Expired,
    Recovery,
    FailedSafe,
}

impl Status {
    /// Terminal statuses never accept Expire / further fund-moving happy edges.
    pub fn is_terminal(self) -> bool {
        matches!(self, Status::Closed | Status::FailedSafe)
    }

    /// Wire name for errors and hashed revisions.
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Proposed => "proposed",
            Status::Accepted => "accepted",
            Status::Funding => "funding",
            Status::Active => "active",
            Status::Verifying => "verifying",
            Status::Settling => "settling",
            Status::Closed => "closed",
            Status::Disputed => "disputed",
            Status::OracleFallback => "oracle_fallback",
            Status::Expired => "expired",
            Status::Recovery => "recovery",
            Status::FailedSafe => "failed_safe",
        }
    }
}

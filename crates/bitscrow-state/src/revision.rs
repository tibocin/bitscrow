// File: crates/bitscrow-state/src/revision.rs
// Purpose: Hashable StateRevision + in-memory StateHead / TransitionRecord.
// Related: hash.rs, machine.rs
// Tags: #revision #head
//
// Why: hash a dedicated revision, not the whole ContractV01, each transition.
// Side effects: none.

use bitscrow_spec::Digest32;
use serde::{Deserialize, Serialize};

use crate::actor::Actor;
use crate::status::Status;

/// Append-only transition metadata (also mirrored into the hashed revision).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionRecord {
    pub prev_digest_hex: Option<String>,
    pub event_kind: String,
    pub actor: Actor,
    pub evidence_refs: Vec<String>,
    pub timestamp: String,
    pub chain_hint: Option<String>,
    pub outcome_id: Option<String>,
}

/// Canonical fields hashed via bitscrow-jcs-v0 → blake2b-256.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRevision {
    pub contract_id: String,
    pub genesis_digest_hex: String,
    pub previous_digest_hex: Option<String>,
    pub version: u64,
    pub status: Status,
    pub event_kind: String,
    pub actor: Actor,
    pub evidence_refs: Vec<String>,
    pub timestamp: String,
    pub chain_hint: Option<String>,
    pub outcome_id: Option<String>,
    pub resume_status: Option<Status>,
}

/// Operational HEAD held by the machine (digest included).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateHead {
    pub contract_id: String,
    pub genesis_digest: Digest32,
    pub digest: Digest32,
    pub previous_digest: Option<Digest32>,
    pub version: u64,
    pub status: Status,
    pub updated_at: String,
    pub last_record: TransitionRecord,
    /// Prior non-terminal status when inside Recovery (stub resume target).
    pub resume_status: Option<Status>,
}

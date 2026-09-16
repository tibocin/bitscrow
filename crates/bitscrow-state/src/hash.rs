// File: crates/bitscrow-state/src/hash.rs
// Purpose: Digest StateRevision with Phase 1 canon profile + blake2b-256.
// Related: revision.rs, bitscrow-spec::canonicalize_value
// Tags: #hash #jcs #blake2b
//
// Why: same bitscrow-jcs-v0 profile as contracts; different message shape.
// Side effects: none.

use bitscrow_spec::{canonicalize_value, digest, Digest32};

use crate::error::StateError;
use crate::revision::StateRevision;

/// JCS + unkeyed blake2b-256 over `revision`.
pub fn digest_revision(revision: &StateRevision) -> Result<Digest32, StateError> {
    let value = serde_json::to_value(revision)
        .map_err(|e| StateError::Digest(e.to_string()))?;
    let canonical =
        canonicalize_value(&value).map_err(|e| StateError::Digest(e.to_string()))?;
    Ok(digest(&canonical))
}

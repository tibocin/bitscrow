// File: crates/bitscrow-spec/src/error.rs
// Purpose: Typed failures for parse, validate, canonicalize, and digest.
// Related: parse.rs, model.rs, lib.rs
// Tags: #error #fail-closed #bitscrow-spec
//
// Why: callers must distinguish dirty YAML from schema misses without panics.
// Side effects: none.

use thiserror::Error;

/// Fail-closed errors for the public bitscrow-spec surface.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpecError {
    /// YAML merge key `<<` — reject before expansion.
    #[error("yaml merge keys are forbidden")]
    YamlMergeKey,

    /// serde/YAML syntax or shape failure.
    #[error("yaml parse error: {0}")]
    YamlParse(String),

    /// Missing/invalid required field or invariant.
    #[error("validation error: {0}")]
    Validation(String),

    /// Canonical JSON (JCS) encoding failed.
    #[error("canonicalize error: {0}")]
    Canonicalize(String),
}

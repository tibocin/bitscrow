// File: crates/bitscrow-spec/src/lib.rs
// Purpose: Public API — parse_yaml → canonicalize → digest (Phase 1 only).
// Related: parse.rs, canon.rs, digest.rs, model.rs
// Tags: #api #bitscrow-spec #phase1
//
// Why: one reversible crate; no sockets, keys, RPC, or chain adapter.
// Side effects: none.

//! BITSCROW v0.1 contract specification library.
//!
//! Pipeline: YAML 1.2 → [`ContractV01`] → bitscrow-jcs-v0 (RFC 8785) →
//! unkeyed blake2b-256 [`Digest32`].
//!
//! # Examples
//!
//! ```
//! use bitscrow_spec::{canonicalize, digest, parse_yaml};
//! let yaml = include_str!("../../../docs/fixtures/valid_v0.1.yaml");
//! let c = parse_yaml(yaml).expect("valid fixture");
//! let d = digest(&canonicalize(&c).expect("jcs"));
//! assert_eq!(d.0.len(), 32);
//! ```

mod canon;
mod digest;
mod error;
mod model;
mod parse;

pub use canon::{canonicalize, canonicalize_value, CanonicalBytes, BITSCROW_JCS_V0};
pub use digest::{digest, Digest32};
pub use error::SpecError;
pub use model::ContractV01;
pub use parse::parse_yaml;

/// Convenience: parse → canonicalize → digest in one call.
pub fn digest_yaml(input: &str) -> Result<Digest32, SpecError> {
    let contract = parse_yaml(input)?;
    let canonical = canonicalize(&contract)?;
    Ok(digest(&canonical))
}

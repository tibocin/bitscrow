// File: crates/bitscrow-spec/src/canon.rs
// Purpose: bitscrow-jcs-v0 = RFC 8785 JCS over UTF-8 JSON of ContractV01.
// Related: model.rs, digest.rs, RFC e28 pin
// Tags: #jcs #canonicalize #bitscrow-jcs-v0
//
// Why: raw YAML must never be hashed; JCS gives deterministic UTF-8 bytes.
// Side effects: none.

use serde_json_canonicalizer::to_vec;

use crate::error::SpecError;
use crate::model::ContractV01;

/// Canonical UTF-8 bytes (RFC 8785) for a validated contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalBytes(pub Vec<u8>);

/// Canon profile name cited by protocol.canon_profile.
pub const BITSCROW_JCS_V0: &str = "bitscrow-jcs-v0";

/// Serialize `contract` with serde_json (nulls kept), then RFC 8785 JCS.
pub fn canonicalize(contract: &ContractV01) -> Result<CanonicalBytes, SpecError> {
    // Insight: Option::None → JSON null via default Serialize (no skip).
    let value = serde_json::to_value(contract)
        .map_err(|e| SpecError::Canonicalize(e.to_string()))?;
    canonicalize_value(&value)
}

/// RFC 8785 JCS over any JSON value (Phase 2 StateRevision reuses this profile).
pub fn canonicalize_value(
    value: &serde_json::Value,
) -> Result<CanonicalBytes, SpecError> {
    let bytes =
        to_vec(value).map_err(|e| SpecError::Canonicalize(e.to_string()))?;
    Ok(CanonicalBytes(bytes))
}

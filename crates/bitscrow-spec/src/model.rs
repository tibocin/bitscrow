// File: crates/bitscrow-spec/src/model.rs
// Purpose: Typed v0.1 ContractV01 — opaque strings for keys/addresses; no private-key types.
// Related: docs/fixtures/valid_v0.1.yaml, parse.rs, canon.rs
// Tags: #model #v0.1 #ContractV01
//
// Why: typed round-trip normalizes YAML key order/whitespace before JCS.
// Side effects: none. Serialize emits JSON null for Option::None (no omit).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Validated BITSCROW v0.1 marketplace authoring document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractV01 {
    pub protocol: Protocol,
    pub metadata: Metadata,
    pub chain_anchor: ChainAnchor,
    pub participants: Vec<Participant>,
    pub funding: Funding,
    pub evidence: Evidence,
    pub commitments: Vec<Commitment>,
    pub oracles: Vec<Oracle>,
    pub outcomes: Vec<Outcome>,
    pub state: State,
    pub head_resolution: HeadResolution,
    pub execution: Execution,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Protocol {
    pub name: String,
    pub version: String,
    pub network: String,
    pub digest_alg: String,
    pub canon_profile: String,
    pub contract_id: String,
    pub contract_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub created_at: String,
    pub expires_at: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChainAnchor {
    pub block_height: u64,
    pub block_hash: String,
    pub confirmations_required: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Participant {
    pub id: String,
    pub role: String,
    /// Opaque; never a private key.
    pub signing_public_key: String,
    pub settlement_address: String,
    pub refund_address: String,
    pub required_contribution_sats: u64,
    pub collateral_sats: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Funding {
    pub currency: String,
    pub required_total_sats: u64,
    pub funding_policy: FundingPolicy,
    pub outpoints: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FundingPolicy {
    pub activation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub items: Vec<EvidenceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceItem {
    pub id: String,
    pub required: bool,
    pub allowed_submitters: Vec<String>,
    pub content_hash: Option<String>,
    pub storage_reference: Option<String>,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commitment {
    pub id: String,
    pub owner: String,
    pub algorithm: String,
    pub commitment: String,
    /// Must be null in authoring fixtures; plaintext secrets never enter digests.
    pub plaintext_secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Oracle {
    pub id: String,
    pub subject: String,
    pub primary: Value,
    pub fallback: Value,
    pub total_failure: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub id: String,
    /// Opaque outcome predicate; not interpreted in Phase 1.
    pub when: Value,
    pub allocations: Vec<Allocation>,
    pub terminal: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Allocation {
    pub participant: String,
    pub amount: Amount,
    pub destination: String,
}

/// Outcome amounts may be symbolic strings or literal sat integers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Sats(u64),
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub genesis_state_hash: Option<String>,
    pub previous_state_hash: Option<String>,
    pub state_version: u64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadResolution {
    pub resolver_scheme: String,
    pub resolver_uri: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub reference_implementation: String,
    pub interpreter: Interpreter,
    pub compute_budget_sats: u64,
    pub storage_budget_sats: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interpreter {
    pub protocol_version: String,
    pub artifact_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signature {
    pub participant: String,
    pub signature: Option<String>,
}

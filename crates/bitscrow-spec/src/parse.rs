// File: crates/bitscrow-spec/src/parse.rs
// Purpose: Strict YAML 1.2 parse into ContractV01 with merge-key fail-closed.
// Related: model.rs, error.rs, docs/fixtures/invalid_dirty.yaml
// Tags: #parse #yaml #fail-closed
//
// Why: serde_yaml expands `<<` merges; we must reject before that expansion.
// Side effects: none. No sockets.

use yaml_rust2::{Yaml, YamlLoader};

use crate::error::SpecError;
use crate::model::ContractV01;

/// JSON-safe integer ceiling (RFC 8785 / IEEE doubles).
const MAX_SATS: u64 = (1u64 << 53) - 1;

/// Parse UTF-8 YAML into a validated `ContractV01`.
pub fn parse_yaml(input: &str) -> Result<ContractV01, SpecError> {
    reject_merge_keys(input)?;
    let contract: ContractV01 = serde_yaml::from_str(input)
        .map_err(|e| SpecError::YamlParse(e.to_string()))?;
    validate(&contract)?;
    Ok(contract)
}

/// Walk yaml-rust2 AST for mapping key `<<` before serde_yaml sees the text.
fn reject_merge_keys(input: &str) -> Result<(), SpecError> {
    let docs = YamlLoader::load_from_str(input)
        .map_err(|e| SpecError::YamlParse(e.to_string()))?;
    for doc in &docs {
        if yaml_contains_merge(doc) {
            return Err(SpecError::YamlMergeKey);
        }
    }
    Ok(())
}

fn yaml_contains_merge(node: &Yaml) -> bool {
    match node {
        Yaml::Hash(map) => {
            for (k, v) in map {
                if matches!(k, Yaml::String(s) if s == "<<") {
                    return true;
                }
                if yaml_contains_merge(k) || yaml_contains_merge(v) {
                    return true;
                }
            }
            false
        }
        Yaml::Array(items) => items.iter().any(yaml_contains_merge),
        _ => false,
    }
}

/// Invariants beyond serde shape (digest profile, secrets, sat bounds).
fn validate(c: &ContractV01) -> Result<(), SpecError> {
    if c.protocol.digest_alg != "blake2b-256" {
        return Err(SpecError::Validation(
            "protocol.digest_alg must be blake2b-256".into(),
        ));
    }
    if c.protocol.canon_profile != "bitscrow-jcs-v0" {
        return Err(SpecError::Validation(
            "protocol.canon_profile must be bitscrow-jcs-v0".into(),
        ));
    }
    if c.protocol.version.is_empty() {
        return Err(SpecError::Validation(
            "protocol.version is required".into(),
        ));
    }
    check_sats(
        "funding.required_total_sats",
        c.funding.required_total_sats,
    )?;
    for p in &c.participants {
        check_sats(
            &format!("participant {}.required_contribution_sats", p.id),
            p.required_contribution_sats,
        )?;
        check_sats(
            &format!("participant {}.collateral_sats", p.id),
            p.collateral_sats,
        )?;
    }
    for cm in &c.commitments {
        if cm.plaintext_secret.is_some() {
            return Err(SpecError::Validation(
                "plaintext_secret must be null".into(),
            ));
        }
    }
    check_sats("execution.compute_budget_sats", c.execution.compute_budget_sats)?;
    check_sats("execution.storage_budget_sats", c.execution.storage_budget_sats)?;
    for o in &c.outcomes {
        for a in &o.allocations {
            if let crate::model::Amount::Sats(n) = a.amount {
                check_sats(&format!("outcome {}.allocation sats", o.id), n)?;
            }
        }
    }
    Ok(())
}

fn check_sats(label: &str, n: u64) -> Result<(), SpecError> {
    if n > MAX_SATS {
        return Err(SpecError::Validation(format!(
            "{label} exceeds 2^53-1 JSON-safe max"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_key_fixture_rejected() {
        let dirty = include_str!("../../../docs/fixtures/invalid_dirty.yaml");
        assert_eq!(parse_yaml(dirty), Err(SpecError::YamlMergeKey));
    }
}

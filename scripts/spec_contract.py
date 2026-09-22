# File: scripts/spec_contract.py
# Purpose: Assemble the v0.1 JSON object from YAML using the field helpers.
# Related: scripts/spec_model.py, crates/bitscrow-spec/src/model.rs
# Tags: #spec #model #crosscheck
#
# Why: keep the fixture mapping next to the Rust struct field names.
# Side effects: none.

"""ContractV01 JSON object for the valid fixture."""

from __future__ import annotations

from spec_model import (
    amount,
    fail,
    flag,
    json_value,
    mapping,
    null_secret,
    opt_text,
    rows,
    strings,
    take,
    text,
    u64,
)


def contract_from_yaml(raw: object) -> dict:
    """JSON object matching serde Serialize of ContractV01. Nulls are kept."""
    doc = mapping(
        raw,
        "contract",
        {
            "protocol",
            "metadata",
            "chain_anchor",
            "participants",
            "funding",
            "evidence",
            "commitments",
            "oracles",
            "outcomes",
            "state",
            "head_resolution",
            "execution",
            "signatures",
        },
    )
    protocol = take(
        doc["protocol"],
        "protocol",
        {
            "name": text,
            "version": text,
            "network": text,
            "digest_alg": text,
            "canon_profile": text,
            "contract_id": text,
            "contract_type": text,
        },
    )
    if protocol["digest_alg"] != "blake2b-256":
        fail("protocol.digest_alg must be blake2b-256")
    if protocol["canon_profile"] != "bitscrow-jcs-v0":
        fail("protocol.canon_profile must be bitscrow-jcs-v0")
    funding = mapping(
        doc["funding"],
        "funding",
        {"currency", "required_total_sats", "funding_policy", "outpoints"},
    )
    evidence = mapping(doc["evidence"], "evidence", {"items"})
    execution = mapping(
        doc["execution"],
        "execution",
        {
            "reference_implementation",
            "interpreter",
            "compute_budget_sats",
            "storage_budget_sats",
        },
    )
    return {
        "protocol": protocol,
        "metadata": take(
            doc["metadata"],
            "metadata",
            {"created_at": text, "expires_at": text, "title": text, "description": text},
        ),
        "chain_anchor": take(
            doc["chain_anchor"],
            "chain_anchor",
            {
                "block_height": u64,
                "block_hash": text,
                "confirmations_required": u64,
            },
        ),
        "participants": rows(
            doc["participants"],
            "participants",
            {
                "id": text,
                "role": text,
                "signing_public_key": text,
                "settlement_address": text,
                "refund_address": text,
                "required_contribution_sats": u64,
                "collateral_sats": u64,
            },
        ),
        "funding": {
            "currency": text(funding["currency"], "funding.currency"),
            "required_total_sats": u64(funding["required_total_sats"], "required_total_sats"),
            "funding_policy": take(
                funding["funding_policy"], "funding_policy", {"activation": text}
            ),
            "outpoints": json_value(funding["outpoints"], "outpoints"),
        },
        "evidence": {
            "items": rows(
                evidence["items"],
                "evidence",
                {
                    "id": text,
                    "required": flag,
                    "allowed_submitters": strings,
                    "content_hash": opt_text,
                    "storage_reference": opt_text,
                    "media_type": opt_text,
                },
            )
        },
        "commitments": rows(
            doc["commitments"],
            "commitments",
            {
                "id": text,
                "owner": text,
                "algorithm": text,
                "commitment": text,
                "plaintext_secret": null_secret,
            },
        ),
        "oracles": rows(
            doc["oracles"],
            "oracles",
            {
                "id": text,
                "subject": text,
                "primary": json_value,
                "fallback": json_value,
                "total_failure": json_value,
            },
        ),
        "outcomes": _outcomes(doc["outcomes"]),
        "state": take(
            doc["state"],
            "state",
            {
                "genesis_state_hash": opt_text,
                "previous_state_hash": opt_text,
                "state_version": u64,
                "status": text,
            },
        ),
        "head_resolution": take(
            doc["head_resolution"],
            "head_resolution",
            {"resolver_scheme": text, "resolver_uri": text},
        ),
        "execution": {
            "reference_implementation": text(
                execution["reference_implementation"], "reference_implementation"
            ),
            "interpreter": take(
                execution["interpreter"],
                "interpreter",
                {"protocol_version": text, "artifact_digest": opt_text},
            ),
            "compute_budget_sats": u64(execution["compute_budget_sats"], "compute_budget_sats"),
            "storage_budget_sats": u64(execution["storage_budget_sats"], "storage_budget_sats"),
        },
        "signatures": rows(
            doc["signatures"],
            "signatures",
            {"participant": text, "signature": opt_text},
        ),
    }


def _outcomes(node: object) -> list:
    if not isinstance(node, list):
        fail("outcomes must be a list")
    built = []
    for index, item in enumerate(node):
        row = mapping(item, f"outcomes[{index}]", {"id", "when", "allocations", "terminal"})
        built.append(
            {
                "id": text(row["id"], "outcome.id"),
                "when": json_value(row["when"], "outcome.when"),
                "allocations": rows(
                    row["allocations"],
                    "allocations",
                    {"participant": text, "amount": amount, "destination": text},
                ),
                "terminal": flag(row["terminal"], "outcome.terminal"),
            }
        )
    return built

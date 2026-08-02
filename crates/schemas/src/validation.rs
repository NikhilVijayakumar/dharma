//! Schema validation for dharma's contract documents.
//!
//! Two responsibilities (proposal 08's Hard Constraints):
//! 1. `task.input_contract_json` / `output_contract_json` are JSON Schema
//!    documents — validate them as such, and validate a runtime value against
//!    them (`validate_value_against_schema`).
//! 2. `task.acceptance_criteria_json` must have at least one entry in EACH of
//!    the three tiers (`happy_path`, `corner_case`, `edge_case`) before a Task
//!    can be assigned — enforced here, not by a CHECK constraint (structure
//!    varies).
//!
//! Also validates Skill bundles at capture time (proposal 03): a Skill must
//! carry a prompt and at least one example; `is_analysis_only` is checked by
//! the Proposal Loop before invocation during drafting.

use anyhow::{bail, Context, Result};

pub const ACCEPTANCE_CRITERIA_TIERS: [&str; 3] = ["happy_path", "corner_case", "edge_case"];

/// Validate that `criteria` is a JSON object with each of the three
/// acceptance-criteria tiers (`happy_path`/`corner_case`/`edge_case`) present
/// and holding at least one entry. Fails otherwise.
pub fn validate_acceptance_criteria(criteria: &serde_json::Value) -> Result<()> {
    let obj = criteria.as_object().ok_or_else(|| {
        anyhow::anyhow!("acceptance_criteria must be a JSON object, got {}", criteria)
    })?;
    for tier in ACCEPTANCE_CRITERIA_TIERS {
        let arr = obj.get(tier).and_then(|v| v.as_array()).ok_or_else(|| {
            anyhow::anyhow!("acceptance_criteria missing array tier '{tier}'")
        })?;
        if arr.is_empty() {
            bail!("acceptance_criteria tier '{tier}' must have at least one entry");
        }
    }
    Ok(())
}

/// Validate that `doc` is a valid JSON Schema document (object with a `type`
/// or `$schema` present). Does not fully compile it — that happens lazily in
/// [`validate_value_against_schema`].
pub fn validate_json_schema_document(doc: &serde_json::Value) -> Result<()> {
    let obj = doc.as_object().ok_or_else(|| {
        anyhow::anyhow!("contract must be a JSON Schema object, got {}", doc)
    })?;
    let has_type = obj.get("type").is_some();
    let has_schema = obj.get("$schema").is_some();
    if !has_type && !has_schema {
        bail!("contract document is not a JSON Schema (no 'type' or '$schema' field)");
    }
    Ok(())
}

/// Validate `value` against a compiled JSON Schema document. Compiles the
/// schema per call — contracts are small and this keeps the API dependency-free
/// for callers that only want a pass/fail.
pub fn validate_value_against_schema(
    value: &serde_json::Value,
    schema_doc: &serde_json::Value,
) -> Result<()> {
    validate_json_schema_document(schema_doc)?;
    let compiled = jsonschema::validator_for(schema_doc)
        .with_context(|| "Failed to compile JSON Schema contract")?;
    let result = compiled.validate(value);
    match result {
        Ok(_) => Ok(()),
        Err(error) => bail!("Value violates contract: {}", error),
    }
}

/// Validate a Skill bundle shape at capture time (proposal 03): a mandatory
/// prompt and at least one worked example. `script` and `template` are
/// optional. Returns the declared effect classification (`analysis_only`).
pub fn validate_skill_bundle(
    prompt: Option<&str>,
    examples: usize,
    script: Option<&str>,
    template: Option<&str>,
) -> Result<bool> {
    let prompt = prompt
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Skill bundle missing mandatory prompt"))?;
    if examples == 0 {
        bail!("Skill bundle requires at least one worked example");
    }
    // Prompt is mandatory, so a bundle is only analysis-only when it has no
    // script and no template that could carry an effect.
    let analysis_only = script.map(str::trim).filter(|s| !s.is_empty()).is_none()
        && template.map(str::trim).filter(|s| !s.is_empty()).is_none();
    let _ = prompt;
    Ok(analysis_only)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn acceptance_criteria_valid_when_all_tiers_non_empty() {
        let v = json!({
            "happy_path": ["a"],
            "corner_case": ["b"],
            "edge_case": ["c"],
        });
        assert!(validate_acceptance_criteria(&v).is_ok());
    }

    #[test]
    fn acceptance_criteria_rejects_empty_tier() {
        let v = json!({
            "happy_path": ["a"],
            "corner_case": [],
            "edge_case": ["c"],
        });
        assert!(validate_acceptance_criteria(&v).is_err());
    }

    #[test]
    fn acceptance_criteria_rejects_missing_tier() {
        let v = json!({ "happy_path": ["a"] });
        assert!(validate_acceptance_criteria(&v).is_err());
    }

    #[test]
    fn contract_validates_value() {
        let schema = json!({
            "type": "object",
            "properties": { "name": { "type": "string" } },
            "required": ["name"],
        });
        assert!(validate_value_against_schema(&json!({"name": "x"}), &schema).is_ok());
        assert!(validate_value_against_schema(&json!({"name": 5}), &schema).is_err());
        assert!(validate_value_against_schema(&json!({}), &schema).is_err());
    }

    #[test]
    fn contract_document_rejected_without_type() {
        assert!(validate_json_schema_document(&json!({"properties": {}})).is_err());
        assert!(validate_json_schema_document(&json!({"type": "object"})).is_ok());
    }

    #[test]
    fn skill_bundle_requires_prompt_and_example() {
        assert!(validate_skill_bundle(Some("prompt"), 1, None, None).is_ok());
        assert!(validate_skill_bundle(None, 1, None, None).is_err());
        assert!(validate_skill_bundle(Some("prompt"), 0, None, None).is_err());
    }

    #[test]
    fn skill_bundle_classifies_analysis_only() {
        assert_eq!(validate_skill_bundle(Some("p"), 1, None, None).unwrap(), true);
        assert_eq!(validate_skill_bundle(Some("p"), 1, Some("x.py"), None).unwrap(), false);
    }
}

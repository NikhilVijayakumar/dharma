//! Dharma MCP wire protocol (docs/proposal/14-mcp-tool-contract.md).
//!
//! Every call carries an explicit `repo_path` — repositories are never
//! bound to a session (proposal 14, Hard Requirement 2). Gate methods
//! (human-approval tools) additionally require `human_approved: true` and a
//! non-empty `reviewed_by`; those methods reject otherwise.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub const PROTOCOL_VERSION: &str = "0.1.0";

/// The 28 tools across 5 concern groups (proposal 14). Order = registry &
/// capture, repo registration & sync, task execution, audit, proposal
/// lifecycle.
pub const TOOL_METHODS: &[&str] = &[
    // registry & capture
    "register_domain_system",
    "register_agent_system",
    "recapture_domain_system",
    "recapture_agent_system",
    "list_domain_systems",
    "list_agent_systems",
    "get_domain_system_info",
    "get_agent_system_info",
    // repo registration & sync
    "register_repo",
    "list_repos",
    "repo_status",
    "unregister_repo",
    "review_capability_manifest",
    "sync_repo",
    "get_repo_config",
    // task execution
    "assign_task",
    "submit_proposal_draft",
    "review_task_proposal",
    "request_handoff",
    "run_skill",
    "submit_completion_validation",
    "task_instance_status",
    // audit
    "run_audit",
    "get_audit_result",
    "override_audit",
    // proposal lifecycle
    "advance_proposal_lifecycle",
    "log_proposal_commit",
    "get_proposal_lifecycle",
];

/// Methods that mutate an approval / lifecycle state machine and therefore
/// require `human_approved: true` + a non-empty `reviewed_by` in `params`.
pub const GATE_METHODS: &[&str] = &[
    "review_capability_manifest",
    "review_task_proposal",
    "advance_proposal_lifecycle",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpMessage {
    Request(McpRequest),
    Response(McpResponse),
    Notification(McpNotification),
    Error(McpError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub params: HashMap<String, Value>,
    /// Explicit per-call repository root. Required by every tool that
    /// touches repo.db; registry/capture tools ignore it.
    #[serde(default)]
    pub repo_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub id: String,
    #[serde(default)]
    pub result: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNotification {
    pub method: String,
    #[serde(default)]
    pub params: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    #[serde(default)]
    pub id: Option<String>,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCapabilities {
    pub version: String,
    pub methods: Vec<String>,
    pub protocol_version: String,
}

impl Default for McpCapabilities {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            methods: TOOL_METHODS.iter().map(|s| s.to_string()).collect(),
            protocol_version: PROTOCOL_VERSION.to_string(),
        }
    }
}

impl McpRequest {
    pub fn get_str(&self, key: &str) -> anyhow::Result<Option<String>> {
        match self.params.get(key) {
            Some(Value::String(s)) => Ok(Some(s.clone())),
            Some(Value::Null) | None => Ok(None),
            Some(v) => Err(anyhow::anyhow!(
                "param '{}' must be a string, got {}",
                key,
                v
            )),
        }
    }

    pub fn require_str(&self, key: &str) -> anyhow::Result<String> {
        self.get_str(key)?
            .ok_or_else(|| anyhow::anyhow!("missing required param '{}'", key))
    }

    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        match self.params.get(key) {
            Some(Value::Bool(b)) => *b,
            _ => default,
        }
    }

    pub fn get_i64(&self, key: &str) -> anyhow::Result<Option<i64>> {
        match self.params.get(key) {
            Some(Value::Number(n)) => n
                .as_i64()
                .map(Some)
                .ok_or_else(|| anyhow::anyhow!("param '{}' is not an integer", key)),
            Some(Value::Null) | None => Ok(None),
            Some(v) => Err(anyhow::anyhow!(
                "param '{}' must be an integer, got {}",
                key,
                v
            )),
        }
    }

    pub fn require_i64(&self, key: &str) -> anyhow::Result<i64> {
        self.get_i64(key)?
            .ok_or_else(|| anyhow::anyhow!("missing required param '{}'", key))
    }

    pub fn get_value(&self, key: &str) -> anyhow::Result<Option<Value>> {
        match self.params.get(key) {
            Some(Value::Null) | None => Ok(None),
            Some(v) => Ok(Some(v.clone())),
        }
    }

    pub fn require_value(&self, key: &str) -> anyhow::Result<Value> {
        self.get_value(key)?
            .ok_or_else(|| anyhow::anyhow!("missing required param '{}'", key))
    }
}

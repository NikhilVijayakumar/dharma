use serde::{Deserialize, Serialize};

/// Newtype for a repo UUID — mirrors samgraha's id conventions. Not a strong
/// invariant carrier; `repo_registration.repo_uuid` is a TEXT UNIQUE column.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoUuid(pub String);

/// Newtype for a domain system id (mcp.db `domain_system_registry.id`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DomainSystemId(pub i64);

/// Newtype for an agent system id (mcp.db `agent_system_registry.id`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentSystemId(pub i64);

/// Newtype for a task instance id (repo.db `task_instance.id`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskInstanceId(pub i64);

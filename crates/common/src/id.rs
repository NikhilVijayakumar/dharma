use serde::{Deserialize, Serialize};

/// Newtype for a repo UUID — mirrors samgraha's id conventions. Not a strong
/// invariant carrier; `repo_registration.repo_uuid` is a TEXT UNIQUE column.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoUuid(pub String);

impl std::fmt::Display for RepoUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Newtype for a domain system id (mcp.db `domain_system_registry.id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DomainSystemId(pub i64);

impl std::fmt::Display for DomainSystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Newtype for an agent system id (mcp.db `agent_system_registry.id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentSystemId(pub i64);

impl std::fmt::Display for AgentSystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Newtype for a task instance id (repo.db `task_instance.id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskInstanceId(pub i64);

impl std::fmt::Display for TaskInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

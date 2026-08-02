-- mcp.db — one row per registered Agent System (e.g. `documentation-
-- management`, `rust-development`, see docs/proposal/04-agent-system-
-- registry.md). Parallel in kind to domain_system_registry (00), never
-- merged with it. `concern` is UNIQUE so task_step.required_capability
-- (in this same db) can be a real FOREIGN KEY against it, not a
-- soft-matched string. `is_privileged` marks the Agent-Management and
-- Default/Bootstrap Agent Systems.

CREATE TABLE IF NOT EXISTS agent_system_registry (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL UNIQUE,
    concern       TEXT    NOT NULL UNIQUE,
    description   TEXT    NOT NULL DEFAULT '',
    is_privileged INTEGER NOT NULL DEFAULT 0,  -- 1 for Agent-Management / Default-Bootstrap
    registered_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_agent_system_registry_privileged ON agent_system_registry(is_privileged);

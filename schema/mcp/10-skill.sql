-- mcp.db — one Agent System's Skills (see docs/proposal/03-skill-model.md,
-- docs/proposal/04-agent-system-registry.md). Scoped by a real
-- `agent_system_id` FOREIGN KEY into agent_system_registry (01). Written
-- only by the Agent-Management Agent System.
--
-- `skill.name` is unique per Agent System, not globally — two different
-- Agent Systems may each register a Skill of the same name without conflict.
--
-- `is_analysis_only` is checked by the Proposal Loop before allowing
-- invocation during drafting (see docs/proposal/07-proposal-execution-
-- protocol.md) — set by the Agent-Management Agent System at
-- registration, never by the Skill's own author unchecked.
CREATE TABLE IF NOT EXISTS skill (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_system_id         INTEGER NOT NULL REFERENCES agent_system_registry(id) ON DELETE CASCADE,
    name                    TEXT    NOT NULL,
    responsibility          TEXT    NOT NULL,
    is_analysis_only        INTEGER NOT NULL DEFAULT 0,
    invocation_input_json   TEXT    NOT NULL,  -- JSON Schema
    invocation_output_json  TEXT    NOT NULL,  -- JSON Schema
    created_at              TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(agent_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_skill_agent_system ON skill(agent_system_id);
CREATE INDEX IF NOT EXISTS idx_skill_analysis_only ON skill(is_analysis_only);

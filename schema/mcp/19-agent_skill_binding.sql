-- mcp.db — the allowlist of Skills an Agent is permitted to invoke (see docs/
-- proposal/01-agent-model.md, docs/proposal/03-skill-model.md). The
-- Agent-Management Agent System must only ever pair an Agent with a
-- Skill from the SAME agent_system_id — a service-layer invariant, not
-- expressible as a SQLite CHECK across two joined tables.
CREATE TABLE IF NOT EXISTS agent_skill_binding (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL REFERENCES agent(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    UNIQUE(agent_id, skill_id)
);
CREATE INDEX IF NOT EXISTS idx_agent_skill_binding_agent ON agent_skill_binding(agent_id);

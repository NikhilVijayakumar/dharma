-- mcp.db — one Agent System's Agents (see docs/proposal/01-agent-model.md,
-- docs/proposal/04-agent-system-registry.md). Scoped by a real
-- `agent_system_id` FOREIGN KEY into agent_system_registry (01) — all Agent
-- Systems' content shares this one db, distinguished by that column.
-- Written only by the Agent-Management Agent System.
--
-- `agent.name` is unique per Agent System, not globally — two different
-- Agent Systems may each register an Agent of the same name without conflict.
-- `content_asset_id` traces the `agent.yaml` capture this row was parsed
-- from (shape per Bodha crew `agent.yaml`: role, numbered goals — see
-- agent_goal, 13 — backstory, plus Dharma's handoff fields below).

CREATE TABLE IF NOT EXISTS agent (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_system_id           INTEGER NOT NULL REFERENCES agent_system_registry(id) ON DELETE CASCADE,
    name                      TEXT    NOT NULL,
    role                      TEXT    NOT NULL,
    handoff_trigger_condition TEXT    NOT NULL DEFAULT '',
    handoff_candidate_role    TEXT    NOT NULL DEFAULT '',  -- free text; fuzzy-matched at handoff time, not an enum/FK
    content_asset_id          INTEGER NOT NULL REFERENCES content_asset(id),  -- the agent.yaml capture
    created_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(agent_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_agent_agent_system ON agent(agent_system_id);

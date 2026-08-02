-- mcp.db — one Agent's goal set (see docs/proposal/01-agent-model.md).
-- Scoped by `agent_id` into agent (08); the owning Agent is itself scoped to
-- an Agent System by `agent.agent_system_id`, so no `agent_system_id` column
-- is needed here.
--
-- `goal_order` is checked 1..8 to enforce the eight-goal cap (see docs/
-- proposal/01-agent-model.md) directly in the schema. `backstory` is
-- mandatory per goal, not a single free-text field on `agent`.
CREATE TABLE IF NOT EXISTS agent_goal (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id   INTEGER NOT NULL REFERENCES agent(id) ON DELETE CASCADE,
    goal_order INTEGER NOT NULL CHECK (goal_order BETWEEN 1 AND 8),
    goal_text  TEXT    NOT NULL,
    backstory  TEXT    NOT NULL,
    UNIQUE(agent_id, goal_order)
);
CREATE INDEX IF NOT EXISTS idx_agent_goal_agent ON agent_goal(agent_id);

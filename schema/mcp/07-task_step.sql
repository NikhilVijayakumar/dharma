-- mcp.db — one Task's ordered execution steps (see docs/proposal/02-task-model.md,
-- docs/proposal/07-proposal-execution-protocol.md). Scoped by `task_id` into
-- task (06).
--
-- `required_capability` names the concern a step needs (e.g. `rust-
-- development`) and is a real FOREIGN KEY against agent_system_registry
-- .concern (01) now that both tables share this one db — resolved to a
-- specific Agent and Agent System at runtime by the Handoff Broker,
-- never hard-coded to a specific Agent instance here (see docs/proposal/
-- 04-agent-system-registry.md, 07-proposal-execution-protocol.md).
CREATE TABLE IF NOT EXISTS task_step (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id             INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    step_order          INTEGER NOT NULL,
    description         TEXT    NOT NULL DEFAULT '',
    required_capability TEXT    NOT NULL REFERENCES agent_system_registry(concern),
    UNIQUE(task_id, step_order)
);
CREATE INDEX IF NOT EXISTS idx_task_step_task ON task_step(task_id);

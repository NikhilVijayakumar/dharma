-- repo.db — tracks the Execution Loop's current position for a Task
-- Instance: which step it is on and which Agent currently owns it (see
-- docs/proposal/07-proposal-execution-protocol.md). At most one row per
-- task_instance; updated in place as the Task progresses (the
-- append-only history lives in handoff_log, 04). `task_instance_id` is a
-- real FK (same repo.db file); the owning-Agent columns are logical
-- references into mcp.db (see 00-task_instance.sql).

CREATE TABLE IF NOT EXISTS execution_state (
    id                              INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id                INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    current_step_order              INTEGER NOT NULL DEFAULT 1,
    current_owning_agent_system_id  INTEGER NOT NULL,  -- logical ref: mcp.db agent_system_registry(id)
    current_owning_agent_id         INTEGER NOT NULL,  -- logical ref: mcp.db agent(id)
    updated_at                      TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

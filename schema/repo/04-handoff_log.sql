-- repo.db — append-only record of every control transfer during the
-- Execution Loop, one row per hop (see docs/proposal/07-proposal-
-- execution-protocol.md). Used by the Handoff Broker for loop/depth
-- detection: a cycle or a depth beyond the configured maximum fails the
-- Task rather than looping forever. `task_instance_id` is a real FK
-- (same repo.db file); `from`/`to` agent columns are logical references
-- into mcp.db, each a compound (agent_system_id, agent_id) pair — a
-- handoff routinely crosses Agent Systems (see docs/proposal/04-agent-
-- system-registry.md), so a bare agent_id cannot say which Agent System
-- it names.

CREATE TABLE IF NOT EXISTS handoff_log (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id      INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    hop_order             INTEGER NOT NULL,
    from_agent_system_id  INTEGER NOT NULL,  -- logical ref: mcp.db agent_system_registry(id)
    from_agent_id         INTEGER NOT NULL,  -- logical ref: mcp.db agent(id)
    to_agent_system_id    INTEGER NOT NULL,  -- logical ref: mcp.db agent_system_registry(id)
    to_agent_id           INTEGER NOT NULL,  -- logical ref: mcp.db agent(id)
    reason                TEXT    NOT NULL DEFAULT '',
    accepted              INTEGER NOT NULL DEFAULT 0,
    created_at            TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, hop_order)
);
CREATE INDEX IF NOT EXISTS idx_handoff_log_task_instance ON handoff_log(task_instance_id);

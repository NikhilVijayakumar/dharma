-- repo.db — one row per draft/revision cycle in the Proposal Loop (see
-- docs/proposal/07-proposal-execution-protocol.md). Append-only: a
-- revision is never edited or deleted, only superseded by a new row with
-- a higher `revision_number`, so the Context Envelope's history stays
-- intact. `task_instance_id` is a real FK (same repo.db file).
--
-- `drafted_by_agent_system_id` + `drafted_by_agent_id` are logical
-- references into mcp.db (see 00-task_instance.sql for why the compound
-- pair, rather than a bare id, is required).

CREATE TABLE IF NOT EXISTS proposal_revision (
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id           INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    revision_number            INTEGER NOT NULL,
    drafted_by_agent_system_id INTEGER NOT NULL,  -- logical ref: mcp.db agent_system_registry(id)
    drafted_by_agent_id        INTEGER NOT NULL,  -- logical ref: mcp.db agent(id)
    draft_json                 TEXT    NOT NULL,  -- intended Agents/Skills, expected handoffs, how Acceptance Criteria will be met
    reviewer_comments          TEXT,
    created_at                 TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, revision_number)
);
CREATE INDEX IF NOT EXISTS idx_proposal_revision_task_instance ON proposal_revision(task_instance_id);

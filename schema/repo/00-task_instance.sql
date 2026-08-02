-- repo.db — one instance of this file per registered repository, living
-- inside that repository (path recorded in mcp.db's repo_registration.
-- repo_db_path, see schema/mcp/15-repo_registration.sql — mirrors
-- samgraha's per-repo knowledge.db). Because this file is already scoped
-- to one
-- repository, there is no `repo_registration_id` column here — unlike
-- the earlier shared-execution.db design, which needed one to
-- disambiguate rows from different repos, that ambiguity no longer
-- exists once each repo gets its own file.
--
-- One row per attempt at a Task assigned to this repository (see docs/
-- proposal/07-proposal-execution-protocol.md). `task_id` is a logical
-- reference into mcp.db's task(id) — not an enforced FK, since mcp.db and
-- this repo.db are separate physical database files (the only remaining
-- cross-database boundary in this schema; see schema/README.md).
--
-- `initiating_agent_system_id` + `initiating_agent_id` together name the
-- Agent that the Task Runtime first assigned this instance to — both are
-- logical references into mcp.db (agent_system_registry(id), agent(id)),
-- for the same reason: Agent ids are only unique within their Agent
-- System, and a Task may hand off across Agent Systems (see docs/
-- proposal/04-agent-system-registry.md), so a bare id would be
-- ambiguous about which Agent System it names.
--
-- `status` tracks which of the two loops (Proposal, Execution) the
-- instance is in; a row can only reach 'executing' after
-- proposal_approval (02) records an approval.
--
-- A 'failed' row is terminal — it is never reworked in place. Rework
-- creates a NEW task_instance row for the same `task_id`, linked back via
-- `previous_task_instance_id` and a bumped `attempt_number`. This is what
-- lets completion_validation (06) keep a plain UNIQUE(task_instance_id):
-- one verdict per attempt, and an attempt is one task_instance row.

CREATE TABLE IF NOT EXISTS task_instance (
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id                     INTEGER NOT NULL,  -- logical ref: mcp.db task(id)
    initiating_agent_system_id  INTEGER NOT NULL,  -- logical ref: mcp.db agent_system_registry(id)
    initiating_agent_id         INTEGER NOT NULL,  -- logical ref: mcp.db agent(id)
    attempt_number              INTEGER NOT NULL DEFAULT 1,
    previous_task_instance_id   INTEGER REFERENCES task_instance(id),  -- set when this attempt reworks a failed one
    status                      TEXT    NOT NULL DEFAULT 'proposing'
                                CHECK (status IN ('proposing','approved','executing','complete','failed')),
    created_at                  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                  TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_task_instance_status ON task_instance(status);
CREATE INDEX IF NOT EXISTS idx_task_instance_task ON task_instance(task_id);

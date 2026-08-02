-- repo.db — records the single user approval event that moves a Task
-- Instance from the Proposal Loop into the Execution Loop (see docs/
-- proposal/07-proposal-execution-protocol.md). At most one row per
-- task_instance — a Task cannot be re-approved once approved; rework
-- after a failure is a new task_instance row (see 00-task_instance.sql),
-- not a re-approval of this one. Both FKs are real (same repo.db file).

CREATE TABLE IF NOT EXISTS proposal_approval (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id     INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    approved_revision_id INTEGER NOT NULL REFERENCES proposal_revision(id),
    approved_by          TEXT    NOT NULL,
    approved_at          TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

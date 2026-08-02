-- repo.db — the Completion Validator's verdict for a Task Instance,
-- checked against the Task's acceptance_criteria_json (mcp.db,
-- 06-task.sql), independent of whichever Agent executed the
-- steps (see docs/proposal/07-proposal-execution-protocol.md). Exactly
-- one row per task_instance, never overwritten: a 'fail' is terminal for
-- that task_instance (see 00-task_instance.sql's status CHECK —
-- 'failed' is an end state). Rework does not reopen this row or its
-- task_instance; it creates a new task_instance row (bumped attempt_
-- number, linked via previous_task_instance_id) which gets its own
-- completion_validation row when it in turn finishes. `task_instance_id`
-- is a real FK (same repo.db file).

CREATE TABLE IF NOT EXISTS completion_validation (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    verdict          TEXT    NOT NULL CHECK (verdict IN ('pass','fail')),
    reason           TEXT    NOT NULL DEFAULT '',
    validated_at     TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

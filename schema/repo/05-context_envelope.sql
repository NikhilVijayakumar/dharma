-- repo.db — append-only accumulated context carried across both the
-- Proposal Loop and the Execution Loop (see docs/proposal/07-proposal-
-- execution-protocol.md). One row per hop (proposal revision or
-- execution handoff); a hop's payload is never rewritten, only extended
-- by the next row — mitigates Context Envelope tampering. `task_
-- instance_id` is a real FK (same repo.db file).
--
-- `hop_kind` distinguishes a proposal-drafting hop from an execution
-- handoff hop; without it, two rows sharing one `hop_order` sequence
-- can't say which loop produced them.

CREATE TABLE IF NOT EXISTS context_envelope (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    hop_order        INTEGER NOT NULL,
    hop_kind         TEXT    NOT NULL CHECK (hop_kind IN ('proposal','execution')),
    payload_json     TEXT    NOT NULL,
    created_at       TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, hop_order)
);
CREATE INDEX IF NOT EXISTS idx_context_envelope_task_instance ON context_envelope(task_instance_id);

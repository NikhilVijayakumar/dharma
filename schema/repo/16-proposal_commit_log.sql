-- repo.db — append-only log of every commit touching a proposal's
-- lifecycle, one row per commit (see docs/raw/proposal.md, docs/proposal/
-- 12-proposal-lifecycle-and-archival.md). `proposal_lifecycle` (15) keeps
-- only the milestone snapshot (draft/finalized/implementation/archive
-- commit); this table is the full history behind that snapshot — every
-- commit made while a proposal was being drafted, finalized, or implemented, so
-- "what actually happened between finalized and verified" is always
-- replayable, not just the two endpoints.

CREATE TABLE IF NOT EXISTS proposal_commit_log (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    proposal_lifecycle_id INTEGER NOT NULL REFERENCES proposal_lifecycle(id) ON DELETE CASCADE,
    commit_hash         TEXT    NOT NULL,
    phase               TEXT    NOT NULL CHECK (phase IN ('draft','finalized','implementation','archive')),
    message             TEXT    NOT NULL DEFAULT '',
    recorded_at         TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(proposal_lifecycle_id, commit_hash)
);
CREATE INDEX IF NOT EXISTS idx_proposal_commit_log_proposal ON proposal_commit_log(proposal_lifecycle_id);

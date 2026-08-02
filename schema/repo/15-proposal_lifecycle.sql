-- repo.db — one row per Proposal document (see docs/raw/proposal.md,
-- docs/proposal/12-proposal-lifecycle-and-archival.md) tracked for THIS
-- repository. Lives in repo.db, not mcp.db, for the same reason audit
-- executions do (proposal 08, "Audit Results Split Between mcp.db and
-- repo.db"): a proposal's lifecycle is a per-repo, per-commit observation
-- (this repo's own git history), not global platform content — the
-- Proposal Standard itself (docs/raw/proposal.md) is the reusable,
-- global part, and that's a file, not a DB row, exactly like
-- architecture.md/crates.md are.
--
-- `status` only ever moves forward: draft -> finalized -> implementing ->
-- verified -> archived. `doc_path` is this repository's own path (e.g.
-- `docs/proposal/12-....md`, or `docs/proposal/archive/12-....md` once
-- archived) — relative to the repo root, not to mcp.db.
--
-- Commit hash columns are the milestone snapshot; the full intermediate
-- history (every commit touched while drafting or implementing) is in
-- proposal_commit_log (16), which this table's milestones are a summary
-- of, not a replacement for.

CREATE TABLE IF NOT EXISTS proposal_lifecycle (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    name                      TEXT    NOT NULL UNIQUE,  -- matches the proposal's filename slug, e.g. "12-proposal-lifecycle-and-archival"
    title                     TEXT    NOT NULL,
    doc_path                  TEXT    NOT NULL,
    status                    TEXT    NOT NULL DEFAULT 'draft'
                              CHECK (status IN ('draft','finalized','implementing','verified','archived')),
    draft_commit_hash         TEXT,  -- first commit that introduced this proposal document
    finalized_commit_hash     TEXT,  -- commit at which the proposal was locked for implementation
    implementation_commit_hash TEXT,  -- final commit, once implementation is complete AND verified
    verified_at               TEXT,
    archive_commit_hash       TEXT,  -- commit that moved the file into docs/proposal/archive/
    archived_at               TEXT,
    created_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    CHECK (
        -- forward chain: a status can't outrun the milestone commits that
        -- justify it. Each status requires EVERY prior milestone — verified
        -- is unreachable without the draft/finalized design commits,
        -- archived is unreachable without verified_at. No skipping.
        (status = 'draft')
        OR (status = 'finalized' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL)
        OR (status = 'implementing' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL)
        OR (status = 'verified' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL
            AND implementation_commit_hash IS NOT NULL AND verified_at IS NOT NULL)
        OR (status = 'archived' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL
            AND implementation_commit_hash IS NOT NULL AND verified_at IS NOT NULL
            AND archive_commit_hash IS NOT NULL AND archived_at IS NOT NULL)
    )
);
CREATE INDEX IF NOT EXISTS idx_proposal_lifecycle_status ON proposal_lifecycle(status);

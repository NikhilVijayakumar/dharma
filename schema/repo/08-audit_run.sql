-- repo.db — one audit execution, keyed by `(commit_hash, domain, kind)`
-- (see docs/proposal/08-schema-and-crate-architecture.md, "Audit
-- Subsystem"). Same-model-same-commit de-duplication lives here: re-running
-- the same audit agent with the same model on the same commit does not
-- create a new run — the UNIQUE key rejects a duplicate, and an
-- audit_override (13) with action 'cancel' retires a run. `domain_id` is a
-- logical reference into mcp.db's domain(id); `domain_name` is a display
-- snapshot (the domain could be re-versioned in mcp.db). `status`:
-- 'running' while executing, 'complete' when results are persisted,
-- 'cancelled' when retired by an override.

CREATE TABLE IF NOT EXISTS audit_run (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    commit_hash  TEXT    NOT NULL,
    domain_id    INTEGER NOT NULL,  -- logical ref: mcp.db domain(id)
    domain_name  TEXT    NOT NULL,
    kind         TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic')),
    status       TEXT    NOT NULL DEFAULT 'running'
                  CHECK (status IN ('running','complete','cancelled')),
    created_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    UNIQUE(commit_hash, domain_id, kind)
);
CREATE INDEX IF NOT EXISTS idx_audit_run_commit ON audit_run(commit_hash);
CREATE INDEX IF NOT EXISTS idx_audit_run_status ON audit_run(status);

-- repo.db — human override or cancel of an audit result (see docs/proposal/
-- 08-schema-and-crate-architecture.md, "Audit Subsystem"). `target` selects
-- what is overridden: `run` (the whole audit_run), `deterministic` (the
-- deterministic score, 09), or `semantic:<model>` (one model's score, 10).
-- `action` is `override` (replace the score; sets `override_score`) or
-- `cancel` (retire the run — the de-duplication escape hatch for a
-- same-commit re-run). Every override records `reason` and `reviewed_by`
-- for auditability.

CREATE TABLE IF NOT EXISTS audit_override (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id   INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    target         TEXT    NOT NULL DEFAULT 'run',  -- 'run' | 'deterministic' | 'semantic:<model>'
    action         TEXT    NOT NULL CHECK (action IN ('override','cancel')),
    override_score REAL,  -- set when action='override'
    reason         TEXT    NOT NULL DEFAULT '',
    reviewed_by    TEXT    NOT NULL DEFAULT '',
    created_at     TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_audit_override_run ON audit_override(audit_run_id);

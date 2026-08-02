-- repo.db — one per-model score within a semantic audit_run (08). The
-- ensemble's models each produce one row here; `reliability_aware_ensemble`
-- (mcp.db, audit_calculation) merges them. `UNIQUE(audit_run_id, model)`
-- enforces one score per model per run — the same-model-same-commit
-- de-duplication of docs/proposal/08-schema-and-crate-architecture.md,
-- "Audit Subsystem". A per-model override or cancel is an audit_override
-- (13) row targeting `semantic:<model>`.
CREATE TABLE IF NOT EXISTS audit_semantic_run (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id  INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    model         TEXT    NOT NULL,
    overall_score REAL    NOT NULL,
    reasoning     TEXT    NOT NULL DEFAULT '',
    UNIQUE(audit_run_id, model)
);
CREATE INDEX IF NOT EXISTS idx_audit_semantic_run_model ON audit_semantic_run(model);

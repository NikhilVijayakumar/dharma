-- repo.db — per (model, dimension) scores within a semantic audit run (see
-- docs/proposal/08-schema-and-crate-architecture.md, "Audit Subsystem").
-- Scoped by `semantic_run_id` into audit_semantic_run (10); each dimension
-- carries the model's own evidence string.

CREATE TABLE IF NOT EXISTS audit_semantic_dimension (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    semantic_run_id INTEGER NOT NULL REFERENCES audit_semantic_run(id) ON DELETE CASCADE,
    dimension       TEXT    NOT NULL,
    score           REAL    NOT NULL,
    evidence        TEXT    NOT NULL DEFAULT '',
    UNIQUE(semantic_run_id, dimension)
);
CREATE INDEX IF NOT EXISTS idx_audit_dimension_run ON audit_semantic_dimension(semantic_run_id);

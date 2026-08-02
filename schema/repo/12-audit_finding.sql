-- repo.db — normalized audit findings (strengths / weaknesses /
-- recommendations) extracted from an audit_run (08), so report templates
-- (mcp.db, audit_template) can iterate them without re-parsing evidence
-- (see docs/proposal/08-schema-and-crate-architecture.md, "Audit
-- Subsystem"). One row per finding.

CREATE TABLE IF NOT EXISTS audit_finding (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    finding_type TEXT    NOT NULL CHECK (finding_type IN ('strength','weakness','recommendation')),
    text         TEXT    NOT NULL,
    sort_order   INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_audit_finding_run ON audit_finding(audit_run_id);

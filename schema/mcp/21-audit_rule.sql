-- mcp.db — deterministic audit rules for one audit_definition (20). Each
-- rule evaluates to pass/fail with a weighted contribution to the
-- deterministic score (`weighted_pass_rate`, see audit_calculation 23).
-- `severity` and `mandatory` drive the failure policy — a mandatory rule
-- failing caps the score regardless of weight. `evidence_type`/`
-- evidence_target` name what the rule inspects (e.g. file presence, file
-- globs) so the deterministic runner knows what to capture as evidence.

CREATE TABLE IF NOT EXISTS audit_rule (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    rule_id            TEXT    NOT NULL,
    description        TEXT    NOT NULL DEFAULT '',
    condition          TEXT    NOT NULL,  -- evaluable rule condition
    message            TEXT    NOT NULL DEFAULT '',  -- emitted when the rule fails
    severity           TEXT    NOT NULL DEFAULT 'error',
    weight             REAL    NOT NULL DEFAULT 1.0,
    mandatory          INTEGER NOT NULL DEFAULT 0,
    evidence_type      TEXT    NOT NULL DEFAULT '',  -- e.g. 'file_presence' | 'file_globs'
    evidence_target    TEXT    NOT NULL DEFAULT '',
    sort_order         INTEGER NOT NULL DEFAULT 0,
    UNIQUE(audit_definition_id, rule_id)
);
CREATE INDEX IF NOT EXISTS idx_audit_rule_definition ON audit_rule(audit_definition_id);

-- mcp.db — score calculation formulas for an audit_definition (20).
-- `formula_type` names the supported formulas: `weighted_pass_rate`
-- (deterministic), `reliability_aware_ensemble` (semantic, weights each
-- model's score by its reliability), and `weighted_merge` (aggregates the
-- deterministic and semantic scores, e.g. deterministic 0.60 / semantic
-- 0.40). The evaluable form lives in `formula_json`; multiple formulas may
-- be chained in `sort_order` (see docs/proposal/08-schema-and-crate-
-- architecture.md, "Audit Subsystem").

CREATE TABLE IF NOT EXISTS audit_calculation (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    formula_type        TEXT    NOT NULL CHECK (formula_type IN ('weighted_pass_rate','reliability_aware_ensemble','weighted_merge')),
    formula_json        TEXT    NOT NULL DEFAULT '{}',
    sort_order          INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_audit_calculation_definition ON audit_calculation(audit_definition_id);

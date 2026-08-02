-- mcp.db — one audit definition for a (Domain System, domain). Audits are
-- Domain System content: providers declare deterministic rules, semantic
-- (per-model) prompts, calculation formulas, weights, and report templates;
-- the executions live per-repo in repo.db (see schema/repo/08-13). `kind`
-- distinguishes the two scoring paths; both may exist for the same target
-- and are merged by `audit_calculation` (23). Scope + standard version pin
-- what the audit checks (e.g. which section-map standard version).

CREATE TABLE IF NOT EXISTS audit_definition (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    domain_id        INTEGER REFERENCES domain(id) ON DELETE CASCADE,  -- NULL = audit spans the whole Domain System
    name             TEXT    NOT NULL,
    kind             TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic')),
    scope            TEXT    NOT NULL DEFAULT '',
    standard_version TEXT    NOT NULL DEFAULT '',
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),  -- the audit definition file
    UNIQUE(domain_system_id, domain_id, kind)
);
CREATE INDEX IF NOT EXISTS idx_audit_definition_domain_system ON audit_definition(domain_system_id);

-- mcp.db — per-domain audit weights: `base_total`, `max_semantic_bonus`,
-- and `final_scale` bound and normalize the merged score; `weights_json`
-- carries the aggregation split (e.g. deterministic 0.60 / semantic 0.40).
-- Scoped by domain_system_id, optionally per domain. Applied by the
-- `weighted_merge` calculation (23) at execution time.

CREATE TABLE IF NOT EXISTS audit_weights (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    domain_id        INTEGER REFERENCES domain(id) ON DELETE CASCADE,  -- NULL = Domain System-wide weights
    base_total       REAL    NOT NULL DEFAULT 100.0,
    max_semantic_bonus REAL NOT NULL DEFAULT 0.0,
    final_scale      REAL    NOT NULL DEFAULT 100.0,
    weights_json     TEXT    NOT NULL DEFAULT '{}',
    UNIQUE(domain_system_id, domain_id)
);

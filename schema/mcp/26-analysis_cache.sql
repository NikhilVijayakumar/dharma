-- mcp.db — reusable analysis cache. Once a (Domain System, capability/
-- domain set) resolution or audit has completed, its outcome is kept here
-- so a subsequently registered repository with the same resolution
-- retrieves it instantly instead of re-running (docs/proposal/08-schema-
-- and-crate-architecture.md, "Sync-to-Repo (Seeding) Flow", step 2).
-- `capability_hash` identifies the requested resolution; `result_json`
-- carries the cached resolution/audit result.

CREATE TABLE IF NOT EXISTS analysis_cache (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    capability_hash  TEXT    NOT NULL,  -- hash of the requested (Domain System, capability/domain set)
    capability_json  TEXT    NOT NULL DEFAULT '[]',  -- the resolved capability set
    result_json      TEXT    NOT NULL DEFAULT '{}',
    created_at       TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(domain_system_id, capability_hash)
);

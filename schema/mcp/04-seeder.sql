-- mcp.db — seeder/reconstruction scripts. A provider may declare a seeder
-- that fills a registered repository's repo.db rows from the synced content;
-- Dharma also ships generic ones (`is_generic = 1`, `domain_system_id` NULL).
-- The sync-to-repo (seeding) flow runs these into repo.db after repository
-- registration (see docs/proposal/08-schema-and-crate-architecture.md,
-- "Sync-to-Repo (Seeding) Flow"). `script_ref` points at the script; source
-- is captured in content_asset, this table only names and invokes it.

CREATE TABLE IF NOT EXISTS seeder (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER REFERENCES domain_system_registry(id) ON DELETE CASCADE,  -- NULL = generic/Dharma-shipped
    name             TEXT    NOT NULL,
    script_ref       TEXT    NOT NULL,  -- path to the seeder/reconstruction script
    is_generic       INTEGER NOT NULL DEFAULT 0,
    description      TEXT    NOT NULL DEFAULT '',
    UNIQUE(domain_system_id, name)
);

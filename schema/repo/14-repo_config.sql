-- repo.db — exactly one row: this repository's own resolved configuration,
-- materialized from `dharma-repo.toml` at registration/sync time (see docs/
-- proposal/11-provider-config-and-repo-sync.md, "Repo Config Table" and
-- "Repo Context Resolution"). Before this table existed, any tool needing
-- e.g. "where does this repo keep its docs / tests / reports" would have to
-- re-parse `dharma-repo.toml` (or worse, ask mcp.db) on every call; after
-- sync, that answer is one row away in the repo's own local db — no toml
-- re-parse, no reach back into MCP's external data directory, mid-session.
--
-- `domain_system_name` + `domain_system_version` mirror mcp.db's
-- `repo_registration.domain_system_id`/`domain_system_version` (logical
-- reference, same cross-database reason as elsewhere in this directory) —
-- cached here so "which Domain System does this repo run" is answerable
-- from repo.db alone. `mcp_dir` records where this repo's data was synced
-- FROM, needed only when a re-sync is requested; ordinary tool calls after
-- the initial sync never need to read it or touch that directory.

CREATE TABLE IF NOT EXISTS repo_config (
    id                     INTEGER PRIMARY KEY CHECK (id = 1),  -- singleton: exactly one row per repo.db
    repo_uuid              TEXT    NOT NULL,  -- logical ref: mcp.db repo_registration(repo_uuid)
    repo_name              TEXT    NOT NULL,
    repo_root              TEXT    NOT NULL,
    domain_system_name     TEXT    NOT NULL,  -- logical ref: mcp.db domain_system_registry(name)
    domain_system_version  TEXT    NOT NULL,
    docs_dir               TEXT,   -- resolved dharma-repo.toml [repository.documentation].root_dir, if set
    implementation_dir     TEXT,   -- resolved [repository.implementation].dir, if set
    scripts_dir            TEXT,   -- resolved [repository.scripts].dir, if set
    tests_dir              TEXT,   -- resolved [repository.tests].dir, if set
    report_dir             TEXT,   -- resolved [report].dir, if set — falls back to .dharma/ (see 11)
    dharma_dir             TEXT    NOT NULL,  -- this repo's own .dharma/ directory (holds this repo.db, assets/, summaries)
    mcp_dir                TEXT    NOT NULL,  -- where this repo synced FROM; read only on re-sync, not on ordinary tool calls
    last_synced_at         TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- mcp.db — the single global database, one instance per MCP deployment,
-- living in MCP's own data directory (mirrors samgraha's mcp_dir(),
-- e.g. ~/.dharma/mcp.db) — never inside a repository. Every table in
-- this directory lives in this one file, so every reference between
-- them is a real, enforced FOREIGN KEY — unlike the previous per-
-- Domain-System / per-Agent-System file split, there is no cross-
-- database reference inside mcp.db at all (see schema/README.md).
--
-- One row per registered Domain System (e.g. `rust-dev-domain`,
-- `electron-dev-domain`, see docs/proposal/05-domain-system-
-- registration.md). Unlike the earlier design, this table no longer
-- points at a separate db file — the Domain System's Section Map,
-- Section Profiles, and Epic/Usecase/Task set (02-07 in this directory)
-- live in this same db, scoped by `domain_system_id`.

CREATE TABLE IF NOT EXISTS domain_system_registry (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL UNIQUE,
    version       TEXT    NOT NULL DEFAULT '0.0.0',
    description   TEXT    NOT NULL DEFAULT '',
    registered_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

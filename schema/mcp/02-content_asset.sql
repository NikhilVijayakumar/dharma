-- mcp.db — the capture ledger: one row per provider-supplied file Dharma
-- captured into its own data directory (the "MCP location"). Dharma never
-- authors content; every `domain`/`section`/`section_profile`/`epic`/
-- `usecase`/`task`/`agent`/`skill` row elsewhere in this db traces back to
-- a row here (see docs/proposal/08-schema-and-crate-architecture.md,
-- "Dharma Is Infrastructure, Not an Author").
--
-- `content_text` is the full original file, kept for lossless
-- reconstruction and as the reconstruction script's input; `content_hash`
-- is a sha256 over it, used to detect provider-side edits and to prove
-- byte-stability of YAML round-trips. Re-capturing a changed file appends
-- a new row (the ledger is append-only); consumers pin the latest via
-- their own `content_asset_id` reference.

CREATE TABLE IF NOT EXISTS content_asset (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    source_system TEXT    NOT NULL,  -- provider that authored the file (e.g. bodha, an agent-management system)
    asset_kind    TEXT    NOT NULL CHECK (asset_kind IN ('yaml','markdown','python','json','text')),
    file_path     TEXT    NOT NULL,  -- path relative to the provider's bundle root
    content_text  TEXT    NOT NULL,
    content_hash  TEXT    NOT NULL,  -- sha256 of content_text
    captured_at   TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_content_asset_source ON content_asset(source_system, file_path);
CREATE INDEX IF NOT EXISTS idx_content_asset_kind ON content_asset(asset_kind);

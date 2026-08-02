-- repo.db — content synced from mcp.db into this repository at registration
-- (the sync-to-repo / seeding flow, see docs/proposal/08-schema-and-crate-
-- architecture.md). One row per synced mcp.db row: the Domain System's
-- domains/section maps/profiles, the applicable Agents, Skills, scripts,
-- prompts, examples, templates, and the audit definitions the repo needs.
-- `kind` names which mcp.db content table the row mirrors; `mcp_row_id` is a
-- logical reference into that table (same cross-database treatment as
-- task_instance.task_id, 00 — not an enforced FK, since mcp.db and this
-- repo.db are separate physical files; see schema/README.md). `seeder_ref`
-- records which seeder script produced the row.

CREATE TABLE IF NOT EXISTS synced_content (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    kind       TEXT    NOT NULL,  -- mcp.db content table this row mirrors ('domain','section','section_profile','epic','usecase','task','agent','skill','skill_prompt','skill_script','skill_example','skill_template','audit_definition','audit_rule','audit_semantic','audit_calculation','audit_weights','audit_template')
    mcp_row_id INTEGER NOT NULL,  -- logical ref: mcp.db <kind>(id)
    content    TEXT    NOT NULL,  -- the synced payload (parsed shape or file text)
    seeder_ref TEXT,  -- seeder script that produced this row
    synced_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(kind, mcp_row_id)
);
CREATE INDEX IF NOT EXISTS idx_synced_content_kind ON synced_content(kind);

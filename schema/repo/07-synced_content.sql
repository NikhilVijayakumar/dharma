-- repo.db — content synced from mcp.db into this repository at registration
-- (the sync-to-repo / seeding flow, see docs/proposal/11-provider-config-
-- and-repo-sync.md). One row per synced mcp.db row: the Domain System's
-- domains/section maps/profiles/epic-usecase-task set, the applicable
-- Agents, Skills, scripts, prompts, examples, templates, the provider's
-- seeders, and the audit definitions the repo needs to run audits locally.
-- `kind` names which mcp.db content table the row mirrors; `mcp_row_id` is a
-- logical reference into that table (same cross-database treatment as
-- task_instance.task_id, 00 — not an enforced FK, since mcp.db and this
-- repo.db are separate physical files; see schema/README.md). `seeder_ref`
-- records which seeder script produced the row.
--
-- `domain_system_id` / `agent_system_id` tag which registered system a row
-- belongs to (logical refs: mcp.db domain_system_registry(id) /
-- agent_system_registry(id)); exactly one is set per row — domain-scoped
-- kinds (domain..task_step, plus a Domain System's seeders) set
-- `domain_system_id`, agent-scoped kinds (agent..agent_skill_binding, plus
-- an Agent System's seeders) set `agent_system_id`. This keeps "which rows
-- belong to which system" answerable from repo.db alone, so a re-sync after
-- a Domain System version bump can invalidate exactly that system's rows
-- without a mid-session reach back into mcp.db.
--
-- `local_path` is the path, relative to this repository's own `.dharma/
-- assets/` directory, where `content` was ALSO written out as a real file
-- on disk — every synced row gets one, uniformly, regardless of `kind`.
-- This is what lets a Script Runtime `exec` a synced skill_script, or a
-- render step read a synced template, without ever reaching outside the
-- repository: without `local_path`, execution would have to resolve back to
-- mcp.db's `content_asset.file_path` / `skill_script.script_ref`, which
-- point into MCP's own (external, outside-the-repo) data directory — and
-- doing that on every single script/skill invocation would mean asking for
-- filesystem permission to an external folder every time, not once at sync.
-- `content` (the DB copy) and the file at `local_path` (the disk copy) are
-- written together and must stay byte-identical; `content` is kept
-- alongside the file so a row is still fully inspectable via SQL alone.

CREATE TABLE IF NOT EXISTS synced_content (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    kind             TEXT    NOT NULL,  -- mcp.db content table this row mirrors ('domain','section','section_profile','epic','usecase','task','task_step','agent','agent_goal','skill','skill_prompt','skill_script','skill_example','skill_template','agent_skill_binding','seeder','audit_definition','audit_rule','audit_semantic','audit_calculation','audit_weights','audit_template')
    mcp_row_id       INTEGER NOT NULL,  -- logical ref: mcp.db <kind>(id)
    domain_system_id INTEGER,  -- logical ref: mcp.db domain_system_registry(id); set iff the row is domain-scoped (incl. a Domain System's seeders)
    agent_system_id  INTEGER,  -- logical ref: mcp.db agent_system_registry(id); set iff the row is agent-scoped (incl. an Agent System's seeders)
    content          TEXT    NOT NULL,  -- the synced payload (parsed shape or file text)
    local_path       TEXT    NOT NULL,  -- path under this repo's .dharma/assets/, e.g. "skill_script/42.py"
    seeder_ref       TEXT,  -- seeder script that produced this row
    synced_at        TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(kind, mcp_row_id),
    CHECK ((domain_system_id IS NOT NULL AND agent_system_id IS NULL)
        OR (domain_system_id IS NULL AND agent_system_id IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_synced_content_kind ON synced_content(kind);
CREATE INDEX IF NOT EXISTS idx_synced_content_domain ON synced_content(domain_system_id);
CREATE INDEX IF NOT EXISTS idx_synced_content_agent ON synced_content(agent_system_id);

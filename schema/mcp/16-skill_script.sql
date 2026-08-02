-- mcp.db — the deterministic execution path for a Skill: a Python (.py)
-- script for now, other languages later (see docs/proposal/03-skill-model.md).
-- Scoped by `skill_id` into skill (14). `script_ref` is a pointer to the
-- executable captured in the MCP location — schema/architecture
-- documentation does not store source code inline (see docs/raw/
-- architecture.md, Out of Scope), so the script's source lives only in
-- content_asset, referenced by `content_asset_id`. At most one row per
-- Skill; unlike the prompt, the script is optional.
CREATE TABLE IF NOT EXISTS skill_script (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    script_ref       TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),  -- the captured .py file
    UNIQUE(skill_id)
);

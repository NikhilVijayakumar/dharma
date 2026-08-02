-- mcp.db — the semantic execution path for a Skill: the mandatory prompt,
-- a Markdown (.md) file (see docs/proposal/03-skill-model.md). Scoped by
-- `skill_id` into skill (14). Exactly one row per Skill — the "prompt
-- mandatory" rule: a Skill without a prompt row is rejected at
-- registration (`schemas` enforces it; see docs/proposal/08-schema-and-
-- crate-architecture.md, Hard Constraints). `template_text` is the parsed
-- prompt; the original `.md` file is captured in content_asset via
-- `content_asset_id`.
CREATE TABLE IF NOT EXISTS skill_prompt (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    template_text    TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),  -- the captured .md file
    UNIQUE(skill_id)
);

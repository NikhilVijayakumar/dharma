-- mcp.db — the semantic execution path for a Skill (see docs/proposal/
-- 03-skill-model.md). Scoped by `skill_id` into skill (10). At most one row
-- per Skill. A Skill may have a skill_prompt row, a skill_script row, or
-- both — never neither.
CREATE TABLE IF NOT EXISTS skill_prompt (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id      INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    template_text TEXT    NOT NULL,
    UNIQUE(skill_id)
);

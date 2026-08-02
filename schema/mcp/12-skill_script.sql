-- mcp.db — the deterministic execution path for a Skill (see docs/proposal/
-- 03-skill-model.md). Scoped by `skill_id` into skill (10). `script_ref` is a
-- pointer to the executable, not the script's source itself — schema/
-- architecture documentation does not store source code (see docs/raw/
-- architecture.md, Out of Scope). At most one row per Skill.
CREATE TABLE IF NOT EXISTS skill_script (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id   INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    script_ref TEXT    NOT NULL,
    UNIQUE(skill_id)
);

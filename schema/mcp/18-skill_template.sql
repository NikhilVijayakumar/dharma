-- mcp.db — a Skill's optional template asset (see docs/proposal/08-schema-
-- and-crate-architecture.md, "Skills"): like tasks, a Skill may provide a
-- template an Agent can use to generate content for the task at hand.
-- Scoped by `skill_id` into skill (14). At most one template per Skill;
-- `content_asset_id` traces the captured template YAML.
CREATE TABLE IF NOT EXISTS skill_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    template_text    TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(skill_id)
);

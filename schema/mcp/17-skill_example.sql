-- mcp.db — one Skill's worked examples (see docs/proposal/03-skill-model.md,
-- docs/proposal/08-schema-and-crate-architecture.md, "Skills"). Scoped by
-- `skill_id` into skill (14). One row per worked example demonstrating
-- correct invocation and expected output, plus `do`s, `don't`s, best
-- practices, and common mistakes. Every Skill requires at least one
-- example row before registration — an Agent-Management Agent System
-- check, not a CHECK constraint (SQLite cannot express "at least one
-- related row exists"). `content_asset_id` traces the example YAML capture.
CREATE TABLE IF NOT EXISTS skill_example (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id             INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    input_json           TEXT    NOT NULL,
    output_json          TEXT    NOT NULL,
    dos_json             TEXT    NOT NULL DEFAULT '[]',
    donts_json           TEXT    NOT NULL DEFAULT '[]',
    best_practices_json  TEXT    NOT NULL DEFAULT '[]',
    common_mistakes_json TEXT    NOT NULL DEFAULT '[]',
    content_asset_id     INTEGER NOT NULL REFERENCES content_asset(id)
);
CREATE INDEX IF NOT EXISTS idx_skill_example_skill ON skill_example(skill_id);

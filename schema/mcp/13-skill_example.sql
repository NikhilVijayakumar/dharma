-- mcp.db — one Skill's worked examples (see docs/proposal/03-skill-model.md).
-- Scoped by `skill_id` into skill (10). One row per worked example
-- demonstrating correct invocation and expected output. Every Skill requires
-- at least one row here before registration — an Agent-Management Agent
-- System check, not a CHECK constraint (SQLite cannot express "at least one
-- related row exists").
CREATE TABLE IF NOT EXISTS skill_example (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id    INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    input_json  TEXT    NOT NULL,
    output_json TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_skill_example_skill ON skill_example(skill_id);

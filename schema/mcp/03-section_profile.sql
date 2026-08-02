-- mcp.db — one Section's fill profile (a Section Map profile; see docs/
-- proposal/02-task-model.md, docs/proposal/05-domain-system-registration.md).
-- Scoped by `section_id` into section (02); the owning Section is itself
-- scoped to a Domain System by `section.domain_system_id`, so no
-- `domain_system_id` column is needed here.

CREATE TABLE IF NOT EXISTS section_profile (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id     INTEGER NOT NULL REFERENCES section(id) ON DELETE CASCADE,
    fill_rule_json TEXT    NOT NULL DEFAULT '{}',
    UNIQUE(section_id)
);

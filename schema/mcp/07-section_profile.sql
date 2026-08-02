-- mcp.db — a Section Profile: how a section (or subsection) is written
-- (shape per Bodha's `section/profile/introduction.yaml` and the inherited
-- `profile-default/scientific-narrative.yaml`). Scoped by `section_id` into
-- section (06); the owning section is itself scoped to a domain, so no
-- `domain_id` column is needed here. At most one profile per section.
--
-- Top-level fields are stored structured (queryable per field); the
-- per-subsection rules and the completion/review/validation collections are
-- JSON arrays. Profiles inherit a default profile: `inherits` names it
-- (e.g. `scientific-narrative`) and defaults apply unless a field is
-- overridden. The full original YAML is kept in content_asset (via
-- `content_asset_id`) for lossless reconstruction.

CREATE TABLE IF NOT EXISTS section_profile (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id           INTEGER NOT NULL UNIQUE REFERENCES section(id) ON DELETE CASCADE,
    inherits             TEXT    NOT NULL DEFAULT 'scientific-narrative',  -- default profile name
    writing_objective    TEXT    NOT NULL DEFAULT '',
    knowledge_goal       TEXT    NOT NULL DEFAULT '',
    reader_goal          TEXT    NOT NULL DEFAULT '',
    required_inputs_json TEXT    NOT NULL DEFAULT '[]',
    expected_outputs_json TEXT   NOT NULL DEFAULT '[]',
    subsection_rules_json TEXT   NOT NULL DEFAULT '[]',  -- per-subsection objective/writing_guidelines/should_answer/transition_to
    completion_checklist_json TEXT NOT NULL DEFAULT '[]',
    review_questions_json TEXT   NOT NULL DEFAULT '[]',
    validation_rules_json TEXT   NOT NULL DEFAULT '[]',
    content_asset_id     INTEGER NOT NULL REFERENCES content_asset(id)  -- the profile YAML capture
);

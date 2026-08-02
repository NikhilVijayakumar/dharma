-- mcp.db — the semantic (per-model) audit definition for one
-- audit_definition (20). `required_models_json` lists the ensemble of
-- models that must each score the target; `prompt_md_path` points at the
-- `.prompt.md` prompt template (the file itself is captured in
-- content_asset, same row id as `content_asset_id`). `metadata_fields_json`
-- and `evidence_requirements_json` describe what each model must return
-- besides its score. Per-model executions land in repo.db's
-- audit_semantic_run (10) / audit_semantic_dimension (11).

CREATE TABLE IF NOT EXISTS audit_semantic (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id   INTEGER NOT NULL UNIQUE REFERENCES audit_definition(id) ON DELETE CASCADE,
    required_models_json  TEXT    NOT NULL DEFAULT '[]',  -- ensemble of model ids
    prompt_md_path        TEXT    NOT NULL,  -- path of the .prompt.md template
    metadata_fields_json  TEXT    NOT NULL DEFAULT '[]',
    evidence_requirements_json TEXT NOT NULL DEFAULT '[]',
    content_asset_id      INTEGER NOT NULL REFERENCES content_asset(id)  -- the .prompt.md capture
);

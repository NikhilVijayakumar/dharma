-- mcp.db — a registered template describing how to reconstruct a captured
-- YAML file back to its original form from DB rows (`rows + template →
-- YAML`). One template per captured YAML asset. Reconstruction must be
-- byte-stable against the captured file when no edits occurred — the
-- `content_asset.content_hash` comparison proves it (see docs/proposal/
-- 08-schema-and-crate-architecture.md, "YAML round-trip"). Templates are
-- declared by the provider or by Dharma's generic reconstruction contract;
-- the rendering is a `seeder`/reconstruction script (04).

CREATE TABLE IF NOT EXISTS yaml_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id) ON DELETE CASCADE,  -- the YAML file this template reconstructs
    name             TEXT    NOT NULL,
    scope            TEXT    NOT NULL DEFAULT 'domain',  -- 'domain' | 'agent' | 'audit' — which content family the rows belong to
    template_text    TEXT    NOT NULL,  -- placeholder syntax the renderer fills from DB rows
    UNIQUE(content_asset_id)
);

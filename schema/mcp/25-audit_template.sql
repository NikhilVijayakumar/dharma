-- mcp.db — report templates for audit output (deterministic/semantic/
-- summary, Markdown/HTML). Bodies use `{{ placeholder }}` for single
-- values and `{{#section}} ... {{/section}}` for iteration over findings,
-- dimensions, and rule results (see docs/proposal/08-schema-and-crate-
-- architecture.md, "Audit Subsystem"). Templates are captured provider
-- content; the rendered report is produced at audit time into repo.db.

CREATE TABLE IF NOT EXISTS audit_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    kind             TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic','summary')),
    format           TEXT    NOT NULL DEFAULT 'markdown' CHECK (format IN ('markdown','html')),
    body             TEXT    NOT NULL,
    content_asset_id INTEGER REFERENCES content_asset(id)  -- the template file capture, when provider-supplied
);
CREATE INDEX IF NOT EXISTS idx_audit_template_definition ON audit_template(audit_definition_id);

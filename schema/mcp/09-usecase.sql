-- mcp.db — one Epic's Usecase set (see docs/proposal/02-task-model.md).
-- Scoped by `epic_id` into epic (08); the owning Epic is itself scoped to a
-- Domain System by `epic.domain_system_id`, so no `domain_system_id` column
-- is needed here. `content_asset_id` traces the task.yaml bundle this
-- Usecase was parsed from.

CREATE TABLE IF NOT EXISTS usecase (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    epic_id          INTEGER NOT NULL REFERENCES epic(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    description      TEXT    NOT NULL DEFAULT '',
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(epic_id, name)
);
CREATE INDEX IF NOT EXISTS idx_usecase_epic ON usecase(epic_id);

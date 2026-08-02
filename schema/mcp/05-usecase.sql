-- mcp.db — one Epic's Usecase set (see docs/proposal/02-task-model.md).
-- Scoped by `epic_id` into epic (04); the owning Epic is itself scoped to a
-- Domain System by `epic.domain_system_id`, so no `domain_system_id` column
-- is needed here.

CREATE TABLE IF NOT EXISTS usecase (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    epic_id     INTEGER NOT NULL REFERENCES epic(id) ON DELETE CASCADE,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0,
    UNIQUE(epic_id, name)
);
CREATE INDEX IF NOT EXISTS idx_usecase_epic ON usecase(epic_id);

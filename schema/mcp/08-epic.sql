-- mcp.db — one Domain System's Epic set (see docs/proposal/02-task-model.md,
-- docs/proposal/05-domain-system-registration.md). Scoped by a real
-- `domain_system_id` FOREIGN KEY into domain_system_registry (00). An Epic
-- can contain another Epic: `parent_id` is a self-referencing foreign key,
-- so the Epic → Usecase → Task hierarchy is fully queryable. Written only
-- by the Agent-Management Agent System; read-only for every repository
-- that selects the Domain System. `content_asset_id` traces the task.yaml
-- bundle this Epic was parsed from.

CREATE TABLE IF NOT EXISTS epic (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    parent_id        INTEGER REFERENCES epic(id),  -- an Epic can contain another Epic
    name             TEXT    NOT NULL,
    objective        TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, parent_id, name)
);
CREATE INDEX IF NOT EXISTS idx_epic_domain_system ON epic(domain_system_id);
CREATE INDEX IF NOT EXISTS idx_epic_parent ON epic(parent_id);

-- mcp.db — one Domain System's Epic set (see docs/proposal/02-task-model.md,
-- docs/proposal/05-domain-system-registration.md). Scoped by a real
-- `domain_system_id` FOREIGN KEY into domain_system_registry (00). Written
-- only by the Agent-Management Agent System; read-only for every repository
-- that selects the Domain System.

CREATE TABLE IF NOT EXISTS epic (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    objective        TEXT    NOT NULL,
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_epic_domain_system ON epic(domain_system_id);

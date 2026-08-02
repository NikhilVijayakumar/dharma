-- mcp.db — one Domain System's Section Map (see docs/proposal/02-task-model.md,
-- docs/proposal/05-domain-system-registration.md). Scoped by a real
-- `domain_system_id` FOREIGN KEY into domain_system_registry (00) — this
-- replaces the earlier one-db-file-per-Domain-System design; all Domain
-- Systems' content now shares this one db, distinguished by that column.
-- Written only by the Agent-Management Agent System; read-only for every
-- repository that selects a given Domain System.

CREATE TABLE IF NOT EXISTS section (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    purpose          TEXT    NOT NULL,
    required         INTEGER NOT NULL DEFAULT 1,
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_section_domain_system ON section(domain_system_id);

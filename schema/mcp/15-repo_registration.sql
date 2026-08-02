-- mcp.db — which repositories are registered with MCP, against which
-- Domain System (see docs/proposal/06-mcp-registration-bootstrap.md). Mirrors
-- samgraha's global registry.db `repository_cache` in spirit: this table
-- lives on the MCP side, not inside any repository, and `repo_db_path`
-- points at that repository's own `repo.db` (see schema/repo/), the same
-- way samgraha's repository_cache.knowledge_db points at each repo's own
-- knowledge.db. `domain_system_id` is now a real FOREIGN KEY (this table
-- and domain_system_registry share the same mcp.db file).

CREATE TABLE IF NOT EXISTS repo_registration (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_uuid             TEXT    NOT NULL UNIQUE,
    repo_name             TEXT    NOT NULL,
    repo_root             TEXT    NOT NULL,
    repo_db_path          TEXT,  -- path to this repo's own repo.db; set once status reaches 'approved'
    domain_system_id      INTEGER NOT NULL REFERENCES domain_system_registry(id),
    domain_system_version TEXT    NOT NULL,  -- version pinned at registration; mismatch blocks Task assignment
    status                TEXT    NOT NULL DEFAULT 'pending'
                          CHECK (status IN ('pending','manifest_proposed','approved','stale')),
    registered_at         TEXT    NOT NULL DEFAULT (datetime('now')),
    approved_at           TEXT
);
CREATE INDEX IF NOT EXISTS idx_repo_registration_uuid ON repo_registration(repo_uuid);
CREATE INDEX IF NOT EXISTS idx_repo_registration_status ON repo_registration(status);

-- mcp.db — one Agent System resolved as applicable to a repository,
-- proposed by the Default/Bootstrap Agent System and gated by human
-- approval before a repo_registration reaches 'approved' (see docs/proposal/
-- 06-mcp-registration-bootstrap.md). Scoped by `repo_registration_id` into
-- repo_registration (27). `status` distinguishes proposed (not yet reviewed),
-- approved, and rejected — a bare boolean can't represent "reviewed and
-- rejected" distinctly from "never reviewed" (06's human reviewer may modify
-- or reject a proposed entry, not just approve it). Both FKs are real now
-- (same db).
CREATE TABLE IF NOT EXISTS capability_manifest (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_registration_id INTEGER NOT NULL REFERENCES repo_registration(id) ON DELETE CASCADE,
    agent_system_id       INTEGER NOT NULL REFERENCES agent_system_registry(id),
    proposed_at           TEXT    NOT NULL DEFAULT (datetime('now')),
    status                TEXT    NOT NULL DEFAULT 'proposed'
                          CHECK (status IN ('proposed','approved','rejected')),
    reviewed_by           TEXT,
    reviewed_at           TEXT,
    UNIQUE(repo_registration_id, agent_system_id)
);
CREATE INDEX IF NOT EXISTS idx_capability_manifest_repo ON capability_manifest(repo_registration_id);
CREATE INDEX IF NOT EXISTS idx_capability_manifest_status ON capability_manifest(status);

//! `repo.db` — one instance per registered repository, living inside that
//! repository's `.dharma/` directory (path recorded in mcp.db's
//! `repo_registration.repo_db_path`). Because the file is already scoped to
//! one repository, there is no `repo_registration_id` column (see
//! `schema/repo/00-task_instance.sql`). Mirrors samgraha's per-repo
//! `knowledge.db`.
//!
//! Cross-database references into mcp.db (e.g. `task_instance.task_id`,
//! `proposal_revision.drafted_by_agent_system_id`) are logical — NOT
//! enforced FKs, since the two files are separate physical databases. The
//! `validate_logical_refs` helper provides the code-level checks.
//!
//! Table numbering 00-16 matches the `schema/repo/*.sql` files; the DDL
//! below is reproduced verbatim from those files (comments elided).

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use tracing::info;

const REPO_MIGRATIONS: &[&str] = &[REPO_V1];

/// REPO_V1 — the full repo.db schema, tables 00-16 + `_schema_version`.
/// Mirrors `schema/repo/*.sql` verbatim.
const REPO_V1: &str = "
CREATE TABLE IF NOT EXISTS _schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS task_instance (
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id                     INTEGER NOT NULL,
    initiating_agent_system_id  INTEGER NOT NULL,
    initiating_agent_id         INTEGER NOT NULL,
    attempt_number              INTEGER NOT NULL DEFAULT 1,
    previous_task_instance_id   INTEGER REFERENCES task_instance(id),
    status                      TEXT    NOT NULL DEFAULT 'proposing'
                                CHECK (status IN ('proposing','approved','executing','complete','failed')),
    created_at                  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                  TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_task_instance_status ON task_instance(status);
CREATE INDEX IF NOT EXISTS idx_task_instance_task ON task_instance(task_id);

CREATE TABLE IF NOT EXISTS proposal_revision (
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id           INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    revision_number            INTEGER NOT NULL,
    drafted_by_agent_system_id INTEGER NOT NULL,
    drafted_by_agent_id        INTEGER NOT NULL,
    draft_json                 TEXT    NOT NULL,
    reviewer_comments          TEXT,
    created_at                 TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, revision_number)
);
CREATE INDEX IF NOT EXISTS idx_proposal_revision_task_instance ON proposal_revision(task_instance_id);

CREATE TABLE IF NOT EXISTS proposal_approval (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id     INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    approved_revision_id INTEGER NOT NULL REFERENCES proposal_revision(id),
    approved_by          TEXT    NOT NULL,
    approved_at          TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

CREATE TABLE IF NOT EXISTS execution_state (
    id                              INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id                INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    current_step_order              INTEGER NOT NULL DEFAULT 1,
    current_owning_agent_system_id  INTEGER NOT NULL,
    current_owning_agent_id         INTEGER NOT NULL,
    updated_at                      TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

CREATE TABLE IF NOT EXISTS handoff_log (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id      INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    hop_order             INTEGER NOT NULL,
    from_agent_system_id  INTEGER NOT NULL,
    from_agent_id         INTEGER NOT NULL,
    to_agent_system_id    INTEGER NOT NULL,
    to_agent_id           INTEGER NOT NULL,
    reason                TEXT    NOT NULL DEFAULT '',
    accepted              INTEGER NOT NULL DEFAULT 0,
    created_at            TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, hop_order)
);
CREATE INDEX IF NOT EXISTS idx_handoff_log_task_instance ON handoff_log(task_instance_id);

CREATE TABLE IF NOT EXISTS context_envelope (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    hop_order        INTEGER NOT NULL,
    hop_kind         TEXT    NOT NULL CHECK (hop_kind IN ('proposal','execution')),
    payload_json     TEXT    NOT NULL,
    created_at       TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id, hop_order)
);
CREATE INDEX IF NOT EXISTS idx_context_envelope_task_instance ON context_envelope(task_instance_id);

CREATE TABLE IF NOT EXISTS completion_validation (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_instance_id INTEGER NOT NULL REFERENCES task_instance(id) ON DELETE CASCADE,
    verdict          TEXT    NOT NULL CHECK (verdict IN ('pass','fail')),
    reason           TEXT    NOT NULL DEFAULT '',
    validated_at     TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(task_instance_id)
);

CREATE TABLE IF NOT EXISTS synced_content (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    kind             TEXT    NOT NULL,
    mcp_row_id       INTEGER NOT NULL,
    domain_system_id INTEGER,
    agent_system_id  INTEGER,
    content          TEXT    NOT NULL,
    local_path       TEXT    NOT NULL,
    seeder_ref       TEXT,
    synced_at        TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(kind, mcp_row_id),
    CHECK ((domain_system_id IS NOT NULL AND agent_system_id IS NULL)
        OR (domain_system_id IS NULL AND agent_system_id IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_synced_content_kind ON synced_content(kind);
CREATE INDEX IF NOT EXISTS idx_synced_content_domain ON synced_content(domain_system_id);
CREATE INDEX IF NOT EXISTS idx_synced_content_agent ON synced_content(agent_system_id);

CREATE TABLE IF NOT EXISTS audit_run (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    commit_hash  TEXT    NOT NULL,
    domain_id    INTEGER NOT NULL,
    domain_name  TEXT    NOT NULL,
    kind         TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic')),
    status       TEXT    NOT NULL DEFAULT 'running'
                  CHECK (status IN ('running','complete','cancelled')),
    created_at   TEXT    NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    UNIQUE(commit_hash, domain_id, kind)
);
CREATE INDEX IF NOT EXISTS idx_audit_run_commit ON audit_run(commit_hash);
CREATE INDEX IF NOT EXISTS idx_audit_run_status ON audit_run(status);

CREATE TABLE IF NOT EXISTS audit_deterministic_result (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id  INTEGER NOT NULL UNIQUE REFERENCES audit_run(id) ON DELETE CASCADE,
    score         REAL    NOT NULL,
    rules_passed  INTEGER NOT NULL DEFAULT 0,
    rules_total   INTEGER NOT NULL DEFAULT 0,
    evidence_json TEXT    NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS audit_semantic_run (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id  INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    model         TEXT    NOT NULL,
    overall_score REAL    NOT NULL,
    reasoning     TEXT    NOT NULL DEFAULT '',
    UNIQUE(audit_run_id, model)
);
CREATE INDEX IF NOT EXISTS idx_audit_semantic_run_model ON audit_semantic_run(model);

CREATE TABLE IF NOT EXISTS audit_semantic_dimension (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    semantic_run_id INTEGER NOT NULL REFERENCES audit_semantic_run(id) ON DELETE CASCADE,
    dimension       TEXT    NOT NULL,
    score           REAL    NOT NULL,
    evidence        TEXT    NOT NULL DEFAULT '',
    UNIQUE(semantic_run_id, dimension)
);
CREATE INDEX IF NOT EXISTS idx_audit_dimension_run ON audit_semantic_dimension(semantic_run_id);

CREATE TABLE IF NOT EXISTS audit_finding (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    finding_type TEXT    NOT NULL CHECK (finding_type IN ('strength','weakness','recommendation')),
    text         TEXT    NOT NULL,
    sort_order   INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_audit_finding_run ON audit_finding(audit_run_id);

CREATE TABLE IF NOT EXISTS audit_override (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id   INTEGER NOT NULL REFERENCES audit_run(id) ON DELETE CASCADE,
    target         TEXT    NOT NULL DEFAULT 'run',
    action         TEXT    NOT NULL CHECK (action IN ('override','cancel')),
    override_score REAL,
    reason         TEXT    NOT NULL DEFAULT '',
    reviewed_by    TEXT    NOT NULL DEFAULT '',
    created_at     TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_audit_override_run ON audit_override(audit_run_id);

CREATE TABLE IF NOT EXISTS repo_config (
    id                     INTEGER PRIMARY KEY CHECK (id = 1),
    repo_uuid              TEXT    NOT NULL,
    repo_name              TEXT    NOT NULL,
    repo_root              TEXT    NOT NULL,
    domain_system_name     TEXT    NOT NULL,
    domain_system_version  TEXT    NOT NULL,
    docs_dir               TEXT,
    implementation_dir     TEXT,
    scripts_dir            TEXT,
    tests_dir              TEXT,
    report_dir             TEXT,
    dharma_dir             TEXT    NOT NULL,
    mcp_dir                TEXT    NOT NULL,
    last_synced_at         TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS proposal_lifecycle (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    name                      TEXT    NOT NULL UNIQUE,
    title                     TEXT    NOT NULL,
    doc_path                  TEXT    NOT NULL,
    status                    TEXT    NOT NULL DEFAULT 'draft'
                              CHECK (status IN ('draft','finalized','implementing','verified','archived')),
    draft_commit_hash         TEXT,
    finalized_commit_hash     TEXT,
    implementation_commit_hash TEXT,
    verified_at               TEXT,
    archive_commit_hash       TEXT,
    archived_at               TEXT,
    created_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    CHECK (
        (status = 'draft')
        OR (status = 'finalized' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL)
        OR (status = 'implementing' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL)
        OR (status = 'verified' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL
            AND implementation_commit_hash IS NOT NULL AND verified_at IS NOT NULL)
        OR (status = 'archived' AND draft_commit_hash IS NOT NULL AND finalized_commit_hash IS NOT NULL
            AND implementation_commit_hash IS NOT NULL AND verified_at IS NOT NULL
            AND archive_commit_hash IS NOT NULL AND archived_at IS NOT NULL)
    )
);
CREATE INDEX IF NOT EXISTS idx_proposal_lifecycle_status ON proposal_lifecycle(status);

CREATE TABLE IF NOT EXISTS proposal_commit_log (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    proposal_lifecycle_id INTEGER NOT NULL REFERENCES proposal_lifecycle(id) ON DELETE CASCADE,
    commit_hash         TEXT    NOT NULL,
    phase               TEXT    NOT NULL CHECK (phase IN ('draft','finalized','implementation','archive')),
    message             TEXT    NOT NULL DEFAULT '',
    recorded_at         TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(proposal_lifecycle_id, commit_hash)
);
CREATE INDEX IF NOT EXISTS idx_proposal_commit_log_proposal ON proposal_commit_log(proposal_lifecycle_id);
";

/// The per-repo repo.db — task execution (00-07), audit executions (08-13),
/// repo config (14), and proposal lifecycle (15-16). One instance per
/// registered repository, inside the repo's own `.dharma/` directory.
pub struct RepoDb {
    conn: Mutex<Connection>,
}

impl RepoDb {
    /// Open or create the repo.db at `path` (e.g. `<repo>/.dharma/repo.db`).
    pub fn open_at(path: &std::path::Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open repo.db at {}", path.display()))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let store = Self { conn: Mutex::new(conn) };
        store.run_migrations()?;
        Ok(store)
    }

    /// Open an in-memory repo.db for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let store = Self { conn: Mutex::new(conn) };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let current_version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM _schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        for (i, migration) in REPO_MIGRATIONS.iter().enumerate() {
            let version = i as i64 + 1;
            if version > current_version {
                info!("Applying repo.db migration REPO_V{}", version);
                conn.execute_batch(migration)?;
                conn.execute(
                    "INSERT INTO _schema_version (version, applied_at) VALUES (?1, datetime('now'))",
                    params![version],
                )?;
            }
        }
        Ok(())
    }

    pub fn schema_version(&self) -> i64 {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM _schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0)
    }

    pub fn conn(&self) -> &Mutex<Connection> {
        &self.conn
    }
}

/// The kinds `synced_content.kind` accepts — the mcp.db content tables a
/// sync mirrors into repo.db (see `schema/repo/07-synced_content.sql`).
pub const SYNCED_CONTENT_KINDS: &[&str] = &[
    "domain",
    "section",
    "section_profile",
    "epic",
    "usecase",
    "task",
    "task_step",
    "agent",
    "agent_goal",
    "skill",
    "skill_prompt",
    "skill_script",
    "skill_example",
    "skill_template",
    "agent_skill_binding",
    "seeder",
    "audit_definition",
    "audit_rule",
    "audit_semantic",
    "audit_calculation",
    "audit_weights",
    "audit_template",
];

/// The kinds of synced_content rows that are domain-scoped (set
/// `domain_system_id`; `agent_system_id` must be NULL). Everything else in
/// `SYNCED_CONTENT_KINDS` is agent-scoped.
pub const SYNCED_CONTENT_DOMAIN_SCOPED: &[&str] = &[
    "domain",
    "section",
    "section_profile",
    "epic",
    "usecase",
    "task",
    "task_step",
    "seeder",
    "audit_definition",
    "audit_rule",
    "audit_semantic",
    "audit_calculation",
    "audit_weights",
    "audit_template",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_migrations_apply_cleanly() {
        let conn = Connection::open_in_memory().unwrap();
        for m in REPO_MIGRATIONS {
            conn.execute_batch(m).unwrap();
        }
        let exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='repo_config'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(exists, 1);
    }

    #[test]
    fn schema_version_reaches_one() {
        let db = RepoDb::open_in_memory().unwrap();
        assert_eq!(db.schema_version(), 1);
    }

    #[test]
    fn repo_config_is_singleton() {
        let db = RepoDb::open_in_memory().unwrap();
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO repo_config (repo_uuid, repo_name, repo_root, domain_system_name, domain_system_version, dharma_dir, mcp_dir) VALUES ('u','r','/r','d','0.0.0','/d','/m')",
                [],
            )
            .unwrap();
        let result = conn.lock().unwrap().execute(
            "INSERT INTO repo_config (repo_uuid, repo_name, repo_root, domain_system_name, domain_system_version, dharma_dir, mcp_dir) VALUES ('u2','r2','/r2','d2','0.0.0','/d2','/m2')",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn synced_content_exactly_one_system_tag() {
        let db = RepoDb::open_in_memory().unwrap();
        let conn = db.conn();
        // both set — must violate the CHECK
        let result = conn.lock().unwrap().execute(
            "INSERT INTO synced_content (kind, mcp_row_id, domain_system_id, agent_system_id, content, local_path) VALUES ('domain',1,1,1,'c','p')",
            [],
        );
        assert!(result.is_err());
        // neither set — must violate the CHECK
        let result = conn.lock().unwrap().execute(
            "INSERT INTO synced_content (kind, mcp_row_id, content, local_path) VALUES ('domain',2,'c','p')",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn handoff_log_requires_acceptance_before_reuse() {
        // no constraint here — just verifying the table accepts the columns
        let db = RepoDb::open_in_memory().unwrap();
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO task_instance (task_id, initiating_agent_system_id, initiating_agent_id) VALUES (1,1,1)",
                [],
            )
            .unwrap();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO handoff_log (task_instance_id, hop_order, from_agent_system_id, from_agent_id, to_agent_system_id, to_agent_id) VALUES (1,1,1,1,1,2)",
                [],
            )
            .unwrap();
        let count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM handoff_log", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}

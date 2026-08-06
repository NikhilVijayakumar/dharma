//! `mcp.db` — the single global database, one instance per MCP deployment,
//! living in MCP's own data directory (mirrors samgraha's `mcp_dir()`,
//! e.g. `~/.dharma/mcp.db`) — never inside a repository. Every table in
//! this module lives in this one file, so every reference between them is a
//! real, enforced FOREIGN KEY (see `schema/mcp/00-domain_system_registry.sql`
//! and `schema/README.md`).
//!
//! Table numbering 00-28 matches the `schema/mcp/*.sql` files; the DDL below
//! is reproduced verbatim from those files (comments elided).

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use tracing::info;

const MCP_MIGRATIONS: &[&str] = &[MCP_V1];

/// MCP_V1 — the full mcp.db schema, tables 00-28 + `_schema_version`.
/// Mirrors `schema/mcp/*.sql` verbatim.
const MCP_V1: &str = "
CREATE TABLE IF NOT EXISTS _schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS domain_system_registry (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL UNIQUE,
    version       TEXT    NOT NULL DEFAULT '0.0.0',
    description   TEXT    NOT NULL DEFAULT '',
    registered_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS agent_system_registry (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT    NOT NULL UNIQUE,
    concern       TEXT    NOT NULL UNIQUE,
    description   TEXT    NOT NULL DEFAULT '',
    is_privileged INTEGER NOT NULL DEFAULT 0,
    registered_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_agent_system_registry_privileged ON agent_system_registry(is_privileged);

CREATE TABLE IF NOT EXISTS content_asset (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    source_system TEXT    NOT NULL,
    asset_kind    TEXT    NOT NULL CHECK (asset_kind IN ('yaml','markdown','python','json','text')),
    file_path     TEXT    NOT NULL,
    content_text  TEXT    NOT NULL,
    content_hash  TEXT    NOT NULL,
    captured_at   TEXT    NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_content_asset_source ON content_asset(source_system, file_path);
CREATE INDEX IF NOT EXISTS idx_content_asset_kind ON content_asset(asset_kind);

CREATE TABLE IF NOT EXISTS yaml_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    scope            TEXT    NOT NULL DEFAULT 'domain',
    template_text    TEXT    NOT NULL,
    UNIQUE(content_asset_id)
);

CREATE TABLE IF NOT EXISTS seeder (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    script_ref       TEXT    NOT NULL,
    is_generic       INTEGER NOT NULL DEFAULT 0,
    description      TEXT    NOT NULL DEFAULT '',
    UNIQUE(domain_system_id, name)
);

CREATE TABLE IF NOT EXISTS domain (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    description      TEXT    NOT NULL DEFAULT '',
    tier             TEXT,
    relationship     TEXT,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_domain_domain_system ON domain(domain_system_id);

CREATE TABLE IF NOT EXISTS section (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_id        INTEGER NOT NULL REFERENCES domain(id) ON DELETE CASCADE,
    section_id       TEXT    NOT NULL,
    title            TEXT    NOT NULL,
    parent_id        INTEGER REFERENCES section(id),
    level            INTEGER NOT NULL DEFAULT 0,
    sort_order       INTEGER NOT NULL DEFAULT 0,
    required         INTEGER NOT NULL DEFAULT 1,
    generated        INTEGER NOT NULL DEFAULT 0,
    source           TEXT    NOT NULL DEFAULT '',
    profile_ref      TEXT,
    purpose          TEXT    NOT NULL DEFAULT '',
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(domain_id, section_id)
);
CREATE INDEX IF NOT EXISTS idx_section_domain ON section(domain_id);
CREATE INDEX IF NOT EXISTS idx_section_parent ON section(parent_id);

CREATE TABLE IF NOT EXISTS section_profile (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id           INTEGER NOT NULL UNIQUE REFERENCES section(id) ON DELETE CASCADE,
    inherits             TEXT    NOT NULL DEFAULT 'scientific-narrative',
    writing_objective    TEXT    NOT NULL DEFAULT '',
    knowledge_goal       TEXT    NOT NULL DEFAULT '',
    reader_goal          TEXT    NOT NULL DEFAULT '',
    required_inputs_json TEXT    NOT NULL DEFAULT '[]',
    expected_outputs_json TEXT   NOT NULL DEFAULT '[]',
    subsection_rules_json TEXT   NOT NULL DEFAULT '[]',
    completion_checklist_json TEXT NOT NULL DEFAULT '[]',
    review_questions_json TEXT   NOT NULL DEFAULT '[]',
    validation_rules_json TEXT   NOT NULL DEFAULT '[]',
    content_asset_id     INTEGER NOT NULL REFERENCES content_asset(id)
);

CREATE TABLE IF NOT EXISTS epic (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    parent_id        INTEGER REFERENCES epic(id),
    name             TEXT    NOT NULL,
    objective        TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    sort_order       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(domain_system_id, parent_id, name)
);
CREATE INDEX IF NOT EXISTS idx_epic_domain_system ON epic(domain_system_id);
CREATE INDEX IF NOT EXISTS idx_epic_parent ON epic(parent_id);

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

CREATE TABLE IF NOT EXISTS task (
    id                       INTEGER PRIMARY KEY AUTOINCREMENT,
    usecase_id               INTEGER NOT NULL REFERENCES usecase(id) ON DELETE CASCADE,
    name                     TEXT    NOT NULL,
    description              TEXT    NOT NULL DEFAULT '',
    input_contract_json      TEXT    NOT NULL,
    output_contract_json     TEXT    NOT NULL,
    acceptance_criteria_json TEXT    NOT NULL,
    template_ref             TEXT,
    content_asset_id         INTEGER NOT NULL REFERENCES content_asset(id),
    sort_order               INTEGER NOT NULL DEFAULT 0,
    UNIQUE(usecase_id, name)
);
CREATE INDEX IF NOT EXISTS idx_task_usecase ON task(usecase_id);

CREATE TABLE IF NOT EXISTS task_step (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id             INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    step_order          INTEGER NOT NULL,
    description         TEXT    NOT NULL DEFAULT '',
    required_capability TEXT    NOT NULL REFERENCES agent_system_registry(concern),
    UNIQUE(task_id, step_order)
);
CREATE INDEX IF NOT EXISTS idx_task_step_task ON task_step(task_id);

CREATE TABLE IF NOT EXISTS agent (
    id                        INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_system_id           INTEGER NOT NULL REFERENCES agent_system_registry(id) ON DELETE CASCADE,
    name                      TEXT    NOT NULL,
    role                      TEXT    NOT NULL,
    handoff_trigger_condition TEXT    NOT NULL DEFAULT '',
    handoff_candidate_role    TEXT    NOT NULL DEFAULT '',
    content_asset_id          INTEGER NOT NULL REFERENCES content_asset(id),
    created_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at                TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(agent_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_agent_agent_system ON agent(agent_system_id);

CREATE TABLE IF NOT EXISTS agent_goal (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id   INTEGER NOT NULL REFERENCES agent(id) ON DELETE CASCADE,
    goal_order INTEGER NOT NULL CHECK (goal_order BETWEEN 1 AND 8),
    goal_text  TEXT    NOT NULL,
    backstory  TEXT    NOT NULL,
    UNIQUE(agent_id, goal_order)
);
CREATE INDEX IF NOT EXISTS idx_agent_goal_agent ON agent_goal(agent_id);

CREATE TABLE IF NOT EXISTS skill (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_system_id         INTEGER NOT NULL REFERENCES agent_system_registry(id) ON DELETE CASCADE,
    name                    TEXT    NOT NULL,
    responsibility          TEXT    NOT NULL,
    is_analysis_only        INTEGER NOT NULL DEFAULT 0,
    invocation_input_json   TEXT    NOT NULL,
    invocation_output_json  TEXT    NOT NULL,
    content_asset_id        INTEGER NOT NULL REFERENCES content_asset(id),
    created_at              TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(agent_system_id, name)
);
CREATE INDEX IF NOT EXISTS idx_skill_agent_system ON skill(agent_system_id);
CREATE INDEX IF NOT EXISTS idx_skill_analysis_only ON skill(is_analysis_only);

CREATE TABLE IF NOT EXISTS skill_prompt (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    template_text    TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(skill_id)
);

CREATE TABLE IF NOT EXISTS skill_script (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    script_ref       TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(skill_id)
);

CREATE TABLE IF NOT EXISTS skill_example (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id             INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    input_json           TEXT    NOT NULL,
    output_json          TEXT    NOT NULL,
    dos_json             TEXT    NOT NULL DEFAULT '[]',
    donts_json           TEXT    NOT NULL DEFAULT '[]',
    best_practices_json  TEXT    NOT NULL DEFAULT '[]',
    common_mistakes_json TEXT    NOT NULL DEFAULT '[]',
    content_asset_id     INTEGER NOT NULL REFERENCES content_asset(id)
);
CREATE INDEX IF NOT EXISTS idx_skill_example_skill ON skill_example(skill_id);

CREATE TABLE IF NOT EXISTS skill_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id         INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    template_text    TEXT    NOT NULL,
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(skill_id)
);

CREATE TABLE IF NOT EXISTS agent_skill_binding (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL REFERENCES agent(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skill(id) ON DELETE CASCADE,
    UNIQUE(agent_id, skill_id)
);
CREATE INDEX IF NOT EXISTS idx_agent_skill_binding_agent ON agent_skill_binding(agent_id);

CREATE TABLE IF NOT EXISTS audit_definition (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    domain_id        INTEGER REFERENCES domain(id) ON DELETE CASCADE,
    name             TEXT    NOT NULL,
    kind             TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic')),
    scope            TEXT    NOT NULL DEFAULT '',
    standard_version TEXT    NOT NULL DEFAULT '',
    content_asset_id INTEGER NOT NULL REFERENCES content_asset(id),
    UNIQUE(domain_system_id, domain_id, kind)
);
CREATE INDEX IF NOT EXISTS idx_audit_definition_domain_system ON audit_definition(domain_system_id);

CREATE TABLE IF NOT EXISTS audit_rule (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    rule_id            TEXT    NOT NULL,
    description        TEXT    NOT NULL DEFAULT '',
    condition          TEXT    NOT NULL,
    message            TEXT    NOT NULL DEFAULT '',
    severity           TEXT    NOT NULL DEFAULT 'error',
    weight             REAL    NOT NULL DEFAULT 1.0,
    mandatory          INTEGER NOT NULL DEFAULT 0,
    evidence_type      TEXT    NOT NULL DEFAULT '',
    evidence_target    TEXT    NOT NULL DEFAULT '',
    sort_order         INTEGER NOT NULL DEFAULT 0,
    UNIQUE(audit_definition_id, rule_id)
);
CREATE INDEX IF NOT EXISTS idx_audit_rule_definition ON audit_rule(audit_definition_id);

CREATE TABLE IF NOT EXISTS audit_semantic (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id   INTEGER NOT NULL UNIQUE REFERENCES audit_definition(id) ON DELETE CASCADE,
    required_models_json  TEXT    NOT NULL DEFAULT '[]',
    prompt_md_path        TEXT    NOT NULL,
    metadata_fields_json  TEXT    NOT NULL DEFAULT '[]',
    evidence_requirements_json TEXT NOT NULL DEFAULT '[]',
    content_asset_id      INTEGER NOT NULL REFERENCES content_asset(id)
);

CREATE TABLE IF NOT EXISTS audit_calculation (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    formula_type        TEXT    NOT NULL CHECK (formula_type IN ('weighted_pass_rate','reliability_aware_ensemble','weighted_merge')),
    formula_json        TEXT    NOT NULL DEFAULT '{}',
    sort_order          INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_audit_calculation_definition ON audit_calculation(audit_definition_id);

CREATE TABLE IF NOT EXISTS audit_weights (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    domain_id        INTEGER REFERENCES domain(id) ON DELETE CASCADE,
    base_total       REAL    NOT NULL DEFAULT 100.0,
    max_semantic_bonus REAL NOT NULL DEFAULT 0.0,
    final_scale      REAL    NOT NULL DEFAULT 100.0,
    weights_json     TEXT    NOT NULL DEFAULT '{}',
    UNIQUE(domain_system_id, domain_id)
);

CREATE TABLE IF NOT EXISTS audit_template (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_definition_id INTEGER NOT NULL REFERENCES audit_definition(id) ON DELETE CASCADE,
    kind             TEXT    NOT NULL CHECK (kind IN ('deterministic','semantic','summary')),
    format           TEXT    NOT NULL DEFAULT 'markdown' CHECK (format IN ('markdown','html')),
    body             TEXT    NOT NULL,
    content_asset_id INTEGER REFERENCES content_asset(id)
);
CREATE INDEX IF NOT EXISTS idx_audit_template_definition ON audit_template(audit_definition_id);

CREATE TABLE IF NOT EXISTS analysis_cache (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_system_id INTEGER NOT NULL REFERENCES domain_system_registry(id) ON DELETE CASCADE,
    capability_hash  TEXT    NOT NULL,
    capability_json  TEXT    NOT NULL DEFAULT '[]',
    result_json      TEXT    NOT NULL DEFAULT '{}',
    created_at       TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(domain_system_id, capability_hash)
);

CREATE TABLE IF NOT EXISTS repo_registration (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_uuid             TEXT    NOT NULL UNIQUE,
    repo_name             TEXT    NOT NULL,
    repo_root             TEXT    NOT NULL,
    repo_db_path          TEXT,
    domain_system_id      INTEGER NOT NULL REFERENCES domain_system_registry(id),
    domain_system_version TEXT    NOT NULL,
    status                TEXT    NOT NULL DEFAULT 'pending'
                          CHECK (status IN ('pending','manifest_proposed','approved','stale')),
    registered_at         TEXT    NOT NULL DEFAULT (datetime('now')),
    approved_at           TEXT
);
CREATE INDEX IF NOT EXISTS idx_repo_registration_uuid ON repo_registration(repo_uuid);
CREATE INDEX IF NOT EXISTS idx_repo_registration_status ON repo_registration(status);

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
";

/// The global mcp.db — registries (00-01), capture ledger (02-04), Domain
/// System content (05-26), and repository registrations (27-28). One
/// instance per MCP deployment at `mcp_dir()/mcp.db`; never inside a repo.
pub struct McpDb {
    conn: Mutex<Connection>,
}

/// A packaged release's pre-bundled `data/mcp.db`, next to this running
/// binary (`<exe_dir>/../data/mcp.db`), if one exists. `None` for a dev
/// build (`cargo run`) or a release binary run from anywhere else.
fn packaged_seed_db() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let candidate = exe.parent()?.parent()?.join("data").join("mcp.db");
    candidate.is_file().then_some(candidate)
}

/// Copy `seed` to `target` only if `target` doesn't exist yet — never
/// overwrites an already-initialized global db. Pulled out of `open()` so
/// the copy-once behavior is directly testable without depending on
/// `std::env::current_exe()`.
fn seed_if_absent(target: &std::path::Path, seed: &std::path::Path) -> Result<()> {
    if target.exists() {
        return Ok(());
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(seed, target)
        .with_context(|| format!("seeding {} from packaged {}", target.display(), seed.display()))?;
    Ok(())
}

impl McpDb {
    /// Open or create the global mcp.db at `mcp_dir()/mcp.db`. If it does
    /// not exist yet and this binary is running from a packaged release
    /// layout (`<exe_dir>/../data/mcp.db` — `xtask`'s Release Bundling
    /// Step, proposal 16), that packaged db seeds the global one on this
    /// first open only. Every MCP client (Claude Code, OpenCode, Antigravity,
    /// Codex) launches `bin/dharma-mcp` directly, never a launcher script —
    /// seeding here, not in a shell wrapper, is what actually reaches them.
    /// After the first open, `mcp_dir()`'s db is the single global store,
    /// unchanged from `docs/release/mcp-configuration.md`'s documented model.
    pub fn open() -> Result<Self> {
        let path = common::env::mcp_dir().join("mcp.db");
        if let Some(seed) = packaged_seed_db() {
            seed_if_absent(&path, &seed)?;
        }
        Self::open_at(&path)
    }

    /// Open or create the mcp.db at an explicit `path` — used by `xtask`'s
    /// Release Bundling Step to write into a packaged `data/mcp.db` instead
    /// of the runtime default (mirrors `RepoDb::open_at`).
    pub fn open_at(path: &std::path::Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open mcp.db at {}", path.display()))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let store = Self { conn: Mutex::new(conn) };
        store.run_migrations()?;
        Ok(store)
    }

    /// Open an in-memory mcp.db for tests.
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
        for (i, migration) in MCP_MIGRATIONS.iter().enumerate() {
            let version = i as i64 + 1;
            if version > current_version {
                info!("Applying mcp.db migration MCP_V{}", version);
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

    /// Full wipe of mcp.db: drops all tables, re-runs migrations.
    pub fn reset(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = OFF;")?;
        for m in MCP_MIGRATIONS.iter().rev() {
            // no-op placeholder for future multi-migration resets
            let _ = m;
        }
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        drop(conn);
        self.run_migrations()?;
        Ok(())
    }
}

/// Table names present in mcp.db — used by synced_content.kind's allowed set.
pub const MCP_TABLES: &[&str] = &[
    "domain_system_registry",
    "agent_system_registry",
    "content_asset",
    "yaml_template",
    "seeder",
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
    "audit_definition",
    "audit_rule",
    "audit_semantic",
    "audit_calculation",
    "audit_weights",
    "audit_template",
    "analysis_cache",
    "repo_registration",
    "capability_manifest",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_migrations_apply_cleanly() {
        let conn = Connection::open_in_memory().unwrap();
        for m in MCP_MIGRATIONS {
            conn.execute_batch(m).unwrap();
        }
        let exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='repo_registration'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(exists, 1);
    }

    #[test]
    fn schema_version_reaches_one() {
        let db = McpDb::open_in_memory().unwrap();
        assert_eq!(db.schema_version(), 1);
    }

    #[test]
    fn seed_if_absent_copies_when_target_missing() {
        let base = std::env::temp_dir().join(format!("dharma-mcp-seed-test-{}", uuid::Uuid::new_v4()));
        let seed_path = base.join("packaged-data").join("mcp.db");
        let target_path = base.join("home-dharma").join("mcp.db");
        {
            let seed_db = McpDb::open_at(&seed_path).unwrap();
            let conn = seed_db.conn();
            conn.lock().unwrap().execute(
                "INSERT INTO domain_system_registry (name, version, description) VALUES ('seeded', '1.0.0', '')",
                [],
            ).unwrap();
        }
        assert!(!target_path.exists());
        seed_if_absent(&target_path, &seed_path).unwrap();
        assert!(target_path.exists());
        let opened = McpDb::open_at(&target_path).unwrap();
        let name: String = opened
            .conn()
            .lock()
            .unwrap()
            .query_row("SELECT name FROM domain_system_registry", [], |r| r.get(0))
            .unwrap();
        assert_eq!(name, "seeded");
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn seed_if_absent_never_overwrites_existing_target() {
        let base = std::env::temp_dir().join(format!("dharma-mcp-seed-test-{}", uuid::Uuid::new_v4()));
        let seed_path = base.join("packaged-data").join("mcp.db");
        let target_path = base.join("home-dharma").join("mcp.db");
        McpDb::open_at(&seed_path).unwrap();
        {
            let target_db = McpDb::open_at(&target_path).unwrap();
            target_db.conn().lock().unwrap().execute(
                "INSERT INTO domain_system_registry (name, version, description) VALUES ('already-here', '1.0.0', '')",
                [],
            ).unwrap();
        }
        seed_if_absent(&target_path, &seed_path).unwrap();
        let opened = McpDb::open_at(&target_path).unwrap();
        let name: String = opened
            .conn()
            .lock()
            .unwrap()
            .query_row("SELECT name FROM domain_system_registry", [], |r| r.get(0))
            .unwrap();
        assert_eq!(name, "already-here");
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn open_at_creates_parent_dirs_and_migrates() {
        let path = std::env::temp_dir()
            .join(format!("dharma-mcp-db-test-{}", uuid::Uuid::new_v4()))
            .join("data")
            .join("mcp.db");
        let db = McpDb::open_at(&path).unwrap();
        assert!(path.exists());
        assert_eq!(db.schema_version(), 1);
        std::fs::remove_dir_all(path.parent().unwrap().parent().unwrap()).ok();
    }

    #[test]
    fn real_fks_enforced() {
        // repo_registration.domain_system_id is a real FK into
        // domain_system_registry — insert without the parent must fail.
        let db = McpDb::open_in_memory().unwrap();
        let conn = db.conn();
        let result = conn.lock().unwrap().execute(
            "INSERT INTO repo_registration (repo_uuid, repo_name, repo_root, domain_system_id, domain_system_version) VALUES ('u','r','/r',999,'0.0.0')",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn content_asset_cascade_deletes_template() {
        let db = McpDb::open_in_memory().unwrap();
        let conn = db.conn();
        {
            let c = conn.lock().unwrap();
            c.execute(
                "INSERT INTO content_asset (source_system, asset_kind, file_path, content_text, content_hash) VALUES ('s','yaml','p','t','h')",
                [],
            )
            .unwrap();
            c.execute(
                "INSERT INTO yaml_template (content_asset_id, name, scope, template_text) VALUES (1,'n','domain','x')",
                [],
            )
            .unwrap();
            c.execute("DELETE FROM content_asset WHERE id = 1", []).unwrap();
        }
        let count: i64 = conn
            .lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM yaml_template", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}

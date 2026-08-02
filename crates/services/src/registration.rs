//! Repo registration + capability manifest + sync engine (mcp.db 27-28,
//! repo.db 07 + 14).
//!
//! `register_repo` records a consuming repository against a Domain System
//! (proposal 06); the Default/Bootstrap Agent System proposes Agent Systems
//! as `capability_manifest` rows which a human reviews (approve/reject);
//! once at least one row is `approved`, `sync_to_repo` copies the Domain
//! System's content in full and the approved Agent Systems' content in
//! filtered form into the repo's own `repo.db` — every row also materialized
//! as a real file under `.dharma/assets/` — writes the `repo_config` single
//! row, and regenerates `domain-summary.md` / `agent-summary.md` (proposal
//! 11). Domain sync is all-or-nothing; agent sync is scoped to approved
//! manifest rows only, never a blocklist.

use anyhow::{Context, Result};
use common::id::{AgentSystemId, DomainSystemId};
use registry::{McpDb, RepoDb};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rusqlite::types::ValueRef;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RepoRegistrationRow {
    pub id: i64,
    pub repo_uuid: String,
    pub repo_name: String,
    pub repo_root: String,
    pub repo_db_path: Option<String>,
    pub domain_system_id: DomainSystemId,
    pub domain_system_version: String,
    pub status: String,
    pub registered_at: String,
    pub approved_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CapabilityManifestRow {
    pub id: i64,
    pub repo_registration_id: i64,
    pub agent_system_id: AgentSystemId,
    pub proposed_at: String,
    pub status: String,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
}

pub fn register_repo(
    db: &McpDb,
    repo_uuid: &str,
    repo_name: &str,
    repo_root: &str,
    domain_system_id: DomainSystemId,
    domain_system_version: &str,
) -> Result<RepoRegistrationRow> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO repo_registration (repo_uuid, repo_name, repo_root, domain_system_id,
                                        domain_system_version)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![repo_uuid, repo_name, repo_root, domain_system_id.0, domain_system_version],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_repo_registration(db, repo_uuid)?
        .ok_or_else(|| anyhow::anyhow!("repo registration {id} not found after insert"))
}

pub fn get_repo_registration(db: &McpDb, repo_uuid: &str) -> Result<Option<RepoRegistrationRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, repo_uuid, repo_name, repo_root, repo_db_path, domain_system_id,
                domain_system_version, status, registered_at, approved_at
         FROM repo_registration WHERE repo_uuid = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![repo_uuid], map_repo_registration)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_repo_registrations(db: &McpDb) -> Result<Vec<RepoRegistrationRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, repo_uuid, repo_name, repo_root, repo_db_path, domain_system_id,
                domain_system_version, status, registered_at, approved_at
         FROM repo_registration ORDER BY registered_at",
    )?;
    let rows = stmt.query_map([], map_repo_registration)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Move a repo_registration to a status. Setting `approved` records
/// `approved_at` and writes `repo_db_path`. Version mismatch handling marks
/// the row `stale` and blocks Task assignment until re-sync (proposal 02/06).
pub fn set_repo_status(
    db: &McpDb,
    repo_uuid: &str,
    status: &str,
    repo_db_path: Option<&str>,
) -> Result<Option<RepoRegistrationRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let affected = match status {
        "approved" => conn.execute(
            "UPDATE repo_registration
             SET status = 'approved', approved_at = datetime('now'), repo_db_path = ?1
             WHERE repo_uuid = ?2",
            rusqlite::params![repo_db_path, repo_uuid],
        )?,
        _ => conn.execute(
            "UPDATE repo_registration SET status = ?1 WHERE repo_uuid = ?2",
            rusqlite::params![status, repo_uuid],
        )?,
    };
    if affected == 0 {
        return Ok(None);
    }
    drop(conn);
    get_repo_registration(db, repo_uuid)
}

fn map_repo_registration(row: &rusqlite::Row) -> rusqlite::Result<RepoRegistrationRow> {
    Ok(RepoRegistrationRow {
        id: row.get(0)?,
        repo_uuid: row.get(1)?,
        repo_name: row.get(2)?,
        repo_root: row.get(3)?,
        repo_db_path: row.get(4)?,
        domain_system_id: DomainSystemId(row.get(5)?),
        domain_system_version: row.get(6)?,
        status: row.get(7)?,
        registered_at: row.get(8)?,
        approved_at: row.get(9)?,
    })
}

// ---------------------------------------------------------------------------
// Capability manifest (table 28)
// ---------------------------------------------------------------------------

pub fn propose_capability(
    db: &McpDb,
    repo_registration_id: i64,
    agent_system_id: AgentSystemId,
) -> Result<CapabilityManifestRow> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO capability_manifest (repo_registration_id, agent_system_id) VALUES (?1, ?2)",
        rusqlite::params![repo_registration_id, agent_system_id.0],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_capability_manifest(db, id)?
        .ok_or_else(|| anyhow::anyhow!("capability manifest {id} not found after insert"))
}

/// Review a proposed capability. Only `approve`/`reject` transitions out of
/// `proposed`; a rejected entry stays rejected (the reviewer can propose a
/// fresh entry via the Default/Bootstrap Agent System instead of flipping).
pub fn review_capability(
    db: &McpDb,
    manifest_id: i64,
    status: &str,
    reviewed_by: &str,
) -> Result<Option<CapabilityManifestRow>> {
    if !["approved", "rejected"].contains(&status) {
        anyhow::bail!("invalid capability review status '{status}'");
    }
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE capability_manifest
         SET status = ?1, reviewed_by = ?2, reviewed_at = datetime('now')
         WHERE id = ?3 AND status = 'proposed'",
        rusqlite::params![status, reviewed_by, manifest_id],
    )?;
    if affected == 0 {
        return Ok(None);
    }
    drop(conn);
    get_capability_manifest(db, manifest_id)
}

pub fn get_capability_manifest(db: &McpDb, id: i64) -> Result<Option<CapabilityManifestRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, repo_registration_id, agent_system_id, proposed_at, status, reviewed_by, reviewed_at
         FROM capability_manifest WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_capability_manifest)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_capability_manifests(
    db: &McpDb,
    repo_registration_id: i64,
) -> Result<Vec<CapabilityManifestRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, repo_registration_id, agent_system_id, proposed_at, status, reviewed_by, reviewed_at
         FROM capability_manifest WHERE repo_registration_id = ?1 ORDER BY proposed_at",
    )?;
    let rows = stmt.query_map(rusqlite::params![repo_registration_id], map_capability_manifest)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn approved_agent_system_ids(db: &McpDb, repo_registration_id: i64) -> Result<Vec<AgentSystemId>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT agent_system_id FROM capability_manifest
         WHERE repo_registration_id = ?1 AND status = 'approved' ORDER BY agent_system_id",
    )?;
    let rows = stmt.query_map(rusqlite::params![repo_registration_id], |r| {
        r.get::<_, i64>(0)
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(AgentSystemId(row?));
    }
    Ok(out)
}

/// Remove a repo registration by repo uuid. `capability_manifest` rows
/// cascade (ON DELETE CASCADE); the repo's own `repo.db` file is left in
/// place (deleting files is the caller's business).
pub fn unregister_repo(db: &McpDb, repo_uuid: &str) -> Result<bool> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let affected = conn.execute(
        "DELETE FROM repo_registration WHERE repo_uuid = ?1",
        rusqlite::params![repo_uuid],
    )?;
    Ok(affected > 0)
}

/// The Default/Bootstrap Agent System proposes every registered Agent System
/// as a candidate capability for a repo registration (proposal 06); a human
/// then reviews each entry. Idempotent — an already-proposed (repo, agent
/// system) pair is not duplicated (UNIQUE).
pub fn propose_all_capabilities(
    db: &McpDb,
    repo_registration_id: i64,
) -> Result<Vec<CapabilityManifestRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO capability_manifest (repo_registration_id, agent_system_id)
         SELECT ?1, id FROM agent_system_registry",
        rusqlite::params![repo_registration_id],
    )?;
    drop(conn);
    list_capability_manifests(db, repo_registration_id)
}

/// Full registration sequence (proposal 06): resolve the Domain System by
/// name, pin the version (the DS's current version unless pinned), record
/// the repo registration, propose every Agent System as a capability, and
/// move the registration to `manifest_proposed` — where it sits until a
/// human approves at least one capability, which triggers the automatic sync.
pub fn bootstrap_repo_registration(
    db: &McpDb,
    repo_uuid: &str,
    repo_name: &str,
    repo_root: &str,
    domain_system_name: &str,
    domain_system_version: Option<&str>,
) -> Result<RepoRegistrationRow> {
    let ds = crate::get_domain_system_by_name(db, domain_system_name)?
        .ok_or_else(|| anyhow::anyhow!("Domain System '{domain_system_name}' is not registered"))?;
    let version = domain_system_version.unwrap_or(&ds.version).to_string();
    let reg = register_repo(db, repo_uuid, repo_name, repo_root, ds.id, &version)?;
    propose_all_capabilities(db, reg.id)?;
    set_repo_status(db, repo_uuid, "manifest_proposed", None)?;
    get_repo_registration(db, repo_uuid)?
        .ok_or_else(|| anyhow::anyhow!("repo registration {repo_uuid} vanished after bootstrap"))
}

/// Read the repo_config singleton (repo.db 14) back as resolved values.
pub fn get_repo_config(repo: &RepoDb) -> Result<Option<RepoConfigValues>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT repo_uuid, repo_name, repo_root, domain_system_name, domain_system_version,
                docs_dir, implementation_dir, scripts_dir, tests_dir, report_dir, dharma_dir, mcp_dir
         FROM repo_config WHERE id = 1",
    )?;
    let mut rows = stmt.query_map([], |row| {
        Ok(RepoConfigValues {
            repo_uuid: row.get(0)?,
            repo_name: row.get(1)?,
            repo_root: row.get(2)?,
            domain_system_name: row.get(3)?,
            domain_system_version: row.get(4)?,
            docs_dir: row.get(5)?,
            implementation_dir: row.get(6)?,
            scripts_dir: row.get(7)?,
            tests_dir: row.get(8)?,
            report_dir: row.get(9)?,
            dharma_dir: row.get(10)?,
            mcp_dir: row.get(11)?,
        })
    })?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn map_capability_manifest(row: &rusqlite::Row) -> rusqlite::Result<CapabilityManifestRow> {
    Ok(CapabilityManifestRow {
        id: row.get(0)?,
        repo_registration_id: row.get(1)?,
        agent_system_id: AgentSystemId(row.get(2)?),
        proposed_at: row.get(3)?,
        status: row.get(4)?,
        reviewed_by: row.get(5)?,
        reviewed_at: row.get(6)?,
    })
}

// ---------------------------------------------------------------------------
// Sync engine (proposal 11)
// ---------------------------------------------------------------------------

/// Resolved values that land in `repo_config` (repo.db 14). Derived from
/// `dharma-repo.toml` by the caller; the Sync Engine only writes them.
#[derive(Debug, Clone)]
pub struct RepoConfigValues {
    pub repo_uuid: String,
    pub repo_name: String,
    pub repo_root: String,
    pub domain_system_name: String,
    pub domain_system_version: String,
    pub docs_dir: Option<String>,
    pub implementation_dir: Option<String>,
    pub scripts_dir: Option<String>,
    pub tests_dir: Option<String>,
    pub report_dir: Option<String>,
    /// This repo's `.dharma/` directory (holds repo.db, assets/, summaries).
    pub dharma_dir: String,
    /// Where this repo synced FROM; read only on re-sync (proposal 11).
    pub mcp_dir: String,
}

#[derive(Debug, Default)]
pub struct SyncReport {
    pub synced_rows: usize,
    pub materialized_files: usize,
    pub domain_summary_path: PathBuf,
    pub agent_summary_path: PathBuf,
    pub missing_coverage: Vec<String>,
}

/// A row copied into `synced_content`. `content` is what is written both to
/// the DB column and to the materialized file at `local_path`.
struct SyncRow {
    kind: String,
    mcp_row_id: i64,
    system_id: i64,
    is_domain_scoped: bool,
    content: String,
    local_path: String,
    seeder_ref: Option<String>,
}

/// Run the full sync for a repo registration whose manifest has at least one
/// `approved` row. Domain content copies in full; agent content is scoped to
/// approved manifest rows. Writes `repo_config`, regenerates both summaries,
/// and moves the registration to `approved` with `repo_db_path` set.
pub fn sync_to_repo(
    mcp: &McpDb,
    repo: &RepoDb,
    reg: &RepoRegistrationRow,
    values: &RepoConfigValues,
) -> Result<SyncReport> {
    let rows = collect_all_sync_rows(mcp, reg)?;
    let mut report = SyncReport::default();

    // Write synced_content rows + materialize files atomically-ish: write
    // every row to repo.db first; file writes happen alongside, so a failure
    // mid-way leaves DB rows without files — the Hard Constraint says
    // content and file must match, so do files first via a staging dir? Keep
    // it simple: files first, then DB rows, both idempotent on re-run.
    for row in &rows {
        write_materialized_file(&values.dharma_dir, row)?;
        report.materialized_files += 1;
    }
    insert_synced_rows(repo, &rows)?;
    report.synced_rows = rows.len();

    write_repo_config(repo, values)?;

    let approved = approved_agent_system_ids(mcp, reg.id)?;
    let missing = missing_coverage(mcp, repo, reg, &approved)?;
    report.missing_coverage = missing.clone();

    report.domain_summary_path = write_domain_summary(&values.dharma_dir, values, &rows)?;
    report.agent_summary_path = write_agent_summary(&values.dharma_dir, values, &rows, &missing)?;

    set_repo_status(
        mcp,
        &reg.repo_uuid,
        "approved",
        Some(&format!("{}/repo.db", values.dharma_dir)),
    )?;
    Ok(report)
}

/// Gather every synced row from mcp.db for one registration: the Domain
/// System's full content (domain-scoped kinds) plus the approved Agent
/// Systems' content (agent-scoped kinds).
fn collect_all_sync_rows(mcp: &McpDb, reg: &RepoRegistrationRow) -> Result<Vec<SyncRow>> {
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let mut out = Vec::new();
    let ds = reg.domain_system_id.0;

    for kind in registry::SYNCED_CONTENT_DOMAIN_SCOPED {
        collect_kind(&conn, kind, Some(ds), None, &mut out)?;
    }
    for kind in registry::SYNCED_CONTENT_KINDS
        .iter()
        .filter(|k| !registry::SYNCED_CONTENT_DOMAIN_SCOPED.contains(k))
    {
        // agent-scoped kinds: one pass per approved agent system
        let approved = approved_agent_system_ids_in_conn(&conn, reg.id)?;
        for asid in approved {
            collect_kind(&conn, kind, None, Some(asid.0), &mut out)?;
        }
    }
    Ok(out)
}

fn approved_agent_system_ids_in_conn(conn: &rusqlite::Connection, repo_registration_id: i64) -> Result<Vec<AgentSystemId>> {
    let mut stmt = conn.prepare(
        "SELECT agent_system_id FROM capability_manifest
         WHERE repo_registration_id = ?1 AND status = 'approved' ORDER BY agent_system_id",
    )?;
    let rows = stmt.query_map(rusqlite::params![repo_registration_id], |r| r.get::<_, i64>(0))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(AgentSystemId(row?));
    }
    Ok(out)
}

/// Fetch one kind's rows scoped to a system id, building content + path.
fn collect_kind(
    conn: &rusqlite::Connection,
    kind: &str,
    domain_system_id: Option<i64>,
    agent_system_id: Option<i64>,
    out: &mut Vec<SyncRow>,
) -> Result<()> {
    let (sql, id_col) = kind_query(kind);
    let system_id = domain_system_id.or(agent_system_id).unwrap();
    let is_domain = domain_system_id.is_some();
    let (id_param, _) = if is_domain {
        ("domain_system_id", domain_system_id.unwrap())
    } else {
        ("agent_system_id", agent_system_id.unwrap())
    };
    let _ = id_col;
    let _ = id_param;

    let mut stmt = conn.prepare(sql)?;
    let names: Vec<String> = (0..stmt.column_count())
        .map(|i| stmt.column_name(i).unwrap_or("c").to_string())
        .collect();
    let rows = stmt.query_map(rusqlite::params![system_id], |row| {
        let row_id: i64 = row.get(0)?;
        let mut obj = serde_json::Map::new();
        for (i, name) in names.iter().enumerate() {
            obj.insert(name.clone(), value_from_ref(row.get_ref(i)?));
        }
        Ok((row_id, Value::Object(obj)))
    })?;

    for item in rows {
        let (row_id, json) = item?;
        let content = if let Some(text) = kind_text_payload(kind, &json) {
            text
        } else {
            serde_json::to_string_pretty(&json)?
        };
        let ext = kind_extension(kind);
        out.push(SyncRow {
            kind: kind.to_string(),
            mcp_row_id: row_id,
            system_id,
            is_domain_scoped: is_domain,
            content,
            local_path: format!("{kind}/{row_id}{ext}"),
            seeder_ref: if kind == "seeder" {
                json.get("script_ref").and_then(|v| v.as_str()).map(str::to_string)
            } else {
                None
            },
        });
    }
    Ok(())
}

/// Per-kind SQL selecting the row plus its scoping system id. `domain_*` and
/// `agent_*` scoping resolved transitively via JOINs.
fn kind_query(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "domain" => (
            "SELECT d.id, d.domain_system_id, d.name, d.description, d.tier, d.relationship, d.content_asset_id, d.sort_order
             FROM domain d WHERE d.domain_system_id = ?1",
            "id",
        ),
        "section" => (
            "SELECT s.id, s.domain_id, s.section_id, s.title, s.parent_id, s.level, s.sort_order, s.required, s.generated, s.source, s.profile_ref, s.purpose, s.content_asset_id
             FROM section s JOIN domain d ON s.domain_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        "section_profile" => (
            "SELECT p.id, p.section_id, p.inherits, p.writing_objective, p.knowledge_goal, p.reader_goal, p.required_inputs_json, p.expected_outputs_json, p.subsection_rules_json, p.completion_checklist_json, p.review_questions_json, p.validation_rules_json, p.content_asset_id
             FROM section_profile p
             JOIN section s ON p.section_id = s.id
             JOIN domain d ON s.domain_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        "epic" => (
            "SELECT e.id, e.domain_system_id, e.parent_id, e.name, e.objective, e.content_asset_id, e.sort_order
             FROM epic e WHERE e.domain_system_id = ?1",
            "id",
        ),
        "usecase" => (
            "SELECT u.id, u.epic_id, u.name, u.description, u.content_asset_id, u.sort_order
             FROM usecase u JOIN epic e ON u.epic_id = e.id WHERE e.domain_system_id = ?1",
            "id",
        ),
        "task" => (
            "SELECT t.id, t.usecase_id, t.name, t.description, t.input_contract_json, t.output_contract_json, t.acceptance_criteria_json, t.template_ref, t.content_asset_id, t.sort_order
             FROM task t
             JOIN usecase u ON t.usecase_id = u.id
             JOIN epic e ON u.epic_id = e.id WHERE e.domain_system_id = ?1",
            "id",
        ),
        "task_step" => (
            "SELECT ts.id, ts.task_id, ts.step_order, ts.description, ts.required_capability
             FROM task_step ts
             JOIN task t ON ts.task_id = t.id
             JOIN usecase u ON t.usecase_id = u.id
             JOIN epic e ON u.epic_id = e.id WHERE e.domain_system_id = ?1",
            "id",
        ),
        "agent" => (
            "SELECT a.id, a.agent_system_id, a.name, a.role, a.handoff_trigger_condition, a.handoff_candidate_role, a.content_asset_id, a.created_at, a.updated_at
             FROM agent a WHERE a.agent_system_id = ?1",
            "id",
        ),
        "agent_goal" => (
            "SELECT g.id, g.agent_id, g.goal_order, g.goal_text, g.backstory
             FROM agent_goal g JOIN agent a ON g.agent_id = a.id WHERE a.agent_system_id = ?1",
            "id",
        ),
        "skill" => (
            "SELECT s.id, s.agent_system_id, s.name, s.responsibility, s.is_analysis_only, s.invocation_input_json, s.invocation_output_json, s.content_asset_id, s.created_at, s.updated_at
             FROM skill s WHERE s.agent_system_id = ?1",
            "id",
        ),
        "skill_prompt" => (
            "SELECT p.id, p.skill_id, p.template_text, p.content_asset_id
             FROM skill_prompt p JOIN skill s ON p.skill_id = s.id WHERE s.agent_system_id = ?1",
            "id",
        ),
        "skill_script" => (
            "SELECT x.id, x.skill_id, x.script_ref, x.content_asset_id, ca.content_text
             FROM skill_script x
             JOIN skill s ON x.skill_id = s.id
             JOIN content_asset ca ON x.content_asset_id = ca.id WHERE s.agent_system_id = ?1",
            "id",
        ),
        "skill_example" => (
            "SELECT e.id, e.skill_id, e.input_json, e.output_json, e.dos_json, e.donts_json, e.best_practices_json, e.common_mistakes_json, e.content_asset_id
             FROM skill_example e JOIN skill s ON e.skill_id = s.id WHERE s.agent_system_id = ?1",
            "id",
        ),
        "skill_template" => (
            "SELECT t.id, t.skill_id, t.name, t.template_text, t.content_asset_id
             FROM skill_template t JOIN skill s ON t.skill_id = s.id WHERE s.agent_system_id = ?1",
            "id",
        ),
        "agent_skill_binding" => (
            "SELECT b.id, b.agent_id, b.skill_id
             FROM agent_skill_binding b JOIN skill s ON b.skill_id = s.id WHERE s.agent_system_id = ?1",
            "id",
        ),
        "seeder" => (
            "SELECT s.id, s.domain_system_id, s.name, s.script_ref, s.is_generic, s.description
             FROM seeder s WHERE s.domain_system_id = ?1",
            "id",
        ),
        "audit_definition" => (
            "SELECT a.id, a.domain_system_id, a.domain_id, a.name, a.kind, a.scope, a.standard_version, a.content_asset_id
             FROM audit_definition a WHERE a.domain_system_id = ?1",
            "id",
        ),
        "audit_rule" => (
            "SELECT r.id, r.audit_definition_id, r.rule_id, r.description, r.condition, r.message, r.severity, r.weight, r.mandatory, r.evidence_type, r.evidence_target, r.sort_order
             FROM audit_rule r JOIN audit_definition d ON r.audit_definition_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        "audit_semantic" => (
            "SELECT m.id, m.audit_definition_id, m.required_models_json, m.prompt_md_path, m.metadata_fields_json, m.evidence_requirements_json, m.content_asset_id
             FROM audit_semantic m JOIN audit_definition d ON m.audit_definition_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        "audit_calculation" => (
            "SELECT c.id, c.audit_definition_id, c.formula_type, c.formula_json, c.sort_order
             FROM audit_calculation c JOIN audit_definition d ON c.audit_definition_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        "audit_weights" => (
            "SELECT w.id, w.domain_system_id, w.domain_id, w.base_total, w.max_semantic_bonus, w.final_scale, w.weights_json
             FROM audit_weights w WHERE w.domain_system_id = ?1",
            "id",
        ),
        "audit_template" => (
            "SELECT t.id, t.audit_definition_id, t.kind, t.format, t.body, t.content_asset_id
             FROM audit_template t JOIN audit_definition d ON t.audit_definition_id = d.id WHERE d.domain_system_id = ?1",
            "id",
        ),
        _ => (
            "SELECT id, 0 AS domain_system_id FROM domain WHERE 0",
            "id",
        ),
    }
}

/// For kinds whose real payload is a text asset (the DB copy and the file
/// must stay byte-identical), return that text. Everything else serializes
/// as JSON.
fn kind_text_payload(kind: &str, json: &Value) -> Option<String> {
    match kind {
        "skill_prompt" => json.get("template_text").and_then(|v| v.as_str()).map(str::to_string),
        "skill_script" => json.get("content_text").and_then(|v| v.as_str()).map(str::to_string),
        "skill_template" => json.get("template_text").and_then(|v| v.as_str()).map(str::to_string),
        "audit_template" => json.get("body").and_then(|v| v.as_str()).map(str::to_string),
        _ => None,
    }
}

fn kind_extension(kind: &str) -> &'static str {
    match kind {
        "skill_script" => ".py",
        "skill_prompt" => ".md",
        "skill_template" => ".md",
        "audit_template" => ".md",
        _ => ".json",
    }
}

fn value_from_ref(v: ValueRef) -> Value {
    match v {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(i) => Value::from(i),
        ValueRef::Real(f) => Value::from(f),
        ValueRef::Text(t) => Value::String(String::from_utf8_lossy(t).into_owned()),
        ValueRef::Blob(b) => Value::String(String::from_utf8_lossy(b).into_owned()),
    }
}

fn write_materialized_file(dharma_dir: &str, row: &SyncRow) -> Result<()> {
    let base = Path::new(dharma_dir).join("assets");
    let path = base.join(&row.local_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, &row.content)
        .with_context(|| format!("Failed to materialize {}", path.display()))?;
    Ok(())
}

fn insert_synced_rows(repo: &RepoDb, rows: &[SyncRow]) -> Result<()> {
    let conn = repo.conn();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    for row in rows {
        tx.execute(
            "INSERT INTO synced_content (kind, mcp_row_id, domain_system_id, agent_system_id,
                                         content, local_path, seeder_ref)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(kind, mcp_row_id) DO UPDATE SET
                domain_system_id = excluded.domain_system_id,
                agent_system_id = excluded.agent_system_id,
                content = excluded.content,
                local_path = excluded.local_path,
                seeder_ref = excluded.seeder_ref,
                synced_at = datetime('now')",
            rusqlite::params![
                row.kind,
                row.mcp_row_id,
                if row.is_domain_scoped { Some(row.system_id) } else { None },
                if row.is_domain_scoped { None } else { Some(row.system_id) },
                row.content,
                row.local_path,
                row.seeder_ref
            ],
        )?;
    }
    tx.commit()?;
    Ok(())
}

fn write_repo_config(repo: &RepoDb, v: &RepoConfigValues) -> Result<()> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO repo_config (id, repo_uuid, repo_name, repo_root, domain_system_name,
                                  domain_system_version, docs_dir, implementation_dir, scripts_dir,
                                  tests_dir, report_dir, dharma_dir, mcp_dir, last_synced_at)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, datetime('now'))
         ON CONFLICT(id) DO UPDATE SET
            repo_uuid = excluded.repo_uuid, repo_name = excluded.repo_name,
            repo_root = excluded.repo_root, domain_system_name = excluded.domain_system_name,
            domain_system_version = excluded.domain_system_version, docs_dir = excluded.docs_dir,
            implementation_dir = excluded.implementation_dir, scripts_dir = excluded.scripts_dir,
            tests_dir = excluded.tests_dir, report_dir = excluded.report_dir,
            dharma_dir = excluded.dharma_dir, mcp_dir = excluded.mcp_dir,
            last_synced_at = datetime('now')",
        rusqlite::params![
            v.repo_uuid,
            v.repo_name,
            v.repo_root,
            v.domain_system_name,
            v.domain_system_version,
            v.docs_dir,
            v.implementation_dir,
            v.scripts_dir,
            v.tests_dir,
            v.report_dir,
            v.dharma_dir,
            v.mcp_dir
        ],
    )?;
    Ok(())
}

/// Cross-reference the synced `task_step.required_capability` values against
/// the approved Agent Systems' concerns. Concerns with no approved match are
/// reported in `agent-summary.md`'s Missing Coverage section (proposal 11).
fn missing_coverage(
    mcp: &McpDb,
    repo: &RepoDb,
    reg: &RepoRegistrationRow,
    approved: &[AgentSystemId],
) -> Result<Vec<String>> {
    // Required capabilities come from the synced domain content (repo.db).
    let required: Vec<String> = {
        let conn = repo.conn();
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT content->>'required_capability'
             FROM synced_content WHERE kind = 'task_step'",
        )?;
        let mut required: Vec<String> = Vec::new();
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        for cap in rows.flatten() {
            if !required.contains(&cap) {
                required.push(cap);
            }
        }
        required
    };

    // Approved concerns from mcp.db.
    let mconn = mcp.conn();
    let mconn = mconn.lock().unwrap();
    let mut stmt = mconn.prepare(
        "SELECT a.concern FROM agent_system_registry a
         JOIN capability_manifest c ON a.id = c.agent_system_id
         WHERE c.repo_registration_id = ?1 AND c.status = 'approved'",
    )?;
    let rows = stmt.query_map(rusqlite::params![reg.id], |r| r.get::<_, String>(0))?;
    let mut approved_concerns: Vec<String> = Vec::new();
    for row in rows {
        approved_concerns.push(row?);
    }
    let _ = approved;

    Ok(required
        .into_iter()
        .filter(|cap| !approved_concerns.contains(cap))
        .collect())
}

fn write_domain_summary(
    dharma_dir: &str,
    v: &RepoConfigValues,
    rows: &[SyncRow],
) -> Result<PathBuf> {
    let mut md = String::new();
    md.push_str(&format!("# Domain Summary — {}\n\n", v.domain_system_name));
    md.push_str(&format!("- **Version:** {}\n", v.domain_system_version));
    md.push_str(&format!("- **Repo:** {} (`{}`)\n", v.repo_name, v.repo_uuid));
    md.push_str(&format!("- **Synced rows:** {}\n\n", rows.len()));

    let domain_rows: Vec<_> = rows.iter().filter(|r| r.is_domain_scoped).collect();
    md.push_str("## Domains\n\n");
    for row in &domain_rows {
        if row.kind == "domain" {
            md.push_str(&format!("- `{}` (id {})\n", row.local_path, row.mcp_row_id));
        }
    }
    if domain_rows.iter().all(|r| r.kind != "domain") {
        md.push_str("_No domain rows synced._\n");
    }
    md.push_str("\n## Content Received\n\n");
    for row in &domain_rows {
        md.push_str(&format!("- `{}` — {}\n", row.kind, row.local_path));
    }
    md.push_str("\nFull copy of the Domain System; nothing omitted.\n");

    let path = Path::new(dharma_dir).join("domain-summary.md");
    std::fs::write(&path, md)?;
    Ok(path)
}

fn write_agent_summary(
    dharma_dir: &str,
    v: &RepoConfigValues,
    rows: &[SyncRow],
    missing: &[String],
) -> Result<PathBuf> {
    let mut md = String::new();
    md.push_str(&format!("# Agent Summary — {}\n\n", v.repo_name));

    let agent_rows: Vec<_> = rows.iter().filter(|r| !r.is_domain_scoped).collect();
    md.push_str("## Agent Systems\n\n");
    let mut by_system: HashMap<i64, Vec<&SyncRow>> = HashMap::new();
    for row in &agent_rows {
        by_system.entry(row.system_id).or_default().push(row);
    }
    for (system_id, sys_rows) in &by_system {
        md.push_str(&format!("### Agent System `{}`\n\n", system_id));
        for row in sys_rows {
            md.push_str(&format!("- `{}` — {}\n", row.kind, row.local_path));
        }
        md.push('\n');
    }
    if by_system.is_empty() {
        md.push_str("_No Agent Systems approved yet._\n\n");
    }

    md.push_str("## Missing Coverage\n\n");
    if missing.is_empty() {
        md.push_str("None — every required capability has an approved Agent System.\n");
    } else {
        for cap in missing {
            md.push_str(&format!("- `{cap}`\n"));
        }
    }
    md.push('\n');

    let path = Path::new(dharma_dir).join("agent-summary.md");
    std::fs::write(&path, md)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mcp_db() -> McpDb {
        McpDb::open_in_memory().unwrap()
    }

    fn repo_db() -> RepoDb {
        RepoDb::open_in_memory().unwrap()
    }

    #[test]
    fn register_and_approve_repo() {
        let db = mcp_db();
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO domain_system_registry (name, version) VALUES ('ds', '1.0.0')",
                [],
            )
            .unwrap();
        let reg = register_repo(&db, "u", "repo", "/repo", DomainSystemId(1), "1.0.0").unwrap();
        assert_eq!(reg.status, "pending");
        set_repo_status(&db, "u", "approved", Some("/repo/.dharma/repo.db")).unwrap();
        let reg = get_repo_registration(&db, "u").unwrap().unwrap();
        assert_eq!(reg.status, "approved");
        assert!(reg.approved_at.is_some());
        assert!(!reg.approved_at.as_deref().unwrap().is_empty());
        assert_eq!(reg.repo_db_path.as_deref(), Some("/repo/.dharma/repo.db"));
    }

    #[test]
    fn capability_manifest_review_lifecycle() {
        let db = mcp_db();
        let conn = db.conn();
        {
            let c = conn.lock().unwrap();
            c.execute("INSERT INTO domain_system_registry (name, version) VALUES ('ds', '1.0.0')", [])
                .unwrap();
            c.execute("INSERT INTO agent_system_registry (name, concern) VALUES ('as', 'cap')", [])
                .unwrap();
        }
        let reg = register_repo(&db, "u", "repo", "/repo", DomainSystemId(1), "1.0.0").unwrap();
        let m = propose_capability(&db, reg.id, AgentSystemId(1)).unwrap();
        assert_eq!(m.status, "proposed");

        // Rejected entries can't be re-approved (review is one-shot).
        let _ = review_capability(&db, m.id, "rejected", "reviewer").unwrap().unwrap();
        assert!(review_capability(&db, m.id, "approved", "reviewer").unwrap().is_none());

        let approved = approved_agent_system_ids(&db, reg.id).unwrap();
        assert!(approved.is_empty());
    }

    #[test]
    fn approve_then_sync_copies_domain_and_agent_content() {
        let mcp = mcp_db();
        let repo = repo_db();
        let temp = std::env::temp_dir().join(format!("dharma-sync-{}", uuid::Uuid::new_v4()));
        let dharma = temp.join(".dharma");
        std::fs::create_dir_all(&dharma).unwrap();

        // Seed mcp.db: a domain system with one domain + one skill.
        {
            let c = mcp.conn();
            let c = c.lock().unwrap();
            c.execute("INSERT INTO domain_system_registry (name, version) VALUES ('ds', '1.0.0')", [])
                .unwrap();
            c.execute("INSERT INTO agent_system_registry (name, concern) VALUES ('writer', 'writing')", [])
                .unwrap();
            c.execute(
                "INSERT INTO content_asset (source_system, asset_kind, file_path, content_text, content_hash) VALUES ('s','yaml','m.yaml','x','h')",
                [],
            )
            .unwrap();
            c.execute(
                "INSERT INTO domain (domain_system_id, name, content_asset_id) VALUES (1, 'paper', 1)",
                [],
            )
            .unwrap();
            c.execute(
                "INSERT INTO skill (agent_system_id, name, responsibility, is_analysis_only, invocation_input_json, invocation_output_json, content_asset_id)
                 VALUES (1, 'draft', 'draft a section', 0, '{}', '{}', 1)",
                [],
            )
            .unwrap();
            c.execute(
                "INSERT INTO skill_prompt (skill_id, template_text, content_asset_id) VALUES (1, 'Write the section.', 1)",
                [],
            )
            .unwrap();
        }

        let reg = register_repo(&mcp, "u", "repo", temp.to_string_lossy().as_ref(), DomainSystemId(1), "1.0.0").unwrap();
        propose_capability(&mcp, reg.id, AgentSystemId(1)).unwrap();
        let man = list_capability_manifests(&mcp, reg.id).unwrap();
        review_capability(&mcp, man[0].id, "approved", "reviewer").unwrap();

        let values = RepoConfigValues {
            repo_uuid: "u".into(),
            repo_name: "repo".into(),
            repo_root: temp.to_string_lossy().into_owned(),
            domain_system_name: "ds".into(),
            domain_system_version: "1.0.0".into(),
            docs_dir: None,
            implementation_dir: None,
            scripts_dir: None,
            tests_dir: None,
            report_dir: None,
            dharma_dir: dharma.to_string_lossy().into_owned(),
            mcp_dir: "/mcp".into(),
        };

        let report = sync_to_repo(&mcp, &repo, &reg, &values).unwrap();
        assert_eq!(report.synced_rows, 3); // domain + skill + skill_prompt
        assert_eq!(report.materialized_files, 3);

        // Files materialized under .dharma/assets/
        assert!(dharma.join("assets/domain/1.json").exists());
        assert!(dharma.join("assets/skill_prompt/1.md").exists());
        assert!(dharma.join("domain-summary.md").exists());
        assert!(dharma.join("agent-summary.md").exists());

        // repo_config singleton written
        let c = repo.conn();
        let c = c.lock().unwrap();
        let (name, version): (String, String) = c
            .query_row(
                "SELECT domain_system_name, domain_system_version FROM repo_config WHERE id = 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(name, "ds");
        assert_eq!(version, "1.0.0");

        // registration approved with repo_db_path
        let reg2 = get_repo_registration(&mcp, "u").unwrap().unwrap();
        assert_eq!(reg2.status, "approved");
        assert!(reg2.repo_db_path.unwrap().contains("repo.db"));

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn missing_coverage_reports_unapproved_concern() {
        let mcp = mcp_db();
        let repo = repo_db();
        {
            let c = mcp.conn();
            let c = c.lock().unwrap();
            c.execute("INSERT INTO domain_system_registry (name, version) VALUES ('ds', '1.0.0')", [])
                .unwrap();
        }
        // Simulate a synced task_step whose required_capability has no approved agent.
        {
            let c = repo.conn();
            let c = c.lock().unwrap();
            c.execute(
                "INSERT INTO synced_content (kind, mcp_row_id, domain_system_id, agent_system_id, content, local_path)
                 VALUES ('task_step', 1, 1, NULL, '{\"required_capability\":\"rust-development\"}', 'task_step/1.json')",
                [],
            )
            .unwrap();
        }
        let reg = register_repo(&mcp, "u", "repo", "/repo", DomainSystemId(1), "1.0.0").unwrap();
        let missing = missing_coverage(&mcp, &repo, &reg, &[]).unwrap();
        assert_eq!(missing, vec!["rust-development".to_string()]);
    }

    #[test]
    fn repo_config_rejects_second_row() {
        let repo = repo_db();
        let v = RepoConfigValues {
            repo_uuid: "u".into(),
            repo_name: "r".into(),
            repo_root: "/r".into(),
            domain_system_name: "ds".into(),
            domain_system_version: "1.0.0".into(),
            docs_dir: None,
            implementation_dir: None,
            scripts_dir: None,
            tests_dir: None,
            report_dir: None,
            dharma_dir: "/r/.dharma".into(),
            mcp_dir: "/m".into(),
        };
        write_repo_config(&repo, &v).unwrap();
        let conn = repo.conn();
        let count: i64 = conn.lock().unwrap()
            .query_row("SELECT COUNT(*) FROM repo_config", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
        // overwrite path: still one row
        write_repo_config(&repo, &v).unwrap();
        let count: i64 = conn.lock().unwrap()
            .query_row("SELECT COUNT(*) FROM repo_config", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}

//! Audit subsystem execution state (repo.db tables 08-13; proposal 08).
//!
//! One `audit_run` per (commit_hash, domain, kind) — re-running the same
//! audit agent with the same model on the same commit does NOT create a new
//! run; the UNIQUE key rejects it, and an `audit_override` with action
//! 'cancel' retires a run to allow a fresh one. Deterministic audits persist
//! a single weighted-pass-rate result (09); semantic audits persist one
//! score per model (10) plus per-dimension breakdowns (11); findings are
//! normalized rows report templates can iterate (12); every human override
//! or cancel is auditable (13). Runs live in repo.db, not mcp.db, because an
//! audit execution is a per-repo, per-commit observation.

use anyhow::{bail, Result};
use registry::RepoDb;

#[derive(Debug, Clone)]
pub struct AuditRunRow {
    pub id: i64,
    pub commit_hash: String,
    pub domain_id: i64,
    pub domain_name: String,
    pub kind: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeterministicResultRow {
    pub id: i64,
    pub audit_run_id: i64,
    pub score: f64,
    pub rules_passed: i64,
    pub rules_total: i64,
    pub evidence_json: String,
}

#[derive(Debug, Clone)]
pub struct SemanticRunRow {
    pub id: i64,
    pub audit_run_id: i64,
    pub model: String,
    pub overall_score: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub struct SemanticDimensionRow {
    pub id: i64,
    pub semantic_run_id: i64,
    pub dimension: String,
    pub score: f64,
    pub evidence: String,
}

#[derive(Debug, Clone)]
pub struct AuditFindingRow {
    pub id: i64,
    pub audit_run_id: i64,
    pub finding_type: String,
    pub text: String,
    pub sort_order: i64,
}

#[derive(Debug, Clone)]
pub struct AuditOverrideRow {
    pub id: i64,
    pub audit_run_id: i64,
    pub target: String,
    pub action: String,
    pub override_score: Option<f64>,
    pub reason: String,
    pub reviewed_by: String,
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// audit_run (table 08)
// ---------------------------------------------------------------------------

/// Start an audit run. Duplicate (commit_hash, domain_id, kind) is rejected
/// by the UNIQUE key — the same-model-same-commit de-duplication of proposal
/// 08. To re-run, retire the existing run via `override_audit_result` with
/// action 'cancel' first.
pub fn start_audit_run(
    repo: &RepoDb,
    commit_hash: &str,
    domain_id: i64,
    domain_name: &str,
    kind: &str,
) -> Result<AuditRunRow> {
    if !["deterministic", "semantic"].contains(&kind) {
        bail!("invalid audit kind '{kind}'");
    }
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO audit_run (commit_hash, domain_id, domain_name, kind) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![commit_hash, domain_id, domain_name, kind],
    )
    .with_context_duplicate(kind, commit_hash)?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_audit_run(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("audit run {id} not found after insert"))
}

/// Mark a run complete. Only a 'running' run can complete; cancelled runs
/// stay cancelled (de-duplication escape hatch, proposal 08).
pub fn complete_audit_run(repo: &RepoDb, run_id: i64) -> Result<Option<AuditRunRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE audit_run SET status = 'complete', completed_at = datetime('now')
         WHERE id = ?1 AND status = 'running'",
        rusqlite::params![run_id],
    )?;
    drop(conn);
    if affected == 0 {
        return Ok(None);
    }
    get_audit_run(repo, run_id)
}

pub fn get_audit_run(repo: &RepoDb, id: i64) -> Result<Option<AuditRunRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, commit_hash, domain_id, domain_name, kind, status, created_at, completed_at
         FROM audit_run WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_audit_run)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_audit_runs_by_commit(repo: &RepoDb, commit_hash: &str) -> Result<Vec<AuditRunRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, commit_hash, domain_id, domain_name, kind, status, created_at, completed_at
         FROM audit_run WHERE commit_hash = ?1 ORDER BY created_at",
    )?;
    let rows = stmt.query_map(rusqlite::params![commit_hash], map_audit_run)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_audit_run(row: &rusqlite::Row) -> rusqlite::Result<AuditRunRow> {
    Ok(AuditRunRow {
        id: row.get(0)?,
        commit_hash: row.get(1)?,
        domain_id: row.get(2)?,
        domain_name: row.get(3)?,
        kind: row.get(4)?,
        status: row.get(5)?,
        created_at: row.get(6)?,
        completed_at: row.get(7)?,
    })
}

// ---------------------------------------------------------------------------
// Results (tables 09-11)
// ---------------------------------------------------------------------------

/// Persist the deterministic result (weighted pass rate + rule counts) for a
/// run. Exactly one per run (UNIQUE audit_run_id).
pub fn record_deterministic_result(
    repo: &RepoDb,
    run_id: i64,
    score: f64,
    rules_passed: i64,
    rules_total: i64,
    evidence_json: &str,
) -> Result<DeterministicResultRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO audit_deterministic_result (audit_run_id, score, rules_passed, rules_total, evidence_json)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![run_id, score, rules_passed, rules_total, evidence_json],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_deterministic_result(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("deterministic result {id} not found after insert"))
}

pub fn get_deterministic_result(repo: &RepoDb, id: i64) -> Result<Option<DeterministicResultRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, score, rules_passed, rules_total, evidence_json
         FROM audit_deterministic_result WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_deterministic_result)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn get_deterministic_result_for_run(
    repo: &RepoDb,
    run_id: i64,
) -> Result<Option<DeterministicResultRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, score, rules_passed, rules_total, evidence_json
         FROM audit_deterministic_result WHERE audit_run_id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![run_id], map_deterministic_result)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn map_deterministic_result(row: &rusqlite::Row) -> rusqlite::Result<DeterministicResultRow> {
    Ok(DeterministicResultRow {
        id: row.get(0)?,
        audit_run_id: row.get(1)?,
        score: row.get(2)?,
        rules_passed: row.get(3)?,
        rules_total: row.get(4)?,
        evidence_json: row.get(5)?,
    })
}

/// Persist one model's semantic score plus its per-dimension breakdown.
/// One `audit_semantic_run` row per (run, model); each dimension is a row
/// scoped into that semantic run.
pub fn record_semantic_score(
    repo: &RepoDb,
    run_id: i64,
    model: &str,
    overall_score: f64,
    reasoning: &str,
    dimensions: &[(&str, f64, &str)],
) -> Result<SemanticRunRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO audit_semantic_run (audit_run_id, model, overall_score, reasoning)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![run_id, model, overall_score, reasoning],
    )?;
    let semantic_id = conn.last_insert_rowid();
    for (dimension, score, evidence) in dimensions {
        conn.execute(
            "INSERT INTO audit_semantic_dimension (semantic_run_id, dimension, score, evidence)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![semantic_id, dimension, score, evidence],
        )?;
    }
    drop(conn);
    get_semantic_run(repo, semantic_id)?
        .ok_or_else(|| anyhow::anyhow!("semantic run {semantic_id} not found after insert"))
}

pub fn get_semantic_run(repo: &RepoDb, id: i64) -> Result<Option<SemanticRunRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, model, overall_score, reasoning
         FROM audit_semantic_run WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_semantic_run)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_semantic_runs_for_run(repo: &RepoDb, run_id: i64) -> Result<Vec<SemanticRunRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, model, overall_score, reasoning
         FROM audit_semantic_run WHERE audit_run_id = ?1 ORDER BY model",
    )?;
    let rows = stmt.query_map(rusqlite::params![run_id], map_semantic_run)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn list_dimensions_for_semantic_run(
    repo: &RepoDb,
    semantic_run_id: i64,
) -> Result<Vec<SemanticDimensionRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, semantic_run_id, dimension, score, evidence
         FROM audit_semantic_dimension WHERE semantic_run_id = ?1 ORDER BY dimension",
    )?;
    let rows = stmt.query_map(rusqlite::params![semantic_run_id], map_semantic_dimension)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_semantic_run(row: &rusqlite::Row) -> rusqlite::Result<SemanticRunRow> {
    Ok(SemanticRunRow {
        id: row.get(0)?,
        audit_run_id: row.get(1)?,
        model: row.get(2)?,
        overall_score: row.get(3)?,
        reasoning: row.get(4)?,
    })
}

fn map_semantic_dimension(row: &rusqlite::Row) -> rusqlite::Result<SemanticDimensionRow> {
    Ok(SemanticDimensionRow {
        id: row.get(0)?,
        semantic_run_id: row.get(1)?,
        dimension: row.get(2)?,
        score: row.get(3)?,
        evidence: row.get(4)?,
    })
}

// ---------------------------------------------------------------------------
// Findings (table 12)
// ---------------------------------------------------------------------------

/// Add one normalized finding (strength / weakness / recommendation) to a
/// run, so report templates can iterate them without re-parsing evidence.
pub fn add_finding(
    repo: &RepoDb,
    run_id: i64,
    finding_type: &str,
    text: &str,
    sort_order: i64,
) -> Result<AuditFindingRow> {
    if !["strength", "weakness", "recommendation"].contains(&finding_type) {
        bail!("invalid finding type '{finding_type}'");
    }
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO audit_finding (audit_run_id, finding_type, text, sort_order)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![run_id, finding_type, text, sort_order],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_finding(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("finding {id} not found after insert"))
}

pub fn get_finding(repo: &RepoDb, id: i64) -> Result<Option<AuditFindingRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, finding_type, text, sort_order FROM audit_finding WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_finding)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_findings_for_run(repo: &RepoDb, run_id: i64) -> Result<Vec<AuditFindingRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, finding_type, text, sort_order
         FROM audit_finding WHERE audit_run_id = ?1 ORDER BY sort_order, id",
    )?;
    let rows = stmt.query_map(rusqlite::params![run_id], map_finding)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_finding(row: &rusqlite::Row) -> rusqlite::Result<AuditFindingRow> {
    Ok(AuditFindingRow {
        id: row.get(0)?,
        audit_run_id: row.get(1)?,
        finding_type: row.get(2)?,
        text: row.get(3)?,
        sort_order: row.get(4)?,
    })
}

// ---------------------------------------------------------------------------
// Overrides (table 13)
// ---------------------------------------------------------------------------

/// Human override or cancel of an audit result. `target` is 'run',
/// 'deterministic', or 'semantic:<model>'; `action` 'override' (requires
/// `override_score`) or 'cancel' (retires the run — the de-duplication
/// escape hatch for a same-commit re-run). Every entry records reason +
/// reviewer for auditability.
pub fn override_audit_result(
    repo: &RepoDb,
    run_id: i64,
    target: &str,
    action: &str,
    override_score: Option<f64>,
    reason: &str,
    reviewed_by: &str,
) -> Result<AuditOverrideRow> {
    let run = get_audit_run(repo, run_id)?
        .ok_or_else(|| anyhow::anyhow!("audit run {run_id} not found"))?;
    match action {
        "override" => {
            if override_score.is_none() {
                bail!("action 'override' requires override_score");
            }
            match target {
                "run" | "deterministic" => {}
                t if t.starts_with("semantic:") => {
                    // the named model must be part of the run's ensemble
                    let model = &t["semantic:".len()..];
                    let models = list_semantic_runs_for_run(repo, run_id)?
                        .into_iter()
                        .map(|r| r.model)
                        .collect::<Vec<_>>();
                    if !models.contains(&model.to_string()) {
                        bail!(
                            "cannot override target '{target}': model '{model}' has no score in run {run_id}"
                        );
                    }
                }
                _ => bail!("invalid override target '{target}'"),
            }
        }
        "cancel" => {
            if run.status != "running" {
                bail!("cannot cancel run {run_id}: status is '{}'", run.status);
            }
        }
        _ => bail!("invalid override action '{action}'"),
    }

    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO audit_override (audit_run_id, target, action, override_score, reason, reviewed_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![run_id, target, action, override_score, reason, reviewed_by],
    )?;
    let id = conn.last_insert_rowid();
    if action == "cancel" {
        conn.execute(
            "UPDATE audit_run SET status = 'cancelled', completed_at = datetime('now')
             WHERE id = ?1",
            rusqlite::params![run_id],
        )?;
    }
    drop(conn);
    get_override(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("override {id} not found after insert"))
}

pub fn get_override(repo: &RepoDb, id: i64) -> Result<Option<AuditOverrideRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, target, action, override_score, reason, reviewed_by, created_at
         FROM audit_override WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_override)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_overrides_for_run(repo: &RepoDb, run_id: i64) -> Result<Vec<AuditOverrideRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, audit_run_id, target, action, override_score, reason, reviewed_by, created_at
         FROM audit_override WHERE audit_run_id = ?1 ORDER BY created_at",
    )?;
    let rows = stmt.query_map(rusqlite::params![run_id], map_override)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_override(row: &rusqlite::Row) -> rusqlite::Result<AuditOverrideRow> {
    Ok(AuditOverrideRow {
        id: row.get(0)?,
        audit_run_id: row.get(1)?,
        target: row.get(2)?,
        action: row.get(3)?,
        override_score: row.get(4)?,
        reason: row.get(5)?,
        reviewed_by: row.get(6)?,
        created_at: row.get(7)?,
    })
}

trait WithContextDuplicate {
    fn with_context_duplicate(self, kind: &str, commit_hash: &str) -> Result<()>;
}

impl WithContextDuplicate for rusqlite::Result<usize> {
    fn with_context_duplicate(self, kind: &str, commit_hash: &str) -> Result<()> {
        match self {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.to_string().contains("UNIQUE") {
                    bail!(
                        "audit run ({kind}, {commit_hash}) already exists for this commit; \
                         retire it with an audit_override('cancel') before re-running"
                    )
                }
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_db() -> RepoDb {
        RepoDb::open_in_memory().unwrap()
    }

    fn start(repo: &RepoDb, kind: &str) -> AuditRunRow {
        start_audit_run(repo, "abc123", 1, "paper", kind).unwrap()
    }

    #[test]
    fn duplicate_run_rejected() {
        let repo = repo_db();
        start(&repo, "deterministic");
        let err = start_audit_run(&repo, "abc123", 1, "paper", "deterministic");
        assert!(err.is_err());
    }

    #[test]
    fn deterministic_run_records_and_completes() {
        let repo = repo_db();
        let run = start(&repo, "deterministic");
        assert_eq!(run.status, "running");

        let res = record_deterministic_result(&repo, run.id, 0.8, 4, 5, r#"{"rules":[{"id":"r1"}]}"#)
            .unwrap();
        assert_eq!(res.rules_total, 5);

        // second result violates UNIQUE(audit_run_id)
        assert!(record_deterministic_result(&repo, run.id, 0.9, 5, 5, "{}").is_err());

        let done = complete_audit_run(&repo, run.id).unwrap().unwrap();
        assert_eq!(done.status, "complete");
        assert!(done.completed_at.is_some());

        // completing twice is a no-op (returns None)
        assert!(complete_audit_run(&repo, run.id).unwrap().is_none());
    }

    #[test]
    fn semantic_run_records_models_and_dimensions() {
        let repo = repo_db();
        let run = start(&repo, "semantic");
        let sr = record_semantic_score(
            &repo,
            run.id,
            "model-a",
            0.7,
            "solid",
            &[("clarity", 0.8, "clear prose"), ("coverage", 0.6, "missed one")],
        )
        .unwrap();
        assert_eq!(sr.model, "model-a");

        let dims = list_dimensions_for_semantic_run(&repo, sr.id).unwrap();
        assert_eq!(dims.len(), 2);
        assert_eq!(dims[0].dimension, "clarity");

        let models = list_semantic_runs_for_run(&repo, run.id).unwrap();
        assert_eq!(models.len(), 1);
    }

    #[test]
    fn findings_require_valid_type() {
        let repo = repo_db();
        let run = start(&repo, "deterministic");
        add_finding(&repo, run.id, "strength", "good", 1).unwrap();
        assert!(add_finding(&repo, run.id, "bogus", "bad", 2).is_err());
        assert_eq!(list_findings_for_run(&repo, run.id).unwrap().len(), 1);
    }

    #[test]
    fn cancel_retires_run_but_keeps_dedup_slot() {
        let repo = repo_db();
        let run = start(&repo, "deterministic");
        let ov = override_audit_result(&repo, run.id, "run", "cancel", None, "retry", "human").unwrap();
        assert_eq!(ov.action, "cancel");
        let run2 = get_audit_run(&repo, run.id).unwrap().unwrap();
        assert_eq!(run2.status, "cancelled");

        // de-duplication holds even for a cancelled run: the (commit, domain,
        // kind) slot is consumed, the run is retired from reports, not deleted.
        assert!(start_audit_run(&repo, "abc123", 1, "paper", "deterministic").is_err());
    }

    #[test]
    fn override_requires_score_and_valid_target() {
        let repo = repo_db();
        let run = start(&repo, "semantic");
        record_semantic_score(&repo, run.id, "model-a", 0.7, "r", &[]).unwrap();

        // override without score rejected
        assert!(override_audit_result(&repo, run.id, "deterministic", "override", None, "r", "h").is_err());
        // override of a model not in the ensemble rejected
        assert!(
            override_audit_result(&repo, run.id, "semantic:ghost", "override", Some(0.9), "r", "h").is_err()
        );
        // valid per-model override lands
        let ov = override_audit_result(&repo, run.id, "semantic:model-a", "override", Some(0.9), "r", "h")
            .unwrap();
        assert_eq!(ov.override_score, Some(0.9));
    }

    #[test]
    fn cancel_rejected_after_complete() {
        let repo = repo_db();
        let run = start(&repo, "deterministic");
        record_deterministic_result(&repo, run.id, 0.5, 1, 2, "{}").unwrap();
        complete_audit_run(&repo, run.id).unwrap();
        assert!(override_audit_result(&repo, run.id, "run", "cancel", None, "r", "h").is_err());
    }
}

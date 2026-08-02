//! Proposal lifecycle & archival (repo.db tables 15-16; proposal 12).
//!
//! Every Proposal document (`docs/proposal/NN-name.md`) gets exactly one
//! `proposal_lifecycle` row in the repo's own repo.db. Status only moves
//! forward — draft → finalized → implementing → verified → archived — and
//! each forward move requires the commit hash that justifies it, enforced by
//! the table's own CHECK constraint (this module's guards give the friendly
//! errors; the CHECK is the backstop). The full commit history behind the
//! milestone snapshot lives in append-only `proposal_commit_log`.

use anyhow::{bail, Context, Result};
use registry::RepoDb;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProposalLifecycleRow {
    pub id: i64,
    pub name: String,
    pub title: String,
    pub doc_path: String,
    pub status: String,
    pub draft_commit_hash: Option<String>,
    pub finalized_commit_hash: Option<String>,
    pub implementation_commit_hash: Option<String>,
    pub verified_at: Option<String>,
    pub archive_commit_hash: Option<String>,
    pub archived_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct ProposalCommitLogRow {
    pub id: i64,
    pub proposal_lifecycle_id: i64,
    pub commit_hash: String,
    pub phase: String,
    pub message: String,
    pub recorded_at: String,
}

const PROPOSAL_PHASES: &[&str] = &["draft", "finalized", "implementation", "archive"];
const STATUS_ORDER: &[&str] = &["draft", "finalized", "implementing", "verified", "archived"];

/// Register a proposal document, starting in `draft` with no commit yet
/// (per proposal 12: "at minimum, in draft status with no commit yet").
pub fn register_proposal(
    repo: &RepoDb,
    name: &str,
    title: &str,
    doc_path: &str,
) -> Result<ProposalLifecycleRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO proposal_lifecycle (name, title, doc_path) VALUES (?1, ?2, ?3)",
        rusqlite::params![name, title, doc_path],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_proposal_by_id(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("proposal lifecycle {id} not found after insert"))
}

/// Record the draft commit that introduced the document. Only while still in
/// `draft` status; the commit is appended to the log phase-tagged 'draft'.
pub fn set_draft_commit(
    repo: &RepoDb,
    name: &str,
    commit_hash: &str,
    message: &str,
) -> Result<Option<ProposalLifecycleRow>> {
    let row = require_status(repo, name, "draft")?;
    update_and_log(repo, &row, "draft_commit_hash", commit_hash, "draft", commit_hash, message)
}

/// draft → finalized. Requires the draft commit to be on record; records the
/// finalizing commit (which may be the same commit that drafted it, per
/// proposal 12's own history) and flips status.
pub fn finalize_proposal(
    repo: &RepoDb,
    name: &str,
    finalized_commit_hash: &str,
    message: &str,
) -> Result<Option<ProposalLifecycleRow>> {
    let row = require_status(repo, name, "draft")?;
    if row.draft_commit_hash.is_none() {
        bail!(
            "cannot finalize '{name}': no draft_commit_hash on record (call set_draft_commit first)"
        );
    }
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "UPDATE proposal_lifecycle
         SET status = 'finalized', finalized_commit_hash = ?1, updated_at = datetime('now')
         WHERE name = ?2",
        rusqlite::params![finalized_commit_hash, name],
    )?;
    drop(conn);
    let row = get_proposal(repo, name)?.unwrap();
    append_log(repo, &row, "finalized", finalized_commit_hash, message)?;
    Ok(Some(row))
}

/// finalized → implementing. No new commit is required by the CHECK — the
/// implementation commits that follow are recorded via `record_commit`.
pub fn start_implementation(repo: &RepoDb, name: &str) -> Result<Option<ProposalLifecycleRow>> {
    let row = require_status(repo, name, "finalized")?;
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "UPDATE proposal_lifecycle SET status = 'implementing', updated_at = datetime('now')
         WHERE name = ?1",
        rusqlite::params![name],
    )?;
    drop(conn);
    let _ = row;
    get_proposal(repo, name)
}

/// implementing → verified. Requires the final implementation commit + the
/// verified timestamp; the implementation commit is logged phase-tagged.
pub fn verify_proposal(
    repo: &RepoDb,
    name: &str,
    implementation_commit_hash: &str,
    message: &str,
) -> Result<Option<ProposalLifecycleRow>> {
    require_status(repo, name, "implementing")?;
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "UPDATE proposal_lifecycle
         SET status = 'verified', implementation_commit_hash = ?1, verified_at = datetime('now'),
             updated_at = datetime('now')
         WHERE name = ?2",
        rusqlite::params![implementation_commit_hash, name],
    )?;
    drop(conn);
    let row = get_proposal(repo, name)?.unwrap();
    append_log(repo, &row, "implementation", implementation_commit_hash, message)?;
    Ok(Some(row))
}

/// verified → archived. Moves the document from `docs/proposal/` to
/// `docs/proposal/archive/` (only against a `verified` row), records the
/// moving commit, and flips status to `archived`. `repo_root` anchors the
/// repo-relative `doc_path`.
pub fn archive_proposal(
    repo: &RepoDb,
    name: &str,
    archive_commit_hash: &str,
    message: &str,
    repo_root: &Path,
) -> Result<Option<ProposalLifecycleRow>> {
    let row = require_status(repo, name, "verified")?;

    let old_path = repo_root.join(&row.doc_path);
    let archive_dir = repo_root.join("docs").join("proposal").join("archive");
    if !old_path.exists() {
        bail!(
            "cannot archive '{name}': source document {} does not exist",
            old_path.display()
        );
    }
    std::fs::create_dir_all(&archive_dir)
        .with_context(|| format!("Failed to create {}", archive_dir.display()))?;
    let file_name = old_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("cannot derive filename from {}", old_path.display()))?;
    let new_path = archive_dir.join(file_name);
    std::fs::rename(&old_path, &new_path)
        .with_context(|| format!("Failed to move {} to {}", old_path.display(), new_path.display()))?;
    let new_doc_path = format!("docs/proposal/archive/{file_name}");

    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "UPDATE proposal_lifecycle
         SET status = 'archived', doc_path = ?1, archive_commit_hash = ?2,
             archived_at = datetime('now'), updated_at = datetime('now')
         WHERE name = ?3",
        rusqlite::params![new_doc_path, archive_commit_hash, name],
    )?;
    drop(conn);
    let row = get_proposal(repo, name)?.unwrap();
    append_log(repo, &row, "archive", archive_commit_hash, message)?;
    Ok(Some(row))
}

/// Append one commit to a proposal's full history, phase-tagged. Deduplicated
/// by (proposal_lifecycle_id, commit_hash) — the same commit is never logged
/// twice for the same proposal.
pub fn record_commit(
    repo: &RepoDb,
    name: &str,
    commit_hash: &str,
    phase: &str,
    message: &str,
) -> Result<Option<ProposalCommitLogRow>> {
    if !PROPOSAL_PHASES.contains(&phase) {
        bail!("invalid commit-log phase '{phase}'");
    }
    let row = get_proposal(repo, name)?
        .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not registered"))?;
    let id = append_log(repo, &row, phase, commit_hash, message)?;
    get_commit_log_entry(repo, id)
}

pub fn get_proposal(repo: &RepoDb, name: &str) -> Result<Option<ProposalLifecycleRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, title, doc_path, status, draft_commit_hash, finalized_commit_hash,
                implementation_commit_hash, verified_at, archive_commit_hash, archived_at,
                created_at, updated_at
         FROM proposal_lifecycle WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], map_proposal)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn get_proposal_by_id(repo: &RepoDb, id: i64) -> Result<Option<ProposalLifecycleRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, title, doc_path, status, draft_commit_hash, finalized_commit_hash,
                implementation_commit_hash, verified_at, archive_commit_hash, archived_at,
                created_at, updated_at
         FROM proposal_lifecycle WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_proposal)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_proposals_by_status(repo: &RepoDb, status: &str) -> Result<Vec<ProposalLifecycleRow>> {
    if !STATUS_ORDER.contains(&status) {
        bail!("invalid proposal status '{status}'");
    }
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, title, doc_path, status, draft_commit_hash, finalized_commit_hash,
                implementation_commit_hash, verified_at, archive_commit_hash, archived_at,
                created_at, updated_at
         FROM proposal_lifecycle WHERE status = ?1 ORDER BY name",
    )?;
    let rows = stmt.query_map(rusqlite::params![status], map_proposal)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn list_commit_log(repo: &RepoDb, name: &str) -> Result<Vec<ProposalCommitLogRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let row = get_proposal_by_id_locked(&conn, name)?;
    let mut stmt = conn.prepare(
        "SELECT id, proposal_lifecycle_id, commit_hash, phase, message, recorded_at
         FROM proposal_commit_log WHERE proposal_lifecycle_id = ?1 ORDER BY id",
    )?;
    let rows = stmt.query_map(rusqlite::params![row.id], map_commit_log)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn get_commit_log_entry(repo: &RepoDb, id: i64) -> Result<Option<ProposalCommitLogRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, proposal_lifecycle_id, commit_hash, phase, message, recorded_at
         FROM proposal_commit_log WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_commit_log)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Guard: the proposal must exist and be in exactly the expected status.
fn require_status(repo: &RepoDb, name: &str, expected: &str) -> Result<ProposalLifecycleRow> {
    let row = get_proposal(repo, name)?
        .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not registered"))?;
    if row.status != expected {
        bail!(
            "cannot advance '{name}': status is '{}', expected '{expected}'",
            row.status
        );
    }
    Ok(row)
}

/// Set a single milestone hash column (used for the draft commit).
fn update_and_log(
    repo: &RepoDb,
    row: &ProposalLifecycleRow,
    column: &str,
    hash: &str,
    phase: &str,
    log_hash: &str,
    message: &str,
) -> Result<Option<ProposalLifecycleRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        &format!(
            "UPDATE proposal_lifecycle SET {column} = ?1, updated_at = datetime('now') WHERE id = ?2"
        ),
        rusqlite::params![hash, row.id],
    )?;
    drop(conn);
    let updated = get_proposal(repo, &row.name)?.unwrap();
    append_log(repo, &updated, phase, log_hash, message)?;
    Ok(Some(updated))
}

fn append_log(
    repo: &RepoDb,
    row: &ProposalLifecycleRow,
    phase: &str,
    commit_hash: &str,
    message: &str,
) -> Result<i64> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO proposal_commit_log (proposal_lifecycle_id, commit_hash, phase, message)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![row.id, commit_hash, phase, message],
    )?;
    let id = conn.last_insert_rowid();
    if id == 0 {
        // INSERT OR IGNORE hit the UNIQUE(proposal_lifecycle_id, commit_hash)
        // — find the existing row so the caller still gets a handle on it.
        let existing = conn.query_row(
            "SELECT id FROM proposal_commit_log
             WHERE proposal_lifecycle_id = ?1 AND commit_hash = ?2",
            rusqlite::params![row.id, commit_hash],
            |r| r.get::<_, i64>(0),
        )?;
        Ok(existing)
    } else {
        Ok(id)
    }
}

fn get_proposal_by_id_locked(conn: &rusqlite::Connection, name: &str) -> Result<ProposalLifecycleRow> {
    let mut stmt = conn.prepare(
        "SELECT id, name, title, doc_path, status, draft_commit_hash, finalized_commit_hash,
                implementation_commit_hash, verified_at, archive_commit_hash, archived_at,
                created_at, updated_at
         FROM proposal_lifecycle WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], map_proposal)?;
    match rows.next() {
        Some(Ok(row)) => Ok(row),
        _ => bail!("proposal '{name}' not registered"),
    }
}

fn map_proposal(row: &rusqlite::Row) -> rusqlite::Result<ProposalLifecycleRow> {
    Ok(ProposalLifecycleRow {
        id: row.get(0)?,
        name: row.get(1)?,
        title: row.get(2)?,
        doc_path: row.get(3)?,
        status: row.get(4)?,
        draft_commit_hash: row.get(5)?,
        finalized_commit_hash: row.get(6)?,
        implementation_commit_hash: row.get(7)?,
        verified_at: row.get(8)?,
        archive_commit_hash: row.get(9)?,
        archived_at: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
    })
}

fn map_commit_log(row: &rusqlite::Row) -> rusqlite::Result<ProposalCommitLogRow> {
    Ok(ProposalCommitLogRow {
        id: row.get(0)?,
        proposal_lifecycle_id: row.get(1)?,
        commit_hash: row.get(2)?,
        phase: row.get(3)?,
        message: row.get(4)?,
        recorded_at: row.get(5)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_db() -> RepoDb {
        RepoDb::open_in_memory().unwrap()
    }

    fn register(repo: &RepoDb, name: &str) -> ProposalLifecycleRow {
        register_proposal(repo, name, "Title", &format!("docs/proposal/{name}.md")).unwrap()
    }

    #[test]
    fn register_starts_in_draft() {
        let repo = repo_db();
        let p = register(&repo, "01-x");
        assert_eq!(p.status, "draft");
        assert!(p.draft_commit_hash.is_none());
    }

    #[test]
    fn full_lifecycle_to_verified() {
        let repo = repo_db();
        let p = register(&repo, "02-y");
        assert_eq!(p.name, "02-y");

        set_draft_commit(&repo, "02-y", "aaa111", "draft it").unwrap();
        let f = finalize_proposal(&repo, "02-y", "bbb222", "finalize").unwrap().unwrap();
        assert_eq!(f.status, "finalized");
        assert_eq!(f.draft_commit_hash.as_deref(), Some("aaa111"));
        assert_eq!(f.finalized_commit_hash.as_deref(), Some("bbb222"));

        let imp = start_implementation(&repo, "02-y").unwrap().unwrap();
        assert_eq!(imp.status, "implementing");

        record_commit(&repo, "02-y", "ccc333", "implementation", "wip").unwrap();
        let v = verify_proposal(&repo, "02-y", "ddd444", "done").unwrap().unwrap();
        assert_eq!(v.status, "verified");
        assert_eq!(v.implementation_commit_hash.as_deref(), Some("ddd444"));
        assert!(v.verified_at.is_some());

        // every milestone hash must appear in the log with its phase
        let log = list_commit_log(&repo, "02-y").unwrap();
        let phases: Vec<_> = log.iter().map(|l| (l.phase.as_str(), l.commit_hash.as_str())).collect();
        assert!(phases.contains(&("draft", "aaa111")));
        assert!(phases.contains(&("finalized", "bbb222")));
        assert!(phases.contains(&("implementation", "ccc333")));
        assert!(phases.contains(&("implementation", "ddd444")));
    }

    #[test]
    fn cannot_finalize_without_draft_commit() {
        let repo = repo_db();
        register(&repo, "03-z");
        let err = finalize_proposal(&repo, "03-z", "bbb222", "finalize");
        assert!(err.is_err());
    }

    #[test]
    fn cannot_skip_forward_states() {
        let repo = repo_db();
        let p = register(&repo, "04-w");
        // draft → verified without finalizing is blocked by the status guard
        assert!(verify_proposal(&repo, "04-w", "x", "skip").is_err());
        // draft_commit + finalize, then straight to verified still blocked
        set_draft_commit(&repo, "04-w", "a", "d").unwrap();
        finalize_proposal(&repo, "04-w", "b", "f").unwrap();
        start_implementation(&repo, "04-w").unwrap();
        let v = verify_proposal(&repo, "04-w", "c", "v").unwrap();
        assert_eq!(v.unwrap().status, "verified");
        let _ = p;
    }

    #[test]
    fn archive_moves_file_and_records_commit() {
        let repo = repo_db();
        register(&repo, "05-v");
        set_draft_commit(&repo, "05-v", "a", "d").unwrap();
        finalize_proposal(&repo, "05-v", "b", "f").unwrap();
        start_implementation(&repo, "05-v").unwrap();
        verify_proposal(&repo, "05-v", "c", "v").unwrap();

        // create the document file under a temp repo root
        let temp = std::env::temp_dir().join(format!("dharma-prop-{}", uuid::Uuid::new_v4()));
        let doc_dir = temp.join("docs").join("proposal");
        std::fs::create_dir_all(&doc_dir).unwrap();
        let doc = doc_dir.join("05-v.md");
        std::fs::write(&doc, "# x").unwrap();

        let arch = archive_proposal(&repo, "05-v", "e", "archived it", &temp).unwrap().unwrap();
        assert_eq!(arch.status, "archived");
        assert_eq!(arch.doc_path, "docs/proposal/archive/05-v.md");
        assert!(arch.archive_commit_hash.is_some());
        assert!(arch.archived_at.is_some());

        // file actually moved
        assert!(!doc.exists());
        assert!(temp.join("docs").join("proposal").join("archive").join("05-v.md").exists());

        let log = list_commit_log(&repo, "05-v").unwrap();
        assert!(log.iter().any(|l| l.phase == "archive" && l.commit_hash == "e"));
        assert!(log.iter().any(|l| l.phase == "draft" && l.commit_hash == "a"));

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn cannot_archive_before_verified() {
        let repo = repo_db();
        register(&repo, "06-u");
        let temp = std::env::temp_dir().join(format!("dharma-prop-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(temp.join("docs/proposal")).unwrap();
        std::fs::write(temp.join("docs/proposal/06-u.md"), "# x").unwrap();
        let err = archive_proposal(&repo, "06-u", "e", "early", &temp);
        assert!(err.is_err());
        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn commit_log_deduplicates_same_hash() {
        let repo = repo_db();
        register(&repo, "07-t");
        set_draft_commit(&repo, "07-t", "abc", "one").unwrap();
        record_commit(&repo, "07-t", "abc", "draft", "again").unwrap();
        let log = list_commit_log(&repo, "07-t").unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].commit_hash, "abc");
    }

    #[test]
    fn invalid_status_and_phase_rejected() {
        let repo = repo_db();
        register(&repo, "08-s");
        assert!(list_proposals_by_status(&repo, "bogus").is_err());
        assert!(record_commit(&repo, "08-s", "abc", "bogus", "m").is_err());
    }
}

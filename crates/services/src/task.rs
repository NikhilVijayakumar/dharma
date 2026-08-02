//! Task Instance lifecycle: the Proposal Loop and Execution Loop (repo.db
//! tables 00-06; proposal 07).
//!
//! A Task is never executed directly. The initiating Agent drafts a proposed
//! solution (`proposal_revision`), the user reviews and may request changes
//! (each round appends a revision), and only an explicit approval
//! (`proposal_approval`) moves the instance into the Execution Loop, where
//! agents hand off control (`handoff_log`, `execution_state`) carrying an
//! append-only `context_envelope`. The Completion Validator's verdict is
//! one `completion_validation` row per instance; a 'fail' is terminal for
//! that instance — rework creates a NEW task_instance row (bumped
//! `attempt_number`, linked via `previous_task_instance_id`).
//!
//! Cross-database references (task_id, agent system/agent ids) are logical
//! pointers into mcp.db, enforced here in code, not by FK constraints.

use anyhow::{bail, Result};
use common::id::TaskInstanceId;
use registry::RepoDb;

#[derive(Debug, Clone)]
pub struct TaskInstanceRow {
    pub id: TaskInstanceId,
    pub task_id: i64,
    pub initiating_agent_system_id: i64,
    pub initiating_agent_id: i64,
    pub attempt_number: i64,
    pub previous_task_instance_id: Option<i64>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct ProposalRevisionRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub revision_number: i64,
    pub drafted_by_agent_system_id: i64,
    pub drafted_by_agent_id: i64,
    pub draft_json: String,
    pub reviewer_comments: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct ProposalApprovalRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub approved_revision_id: i64,
    pub approved_by: String,
    pub approved_at: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionStateRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub current_step_order: i64,
    pub current_owning_agent_system_id: i64,
    pub current_owning_agent_id: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct HandoffLogRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub hop_order: i64,
    pub from_agent_system_id: i64,
    pub from_agent_id: i64,
    pub to_agent_system_id: i64,
    pub to_agent_id: i64,
    pub reason: String,
    pub accepted: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct ContextEnvelopeRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub hop_order: i64,
    pub hop_kind: String,
    pub payload_json: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct CompletionValidationRow {
    pub id: i64,
    pub task_instance_id: TaskInstanceId,
    pub verdict: String,
    pub reason: String,
    pub validated_at: String,
}

// ---------------------------------------------------------------------------
// task_instance (table 00)
// ---------------------------------------------------------------------------

/// Assign a Task to this repo: create a fresh instance in the Proposal Loop.
/// Rework after a failure is a NEW instance, linked back via
/// `previous_task_instance_id` and a bumped `attempt_number`.
pub fn create_task_instance(
    repo: &RepoDb,
    task_id: i64,
    agent_system_id: i64,
    agent_id: i64,
) -> Result<TaskInstanceRow> {
    create_task_instance_with_attempt(repo, task_id, agent_system_id, agent_id, 1, None)
}

pub fn rework_task_instance(
    repo: &RepoDb,
    failed_instance_id: TaskInstanceId,
    agent_system_id: i64,
    agent_id: i64,
) -> Result<TaskInstanceRow> {
    let failed = get_task_instance(repo, failed_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("task instance {failed_instance_id} not found"))?;
    if failed.status != "failed" {
        bail!(
            "cannot rework task instance {}: status is '{}', not 'failed'",
            failed.id.0,
            failed.status
        );
    }
    create_task_instance_with_attempt(
        repo,
        failed.task_id,
        agent_system_id,
        agent_id,
        failed.attempt_number + 1,
        Some(failed.id.0),
    )
}

fn create_task_instance_with_attempt(
    repo: &RepoDb,
    task_id: i64,
    agent_system_id: i64,
    agent_id: i64,
    attempt_number: i64,
    previous: Option<i64>,
) -> Result<TaskInstanceRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO task_instance (task_id, initiating_agent_system_id, initiating_agent_id,
                                    attempt_number, previous_task_instance_id, status, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 'proposing', datetime('now'))",
        rusqlite::params![task_id, agent_system_id, agent_id, attempt_number, previous],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    get_task_instance(repo, TaskInstanceId(id))?
        .ok_or_else(|| anyhow::anyhow!("task instance {id} not found after insert"))
}

pub fn get_task_instance(repo: &RepoDb, id: TaskInstanceId) -> Result<Option<TaskInstanceRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_id, initiating_agent_system_id, initiating_agent_id, attempt_number,
                previous_task_instance_id, status, created_at, updated_at
         FROM task_instance WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id.0], map_task_instance)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_task_instances_by_status(repo: &RepoDb, status: &str) -> Result<Vec<TaskInstanceRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_id, initiating_agent_system_id, initiating_agent_id, attempt_number,
                previous_task_instance_id, status, created_at, updated_at
         FROM task_instance WHERE status = ?1 ORDER BY created_at",
    )?;
    let rows = stmt.query_map(rusqlite::params![status], map_task_instance)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_task_instance(row: &rusqlite::Row) -> rusqlite::Result<TaskInstanceRow> {
    Ok(TaskInstanceRow {
        id: TaskInstanceId(row.get(0)?),
        task_id: row.get(1)?,
        initiating_agent_system_id: row.get(2)?,
        initiating_agent_id: row.get(3)?,
        attempt_number: row.get(4)?,
        previous_task_instance_id: row.get(5)?,
        status: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

fn touch_instance(conn: &rusqlite::Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE task_instance SET updated_at = datetime('now') WHERE id = ?1",
        rusqlite::params![id],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Proposal Loop (tables 01-02)
// ---------------------------------------------------------------------------

/// Append one proposal revision for an instance in the Proposal Loop. Each
/// revision also adds a `context_envelope` entry (hop_kind 'proposal') so the
/// drafting history stays in the same append-only envelope as execution hops.
pub fn draft_proposal(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    agent_system_id: i64,
    agent_id: i64,
    draft_json: &str,
) -> Result<ProposalRevisionRow> {
    let conn = repo.conn();
    let mut conn = conn.lock().unwrap();
    let instance = get_task_instance_locked(&conn, task_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("task instance {task_instance_id} not found"))?;
    if instance.status != "proposing" {
        bail!(
            "cannot draft proposal for task instance {}: status is '{}', not 'proposing'",
            task_instance_id.0,
            instance.status
        );
    }

    let revision_number = next_revision_number(&conn, task_instance_id)?;
    conn.execute(
        "INSERT INTO proposal_revision (task_instance_id, revision_number, drafted_by_agent_system_id,
                                        drafted_by_agent_id, draft_json, reviewer_comments)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL)",
        rusqlite::params![task_instance_id.0, revision_number, agent_system_id, agent_id, draft_json],
    )?;
    let revision_id = conn.last_insert_rowid();

    append_context_envelope_locked(
        &mut conn,
        task_instance_id,
        "proposal",
        &format!(
            r#"{{"revision_number":{revision_number},"draft":{}}}"#,
            draft_json
        ),
    )?;
    touch_instance(&conn, task_instance_id.0)?;
    drop(conn);

    get_proposal_revision(repo, revision_id)?
        .ok_or_else(|| anyhow::anyhow!("proposal revision {revision_id} not found after insert"))
}

/// Attach reviewer feedback to a revision (revision rows are otherwise
/// append-only). Returns the updated revision.
pub fn comment_on_revision(
    repo: &RepoDb,
    revision_id: i64,
    reviewer_comments: &str,
) -> Result<Option<ProposalRevisionRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE proposal_revision SET reviewer_comments = ?1 WHERE id = ?2",
        rusqlite::params![reviewer_comments, revision_id],
    )?;
    drop(conn);
    if affected == 0 {
        return Ok(None);
    }
    get_proposal_revision(repo, revision_id)
}

/// The user's single approval event. Moves the instance to 'approved',
/// recording which revision was approved. At most one approval per instance
/// (UNIQUE(task_instance_id)) — a Task is approved once, never re-approved.
pub fn approve_proposal(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    approved_revision_id: i64,
    approved_by: &str,
) -> Result<ProposalApprovalRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let instance = get_task_instance_locked(&conn, task_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("task instance {task_instance_id} not found"))?;
    if instance.status != "proposing" {
        bail!(
            "cannot approve task instance {}: status is '{}', not 'proposing'",
            task_instance_id.0,
            instance.status
        );
    }
    // the approved revision must belong to this instance
    let owns: i64 = conn.query_row(
        "SELECT COUNT(*) FROM proposal_revision WHERE id = ?1 AND task_instance_id = ?2",
        rusqlite::params![approved_revision_id, task_instance_id.0],
        |r| r.get(0),
    )?;
    if owns == 0 {
        bail!(
            "revision {approved_revision_id} does not belong to task instance {}",
            task_instance_id.0
        );
    }
    conn.execute(
        "INSERT INTO proposal_approval (task_instance_id, approved_revision_id, approved_by)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![task_instance_id.0, approved_revision_id, approved_by],
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "UPDATE task_instance SET status = 'approved', updated_at = datetime('now')
         WHERE id = ?1",
        rusqlite::params![task_instance_id.0],
    )?;
    drop(conn);

    get_proposal_approval(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("proposal approval {id} not found after insert"))
}

pub fn get_proposal_revision(repo: &RepoDb, id: i64) -> Result<Option<ProposalRevisionRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, revision_number, drafted_by_agent_system_id,
                drafted_by_agent_id, draft_json, reviewer_comments, created_at
         FROM proposal_revision WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_proposal_revision)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_proposal_revisions(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
) -> Result<Vec<ProposalRevisionRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, revision_number, drafted_by_agent_system_id,
                drafted_by_agent_id, draft_json, reviewer_comments, created_at
         FROM proposal_revision WHERE task_instance_id = ?1 ORDER BY revision_number",
    )?;
    let rows = stmt.query_map(rusqlite::params![task_instance_id.0], map_proposal_revision)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

pub fn get_proposal_approval(
    repo: &RepoDb,
    id: i64,
) -> Result<Option<ProposalApprovalRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, approved_revision_id, approved_by, approved_at
         FROM proposal_approval WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_proposal_approval)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn map_proposal_revision(row: &rusqlite::Row) -> rusqlite::Result<ProposalRevisionRow> {
    Ok(ProposalRevisionRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        revision_number: row.get(2)?,
        drafted_by_agent_system_id: row.get(3)?,
        drafted_by_agent_id: row.get(4)?,
        draft_json: row.get(5)?,
        reviewer_comments: row.get(6)?,
        created_at: row.get(7)?,
    })
}

fn map_proposal_approval(row: &rusqlite::Row) -> rusqlite::Result<ProposalApprovalRow> {
    Ok(ProposalApprovalRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        approved_revision_id: row.get(2)?,
        approved_by: row.get(3)?,
        approved_at: row.get(4)?,
    })
}

fn next_revision_number(conn: &rusqlite::Connection, task_instance_id: TaskInstanceId) -> Result<i64> {
    let n: Option<i64> = conn
        .query_row(
            "SELECT MAX(revision_number) FROM proposal_revision WHERE task_instance_id = ?1",
            rusqlite::params![task_instance_id.0],
            |r| r.get(0),
        )
        .unwrap_or(None);
    Ok(n.map_or(1, |m| m + 1))
}

// ---------------------------------------------------------------------------
// Execution Loop (tables 03-04)
// ---------------------------------------------------------------------------

/// Open the Execution Loop for an approved instance: create its
/// `execution_state` row (first step, initiating agent) and move the
/// instance to 'executing'. Refuses any instance that has not been approved.
pub fn begin_execution(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    agent_system_id: i64,
    agent_id: i64,
) -> Result<ExecutionStateRow> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let instance = get_task_instance_locked(&conn, task_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("task instance {task_instance_id} not found"))?;
    if instance.status != "approved" {
        bail!(
            "cannot begin execution for task instance {}: status is '{}', not 'approved'",
            task_instance_id.0,
            instance.status
        );
    }
    conn.execute(
        "INSERT INTO execution_state (task_instance_id, current_step_order,
                                      current_owning_agent_system_id, current_owning_agent_id)
         VALUES (?1, 1, ?2, ?3)",
        rusqlite::params![task_instance_id.0, agent_system_id, agent_id],
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "UPDATE task_instance SET status = 'executing', updated_at = datetime('now')
         WHERE id = ?1",
        rusqlite::params![task_instance_id.0],
    )?;
    drop(conn);

    get_execution_state(repo, task_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("execution state {id} not found after insert"))
}

/// Gate a Skill invocation against the Task Instance it's being invoked
/// within (proposal 07's Hard Constraint: "Proposal Loop Skills must be
/// analysis-only — effect-capable Skills are inert until the Execution
/// Loop"). An analysis-only Skill (`is_analysis_only`) may run in any
/// status; an effect-capable Skill (a bound script or template) may only
/// run while the instance is `executing`. Called by both `run_skill`
/// entry points (CLI and MCP) before a script is ever spawned.
pub fn check_skill_invocation_allowed(status: &str, is_analysis_only: bool) -> Result<()> {
    if is_analysis_only || status == "executing" {
        return Ok(());
    }
    bail!(
        "cannot run effect-capable skill for a task instance with status '{status}': \
         effect-capable skills are inert until the Execution Loop ('executing')"
    );
}

/// Update the Execution Loop's current position in place (the append-only
/// history lives in `handoff_log`). Returns the updated state.
pub fn advance_execution(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    next_step_order: i64,
    agent_system_id: i64,
    agent_id: i64,
) -> Result<Option<ExecutionStateRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE execution_state
         SET current_step_order = ?1, current_owning_agent_system_id = ?2,
             current_owning_agent_id = ?3, updated_at = datetime('now')
         WHERE task_instance_id = ?4",
        rusqlite::params![next_step_order, agent_system_id, agent_id, task_instance_id.0],
    )?;
    drop(conn);
    if affected == 0 {
        return Ok(None);
    }
    get_execution_state(repo, task_instance_id)
}

/// Record one handoff hop (append-only) and mirror the accepted transfer
/// into `execution_state`. Returns the new handoff row.
pub fn record_handoff(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    from_agent_system_id: i64,
    from_agent_id: i64,
    to_agent_system_id: i64,
    to_agent_id: i64,
    reason: &str,
    accepted: bool,
    payload_json: Option<&str>,
) -> Result<HandoffLogRow> {
    let conn = repo.conn();
    let mut conn = conn.lock().unwrap();
    let hop_order = next_handoff_hop(&conn, task_instance_id)?;
    conn.execute(
        "INSERT INTO handoff_log (task_instance_id, hop_order, from_agent_system_id,
                                  from_agent_id, to_agent_system_id, to_agent_id, reason, accepted)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            task_instance_id.0,
            hop_order,
            from_agent_system_id,
            from_agent_id,
            to_agent_system_id,
            to_agent_id,
            reason,
            accepted as i64
        ],
    )?;
    let id = conn.last_insert_rowid();
    if accepted {
        conn.execute(
            "UPDATE execution_state
             SET current_owning_agent_system_id = ?1, current_owning_agent_id = ?2,
                 updated_at = datetime('now')
             WHERE task_instance_id = ?3",
            rusqlite::params![to_agent_system_id, to_agent_id, task_instance_id.0],
        )?;
    }
    append_context_envelope_locked(
        &mut conn,
        task_instance_id,
        "execution",
        payload_json.unwrap_or("{}"),
    )?;
    drop(conn);

    get_handoff_log_entry(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("handoff log entry {id} not found after insert"))
}

pub fn get_execution_state(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
) -> Result<Option<ExecutionStateRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, current_step_order, current_owning_agent_system_id,
                current_owning_agent_id, updated_at
         FROM execution_state WHERE task_instance_id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![task_instance_id.0], map_execution_state)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn get_handoff_log_entry(repo: &RepoDb, id: i64) -> Result<Option<HandoffLogRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, hop_order, from_agent_system_id, from_agent_id,
                to_agent_system_id, to_agent_id, reason, accepted, created_at
         FROM handoff_log WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_handoff_log)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_handoff_log(repo: &RepoDb, task_instance_id: TaskInstanceId) -> Result<Vec<HandoffLogRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, hop_order, from_agent_system_id, from_agent_id,
                to_agent_system_id, to_agent_id, reason, accepted, created_at
         FROM handoff_log WHERE task_instance_id = ?1 ORDER BY hop_order",
    )?;
    let rows = stmt.query_map(rusqlite::params![task_instance_id.0], map_handoff_log)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_execution_state(row: &rusqlite::Row) -> rusqlite::Result<ExecutionStateRow> {
    Ok(ExecutionStateRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        current_step_order: row.get(2)?,
        current_owning_agent_system_id: row.get(3)?,
        current_owning_agent_id: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

fn map_handoff_log(row: &rusqlite::Row) -> rusqlite::Result<HandoffLogRow> {
    Ok(HandoffLogRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        hop_order: row.get(2)?,
        from_agent_system_id: row.get(3)?,
        from_agent_id: row.get(4)?,
        to_agent_system_id: row.get(5)?,
        to_agent_id: row.get(6)?,
        reason: row.get(7)?,
        accepted: row.get(8)?,
        created_at: row.get(9)?,
    })
}

fn next_handoff_hop(conn: &rusqlite::Connection, task_instance_id: TaskInstanceId) -> Result<i64> {
    let n: Option<i64> = conn
        .query_row(
            "SELECT MAX(hop_order) FROM handoff_log WHERE task_instance_id = ?1",
            rusqlite::params![task_instance_id.0],
            |r| r.get(0),
        )
        .unwrap_or(None);
    Ok(n.map_or(1, |m| m + 1))
}

// ---------------------------------------------------------------------------
// Context Envelope (table 05)
// ---------------------------------------------------------------------------

/// Append one context envelope hop (proposal drafting or execution handoff),
/// in a single hop_order sequence shared across both loops.
pub fn append_context_envelope(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    hop_kind: &str,
    payload_json: &str,
) -> Result<ContextEnvelopeRow> {
    let conn = repo.conn();
    let mut conn = conn.lock().unwrap();
    let id = append_context_envelope_locked(&mut conn, task_instance_id, hop_kind, payload_json)?;
    drop(conn);
    get_context_envelope(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("context envelope {id} not found after insert"))
}

fn append_context_envelope_locked(
    conn: &mut rusqlite::Connection,
    task_instance_id: TaskInstanceId,
    hop_kind: &str,
    payload_json: &str,
) -> Result<i64> {
    let hop_order = next_envelope_hop(conn, task_instance_id)?;
    conn.execute(
        "INSERT INTO context_envelope (task_instance_id, hop_order, hop_kind, payload_json)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![task_instance_id.0, hop_order, hop_kind, payload_json],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_context_envelope(repo: &RepoDb, id: i64) -> Result<Option<ContextEnvelopeRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, hop_order, hop_kind, payload_json, created_at
         FROM context_envelope WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_context_envelope)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

/// Replay a task instance's full context envelope, oldest hop first.
pub fn replay_context_envelope(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
) -> Result<Vec<ContextEnvelopeRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, hop_order, hop_kind, payload_json, created_at
         FROM context_envelope WHERE task_instance_id = ?1 ORDER BY hop_order",
    )?;
    let rows = stmt.query_map(rusqlite::params![task_instance_id.0], map_context_envelope)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn map_context_envelope(row: &rusqlite::Row) -> rusqlite::Result<ContextEnvelopeRow> {
    Ok(ContextEnvelopeRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        hop_order: row.get(2)?,
        hop_kind: row.get(3)?,
        payload_json: row.get(4)?,
        created_at: row.get(5)?,
    })
}

fn next_envelope_hop(conn: &rusqlite::Connection, task_instance_id: TaskInstanceId) -> Result<i64> {
    let n: Option<i64> = conn
        .query_row(
            "SELECT MAX(hop_order) FROM context_envelope WHERE task_instance_id = ?1",
            rusqlite::params![task_instance_id.0],
            |r| r.get(0),
        )
        .unwrap_or(None);
    Ok(n.map_or(1, |m| m + 1))
}

// ---------------------------------------------------------------------------
// Completion Validator (table 06)
// ---------------------------------------------------------------------------

/// The Completion Validator's verdict, checked against the Task's acceptance
/// criteria (mcp.db) independent of the executing Agent. Exactly one verdict
/// per instance: 'pass' → instance 'complete'; 'fail' → instance 'failed'
/// (terminal — rework is a new instance).
pub fn complete_task_instance(
    repo: &RepoDb,
    task_instance_id: TaskInstanceId,
    verdict: &str,
    reason: &str,
) -> Result<CompletionValidationRow> {
    if !["pass", "fail"].contains(&verdict) {
        bail!("invalid completion verdict '{verdict}'");
    }
    let new_status = if verdict == "pass" { "complete" } else { "failed" };
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let instance = get_task_instance_locked(&conn, task_instance_id)?
        .ok_or_else(|| anyhow::anyhow!("task instance {task_instance_id} not found"))?;
    if !["executing", "approved"].contains(&instance.status.as_str()) {
        bail!(
            "cannot complete task instance {}: status is '{}'",
            task_instance_id.0,
            instance.status
        );
    }
    conn.execute(
        "INSERT INTO completion_validation (task_instance_id, verdict, reason)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![task_instance_id.0, verdict, reason],
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "UPDATE task_instance SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![new_status, task_instance_id.0],
    )?;
    drop(conn);

    get_completion_validation(repo, id)?
        .ok_or_else(|| anyhow::anyhow!("completion validation {id} not found after insert"))
}

pub fn get_completion_validation(
    repo: &RepoDb,
    id: i64,
) -> Result<Option<CompletionValidationRow>> {
    let conn = repo.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, task_instance_id, verdict, reason, validated_at
         FROM completion_validation WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_completion_validation)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn map_completion_validation(row: &rusqlite::Row) -> rusqlite::Result<CompletionValidationRow> {
    Ok(CompletionValidationRow {
        id: row.get(0)?,
        task_instance_id: TaskInstanceId(row.get(1)?),
        verdict: row.get(2)?,
        reason: row.get(3)?,
        validated_at: row.get(4)?,
    })
}

fn get_task_instance_locked(
    conn: &rusqlite::Connection,
    id: TaskInstanceId,
) -> Result<Option<TaskInstanceRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, initiating_agent_system_id, initiating_agent_id, attempt_number,
                previous_task_instance_id, status, created_at, updated_at
         FROM task_instance WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id.0], map_task_instance)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_db() -> RepoDb {
        RepoDb::open_in_memory().unwrap()
    }

    #[test]
    fn instance_starts_proposing() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        assert_eq!(ti.attempt_number, 1);
        assert_eq!(ti.status, "proposing");
        assert!(ti.previous_task_instance_id.is_none());
    }

    #[test]
    fn draft_revise_approve_enters_execution() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();

        let r1 = draft_proposal(&repo, ti.id, 1, 1, r#"{"plan":"a"}"#).unwrap();
        assert_eq!(r1.revision_number, 1);
        comment_on_revision(&repo, r1.id, "expand section 2").unwrap();
        let r2 = draft_proposal(&repo, ti.id, 1, 1, r#"{"plan":"a+2"}"#).unwrap();
        assert_eq!(r2.revision_number, 2);

        let revs = list_proposal_revisions(&repo, ti.id).unwrap();
        assert_eq!(revs.len(), 2);
        assert_eq!(revs[0].reviewer_comments.as_deref(), Some("expand section 2"));

        let approval = approve_proposal(&repo, ti.id, r2.id, "user").unwrap();
        assert_eq!(approval.approved_revision_id, r2.id);
        assert_eq!(get_task_instance(&repo, ti.id).unwrap().unwrap().status, "approved");

        let es = begin_execution(&repo, ti.id, 1, 1).unwrap();
        assert_eq!(es.current_step_order, 1);
        assert_eq!(es.current_owning_agent_id, 1);
        assert_eq!(get_task_instance(&repo, ti.id).unwrap().unwrap().status, "executing");
    }

    #[test]
    fn cannot_approve_twice() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        let r1 = draft_proposal(&repo, ti.id, 1, 1, "{}").unwrap();
        approve_proposal(&repo, ti.id, r1.id, "user").unwrap();
        // second approval violates UNIQUE(task_instance_id)
        let err = approve_proposal(&repo, ti.id, r1.id, "user");
        assert!(err.is_err());
    }

    #[test]
    fn begin_execution_requires_approval() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        draft_proposal(&repo, ti.id, 1, 1, "{}").unwrap();
        let err = begin_execution(&repo, ti.id, 1, 1);
        assert!(err.is_err());
    }

    #[test]
    fn handoff_advances_owner_and_envelope() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        draft_proposal(&repo, ti.id, 1, 1, "{}").unwrap();
        let rev = list_proposal_revisions(&repo, ti.id).unwrap();
        approve_proposal(&repo, ti.id, rev[0].id, "user").unwrap();
        begin_execution(&repo, ti.id, 1, 1).unwrap();

        let hop = record_handoff(
            &repo,
            ti.id,
            1, 1,   // from agent system 1, agent 1
            2, 2,   // to agent system 2, agent 2
            "needs a second capability",
            true,
            Some(r#"{"output":"partial"}"#),
        )
        .unwrap();
        assert_eq!(hop.hop_order, 1);
        assert!(hop.accepted);

        let es = get_execution_state(&repo, ti.id).unwrap().unwrap();
        assert_eq!(es.current_owning_agent_system_id, 2);
        assert_eq!(es.current_owning_agent_id, 2);

        // one proposal envelope hop + one execution hop
        let env = replay_context_envelope(&repo, ti.id).unwrap();
        assert_eq!(env.len(), 2);
        assert_eq!(env[0].hop_kind, "proposal");
        assert_eq!(env[1].hop_kind, "execution");
    }

    #[test]
    fn completion_pass_then_fail_rework() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        draft_proposal(&repo, ti.id, 1, 1, "{}").unwrap();
        let rev = list_proposal_revisions(&repo, ti.id).unwrap();
        approve_proposal(&repo, ti.id, rev[0].id, "user").unwrap();
        begin_execution(&repo, ti.id, 1, 1).unwrap();

        let v = complete_task_instance(&repo, ti.id, "fail", "criteria unmet").unwrap();
        assert_eq!(v.verdict, "fail");
        let ti_failed = get_task_instance(&repo, ti.id).unwrap().unwrap();
        assert_eq!(ti_failed.status, "failed");

        // rework: new instance, bumped attempt, linked back
        let rework = rework_task_instance(&repo, ti.id, 3, 3).unwrap();
        assert_eq!(rework.attempt_number, 2);
        assert_eq!(rework.previous_task_instance_id, Some(ti.id.0));
        assert_eq!(rework.status, "proposing");
    }

    #[test]
    fn completion_is_one_per_instance() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        draft_proposal(&repo, ti.id, 1, 1, "{}").unwrap();
        let rev = list_proposal_revisions(&repo, ti.id).unwrap();
        approve_proposal(&repo, ti.id, rev[0].id, "user").unwrap();
        begin_execution(&repo, ti.id, 1, 1).unwrap();
        complete_task_instance(&repo, ti.id, "pass", "ok").unwrap();
        // second verdict violates UNIQUE(task_instance_id)
        assert!(complete_task_instance(&repo, ti.id, "pass", "again").is_err());
    }

    #[test]
    fn cannot_redo_an_already_reworked_instance() {
        let repo = repo_db();
        let ti = create_task_instance(&repo, 1, 1, 1).unwrap();
        let err = rework_task_instance(&repo, ti.id, 2, 2);
        assert!(err.is_err()); // not 'failed' yet
    }

    #[test]
    fn analysis_only_skill_runs_in_any_status() {
        for status in ["proposing", "approved", "executing", "complete", "failed"] {
            assert!(check_skill_invocation_allowed(status, true).is_ok());
        }
    }

    #[test]
    fn effect_capable_skill_only_runs_while_executing() {
        assert!(check_skill_invocation_allowed("executing", false).is_ok());
        for status in ["proposing", "approved", "complete", "failed"] {
            let err = check_skill_invocation_allowed(status, false);
            assert!(err.is_err(), "expected '{status}' to reject an effect-capable skill");
        }
    }
}

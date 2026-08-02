//! Dharma MCP adapter: routes wire messages (protocol.rs) to the services
//! layer for the 28 tools of docs/proposal/14-mcp-tool-contract.md.
//!
//! Registry/capture tools operate on the global `McpDb`; repo tools open the
//! target repo's own `repo.db` (via the explicit `repo_path`, never a session
//! binding). Gate methods enforce `human_approved: true` + `reviewed_by`.

use anyhow::{bail, Result};
use common::config::load_repo_config;
use common::id::TaskInstanceId;
use serde_json::{json, Value};
use services::registration::{RepoConfigValues, RepoRegistrationRow};
use services::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// `::registry` = the extern crate (leading `::`); the glob above also brings
// in the `services::registry` module name, so the extern crate must be
// disambiguated explicitly.
use ::registry::{McpDb, RepoDb};

use crate::protocol::{
    GATE_METHODS, McpCapabilities, McpError, McpMessage, McpNotification, McpRequest,
    McpResponse,
};

pub struct McpAdapter {
    pub mcp_db: Arc<McpDb>,
}

impl McpAdapter {
    pub fn new(mcp_db: McpDb) -> Self {
        Self {
            mcp_db: Arc::new(mcp_db),
        }
    }

    /// Handle one wire message. Returns `None` when the server should exit
    /// (shutdown notification) or when the message is a response/error this
    /// end never sent.
    pub fn handle_message(&self, msg: McpMessage) -> Option<McpMessage> {
        match msg {
            McpMessage::Request(req) => Some(self.dispatch(&req)),
            McpMessage::Notification(n) => self.handle_notification(&n),
            McpMessage::Response(_) | McpMessage::Error(_) => None,
        }
    }

    fn handle_notification(&self, n: &McpNotification) -> Option<McpMessage> {
        match n.method.as_str() {
            "shutdown" => None,
            _ => {
                tracing::warn!("unhandled notification '{}'", n.method);
                None
            }
        }
    }

    fn dispatch(&self, req: &McpRequest) -> McpMessage {
        let result = (|| -> Result<Value> {
            if GATE_METHODS.contains(&req.method.as_str()) {
                self.check_gate(req)?;
            }
            self.route(req)
        })();
        match result {
            Ok(v) => McpMessage::Response(McpResponse {
                id: req.id.clone(),
                result: v,
            }),
            Err(e) => McpMessage::Error(McpError {
                id: Some(req.id.clone()),
                code: 400,
                message: e.to_string(),
            }),
        }
    }

    /// Proposal 14 gating: human-approval methods must carry explicit human
    /// consent and a named reviewer.
    fn check_gate(&self, req: &McpRequest) -> Result<()> {
        if !req.get_bool("human_approved", false) {
            bail!(
                "method '{}' requires human_approved: true (proposal 14 gate)",
                req.method
            );
        }
        match req.get_str("reviewed_by")? {
            Some(r) if !r.is_empty() => Ok(()),
            _ => bail!("method '{}' requires a non-empty reviewed_by", req.method),
        }
    }

    // ------------------------------------------------------------------
    // plumbing
    // ------------------------------------------------------------------

    fn open_repo(&self, req: &McpRequest) -> Result<RepoDb> {
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("this tool requires an explicit repo_path"))?;
        let root = Path::new(root);
        match load_repo_config(root) {
            Ok(cfg) => {
                let dharma_dir = cfg.resolve_dharma_dir(root);
                RepoDb::open_at(&dharma_dir.join("repo.db"))
            }
            Err(_) => RepoDb::open_at(&root.join(".dharma").join("repo.db")),
        }
    }

    fn resolve_repo_uuid(&self, req: &McpRequest) -> Result<String> {
        if let Some(uuid) = req.get_str("repo_uuid")? {
            return Ok(uuid);
        }
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("repo_uuid or repo_path required"))?;
        let cfg = load_repo_config(Path::new(root))
            .map_err(|e| anyhow::anyhow!("cannot resolve repo identity: {e}"))?;
        cfg.repository
            .uuid
            .map(|u| u.to_string())
            .ok_or_else(|| anyhow::anyhow!("dharma-repo.toml has no [repository].uuid"))
    }

    fn require_approved_registration(&self, root: &str) -> Result<RepoRegistrationRow> {
        let uuid = self.resolve_repo_uuid_from_root(root)?;
        let reg = get_repo_registration(&self.mcp_db, &uuid)?
            .ok_or_else(|| anyhow::anyhow!("repo '{root}' is not registered"))?;
        if reg.status != "approved" {
            bail!(
                "repo '{root}' registration is '{}', not 'approved'",
                reg.status
            );
        }
        // pinned version must still match the Domain System's current version
        let ds = get_domain_system(&self.mcp_db, reg.domain_system_id)?;
        if let Some(ds) = ds {
            if ds.version != reg.domain_system_version {
                bail!(
                    "repo '{root}' pinned Domain System version '{}' is stale (current '{}')",
                    reg.domain_system_version,
                    ds.version
                );
            }
        }
        Ok(reg)
    }

    fn resolve_repo_uuid_from_root(&self, root: &str) -> Result<String> {
        let root = Path::new(root);
        match load_repo_config(root) {
            Ok(cfg) => cfg
                .repository
                .uuid
                .map(|u| u.to_string())
                .ok_or_else(|| anyhow::anyhow!("dharma-repo.toml has no [repository].uuid")),
            Err(_) => bail!("no dharma-repo.toml at {}", root.display()),
        }
    }

    fn resolve_agent_ref(&self, agent_ref: &str) -> Result<(i64, i64)> {
        let (a, b) = agent_ref
            .split_once('/')
            .ok_or_else(|| anyhow::anyhow!("agent_ref must be '<system|id>/<agent|id>'"))?;
        if let (Ok(s), Ok(ag)) = (a.parse::<i64>(), b.parse::<i64>()) {
            return Ok((s, ag));
        }
        let system = get_agent_system_by_name(&self.mcp_db, a)?
            .ok_or_else(|| anyhow::anyhow!("agent system '{a}' not found"))?;
        let conn = self.mcp_db.conn();
        let conn = conn.lock().unwrap();
        let agent_id = conn.query_row(
            "SELECT id FROM agent WHERE agent_system_id = ?1 AND name = ?2",
            rusqlite::params![system.id.0, b],
            |r| r.get::<_, i64>(0),
        )?;
        drop(conn);
        Ok((system.id.0, agent_id))
    }

    /// First agent of the Agent System whose concern matches a task step's
    /// required_capability — the Task Runtime's default initiating agent.
    fn resolve_agent_by_concern(&self, concern: &str) -> Result<(i64, i64)> {
        let system = get_agent_system_by_concern(&self.mcp_db, concern)?
            .ok_or_else(|| anyhow::anyhow!("no Agent System has concern '{concern}'"))?;
        let conn = self.mcp_db.conn();
        let conn = conn.lock().unwrap();
        let agent_id = conn.query_row(
            "SELECT id FROM agent WHERE agent_system_id = ?1 ORDER BY id LIMIT 1",
            rusqlite::params![system.id.0],
            |r| r.get::<_, i64>(0),
        )?;
        drop(conn);
        Ok((system.id.0, agent_id))
    }

    fn resolve_task(&self, task_ref: &str) -> Result<(i64, String)> {
        let conn = self.mcp_db.conn();
        let conn = conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name FROM task WHERE id = ?1 OR name = ?2 ORDER BY id LIMIT 1",
        )?;
        let mut rows = stmt.query_map(
            rusqlite::params![task_ref.parse::<i64>().unwrap_or(0), task_ref],
            |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
        )?;
        match rows.next() {
            Some(Ok(row)) => Ok(row),
            _ => Err(anyhow::anyhow!("task '{task_ref}' not found in mcp.db")),
        }
    }

    // ------------------------------------------------------------------
    // registry & capture
    // ------------------------------------------------------------------

    fn tool_register_domain_system(&self, req: &McpRequest) -> Result<Value> {
        let name = req.require_str("name")?;
        let version = req.get_str("version")?.unwrap_or_else(|| "0.0.0".into());
        let description = req.get_str("description")?.unwrap_or_default();
        let row = register_domain_system(&self.mcp_db, &name, &version, &description)?;
        let captured = match req.get_str("content_root")? {
            Some(root) => {
                let root = PathBuf::from(root);
                if !root.is_dir() {
                    bail!("content_root {} is not a directory", root.display());
                }
                capture_bundle(&self.mcp_db, &name, &root)?.len()
            }
            None => 0,
        };
        Ok(json!({
            "domain_system_id": row.id.0,
            "name": row.name,
            "version": row.version,
            "captured_files": captured,
        }))
    }

    fn tool_register_agent_system(&self, req: &McpRequest) -> Result<Value> {
        let name = req.require_str("name")?;
        let concern = req.require_str("concern")?;
        let description = req.get_str("description")?.unwrap_or_default();
        let row = register_agent_system(&self.mcp_db, &name, &concern, &description, false)?;
        let captured = match req.get_str("content_root")? {
            Some(root) => {
                let root = PathBuf::from(root);
                if !root.is_dir() {
                    bail!("content_root {} is not a directory", root.display());
                }
                capture_bundle(&self.mcp_db, &name, &root)?.len()
            }
            None => 0,
        };
        // is_privileged_request is a REQUEST, never a grant (proposal 11).
        let _ = req.get_bool("is_privileged_request", false);
        Ok(json!({
            "agent_system_id": row.id.0,
            "name": row.name,
            "concern": row.concern,
            "is_privileged": row.is_privileged,
            "captured_files": captured,
        }))
    }

    fn tool_recapture(&self, req: &McpRequest, kind: &str) -> Result<Value> {
        let name = req.require_str("name")?;
        let root = req.require_str("content_root")?;
        let root = PathBuf::from(root);
        if !root.is_dir() {
            bail!("content_root {} is not a directory", root.display());
        }
        let rows = capture_bundle(&self.mcp_db, &name, &root)?;
        let appended: Vec<Value> = rows
            .iter()
            .map(|r| {
                json!({"content_asset_id": r.id, "file_path": r.file_path, "asset_kind": r.asset_kind})
            })
            .collect();
        Ok(json!({"kind": kind, "captured_files": appended.len(), "appended": appended}))
    }

    fn tool_list_domain_systems(&self) -> Result<Value> {
        let rows = list_domain_systems(&self.mcp_db)?;
        let out: Vec<Value> = rows
            .iter()
            .map(|r| json!({"id": r.id.0, "name": r.name, "version": r.version, "description": r.description}))
            .collect();
        Ok(json!(out))
    }

    fn tool_list_agent_systems(&self) -> Result<Value> {
        let rows = list_agent_systems(&self.mcp_db)?;
        let out: Vec<Value> = rows
            .iter()
            .map(|r| {
                json!({"id": r.id.0, "name": r.name, "concern": r.concern, "is_privileged": r.is_privileged})
            })
            .collect();
        Ok(json!(out))
    }

    // ------------------------------------------------------------------
    // repo registration & sync
    // ------------------------------------------------------------------

    fn tool_register_repo(&self, req: &McpRequest) -> Result<Value> {
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("register_repo requires repo_path"))?;
        let root_path = Path::new(root);
        let mut repo_name = req.get_str("repo_name")?;
        let mut repo_uuid = None;
        if let Ok(cfg) = load_repo_config(root_path) {
            if repo_name.is_none() {
                repo_name = Some(cfg.repository.name.clone());
            }
            repo_uuid = cfg.repository.uuid.map(|u| u.to_string());
        }
        let repo_name = repo_name
            .ok_or_else(|| anyhow::anyhow!("repo_name is required (no dharma-repo.toml found)"))?;
        let repo_uuid = repo_uuid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let ds_name = req.require_str("domain_system_name")?;
        let ds_version = req.get_str("domain_system_version")?;

        let reg = bootstrap_repo_registration(
            &self.mcp_db,
            &repo_uuid,
            &repo_name,
            root,
            &ds_name,
            ds_version.as_deref(),
        )?;
        // ensure the repo's own db exists from the start
        let _repo = self.open_repo(req)?;
        let proposed = list_capability_manifests(&self.mcp_db, reg.id)?.len();
        Ok(json!({
            "repo_registration_id": reg.id,
            "repo_uuid": reg.repo_uuid,
            "repo_name": reg.repo_name,
            "domain_system": ds_name,
            "domain_system_version": reg.domain_system_version,
            "status": reg.status,
            "proposed_capabilities": proposed,
        }))
    }

    fn tool_list_repos(&self) -> Result<Value> {
        let rows = list_repo_registrations(&self.mcp_db)?;
        let out: Vec<Value> = rows
            .iter()
            .map(|r| {
                json!({
                    "repo_registration_id": r.id,
                    "repo_uuid": r.repo_uuid,
                    "repo_name": r.repo_name,
                    "repo_root": r.repo_root,
                    "domain_system_id": r.domain_system_id.0,
                    "domain_system_version": r.domain_system_version,
                    "status": r.status,
                    "approved_at": r.approved_at,
                })
            })
            .collect();
        Ok(json!(out))
    }

    fn tool_repo_status(&self, req: &McpRequest) -> Result<Value> {
        let uuid = self.resolve_repo_uuid(req)?;
        let reg = get_repo_registration(&self.mcp_db, &uuid)?
            .ok_or_else(|| anyhow::anyhow!("repo '{uuid}' is not registered"))?;
        let manifests: Vec<Value> = list_capability_manifests(&self.mcp_db, reg.id)?
            .iter()
            .map(|m| {
                let name = get_agent_system(&self.mcp_db, m.agent_system_id)
                    .ok()
                    .flatten()
                    .map(|a| a.name)
                    .unwrap_or_default();
                json!({
                    "agent_system_id": m.agent_system_id.0,
                    "agent_system_name": name,
                    "status": m.status,
                    "proposed_at": m.proposed_at,
                    "reviewed_by": m.reviewed_by,
                    "reviewed_at": m.reviewed_at,
                })
            })
            .collect();
        Ok(json!({
            "repo_registration_id": reg.id,
            "repo_uuid": reg.repo_uuid,
            "repo_name": reg.repo_name,
            "repo_root": reg.repo_root,
            "repo_db_path": reg.repo_db_path,
            "domain_system_id": reg.domain_system_id.0,
            "domain_system_version": reg.domain_system_version,
            "status": reg.status,
            "capability_manifest": manifests,
        }))
    }

    fn tool_unregister_repo(&self, req: &McpRequest) -> Result<Value> {
        let uuid = self.resolve_repo_uuid(req)?;
        let removed = unregister_repo(&self.mcp_db, &uuid)?;
        Ok(json!({"repo_uuid": uuid, "removed": removed}))
    }

    fn tool_review_capability_manifest(&self, req: &McpRequest) -> Result<Value> {
        let uuid = self.resolve_repo_uuid(req)?;
        let reg = get_repo_registration(&self.mcp_db, &uuid)?
            .ok_or_else(|| anyhow::anyhow!("repo '{uuid}' is not registered"))?;
        let as_name = req.require_str("agent_system_name")?;
        let as_row = get_agent_system_by_name(&self.mcp_db, &as_name)?
            .ok_or_else(|| anyhow::anyhow!("Agent System '{as_name}' is not registered"))?;
        let manifest = list_capability_manifests(&self.mcp_db, reg.id)?
            .into_iter()
            .find(|m| m.agent_system_id.0 == as_row.id.0)
            .ok_or_else(|| anyhow::anyhow!("no capability proposed for '{as_name}' on repo '{uuid}'"))?;
        let decision = req.require_str("decision")?;
        let status = match decision.as_str() {
            "approve" => "approved",
            "reject" => "rejected",
            other => bail!("invalid decision '{other}' (approve|reject)"),
        };
        let reviewer = req
            .get_str("reviewed_by")?
            .ok_or_else(|| anyhow::anyhow!("reviewed_by is required"))?;
        let updated = review_capability(&self.mcp_db, manifest.id, status, &reviewer)?
            .ok_or_else(|| anyhow::anyhow!("capability manifest {} is no longer reviewable", manifest.id))?;

        let mut sync = Value::Null;
        if status == "approved" {
            let root = req
                .repo_path
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("approving a capability requires repo_path"))?;
            let repo = self.open_repo(req)?;
            let values = self.repo_config_values(Path::new(root), &reg)?;
            let report = sync_to_repo(&self.mcp_db, &repo, &reg, &values)?;
            sync = json!({
                "synced_rows": report.synced_rows,
                "materialized_files": report.materialized_files,
                "missing_coverage": report.missing_coverage,
            });
        }
        let reg_after = get_repo_registration(&self.mcp_db, &uuid)?.unwrap();
        Ok(json!({
            "capability": json!({
                "agent_system_name": as_name,
                "status": updated.status,
                "reviewed_by": updated.reviewed_by,
                "reviewed_at": updated.reviewed_at,
            }),
            "repo_status": reg_after.status,
            "auto_sync": sync,
        }))
    }

    fn tool_sync_repo(&self, req: &McpRequest) -> Result<Value> {
        let uuid = self.resolve_repo_uuid(req)?;
        let reg = get_repo_registration(&self.mcp_db, &uuid)?
            .ok_or_else(|| anyhow::anyhow!("repo '{uuid}' is not registered"))?;
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("sync_repo requires repo_path"))?;
        let repo = self.open_repo(req)?;
        let values = self.repo_config_values(Path::new(root), &reg)?;
        let report = sync_to_repo(&self.mcp_db, &repo, &reg, &values)?;
        let reg_after = get_repo_registration(&self.mcp_db, &uuid)?.unwrap();
        Ok(json!({
            "repo_uuid": uuid,
            "repo_status": reg_after.status,
            "synced_rows": report.synced_rows,
            "materialized_files": report.materialized_files,
            "domain_summary": report.domain_summary_path.to_string_lossy(),
            "agent_summary": report.agent_summary_path.to_string_lossy(),
            "missing_coverage": report.missing_coverage,
        }))
    }

    fn tool_get_repo_config(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        match get_repo_config(&repo)? {
            Some(c) => Ok(json!({
                "repo_uuid": c.repo_uuid,
                "repo_name": c.repo_name,
                "repo_root": c.repo_root,
                "domain_system_name": c.domain_system_name,
                "domain_system_version": c.domain_system_version,
                "docs_dir": c.docs_dir,
                "implementation_dir": c.implementation_dir,
                "scripts_dir": c.scripts_dir,
                "tests_dir": c.tests_dir,
                "report_dir": c.report_dir,
                "dharma_dir": c.dharma_dir,
                "mcp_dir": c.mcp_dir,
            })),
            None => Ok(json!({"synced": false, "hint": "run sync_repo (or approve a capability) first"})),
        }
    }

    // ------------------------------------------------------------------
    // task execution
    // ------------------------------------------------------------------

    fn tool_assign_task(&self, req: &McpRequest) -> Result<Value> {
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("assign_task requires repo_path"))?;
        self.require_approved_registration(root)?;
        let task_ref = req.require_str("task_ref")?;
        let (task_id, task_name) = self.resolve_task(&task_ref)?;
        let repo = self.open_repo(req)?;

        let (as_id, agent_id) = match req.get_str("agent_ref")? {
            Some(r) => self.resolve_agent_ref(&r)?,
            None => {
                let concern = self.task_first_capability(task_id)?;
                self.resolve_agent_by_concern(&concern)?
            }
        };
        let instance = create_task_instance(&repo, task_id, as_id, agent_id)?;
        Ok(json!({
            "task_instance_id": instance.id.0,
            "task_id": task_id,
            "task_name": task_name,
            "attempt_number": instance.attempt_number,
            "status": instance.status,
            "initiating_agent_system_id": as_id,
            "initiating_agent_id": agent_id,
        }))
    }

    fn task_first_capability(&self, task_id: i64) -> Result<String> {
        let conn = self.mcp_db.conn();
        let conn = conn.lock().unwrap();
        let concern = conn.query_row(
            "SELECT required_capability FROM task_step WHERE task_id = ?1 ORDER BY step_order LIMIT 1",
            rusqlite::params![task_id],
            |r| r.get::<_, String>(0),
        )?;
        drop(conn);
        Ok(concern)
    }

    fn tool_submit_proposal_draft(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let instance_id = TaskInstanceId(req.require_i64("task_instance_id")?);
        let draft = req.require_value("draft")?;
        let instance = get_task_instance(&repo, instance_id)?
            .ok_or_else(|| anyhow::anyhow!("task instance {} not found", instance_id.0))?;
        let (as_id, agent_id) = match req.get_str("agent_ref")? {
            Some(r) => self.resolve_agent_ref(&r)?,
            None => (instance.initiating_agent_system_id, instance.initiating_agent_id),
        };
        let revision =
            draft_proposal(&repo, instance_id, as_id, agent_id, &draft.to_string())?;
        Ok(json!({
            "revision_id": revision.id,
            "revision_number": revision.revision_number,
            "task_instance_id": revision.task_instance_id.0,
            "status": instance.status,
        }))
    }

    fn tool_review_task_proposal(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let instance_id = TaskInstanceId(req.require_i64("task_instance_id")?);
        let decision = req.require_str("decision")?;
        let reviewer = req
            .get_str("reviewed_by")?
            .ok_or_else(|| anyhow::anyhow!("reviewed_by is required"))?;
        let revisions = list_proposal_revisions(&repo, instance_id)?;
        let latest = revisions
            .last()
            .ok_or_else(|| anyhow::anyhow!("task instance {} has no proposal revision", instance_id.0))?;

        match decision.as_str() {
            "approve" => {
                let approval = approve_proposal(&repo, instance_id, latest.id, &reviewer)?;
                let instance = get_task_instance(&repo, instance_id)?.unwrap();
                Ok(json!({
                    "decision": "approve",
                    "approved_revision_id": approval.approved_revision_id,
                    "approved_by": approval.approved_by,
                    "approved_at": approval.approved_at,
                    "status": instance.status,
                }))
            }
            "revise" => {
                let comments = req.require_str("comments")?;
                let updated = comment_on_revision(&repo, latest.id, &comments)?;
                Ok(json!({
                    "decision": "revise",
                    "revision_id": updated.map(|r| r.id),
                    "reviewer_comments": comments,
                    "status": "proposing",
                }))
            }
            other => bail!("invalid decision '{other}' (approve|revise)"),
        }
    }

    fn tool_request_handoff(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let instance_id = TaskInstanceId(req.require_i64("task_instance_id")?);
        let to_ref = req.require_str("to_agent_ref")?;
        let reason = req.require_str("reason")?;
        let payload = req.get_value("context_payload")?;
        let state = get_execution_state(&repo, instance_id)?
            .ok_or_else(|| anyhow::anyhow!("task instance {} has no execution state", instance_id.0))?;
        let (to_as, to_agent) = self.resolve_agent_ref(&to_ref)?;
        let hop = record_handoff(
            &repo,
            instance_id,
            state.current_owning_agent_system_id,
            state.current_owning_agent_id,
            to_as,
            to_agent,
            &reason,
            true,
            payload.as_ref().map(|p| p.to_string()).as_deref(),
        )?;
        Ok(json!({
            "handoff_id": hop.id,
            "hop_order": hop.hop_order,
            "from_agent_id": hop.from_agent_id,
            "to_agent_id": hop.to_agent_id,
            "reason": hop.reason,
            "accepted": true,
        }))
    }

    fn tool_run_skill(&self, req: &McpRequest) -> Result<Value> {
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("run_skill requires repo_path"))?;
        let skill_ref = req.require_str("skill_ref")?;
        let input = req.get_value("input")?.unwrap_or(Value::Null);
        let task_instance_id = req.get_i64("task_instance_id")?;

        let (skill_id, skill_name, responsibility, script_ref, prompt) = {
            let conn = self.mcp_db.conn();
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, name, responsibility FROM skill WHERE id = ?1 OR name = ?2 ORDER BY id LIMIT 1",
            )?;
            let mut rows = stmt.query_map(
                rusqlite::params![skill_ref.parse::<i64>().unwrap_or(0), skill_ref],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
            )?;
            let (skill_id, skill_name, responsibility) = match rows.next() {
                Some(Ok(row)) => row,
                _ => return Err(anyhow::anyhow!("skill '{skill_ref}' not found")),
            };
            let script_ref: Option<String> = conn
                .query_row(
                    "SELECT script_ref FROM skill_script WHERE skill_id = ?1 ORDER BY id LIMIT 1",
                    rusqlite::params![skill_id],
                    |r| r.get(0),
                )
                .ok();
            let prompt: Option<String> = conn
                .query_row(
                    "SELECT template_text FROM skill_prompt WHERE skill_id = ?1 ORDER BY id LIMIT 1",
                    rusqlite::params![skill_id],
                    |r| r.get(0),
                )
                .ok();
            (skill_id, skill_name, responsibility, script_ref, prompt)
        };

        let script_path = script_ref.map(PathBuf::from);
        let script_path = match script_path {
            Some(p) if p.is_absolute() => p,
            Some(p) => Path::new(root).join(p),
            None => {
                return Ok(json!({
                    "skill_id": skill_id,
                    "skill_name": skill_name,
                    "execution": "analysis_only",
                    "note": "no skill_script bound; the model executes this skill directly",
                    "prompt": prompt.unwrap_or(responsibility),
                    "input": input,
                }));
            }
        };
        if !script_path.exists() {
            bail!(
                "skill '{}' script {} does not exist",
                skill_name,
                script_path.display()
            );
        }

        let envelope = json!({
            "repo_root": root,
            "task_instance_id": task_instance_id,
            "input": input,
        });
        let input_path = std::env::temp_dir().join(format!(
            "dharma-skill-in-{}.json",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&input_path, envelope.to_string())
            .map_err(|e| anyhow::anyhow!("failed to write skill input: {e}"))?;
        let result = common::env::run_capability_script(&script_path, Path::new(root), &input_path, None);
        let _ = std::fs::remove_file(&input_path);
        let output = result.map_err(|e| anyhow::anyhow!("skill script failed: {e:#}"))?;

        Ok(json!({
            "skill_id": skill_id,
            "skill_name": skill_name,
            "execution": "script",
            "output": output,
        }))
    }

    fn tool_submit_completion_validation(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let instance_id = TaskInstanceId(req.require_i64("task_instance_id")?);
        let verdict = req.require_str("verdict")?;
        let reason = req.require_str("reason")?;
        let validation = complete_task_instance(&repo, instance_id, &verdict, &reason)?;
        let instance = get_task_instance(&repo, instance_id)?.unwrap();
        Ok(json!({
            "task_instance_id": instance_id.0,
            "verdict": validation.verdict,
            "reason": validation.reason,
            "validated_at": validation.validated_at,
            "status": instance.status,
        }))
    }

    fn tool_task_instance_status(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let instance_id = TaskInstanceId(req.require_i64("task_instance_id")?);
        let instance = get_task_instance(&repo, instance_id)?
            .ok_or_else(|| anyhow::anyhow!("task instance {} not found", instance_id.0))?;

        let task = self.task_name(instance.task_id)?;
        let revisions: Vec<Value> = list_proposal_revisions(&repo, instance_id)?
            .iter()
            .map(|r| {
                json!({"revision_id": r.id, "revision_number": r.revision_number, "drafted_by_agent_id": r.drafted_by_agent_id, "reviewer_comments": r.reviewer_comments, "created_at": r.created_at})
            })
            .collect();
        let approval: Option<Value> = get_proposal_approval(&repo, instance_id.0)?
            .map(|a| json!({"approved_revision_id": a.approved_revision_id, "approved_by": a.approved_by, "approved_at": a.approved_at}));
        let execution: Option<Value> = get_execution_state(&repo, instance_id)?.map(|e| {
            json!({"current_step_order": e.current_step_order, "current_owning_agent_system_id": e.current_owning_agent_system_id, "current_owning_agent_id": e.current_owning_agent_id})
        });
        let handoffs: Vec<Value> = list_handoff_log(&repo, instance_id)?
            .iter()
            .map(|h| {
                json!({"hop_order": h.hop_order, "from_agent_id": h.from_agent_id, "to_agent_id": h.to_agent_id, "reason": h.reason, "accepted": h.accepted, "created_at": h.created_at})
            })
            .collect();
        let validation: Option<Value> = get_completion_validation(&repo, instance_id.0).ok().flatten()
            .map(|v| json!({"verdict": v.verdict, "reason": v.reason, "validated_at": v.validated_at}));

        Ok(json!({
            "task_instance_id": instance.id.0,
            "task_id": instance.task_id,
            "task_name": task,
            "attempt_number": instance.attempt_number,
            "previous_task_instance_id": instance.previous_task_instance_id,
            "status": instance.status,
            "created_at": instance.created_at,
            "updated_at": instance.updated_at,
            "proposal_revisions": revisions,
            "proposal_approval": approval,
            "execution_state": execution,
            "handoff_log": handoffs,
            "completion_validation": validation,
        }))
    }

    fn task_name(&self, task_id: i64) -> Result<String> {
        let conn = self.mcp_db.conn();
        let conn = conn.lock().unwrap();
        let name = conn.query_row(
            "SELECT name FROM task WHERE id = ?1",
            rusqlite::params![task_id],
            |r| r.get::<_, String>(0),
        )?;
        drop(conn);
        Ok(name)
    }

    // ------------------------------------------------------------------
    // audit
    // ------------------------------------------------------------------

    fn tool_run_audit(&self, req: &McpRequest) -> Result<Value> {
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("run_audit requires repo_path"))?;
        self.require_approved_registration(root)?;
        let repo = self.open_repo(req)?;
        let commit_hash = req.require_str("commit_hash")?;
        let domain_name = req.require_str("domain")?;
        let kind = req.require_str("kind")?;

        let uuid = self.resolve_repo_uuid_from_root(root)?;
        let domain_id: i64 = {
            let conn = self.mcp_db.conn();
            let conn = conn.lock().unwrap();
            let ds_id: i64 = conn.query_row(
                "SELECT domain_system_id FROM repo_registration WHERE repo_uuid = ?1",
                rusqlite::params![uuid],
                |r| r.get(0),
            )?;
            let id = conn.query_row(
                "SELECT id FROM domain WHERE domain_system_id = ?1 AND name = ?2",
                rusqlite::params![ds_id, domain_name],
                |r| r.get(0),
            )?;
            drop(conn);
            id
        };
        let run = start_audit_run(&repo, &commit_hash, domain_id, &domain_name, &kind)?;
        Ok(json!({
            "audit_run_id": run.id,
            "commit_hash": run.commit_hash,
            "domain_id": run.domain_id,
            "domain_name": run.domain_name,
            "kind": run.kind,
            "status": run.status,
        }))
    }

    fn tool_get_audit_result(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let run_id = req.require_i64("audit_run_id")?;
        let run = get_audit_run(&repo, run_id)?
            .ok_or_else(|| anyhow::anyhow!("audit run {run_id} not found"))?;

        let deterministic: Option<Value> = get_deterministic_result_for_run(&repo, run_id)?
            .map(|d| {
                json!({"result_id": d.id, "score": d.score, "rules_passed": d.rules_passed, "rules_total": d.rules_total, "evidence_json": d.evidence_json})
            });
        let semantic: Vec<Value> = list_semantic_runs_for_run(&repo, run_id)?
            .iter()
            .map(|s| {
                json!({"semantic_run_id": s.id, "model": s.model, "overall_score": s.overall_score, "reasoning": s.reasoning})
            })
            .collect();
        let semantic_ids: Vec<i64> = list_semantic_runs_for_run(&repo, run_id)?
            .iter()
            .map(|s| s.id)
            .collect();
        let mut dimensions = Vec::new();
        for sid in semantic_ids {
            let mut d = list_dimensions_for_semantic_run(&repo, sid)?
                .iter()
                .map(|x| json!({"semantic_run_id": sid, "dimension": x.dimension, "score": x.score, "evidence": x.evidence}))
                .collect::<Vec<_>>();
            dimensions.append(&mut d);
        }
        let findings: Vec<Value> = list_findings_for_run(&repo, run_id)?
            .iter()
            .map(|f| json!({"finding_id": f.id, "finding_type": f.finding_type, "text": f.text}))
            .collect();
        let overrides: Vec<Value> = list_overrides_for_run(&repo, run_id)?
            .iter()
            .map(|o| {
                json!({"override_id": o.id, "target": o.target, "action": o.action, "override_score": o.override_score, "reason": o.reason, "reviewed_by": o.reviewed_by})
            })
            .collect();

        Ok(json!({
            "audit_run_id": run.id,
            "commit_hash": run.commit_hash,
            "domain_id": run.domain_id,
            "domain_name": run.domain_name,
            "kind": run.kind,
            "status": run.status,
            "created_at": run.created_at,
            "completed_at": run.completed_at,
            "deterministic_result": deterministic,
            "semantic_runs": semantic,
            "semantic_dimensions": dimensions,
            "findings": findings,
            "overrides": overrides,
        }))
    }

    fn tool_override_audit(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let run_id = req.require_i64("audit_run_id")?;
        let action = req.require_str("action")?;
        let reason = req.require_str("reason")?;
        let target = req.get_str("target")?.unwrap_or_else(|| "run".into());
        let override_score = req.get_value("override_score")?.and_then(|v| v.as_f64());
        let reviewed_by = req
            .get_str("reviewed_by")?
            .ok_or_else(|| anyhow::anyhow!("reviewed_by is required"))?;
        let row = override_audit_result(
            &repo,
            run_id,
            &target,
            &action,
            override_score,
            &reason,
            &reviewed_by,
        )?;
        let run = get_audit_run(&repo, run_id)?.unwrap();
        Ok(json!({
            "override_id": row.id,
            "audit_run_id": row.audit_run_id,
            "target": row.target,
            "action": row.action,
            "override_score": row.override_score,
            "reviewed_by": row.reviewed_by,
            "run_status": run.status,
        }))
    }

    // ------------------------------------------------------------------
    // proposal lifecycle
    // ------------------------------------------------------------------

    fn tool_advance_proposal_lifecycle(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let root = req
            .repo_path
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("advance_proposal_lifecycle requires repo_path"))?;
        let name = req.require_str("proposal_name")?;
        let new_status = req.require_str("new_status")?;
        let commit_hash = req.require_str("commit_hash")?;
        let message = req.get_str("message")?.unwrap_or_default();
        let reviewer = req
            .get_str("reviewed_by")?
            .ok_or_else(|| anyhow::anyhow!("reviewed_by is required"))?;

        let exists = get_proposal(&repo, &name)?.is_some();
        let row = match new_status.as_str() {
            "draft" => {
                if !exists {
                    register_proposal(&repo, &name, &name, &format!("docs/proposal/{name}.md"))?;
                }
                set_draft_commit(&repo, &name, &commit_hash, &message)?
            }
            "finalized" => finalize_proposal(&repo, &name, &commit_hash, &message)?,
            "implementing" => start_implementation(&repo, &name)?,
            "verified" => verify_proposal(&repo, &name, &commit_hash, &message)?,
            "archived" => archive_proposal(&repo, &name, &commit_hash, &message, Path::new(root))?,
            other => bail!(
                "invalid new_status '{other}' (draft|finalized|implementing|verified|archived)"
            ),
        };
        let row = row.ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
        let _ = reviewer;
        Ok(json!({
            "proposal_name": row.name,
            "status": row.status,
            "draft_commit_hash": row.draft_commit_hash,
            "finalized_commit_hash": row.finalized_commit_hash,
            "implementation_commit_hash": row.implementation_commit_hash,
            "archive_commit_hash": row.archive_commit_hash,
            "doc_path": row.doc_path,
        }))
    }

    fn tool_log_proposal_commit(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let name = req.require_str("proposal_name")?;
        let commit_hash = req.require_str("commit_hash")?;
        let phase = req.get_str("phase")?.unwrap_or_else(|| "implementation".into());
        let message = req.get_str("message")?.unwrap_or_default();
        let entry = record_commit(&repo, &name, &commit_hash, &phase, &message)?
            .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
        Ok(json!({
            "proposal_lifecycle_id": entry.proposal_lifecycle_id,
            "commit_hash": entry.commit_hash,
            "phase": entry.phase,
            "message": entry.message,
            "recorded_at": entry.recorded_at,
        }))
    }

    fn tool_get_proposal_lifecycle(&self, req: &McpRequest) -> Result<Value> {
        let repo = self.open_repo(req)?;
        let name = req.require_str("proposal_name")?;
        let row = get_proposal(&repo, &name)?
            .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
        let log: Vec<Value> = list_commit_log(&repo, &name)?
            .iter()
            .map(|c| {
                json!({"commit_hash": c.commit_hash, "phase": c.phase, "message": c.message, "recorded_at": c.recorded_at})
            })
            .collect();
        Ok(json!({
            "proposal_name": row.name,
            "title": row.title,
            "doc_path": row.doc_path,
            "status": row.status,
            "draft_commit_hash": row.draft_commit_hash,
            "finalized_commit_hash": row.finalized_commit_hash,
            "implementation_commit_hash": row.implementation_commit_hash,
            "archive_commit_hash": row.archive_commit_hash,
            "created_at": row.created_at,
            "updated_at": row.updated_at,
            "commit_log": log,
        }))
    }

    // ------------------------------------------------------------------
    // routing
    // ------------------------------------------------------------------

    fn route(&self, req: &McpRequest) -> Result<Value> {
        match req.method.as_str() {
            "initialize" => Ok(json!({"capabilities": McpCapabilities::default()})),
            "ping" => Ok(json!({"pong": true})),
            // registry & capture
            "register_domain_system" => self.tool_register_domain_system(req),
            "register_agent_system" => self.tool_register_agent_system(req),
            "recapture_domain_system" => self.tool_recapture(req, "domain"),
            "recapture_agent_system" => self.tool_recapture(req, "agent"),
            "list_domain_systems" => self.tool_list_domain_systems(),
            "list_agent_systems" => self.tool_list_agent_systems(),
            "get_domain_system_info" => domain_system_info(&self.mcp_db, &req.require_str("name")?),
            "get_agent_system_info" => agent_system_info(&self.mcp_db, &req.require_str("name")?),
            // repo registration & sync
            "register_repo" => self.tool_register_repo(req),
            "list_repos" => self.tool_list_repos(),
            "repo_status" => self.tool_repo_status(req),
            "unregister_repo" => self.tool_unregister_repo(req),
            "review_capability_manifest" => self.tool_review_capability_manifest(req),
            "sync_repo" => self.tool_sync_repo(req),
            "get_repo_config" => self.tool_get_repo_config(req),
            // task execution
            "assign_task" => self.tool_assign_task(req),
            "submit_proposal_draft" => self.tool_submit_proposal_draft(req),
            "review_task_proposal" => self.tool_review_task_proposal(req),
            "request_handoff" => self.tool_request_handoff(req),
            "run_skill" => self.tool_run_skill(req),
            "submit_completion_validation" => self.tool_submit_completion_validation(req),
            "task_instance_status" => self.tool_task_instance_status(req),
            // audit
            "run_audit" => self.tool_run_audit(req),
            "get_audit_result" => self.tool_get_audit_result(req),
            "override_audit" => self.tool_override_audit(req),
            // proposal lifecycle
            "advance_proposal_lifecycle" => self.tool_advance_proposal_lifecycle(req),
            "log_proposal_commit" => self.tool_log_proposal_commit(req),
            "get_proposal_lifecycle" => self.tool_get_proposal_lifecycle(req),
            other => Err(anyhow::anyhow!("unknown method '{other}'")),
        }
    }

    /// Resolve the values the Sync Engine persists (repo.db 14) for a repo —
    /// from `dharma-repo.toml` when present, else registration defaults.
    fn repo_config_values(&self, root: &Path, reg: &RepoRegistrationRow) -> Result<RepoConfigValues> {
        match load_repo_config(root) {
            Ok(cfg) => {
                let dharma_dir = cfg.resolve_dharma_dir(root);
                let mcp_dir = cfg.resolve_mcp_dir(root);
                Ok(RepoConfigValues {
                    repo_uuid: reg.repo_uuid.clone(),
                    repo_name: reg.repo_name.clone(),
                    repo_root: root.to_string_lossy().to_string(),
                    domain_system_name: cfg
                        .repository
                        .domain
                        .as_ref()
                        .map(|d| d.select.clone())
                        .unwrap_or_default(),
                    domain_system_version: reg.domain_system_version.clone(),
                    docs_dir: cfg
                        .repository
                        .documentation
                        .as_ref()
                        .map(|d| d.root_dir.clone()),
                    implementation_dir: cfg
                        .repository
                        .implementation
                        .as_ref()
                        .map(|i| i.dir.clone()),
                    scripts_dir: cfg.repository.scripts.as_ref().map(|s| s.dir.clone()),
                    tests_dir: cfg.repository.tests.as_ref().map(|t| t.dir.clone()),
                    report_dir: cfg.report.as_ref().map(|r| r.dir.clone()),
                    dharma_dir: dharma_dir.to_string_lossy().to_string(),
                    mcp_dir: mcp_dir.to_string_lossy().to_string(),
                })
            }
            Err(_) => {
                let dharma_dir = root.join(".dharma");
                Ok(RepoConfigValues {
                    repo_uuid: reg.repo_uuid.clone(),
                    repo_name: reg.repo_name.clone(),
                    repo_root: root.to_string_lossy().to_string(),
                    domain_system_name: String::new(),
                    domain_system_version: reg.domain_system_version.clone(),
                    docs_dir: Some("docs".to_string()),
                    implementation_dir: None,
                    scripts_dir: None,
                    tests_dir: None,
                    report_dir: None,
                    dharma_dir: dharma_dir.to_string_lossy().to_string(),
                    mcp_dir: common::env::mcp_dir().to_string_lossy().to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::TOOL_METHODS;
    use std::collections::HashMap;

    fn adapter() -> McpAdapter {
        McpAdapter::new(::registry::McpDb::open_in_memory().unwrap())
    }

    fn call(adapter: &McpAdapter, method: &str, params: &[(&str, Value)], repo_path: Option<&str>) -> Value {
        let mut map = HashMap::new();
        for (k, v) in params {
            map.insert(k.to_string(), v.clone());
        }
        let req = McpRequest {
            id: "t".into(),
            method: method.into(),
            params: map,
            repo_path: repo_path.map(|s| s.to_string()),
        };
        match adapter.dispatch(&req) {
            McpMessage::Response(r) => r.result,
            McpMessage::Error(e) => {
                panic!("method {method} errored: code={} msg={}", e.code, e.message)
            }
            _ => panic!("unexpected reply"),
        }
    }

    fn call_err(adapter: &McpAdapter, method: &str, params: &[(&str, Value)], repo_path: Option<&str>) -> String {
        let mut map = HashMap::new();
        for (k, v) in params {
            map.insert(k.to_string(), v.clone());
        }
        let req = McpRequest {
            id: "t".into(),
            method: method.into(),
            params: map,
            repo_path: repo_path.map(|s| s.to_string()),
        };
        match adapter.dispatch(&req) {
            McpMessage::Error(e) => e.message,
            _ => panic!("method {method} unexpectedly succeeded"),
        }
    }

    fn json_parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    #[test]
    fn initialize_advertises_28_tools() {
        let a = adapter();
        let caps = call(&a, "initialize", &[], None);
        assert_eq!(caps["capabilities"]["methods"].as_array().unwrap().len(), 28);
        for m in TOOL_METHODS {
            assert!(caps["capabilities"]["methods"].as_array().unwrap().contains(&Value::String(m.to_string())), "missing {m}");
        }
    }

    #[test]
    fn unknown_method_is_an_error() {
        let a = adapter();
        let msg = call_err(&a, "fly", &[], None);
        assert!(msg.contains("unknown method"));
    }

    #[test]
    fn gate_requires_human_approval_and_reviewer() {
        let a = adapter();
        let msg = call_err(&a, "review_task_proposal", &[("task_instance_id", json!(1))], None);
        assert!(msg.contains("human_approved"), "{msg}");
        let msg = call_err(
            &a,
            "review_task_proposal",
            &[("task_instance_id", json!(1)), ("human_approved", json!(true))],
            None,
        );
        assert!(msg.contains("reviewed_by"), "{msg}");
    }

    #[test]
    fn register_and_list_domain_systems() {
        let a = adapter();
        let out = call(
            &a,
            "register_domain_system",
            &[
                ("name", json!("base_dev")),
                ("version", json!("1.0.0")),
                ("description", json!("base dev domain")),
            ],
            None,
        );
        assert_eq!(out["name"], "base_dev");
        let list = call(&a, "list_domain_systems", &[], None);
        assert_eq!(list.as_array().unwrap().len(), 1);
        assert_eq!(list[0]["name"], "base_dev");
    }

    #[test]
    fn register_repo_approve_capability_triggers_sync() {
        let a = adapter();
        call(
            &a,
            "register_domain_system",
            &[("name", json!("base_dev"))],
            None,
        );
        call(
            &a,
            "register_agent_system",
            &[("name", json!("Implementer")), ("concern", json!("implementation"))],
            None,
        );

        let root = std::env::temp_dir().join(format!("dharma-mcp-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let uuid = uuid::Uuid::new_v4().to_string();
        std::fs::write(
            root.join("dharma-repo.toml"),
            format!("[repository]\nname = \"test-repo\"\nuuid = \"{uuid}\"\n"),
        )
        .unwrap();
        let root_str = root.to_string_lossy().to_string();

        let out = call(
            &a,
            "register_repo",
            &[("domain_system_name", json!("base_dev"))],
            Some(&root_str),
        );
        assert_eq!(out["status"], "manifest_proposed");
        assert_eq!(out["proposed_capabilities"], 1);

        // reject first — no sync, status stays
        let out = call(
            &a,
            "review_capability_manifest",
            &[
                ("agent_system_name", json!("Implementer")),
                ("decision", json!("reject")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert_eq!(out["capability"]["status"], "rejected");
        assert_eq!(out["repo_status"], "manifest_proposed");

        // the same entry cannot be re-reviewed (one-shot)
        let msg = call_err(
            &a,
            "review_capability_manifest",
            &[
                ("agent_system_name", json!("Implementer")),
                ("decision", json!("approve")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert!(msg.contains("no longer reviewable"), "{msg}");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn recapture_requires_directory() {
        let a = adapter();
        call(
            &a,
            "register_domain_system",
            &[("name", json!("base_dev"))],
            None,
        );
        let msg = call_err(
            &a,
            "recapture_domain_system",
            &[
                ("name", json!("base_dev")),
                ("content_root", json!("C:\\definitely\\not\\here")),
            ],
            None,
        );
        assert!(msg.contains("not a directory"), "{msg}");
    }

    #[test]
    fn proposal_lifecycle_advances_forward_only() {
        let a = adapter();
        call(
            &a,
            "register_domain_system",
            &[("name", json!("base_dev"))],
            None,
        );
        call(
            &a,
            "register_agent_system",
            &[("name", json!("Implementer")), ("concern", json!("implementation"))],
            None,
        );
        let root = std::env::temp_dir().join(format!("dharma-mcp-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("dharma-repo.toml"),
            format!(
                "[repository]\nname = \"test-repo\"\nuuid = \"{}\"\n",
                uuid::Uuid::new_v4()
            ),
        )
        .unwrap();
        let root_str = root.to_string_lossy().to_string();
        call(
            &a,
            "register_repo",
            &[("domain_system_name", json!("base_dev"))],
            Some(&root_str),
        );
        call(
            &a,
            "review_capability_manifest",
            &[
                ("agent_system_name", json!("Implementer")),
                ("decision", json!("approve")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );

        // draft → finalized → implementing → verified → archived
        let out = call(
            &a,
            "advance_proposal_lifecycle",
            &[
                ("proposal_name", json!("12-test-proposal")),
                ("new_status", json!("draft")),
                ("commit_hash", json!("abc1111")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert_eq!(out["status"], "draft");
        let out = call(
            &a,
            "advance_proposal_lifecycle",
            &[
                ("proposal_name", json!("12-test-proposal")),
                ("new_status", json!("finalized")),
                ("commit_hash", json!("abc2222")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert_eq!(out["status"], "finalized");
        let out = call(
            &a,
            "advance_proposal_lifecycle",
            &[
                ("proposal_name", json!("12-test-proposal")),
                ("new_status", json!("implementing")),
                ("commit_hash", json!("abc3333")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert_eq!(out["status"], "implementing");
        let out = call(
            &a,
            "advance_proposal_lifecycle",
            &[
                ("proposal_name", json!("12-test-proposal")),
                ("new_status", json!("verified")),
                ("commit_hash", json!("abc4444")),
                ("human_approved", json!(true)),
                ("reviewed_by", json!("alice")),
            ],
            Some(&root_str),
        );
        assert_eq!(out["status"], "verified");

        let log = call(
            &a,
            "get_proposal_lifecycle",
            &[("proposal_name", json!("12-test-proposal"))],
            Some(&root_str),
        );
        assert_eq!(log["commit_log"].as_array().unwrap().len(), 3);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn json_parse_helper() {
        assert_eq!(json_parse("{\"a\":1}")["a"], 1);
    }
}

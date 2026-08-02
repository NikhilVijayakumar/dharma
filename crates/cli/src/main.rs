//! dharma CLI — a human/admin front-end over the services layer.
//!
//! Mirrors the MCP tool surface (docs/proposal/14) as subcommands that talk
//! to mcp.db / repo.db directly. Approval-like mutations require `--yes`
//! plus `--reviewed-by <name>` (the CLI's equivalent of the MCP
//! `human_approved` + `reviewed_by` gate).

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use common::config::load_repo_config;
use common::id::TaskInstanceId;
use services::registration::{RepoConfigValues, RepoRegistrationRow};
use services::*;
use std::path::{Path, PathBuf};

// `::registry` = the extern crate (leading `::`); the glob above also brings
// in the `services::registry` module name.
use ::registry::{McpDb, RepoDb};

#[derive(Parser)]
#[command(name = "dharma", version, about = "Dharma: infrastructure for AI-agented documentation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Register a Domain System (mcp.db 00).
    RegisterDomainSystem {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "0.0.0")]
        version: String,
        #[arg(long, default_value = "")]
        description: String,
        /// Optional provider content root to capture into the ledger.
        #[arg(long)]
        content_root: Option<String>,
    },
    /// Register an Agent System (mcp.db 01).
    RegisterAgentSystem {
        #[arg(long)]
        name: String,
        #[arg(long)]
        concern: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long)]
        content_root: Option<String>,
    },
    /// List registered Domain Systems.
    ListDomainSystems,
    /// List registered Agent Systems.
    ListAgentSystems,
    /// Full info tree for one Domain System.
    DomainSystemInfo {
        #[arg(long)]
        name: String,
    },
    /// Full info tree for one Agent System.
    AgentSystemInfo {
        #[arg(long)]
        name: String,
    },
    /// Register a consuming repo against a Domain System (mcp.db 27-28).
    RepoRegister {
        #[arg(long)]
        root: String,
        #[arg(long)]
        domain_system: String,
        #[arg(long)]
        repo_name: Option<String>,
        #[arg(long)]
        repo_uuid: Option<String>,
        #[arg(long)]
        domain_system_version: Option<String>,
    },
    /// List repo registrations.
    RepoList,
    /// Show one repo's registration + capability manifest.
    RepoStatus {
        #[arg(long)]
        root: Option<String>,
        #[arg(long)]
        uuid: Option<String>,
    },
    /// Approve a proposed capability (gate: --yes + --reviewed-by).
    RepoApprove {
        #[arg(long)]
        root: String,
        #[arg(long)]
        agent_system: String,
        #[arg(long)]
        reviewed_by: String,
        #[arg(long)]
        yes: bool,
    },
    /// Reject a proposed capability (gate: --yes + --reviewed-by).
    RepoReject {
        #[arg(long)]
        root: String,
        #[arg(long)]
        agent_system: String,
        #[arg(long)]
        reviewed_by: String,
        #[arg(long)]
        yes: bool,
    },
    /// Re-sync an approved repo (proposal 11).
    RepoSync {
        #[arg(long)]
        root: String,
    },
    /// Show a repo's synced config (repo.db 14).
    RepoConfig {
        #[arg(long)]
        root: String,
    },
    /// Assign a Task (create a Task Instance, repo.db 00).
    TaskAssign {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_ref: String,
        #[arg(long)]
        agent_ref: Option<String>,
    },
    /// Full status of a Task Instance.
    TaskStatus {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_instance: i64,
    },
    /// Draft a proposal revision for a Task Instance.
    ProposalDraft {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_instance: i64,
        #[arg(long)]
        agent_ref: Option<String>,
        #[arg(long)]
        draft: String,
    },
    /// Review a proposal revision (gate: --yes + --reviewed-by).
    ProposalReview {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_instance: i64,
        #[arg(long)]
        decision: String,
        #[arg(long)]
        comments: Option<String>,
        #[arg(long)]
        reviewed_by: String,
        #[arg(long)]
        yes: bool,
    },
    /// Record a handoff hop.
    Handoff {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_instance: i64,
        #[arg(long)]
        to_agent_ref: String,
        #[arg(long)]
        reason: String,
    },
    /// Run a skill (scripted or analysis-only). An effect-capable skill
    /// (bound script/template) is refused unless the Task Instance is
    /// 'executing' (proposal 07).
    RunSkill {
        #[arg(long)]
        root: String,
        #[arg(long)]
        skill_ref: String,
        #[arg(long)]
        task_instance: i64,
        #[arg(long)]
        input: Option<String>,
    },
    /// Record the Completion Validator's verdict (repo.db 06).
    CompleteTask {
        #[arg(long)]
        root: String,
        #[arg(long)]
        task_instance: i64,
        #[arg(long)]
        verdict: String,
        #[arg(long)]
        reason: String,
    },
    /// Start an audit run (repo.db 08).
    AuditRun {
        #[arg(long)]
        root: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        domain: String,
        #[arg(long)]
        kind: String,
    },
    /// Full audit result for a run.
    AuditShow {
        #[arg(long)]
        root: String,
        #[arg(long)]
        run: i64,
    },
    /// Override or cancel an audit result (gate: --yes + --reviewed-by).
    AuditOverride {
        #[arg(long)]
        root: String,
        #[arg(long)]
        run: i64,
        #[arg(long)]
        action: String,
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        score: Option<f64>,
        #[arg(long)]
        reason: String,
        #[arg(long)]
        reviewed_by: String,
        #[arg(long)]
        yes: bool,
    },
    /// Advance a proposal's lifecycle (gate: --yes + --reviewed-by).
    ProposalAdvance {
        #[arg(long)]
        root: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        status: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        reviewed_by: String,
        #[arg(long)]
        yes: bool,
    },
    /// Log a commit against a proposal's history.
    ProposalLog {
        #[arg(long)]
        root: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        phase: Option<String>,
        #[arg(long)]
        message: Option<String>,
    },
    /// Show a proposal's lifecycle + commit log.
    ProposalShow {
        #[arg(long)]
        root: String,
        #[arg(long)]
        name: String,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::WARN)
        .init();
    let cli = Cli::parse();
    let mcp = McpDb::open().context("Failed to open mcp.db")?;
    run(cli, &mcp)
}

fn run(cli: Cli, mcp: &McpDb) -> Result<()> {
    match cli.command {
        Command::RegisterDomainSystem { name, version, description, content_root } => {
            let row = register_domain_system(mcp, &name, &version, &description)?;
            let mut captured = 0usize;
            if let Some(root) = content_root {
                let root = Path::new(&root);
                if !root.is_dir() {
                    bail!("content_root {} is not a directory", root.display());
                }
                captured = capture_bundle(mcp, &name, root)?.len();
            }
            println!(
                "domain_system_id={} name={} version={} captured_files={}",
                row.id.0, row.name, row.version, captured
            );
        }
        Command::RegisterAgentSystem { name, concern, description, content_root } => {
            let row = register_agent_system(mcp, &name, &concern, &description, false)?;
            let mut captured = 0usize;
            if let Some(root) = content_root {
                let root = Path::new(&root);
                if !root.is_dir() {
                    bail!("content_root {} is not a directory", root.display());
                }
                captured = capture_bundle(mcp, &name, root)?.len();
            }
            println!(
                "agent_system_id={} name={} concern={} captured_files={}",
                row.id.0, row.name, row.concern, captured
            );
        }
        Command::ListDomainSystems => {
            for r in list_domain_systems(mcp)? {
                println!("{} {} {}", r.id.0, r.name, r.version);
            }
        }
        Command::ListAgentSystems => {
            for r in list_agent_systems(mcp)? {
                println!(
                    "{} {} concern={} privileged={}",
                    r.id.0, r.name, r.concern, r.is_privileged
                );
            }
        }
        Command::DomainSystemInfo { name } => {
            println!("{}", serde_json::to_string_pretty(&domain_system_info(mcp, &name)?)?);
        }
        Command::AgentSystemInfo { name } => {
            println!("{}", serde_json::to_string_pretty(&agent_system_info(mcp, &name)?)?);
        }
        Command::RepoRegister { root, domain_system, repo_name, repo_uuid, domain_system_version } => {
            let root_path = Path::new(&root);
            let mut name = repo_name;
            let mut uuid = repo_uuid.map(|u| u.to_string());
            if let Ok(cfg) = load_repo_config(root_path) {
                if name.is_none() {
                    name = Some(cfg.repository.name.clone());
                }
                if uuid.is_none() {
                    uuid = cfg.repository.uuid.map(|u| u.to_string());
                }
            }
            let name = name.ok_or_else(|| anyhow::anyhow!("repo_name required (no dharma-repo.toml)"))?;
            let uuid = uuid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let reg = bootstrap_repo_registration(
                mcp, &uuid, &name, &root, &domain_system, domain_system_version.as_deref(),
            )?;
            let proposed = list_capability_manifests(mcp, reg.id)?.len();
            println!(
                "repo_registration_id={} repo_uuid={} status={} proposed_capabilities={}",
                reg.id, reg.repo_uuid, reg.status, proposed
            );
        }
        Command::RepoList => {
            for r in list_repo_registrations(mcp)? {
                println!(
                    "{} {} root={} ds_id={} status={}",
                    r.repo_uuid, r.repo_name, r.repo_root, r.domain_system_id.0, r.status
                );
            }
        }
        Command::RepoStatus { root, uuid } => {
            let uuid = match (uuid, root) {
                (Some(u), _) => u,
                (None, Some(root)) => uuid_from_root(&root)?,
                (None, None) => bail!("provide --root or --uuid"),
            };
            let reg = get_repo_registration(mcp, &uuid)?
                .ok_or_else(|| anyhow::anyhow!("repo '{uuid}' not registered"))?;
            println!("repo_uuid={} name={} status={}", reg.repo_uuid, reg.repo_name, reg.status);
            for m in list_capability_manifests(mcp, reg.id)? {
                let as_name = get_agent_system(mcp, m.agent_system_id)?.map(|a| a.name).unwrap_or_default();
                println!(
                    "  capability agent_system={} status={} reviewed_by={:?}",
                    as_name, m.status, m.reviewed_by
                );
            }
        }
        Command::RepoApprove { root, agent_system, reviewed_by, yes } => {
            review_gate(&reviewed_by, yes)?;
            review_capability_flow(mcp, &root, &agent_system, "approved", &reviewed_by)?;
            println!("approved {agent_system} for {root}");
        }
        Command::RepoReject { root, agent_system, reviewed_by, yes } => {
            review_gate(&reviewed_by, yes)?;
            review_capability_flow(mcp, &root, &agent_system, "rejected", &reviewed_by)?;
            println!("rejected {agent_system} for {root}");
        }
        Command::RepoSync { root } => {
            let uuid = uuid_from_root(&root)?;
            let reg = get_repo_registration(mcp, &uuid)?
                .ok_or_else(|| anyhow::anyhow!("repo '{root}' not registered"))?;
            let repo = open_repo(&root)?;
            let values = repo_config_values(Path::new(&root), &reg)?;
            let report = sync_to_repo(mcp, &repo, &reg, &values)?;
            println!(
                "synced_rows={} materialized_files={} missing_coverage={}",
                report.synced_rows,
                report.materialized_files,
                report.missing_coverage.join(", ")
            );
        }
        Command::RepoConfig { root } => {
            let repo = open_repo(&root)?;
            match get_repo_config(&repo)? {
                Some(c) => {
                    println!(
                        "repo_uuid={} name={} domain={} {} dharma_dir={} mcp_dir={}",
                        c.repo_uuid, c.repo_name, c.domain_system_name, c.domain_system_version,
                        c.dharma_dir, c.mcp_dir
                    );
                }
                None => println!("not synced yet (run repo sync / approve a capability)"),
            }
        }
        Command::TaskAssign { root, task_ref, agent_ref } => {
            require_approved(mcp, &root)?;
            let (task_id, task_name) = resolve_task(mcp, &task_ref)?;
            let repo = open_repo(&root)?;
            let (as_id, agent_id) = match agent_ref {
                Some(r) => resolve_agent_ref(mcp, &r)?,
                None => resolve_agent_by_concern(mcp, &task_first_capability(mcp, task_id)?)?,
            };
            let instance = create_task_instance(&repo, task_id, as_id, agent_id)?;
            println!(
                "task_instance_id={} task={} status={}",
                instance.id.0, task_name, instance.status
            );
        }
        Command::TaskStatus { root, task_instance } => {
            let repo = open_repo(&root)?;
            let instance = get_task_instance(&repo, TaskInstanceId(task_instance))?
                .ok_or_else(|| anyhow::anyhow!("task instance {task_instance} not found"))?;
            println!(
                "task_instance_id={} task_id={} attempt={} status={}",
                instance.id.0, instance.task_id, instance.attempt_number, instance.status
            );
            for r in list_proposal_revisions(&repo, instance.id)? {
                println!("  revision {} by agent {}: {}", r.id, r.drafted_by_agent_id, r.reviewer_comments.as_deref().unwrap_or(""));
            }
            if let Some(e) = get_execution_state(&repo, instance.id)? {
                println!(
                    "  execution step={} owner={}",
                    e.current_step_order, e.current_owning_agent_id
                );
            }
        }
        Command::ProposalDraft { root, task_instance, agent_ref, draft } => {
            let repo = open_repo(&root)?;
            let instance = get_task_instance(&repo, TaskInstanceId(task_instance))?
                .ok_or_else(|| anyhow::anyhow!("task instance {task_instance} not found"))?;
            let (as_id, agent_id) = match agent_ref {
                Some(r) => resolve_agent_ref(mcp, &r)?,
                None => (instance.initiating_agent_system_id, instance.initiating_agent_id),
            };
            let rev = draft_proposal(&repo, instance.id, as_id, agent_id, &draft)?;
            println!("revision_id={} number={}", rev.id, rev.revision_number);
        }
        Command::ProposalReview { root, task_instance, decision, comments, reviewed_by, yes } => {
            review_gate(&reviewed_by, yes)?;
            let repo = open_repo(&root)?;
            let instance = TaskInstanceId(task_instance);
            let revisions = list_proposal_revisions(&repo, instance)?;
            let latest = revisions.last().ok_or_else(|| anyhow::anyhow!("no revisions to review"))?;
            match decision.as_str() {
                "approve" => {
                    let a = approve_proposal(&repo, instance, latest.id, &reviewed_by)?;
                    println!("approved revision {} by {}", a.approved_revision_id, a.approved_by);
                }
                "revise" => {
                    let comments = comments.ok_or_else(|| anyhow::anyhow!("revise requires --comments"))?;
                    comment_on_revision(&repo, latest.id, &comments)?;
                    println!("feedback on revision {}", latest.id);
                }
                other => bail!("invalid decision '{other}' (approve|revise)"),
            }
        }
        Command::Handoff { root, task_instance, to_agent_ref, reason } => {
            let repo = open_repo(&root)?;
            let instance = TaskInstanceId(task_instance);
            let state = get_execution_state(&repo, instance)?
                .ok_or_else(|| anyhow::anyhow!("no execution state"))?;
            let (to_as, to_agent) = resolve_agent_ref(mcp, &to_agent_ref)?;
            let hop = record_handoff(
                &repo, instance, state.current_owning_agent_system_id,
                state.current_owning_agent_id, to_as, to_agent, &reason, true, None,
            )?;
            println!("handoff_id={} hop={} to_agent={}", hop.id, hop.hop_order, hop.to_agent_id);
        }
        Command::RunSkill { root, skill_ref, task_instance, input } => {
            let input_value = input.map(|s| serde_json::from_str(&s)).transpose()?.unwrap_or(serde_json::Value::Null);
            let root_path = Path::new(&root);
            let (skill_id, skill_name, is_analysis_only, script_ref, prompt) = skill_lookup(mcp, &skill_ref)?;
            let repo = open_repo(&root)?;
            let instance = get_task_instance(&repo, TaskInstanceId(task_instance))?
                .ok_or_else(|| anyhow::anyhow!("task instance {task_instance} not found"))?;
            check_skill_invocation_allowed(&instance.status, is_analysis_only)?;
            let script_path = script_ref.map(|s| {
                let p = PathBuf::from(s);
                if p.is_absolute() { p } else { root_path.join(p) }
            });
            match script_path {
                Some(p) if p.exists() => {
                    let envelope = serde_json::json!({"repo_root": root, "input": input_value});
                    let in_path = std::env::temp_dir().join(format!("dharma-cli-in-{}.json", uuid::Uuid::new_v4()));
                    std::fs::write(&in_path, envelope.to_string())?;
                    let out = common::env::run_capability_script(&p, root_path, &in_path, None);
                    let _ = std::fs::remove_file(&in_path);
                    println!("{}", serde_json::to_string_pretty(&out?)?);
                }
                Some(p) => bail!("skill script {} does not exist", p.display()),
                None => {
                    println!("analysis_only (no script): {}", prompt.unwrap_or(skill_name));
                    println!("{}", serde_json::to_string_pretty(&input_value)?);
                }
            }
            let _ = skill_id;
        }
        Command::CompleteTask { root, task_instance, verdict, reason } => {
            let repo = open_repo(&root)?;
            let v = complete_task_instance(&repo, TaskInstanceId(task_instance), &verdict, &reason)?;
            let instance = get_task_instance(&repo, v.task_instance_id)?.unwrap();
            println!("verdict={} status={}", v.verdict, instance.status);
        }
        Command::AuditRun { root, commit, domain, kind } => {
            require_approved(mcp, &root)?;
            let repo = open_repo(&root)?;
            let uuid = uuid_from_root(&root)?;
            let (domain_id, _) = resolve_domain(mcp, &uuid, &domain)?;
            let run = start_audit_run(&repo, &commit, domain_id, &domain, &kind)?;
            println!("audit_run_id={} status={}", run.id, run.status);
        }
        Command::AuditShow { root, run } => {
            let repo = open_repo(&root)?;
            let r = get_audit_run(&repo, run)?.ok_or_else(|| anyhow::anyhow!("run {run} not found"))?;
            println!(
                "run={} commit={} domain={} kind={} status={}",
                r.id, r.commit_hash, r.domain_name, r.kind, r.status
            );
            if let Some(d) = get_deterministic_result_for_run(&repo, run)? {
                println!("  deterministic score={} ({}/{})", d.score, d.rules_passed, d.rules_total);
            }
            for s in list_semantic_runs_for_run(&repo, run)? {
                println!("  semantic {} score={}", s.model, s.overall_score);
            }
            for f in list_findings_for_run(&repo, run)? {
                println!("  finding [{}] {}", f.finding_type, f.text);
            }
        }
        Command::AuditOverride { root, run, action, target, score, reason, reviewed_by, yes } => {
            review_gate(&reviewed_by, yes)?;
            let repo = open_repo(&root)?;
            let row = override_audit_result(
                &repo, run, target.as_deref().unwrap_or("run"), &action, score, &reason, &reviewed_by,
            )?;
            println!("override_id={} action={} target={}", row.id, row.action, row.target);
        }
        Command::ProposalAdvance { root, name, status, commit, message, reviewed_by, yes } => {
            review_gate(&reviewed_by, yes)?;
            let repo = open_repo(&root)?;
            let message = message.unwrap_or_default();
            let row = match status.as_str() {
                "draft" => {
                    if get_proposal(&repo, &name)?.is_none() {
                        register_proposal(&repo, &name, &name, &format!("docs/proposal/{name}.md"))?;
                    }
                    set_draft_commit(&repo, &name, &commit, &message)?
                }
                "finalized" => finalize_proposal(&repo, &name, &commit, &message)?,
                "implementing" => start_implementation(&repo, &name)?,
                "verified" => verify_proposal(&repo, &name, &commit, &message)?,
                "archived" => archive_proposal(&repo, &name, &commit, &message, Path::new(&root))?,
                other => bail!("invalid status '{other}'"),
            };
            let row = row.ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
            println!("{} status={}", row.name, row.status);
        }
        Command::ProposalLog { root, name, commit, phase, message } => {
            let repo = open_repo(&root)?;
            let phase = phase.unwrap_or_else(|| "implementation".into());
            let message = message.unwrap_or_default();
            let entry = record_commit(&repo, &name, &commit, &phase, &message)?
                .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
            println!(
                "logged {} phase={} id={}",
                entry.commit_hash, entry.phase, entry.proposal_lifecycle_id
            );
        }
        Command::ProposalShow { root, name } => {
            let repo = open_repo(&root)?;
            let row = get_proposal(&repo, &name)?
                .ok_or_else(|| anyhow::anyhow!("proposal '{name}' not found"))?;
            println!(
                "{} status={} doc={} draft={:?} verified={:?}",
                row.name, row.status, row.doc_path, row.draft_commit_hash, row.verified_at
            );
            for c in list_commit_log(&repo, &name)? {
                println!("  {} {} {}", c.commit_hash, c.phase, c.message);
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers (mirror the MCP adapter's plumbing)
// ---------------------------------------------------------------------------

fn review_gate(reviewed_by: &str, yes: bool) -> Result<()> {
    if !yes {
        bail!("human approval required: pass --yes (and --reviewed-by) to confirm");
    }
    if reviewed_by.trim().is_empty() {
        bail!("--reviewed-by must name the reviewer");
    }
    Ok(())
}

fn review_capability_flow(mcp: &McpDb, root: &str, agent_system: &str, status: &str, reviewed_by: &str) -> Result<()> {
    let uuid = uuid_from_root(root)?;
    let reg = get_repo_registration(mcp, &uuid)?
        .ok_or_else(|| anyhow::anyhow!("repo '{root}' not registered"))?;
    let as_row = get_agent_system_by_name(mcp, agent_system)?
        .ok_or_else(|| anyhow::anyhow!("Agent System '{agent_system}' not registered"))?;
    let manifest = list_capability_manifests(mcp, reg.id)?
        .into_iter()
        .find(|m| m.agent_system_id.0 == as_row.id.0)
        .ok_or_else(|| anyhow::anyhow!("no capability proposed for '{agent_system}'"))?;
    review_capability(mcp, manifest.id, status, reviewed_by)?
        .ok_or_else(|| anyhow::anyhow!("capability {} no longer reviewable", manifest.id))?;
    if status == "approved" {
        let repo = open_repo(root)?;
        let values = repo_config_values(Path::new(root), &reg)?;
        let report = sync_to_repo(mcp, &repo, &reg, &values)?;
        println!("  auto-sync: {} rows, {} files", report.synced_rows, report.materialized_files);
    }
    Ok(())
}

fn uuid_from_root(root: &str) -> Result<String> {
    let cfg = load_repo_config(Path::new(root))
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    cfg.repository
        .uuid
        .map(|u| u.to_string())
        .ok_or_else(|| anyhow::anyhow!("dharma-repo.toml has no [repository].uuid"))
}

fn open_repo(root: &str) -> Result<RepoDb> {
    let root_path = Path::new(root);
    match load_repo_config(root_path) {
        Ok(cfg) => {
            let dharma_dir = cfg.resolve_dharma_dir(root_path);
            RepoDb::open_at(&dharma_dir.join("repo.db"))
        }
        Err(_) => RepoDb::open_at(&root_path.join(".dharma").join("repo.db")),
    }
}

fn repo_config_values(root: &Path, reg: &RepoRegistrationRow) -> Result<RepoConfigValues> {
    match load_repo_config(root) {
        Ok(cfg) => {
            let dharma_dir = cfg.resolve_dharma_dir(root);
            let mcp_dir = cfg.resolve_mcp_dir(root);
            Ok(RepoConfigValues {
                repo_uuid: reg.repo_uuid.clone(),
                repo_name: reg.repo_name.clone(),
                repo_root: root.to_string_lossy().to_string(),
                domain_system_name: cfg.repository.domain.as_ref().map(|d| d.select.clone()).unwrap_or_default(),
                domain_system_version: reg.domain_system_version.clone(),
                docs_dir: cfg.repository.documentation.as_ref().map(|d| d.root_dir.clone()),
                implementation_dir: cfg.repository.implementation.as_ref().map(|i| i.dir.clone()),
                scripts_dir: cfg.repository.scripts.as_ref().map(|s| s.dir.clone()),
                tests_dir: cfg.repository.tests.as_ref().map(|t| t.dir.clone()),
                report_dir: cfg.report.as_ref().map(|r| r.dir.clone()),
                dharma_dir: dharma_dir.to_string_lossy().to_string(),
                mcp_dir: mcp_dir.to_string_lossy().to_string(),
            })
        }
        Err(_) => Ok(RepoConfigValues {
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
            dharma_dir: root.join(".dharma").to_string_lossy().to_string(),
            mcp_dir: common::env::mcp_dir().to_string_lossy().to_string(),
        }),
    }
}

fn require_approved(mcp: &McpDb, root: &str) -> Result<()> {
    let uuid = uuid_from_root(root)?;
    let reg = get_repo_registration(mcp, &uuid)?
        .ok_or_else(|| anyhow::anyhow!("repo '{root}' not registered"))?;
    if reg.status != "approved" {
        bail!("repo '{root}' registration is '{}', not 'approved'", reg.status);
    }
    if let Some(ds) = get_domain_system(mcp, reg.domain_system_id)? {
        if ds.version != reg.domain_system_version {
            bail!(
                "repo pinned version '{}' is stale (Domain System is now '{}')",
                reg.domain_system_version,
                ds.version
            );
        }
    }
    Ok(())
}

fn resolve_task(mcp: &McpDb, task_ref: &str) -> Result<(i64, String)> {
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name FROM task WHERE id = ?1 OR name = ?2 ORDER BY id LIMIT 1")?;
    let mut rows = stmt.query_map(
        rusqlite::params![task_ref.parse::<i64>().unwrap_or(0), task_ref],
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
    )?;
    match rows.next() {
        Some(Ok(row)) => Ok(row),
        _ => bail!("task '{task_ref}' not found in mcp.db"),
    }
}

fn task_first_capability(mcp: &McpDb, task_id: i64) -> Result<String> {
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let concern = conn.query_row(
        "SELECT required_capability FROM task_step WHERE task_id = ?1 ORDER BY step_order LIMIT 1",
        rusqlite::params![task_id],
        |r| r.get::<_, String>(0),
    )?;
    Ok(concern)
}

fn resolve_agent_ref(mcp: &McpDb, agent_ref: &str) -> Result<(i64, i64)> {
    let (a, b) = agent_ref
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("agent_ref must be '<system|id>/<agent|id>'"))?;
    if let (Ok(s), Ok(ag)) = (a.parse::<i64>(), b.parse::<i64>()) {
        return Ok((s, ag));
    }
    let system = get_agent_system_by_name(mcp, a)?
        .ok_or_else(|| anyhow::anyhow!("agent system '{a}' not found"))?;
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let agent_id = conn.query_row(
        "SELECT id FROM agent WHERE agent_system_id = ?1 AND name = ?2",
        rusqlite::params![system.id.0, b],
        |r| r.get::<_, i64>(0),
    )?;
    Ok((system.id.0, agent_id))
}

fn resolve_agent_by_concern(mcp: &McpDb, concern: &str) -> Result<(i64, i64)> {
    let system = get_agent_system_by_concern(mcp, concern)?
        .ok_or_else(|| anyhow::anyhow!("no Agent System has concern '{concern}'"))?;
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let agent_id = conn.query_row(
        "SELECT id FROM agent WHERE agent_system_id = ?1 ORDER BY id LIMIT 1",
        rusqlite::params![system.id.0],
        |r| r.get::<_, i64>(0),
    )?;
    Ok((system.id.0, agent_id))
}

fn skill_lookup(mcp: &McpDb, skill_ref: &str) -> Result<(i64, String, bool, Option<String>, Option<String>)> {
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, is_analysis_only FROM skill WHERE id = ?1 OR name = ?2 ORDER BY id LIMIT 1",
    )?;
    let mut rows = stmt.query_map(
        rusqlite::params![skill_ref.parse::<i64>().unwrap_or(0), skill_ref],
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, bool>(2)?)),
    )?;
    let (skill_id, skill_name, is_analysis_only) = match rows.next() {
        Some(Ok(row)) => row,
        _ => bail!("skill '{skill_ref}' not found"),
    };
    let script = conn
        .query_row(
            "SELECT script_ref FROM skill_script WHERE skill_id = ?1 ORDER BY id LIMIT 1",
            rusqlite::params![skill_id],
            |r| r.get(0),
        )
        .ok();
    let prompt = conn
        .query_row(
            "SELECT template_text FROM skill_prompt WHERE skill_id = ?1 ORDER BY id LIMIT 1",
            rusqlite::params![skill_id],
            |r| r.get(0),
        )
        .ok();
    Ok((skill_id, skill_name, is_analysis_only, script, prompt))
}

fn resolve_domain(mcp: &McpDb, uuid: &str, domain_name: &str) -> Result<(i64, String)> {
    let conn = mcp.conn();
    let conn = conn.lock().unwrap();
    let ds_id: i64 = conn.query_row(
        "SELECT domain_system_id FROM repo_registration WHERE repo_uuid = ?1",
        rusqlite::params![uuid],
        |r| r.get(0),
    )?;
    let id: i64 = conn.query_row(
        "SELECT id FROM domain WHERE domain_system_id = ?1 AND name = ?2",
        rusqlite::params![ds_id, domain_name],
        |r| r.get(0),
    )?;
    Ok((id, domain_name.to_string()))
}

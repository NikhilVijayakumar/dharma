//! MCP `tools/list` definitions for the 26 tools in
//! docs/proposal/14-mcp-tool-contract.md. Names/params must track that
//! table and `adapter::route`'s match arms exactly.

use serde_json::{Value, json};

struct Field {
    name: &'static str,
    kind: &'static str,
    required: bool,
    description: &'static str,
}

const fn f(name: &'static str, kind: &'static str, required: bool, description: &'static str) -> Field {
    Field { name, kind, required, description }
}

const REPO_PATH: Field = f("repo_path", "string", true, "Absolute path to the target repository");
const HUMAN_APPROVED: Field = f("human_approved", "boolean", true, "Must be true — explicit human authorization for this gated action");
const REVIEWED_BY: Field = f("reviewed_by", "string", true, "Identity of the human reviewer");

struct ToolSpec {
    name: &'static str,
    description: &'static str,
    fields: &'static [Field],
}

fn schema(fields: &[Field]) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for field in fields {
        properties.insert(
            field.name.to_string(),
            json!({"type": field.kind, "description": field.description}),
        );
        if field.required {
            required.push(field.name);
        }
    }
    json!({"type": "object", "properties": Value::Object(properties), "required": required})
}

pub fn tool_defs() -> Vec<Value> {
    TOOLS
        .iter()
        .map(|t| json!({"name": t.name, "description": t.description, "inputSchema": schema(t.fields)}))
        .collect()
}

static TOOLS: &[ToolSpec] = &[
    ToolSpec { name: "register_domain_system", description: "Register a Domain System (name, version, description, content_root); returns domain_system_id.", fields: &[
        f("name", "string", true, "Domain System name"),
        f("version", "string", true, "Domain System version"),
        f("description", "string", true, "Domain System description"),
        f("content_root", "string", true, "Path to the Domain System's content root"),
    ]},
    ToolSpec { name: "register_agent_system", description: "Register an Agent System (name, concern, description, content_root, is_privileged_request); returns agent_system_id.", fields: &[
        f("name", "string", true, "Agent System name"),
        f("concern", "string", true, "Agent System concern"),
        f("description", "string", true, "Agent System description"),
        f("content_root", "string", true, "Path to the Agent System's content root"),
        f("is_privileged_request", "boolean", true, "Whether this Agent System requests privileged capabilities"),
    ]},
    ToolSpec { name: "recapture_domain_system", description: "Re-capture a registered Domain System's content; appends new content_asset rows.", fields: &[
        f("name", "string", true, "Domain System name"),
    ]},
    ToolSpec { name: "recapture_agent_system", description: "Re-capture a registered Agent System's content; appends new content_asset rows.", fields: &[
        f("name", "string", true, "Agent System name"),
    ]},
    ToolSpec { name: "list_domain_systems", description: "List every registered Domain System (name, version, description).", fields: &[] },
    ToolSpec { name: "list_agent_systems", description: "List every registered Agent System (name, concern, description, is_privileged).", fields: &[] },
    ToolSpec { name: "get_domain_system_info", description: "Full domain/section/section_profile/epic/usecase/task tree for a Domain System.", fields: &[
        f("name", "string", true, "Domain System name"),
    ]},
    ToolSpec { name: "get_agent_system_info", description: "Full agent/skill tree for an Agent System.", fields: &[
        f("name", "string", true, "Agent System name"),
    ]},
    ToolSpec { name: "register_repo", description: "Register a repository against a Domain System; returns repo_registration_id and status: 'pending'.", fields: &[
        REPO_PATH,
        f("repo_name", "string", true, "Repository display name"),
        f("domain_system_name", "string", true, "Domain System to register against"),
        f("domain_system_version", "string", false, "Domain System version (defaults to latest)"),
    ]},
    ToolSpec { name: "list_repos", description: "List every registered repository (repo_uuid, name, domain_system, status).", fields: &[] },
    ToolSpec { name: "repo_status", description: "Full repo_registration + capability_manifest state for a repository.", fields: &[REPO_PATH] },
    ToolSpec { name: "unregister_repo", description: "Remove a repository's registration.", fields: &[REPO_PATH] },
    ToolSpec { name: "review_capability_manifest", description: "Approve or reject a repository's Capability Manifest for an Agent System. Gated: requires human_approved and reviewed_by. Approval triggers automatic sync.", fields: &[
        REPO_PATH,
        f("agent_system_name", "string", true, "Agent System being reviewed"),
        f("decision", "string", true, "'approve' or 'reject'"),
        HUMAN_APPROVED,
        REVIEWED_BY,
    ]},
    ToolSpec { name: "sync_repo", description: "Explicit re-sync of mcp.db state into a repository's repo.db (e.g. after a Domain/Agent System update).", fields: &[REPO_PATH] },
    ToolSpec { name: "get_repo_config", description: "The repo_config row (docs/tests/scripts/implementation/report dirs) for a repository.", fields: &[REPO_PATH] },
    ToolSpec { name: "assign_task", description: "Assign a Task to an agent; returns task_instance_id and initiating_agent.", fields: &[
        REPO_PATH,
        f("task_ref", "string", true, "Reference to the Task to assign"),
    ]},
    ToolSpec { name: "submit_proposal_draft", description: "Submit a proposal draft for a Task Instance; returns revision_id.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
        f("agent_ref", "string", true, "Agent submitting the draft"),
        f("draft", "string", true, "Draft content"),
    ]},
    ToolSpec { name: "review_task_proposal", description: "Approve or request revision of a Task proposal. Gated: requires human_approved and reviewed_by.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
        f("decision", "string", true, "'approve' or 'revise'"),
        HUMAN_APPROVED,
        REVIEWED_BY,
        f("comments", "string", false, "Reviewer comments"),
    ]},
    ToolSpec { name: "request_handoff", description: "Request handoff of a Task Instance to another agent; returns 'accepted' or 'rejected'.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
        f("to_agent_ref", "string", true, "Agent to hand off to"),
        f("reason", "string", true, "Reason for the handoff"),
    ]},
    ToolSpec { name: "run_skill", description: "Run a Skill against a Task Instance; returns the Skill's output. The only tool that may have real effect.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
        f("skill_ref", "string", true, "Reference to the Skill to run"),
        f("input", "string", true, "Input passed to the Skill"),
    ]},
    ToolSpec { name: "submit_completion_validation", description: "Submit completion validation for a Task Instance; returns a verdict.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
    ]},
    ToolSpec { name: "task_instance_status", description: "Full state/history for a Task Instance.", fields: &[
        REPO_PATH,
        f("task_instance_id", "string", true, "Task Instance id"),
    ]},
    ToolSpec { name: "run_audit", description: "Run an audit; returns audit_run_id.", fields: &[
        REPO_PATH,
        f("domain", "string", true, "Domain to audit"),
        f("kind", "string", true, "Audit kind"),
    ]},
    ToolSpec { name: "get_audit_result", description: "Scores, evidence, and findings for an audit run.", fields: &[
        REPO_PATH,
        f("audit_run_id", "string", true, "Audit run id"),
    ]},
    ToolSpec { name: "override_audit", description: "Override or cancel an audit verdict. Requires reviewed_by (not the full human_approved gate — a correction, not an authorization to act).", fields: &[
        REPO_PATH,
        f("audit_run_id", "string", true, "Audit run id"),
        f("action", "string", true, "'override' or 'cancel'"),
        f("reason", "string", true, "Reason for the override/cancel"),
        REVIEWED_BY,
        f("override_score", "string", false, "Replacement score when action is 'override'"),
    ]},
    ToolSpec { name: "advance_proposal_lifecycle", description: "Advance a proposal's lifecycle status. Gated: requires human_approved and reviewed_by.", fields: &[
        REPO_PATH,
        f("proposal_name", "string", true, "Proposal name"),
        f("new_status", "string", true, "Status to advance to"),
        f("commit_hash", "string", true, "Commit hash for this transition"),
        HUMAN_APPROVED,
        REVIEWED_BY,
    ]},
    ToolSpec { name: "log_proposal_commit", description: "Append a commit log entry for a proposal. Plain append, no gate.", fields: &[
        REPO_PATH,
        f("proposal_name", "string", true, "Proposal name"),
        f("commit_hash", "string", true, "Commit hash"),
        f("phase", "string", true, "Lifecycle phase this commit belongs to"),
        f("message", "string", true, "Commit message"),
    ]},
    ToolSpec { name: "get_proposal_lifecycle", description: "A proposal's lifecycle row plus its full commit log.", fields: &[
        REPO_PATH,
        f("proposal_name", "string", true, "Proposal name"),
    ]},
];

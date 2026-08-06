# Proposal: MCP Tool Contract

> Status: Draft — design-only, no implementation code. Conforms to `docs/raw/proposal.md` standard.
> **Decides two open questions from `00-overview.md`'s What Is Still Open** ("The MCP transport/tool surface itself"), per explicit direction: (1) human-authorization-gate actions (Capability Manifest approve/reject, Task proposal approve/revise, Proposal Lifecycle status advance) **are** MCP tools an agent can call, gated by a required `human_approved` field plus a `reviewed_by` identity on every such call — not CLI-only. (2) `mcp.db → repo.db` sync **runs automatically** the moment a Capability Manifest reaches `approved`; `sync_repo` remains as an explicit tool only for re-sync after a Domain/Agent System update.

## Purpose

This document defines the standard for Dharma's MCP wire-level tool contract: the concrete tool names, request/response shapes, and dispatch convention that expose the `services` crate's operations (proposals 04-13) to an MCP client.

Unlike proposals 01-13, which describe entities and flows without naming a wire format, this document names one — reusing Samgraha's proven MCP dispatch shape (a single JSON-RPC-style envelope: `method` + `params` + an explicit `repo_path`, dispatched by a flat `match` over method strings) rather than inventing a new protocol convention, per the same "reuse over reinvent" principle applied throughout this proposal set.

## System Overview

### Overview

Every dharma tool call is one `McpRequest { id, method, params, repo_path }`; every response is one `McpResponse { id, result }` or `McpError { id, code, message }` — identical in shape to Samgraha's `crates/mcp/src/protocol.rs`. `repo_path` is explicit on every call that targets a specific repository (never a session-bound "current repo"), matching Samgraha's own fix for multi-repo MCP sessions (`docs/errors-list/2026-07-08-no-global-repo-registration.md` in Samgraha). Tools are fine-grained — one verb-noun tool per operation, not a handful of tools multiplexed by an internal `action` parameter — matching Samgraha's 18-tool surface for a comparable-scope system; Dharma's larger scope (Agent/Skill/Domain-System-as-first-class, Task execution, audit, Proposal Lifecycle) yields 28.

### Structural Approach

Tools group into five concerns, each mapping to one `services` responsibility area already named in proposal 08: **Registry & Capture** (Domain/Agent System providers), **Repo Registration & Sync** (proposal 06/11), **Task Execution** (proposal 07), **Audit** (proposal 08), and **Proposal Lifecycle** (proposal 12). No tool spans two concerns; a client composes multiple calls for a multi-concern operation, exactly as Samgraha composes `register_standard` + `seed_standard` rather than one combined call.

### Diagram

```text
MCP Client ──McpRequest{method, params, repo_path}──▶ mcp crate (dispatch by method string)
                                                              │
                                                              ▼
                                                       services crate (per proposal 08)
                                                              │
                                                              ▼
                                                       registry crate ──▶ mcp.db / repo.db
                                                              │
MCP Client ◀──McpResponse{result} | McpError{code,message}───┘
```

## Component Model

### Registry & Capture (provider-facing)

| Tool | Params | Result |
|---|---|---|
| `register_domain_system` | `name, version, description, content_root` | `domain_system_id` |
| `register_agent_system` | `name, concern, description, content_root, is_privileged_request` | `agent_system_id` |
| `recapture_domain_system` | `name` | new `content_asset` rows appended |
| `recapture_agent_system` | `name` | new `content_asset` rows appended |
| `list_domain_systems` | *(none)* | `[{name, version, description}]` |
| `list_agent_systems` | *(none)* | `[{name, concern, description, is_privileged}]` |
| `get_domain_system_info` | `name` | full domain/section/section_profile/epic/usecase/task tree |
| `get_agent_system_info` | `name` | full agent/skill tree |

- **Responsibility:** Wraps proposal 08's capture flow and proposals 04/05's registries.
- **Ownership:** No schema of its own; calls `services`' capture and registry-lookup operations.
- **Interfaces:** `register_*`/`recapture_*` write to `mcp.db` via `content_asset`; `list_*`/`get_*_info` are read-only.

### Repo Registration & Sync

| Tool | Params | Result |
|---|---|---|
| `register_repo` | `repo_path, repo_name, domain_system_name, domain_system_version?` | `repo_registration_id, status: 'pending'` |
| `list_repos` | *(none)* | `[{repo_uuid, name, domain_system, status}]` |
| `repo_status` | `repo_path` | full `repo_registration` + `capability_manifest` state |
| `unregister_repo` | `repo_path` | removed |
| `review_capability_manifest` | `repo_path, agent_system_name, decision: 'approve'\|'reject', human_approved: true, reviewed_by` | updated `capability_manifest` row |
| `sync_repo` | `repo_path` | re-sync result (explicit re-sync path; see System Overview for the automatic-on-approval path) |
| `get_repo_config` | `repo_path` | `repo_config` row (docs/tests/scripts/implementation/report dirs) |

- **Responsibility:** Wraps proposal 06's registration sequence and proposal 11's sync/materialization flow.
- **Ownership:** No schema of its own; `register_repo`/`review_capability_manifest` write `mcp.db`; sync (automatic or via `sync_repo`) writes the target repository's `repo.db` and `.dharma/assets/`.
- **Interfaces:** `review_capability_manifest` is a human-authorization gate — see Security.

### Task Execution

| Tool | Params | Result |
|---|---|---|
| `assign_task` | `repo_path, task_ref` | `task_instance_id, initiating_agent` |
| `submit_proposal_draft` | `repo_path, task_instance_id, agent_ref, draft` | `revision_id` |
| `review_task_proposal` | `repo_path, task_instance_id, decision: 'approve'\|'revise', human_approved: true, reviewed_by, comments?` | updated Task Instance status |
| `request_handoff` | `repo_path, task_instance_id, to_agent_ref, reason` | `accepted \| rejected` |
| `run_skill` | `repo_path, task_instance_id, skill_ref, input` | Skill output |
| `submit_completion_validation` | `repo_path, task_instance_id` | verdict |
| `task_instance_status` | `repo_path, task_instance_id` | full state/history |

- **Responsibility:** Wraps proposal 07's Propose→Review→Approve→Execute lifecycle.
- **Ownership:** No schema of its own; writes `repo.db`'s `task_instance`/`proposal_revision`/`execution_state`/`handoff_log`/`completion_validation`.
- **Interfaces:** `review_task_proposal` is the Task-level human-authorization gate — see Security. `run_skill` is the only tool that may have real effect (an analysis-only Skill has none by definition; see proposal 03).

### Audit

| Tool | Params | Result |
|---|---|---|
| `run_audit` | `repo_path, domain, kind` | `audit_run_id` |
| `get_audit_result` | `repo_path, audit_run_id` | scores, evidence, findings |
| `override_audit` | `repo_path, audit_run_id, action: 'override'\|'cancel', reason, reviewed_by, override_score?` | updated verdict |

- **Responsibility:** Wraps proposal 08's audit subsystem.
- **Ownership:** No schema of its own; writes `repo.db`'s `audit_run` and children.
- **Interfaces:** `override_audit` requires `reviewed_by` (a human identity) but not the full `human_approved` gate flag — overriding an already-computed score is a correction, not an authorization to act.

### Proposal Lifecycle

| Tool | Params | Result |
|---|---|---|
| `advance_proposal_lifecycle` | `repo_path, proposal_name, new_status, commit_hash, human_approved: true, reviewed_by` | updated `proposal_lifecycle` row |
| `log_proposal_commit` | `repo_path, proposal_name, commit_hash, phase, message` | appended `proposal_commit_log` row |
| `get_proposal_lifecycle` | `repo_path, proposal_name` | row + full commit log |

- **Responsibility:** Wraps proposal 12's lifecycle tracking.
- **Ownership:** No schema of its own; writes `repo.db`'s `proposal_lifecycle`/`proposal_commit_log`.
- **Interfaces:** `advance_proposal_lifecycle` is a human-authorization gate — see Security. `log_proposal_commit` is a plain append, no gate.

## Communication

### Communication Paths

**MCP Client → `mcp` crate**
- **Pattern:** Synchronous request/response, per Samgraha's `McpMessage`/`McpRequest`/`McpResponse`/`McpError` shape.
- **Contract:** Client sends `{id, method, params, repo_path}`; `mcp` dispatches by `method` string (flat `match`, per Samgraha's `adapter.rs`) to a `services` call, and returns `{id, result}` or `{id, code, message}`.

**Gate tools → `services`**
- **Pattern:** Synchronous, identical dispatch path — no separate protocol for gated vs. ungated tools.
- **Contract:** A gate tool's `params` always include `human_approved: true` and `reviewed_by`; `services` rejects the call if either is absent, before touching `registry`.

### Communication Diagram

```text
Client → mcp : McpRequest{method: "review_capability_manifest", params: {..., human_approved: true, reviewed_by: "alice"}, repo_path}
mcp → services : reviewCapabilityManifest(params)
services → services : reject if human_approved != true or reviewed_by empty
services → registry : write capability_manifest.status, reviewed_by, reviewed_at
registry → services : ok
services → mcp : result
mcp → Client : McpResponse{id, result}
```

## Data Flow

### Data Paths

**Gate-Call Path**
- **Entry point:** A tool listed with a human-authorization requirement in Component Model (`review_capability_manifest`, `review_task_proposal`, `advance_proposal_lifecycle`).
- **Transformations:** `services` checks `human_approved: true` and a non-empty `reviewed_by` are both present before any write; the write itself records `reviewed_by` and a timestamp alongside the state change.
- **Ownership boundary:** No tool other than these three may cause a state transition that proposals 06/07/12 classify as requiring human authorization.
- **Exit point:** A committed, attributed state change, or a rejection if either required field is missing.

**Auto-Sync Path**
- **Entry point:** `review_capability_manifest` commits an `approved` status.
- **Transformations:** `services` immediately invokes the same sync operation `sync_repo` exposes, without a separate client call.
- **Ownership boundary:** Identical to `sync_repo`'s own Data Flow (proposal 11) — full Domain System copy, filtered Agent System copy, materialization to `.dharma/assets/`, summary generation.
- **Exit point:** A synced `repo.db`, ready for Task assignment, with no separate "now sync" step for the client to remember.

### Data Flow Diagram

```text
review_capability_manifest(human_approved: true, reviewed_by) ──▶ capability_manifest.status = 'approved'
                                                                          │
                                                          (automatic, same call)
                                                                          ▼
                                                                sync_repo's own flow ──▶ repo.db + .dharma/assets/
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| `mcp.db` writes (registries, capture, registration, capability manifest) | `registry` crate, via `services`, via the tools in Registry & Capture / Repo Registration & Sync |
| `repo.db` writes (Task execution, audit, proposal lifecycle) | `registry` crate, via `services`, via the tools in Task Execution / Audit / Proposal Lifecycle |
| Every gate call's `reviewed_by` + timestamp | The table the gate writes to (`capability_manifest`, `proposal_approval`, `proposal_lifecycle`) — permanent, queryable attribution |

## Security

### Trust Boundaries

- **MCP Client → gate tools:** Weaker than a CLI-only design would give — `human_approved` is a boolean field an MCP client (and, in principle, an agent driving that client) could set without a human actually having approved anything. This tradeoff is deliberate and explicit (see Rationale); it is not a claim that the field is unforgeable.
- **MCP Client → non-gate tools:** Untrusted input, validated by `schemas` before any write, as in every other proposal in this set.

### Threat Model

- **An agent sets `human_approved: true` without a human actually approving:** The structural weakness Option 2 (chosen for this proposal) accepts. Mitigation: `reviewed_by` is mandatory and permanently recorded alongside every gate call — the action is always attributable after the fact, even though it cannot be prevented before the fact the way a CLI-only design would. This is a deliberate trade of prevention for auditability (Philosophy(10) principle 5, "everything is traceable to a source" — applied here in place of principle 1's stronger "structural gate" where the user explicitly chose flexibility over it).
- **`reviewed_by` set to a meaningless placeholder:** A call sets `reviewed_by: "system"` or similar to satisfy the field without real attribution. Mitigation: out of scope for the wire contract itself — an operational/audit concern (a human reviewing `capability_manifest`/`proposal_lifecycle` rows for implausible `reviewed_by` values), not something the protocol layer can structurally prevent.
- **Auto-sync running against a partially-approved manifest:** `review_capability_manifest` triggers sync before all intended Agent Systems are approved, and a later approval doesn't re-trigger. Mitigation: sync is idempotent per proposal 11 (re-running it re-derives `synced_content` from the current approved set) — every `review_capability_manifest` call re-triggers it, not just the first.

## Rationale

### Reuse Samgraha's Wire Shape and Dispatch Convention
- **Context:** Samgraha's MCP server already has a working, tested `McpMessage`/`McpRequest`/`McpResponse` envelope and a flat `method`-string dispatch, for a comparable-scope system.
- **Decision:** Reuse the exact envelope shape and dispatch style rather than designing a new one.
- **Alternatives Considered:** A typed-per-tool RPC framework; GraphQL-style single endpoint with a query language.
- **Rejection Reason:** No architectural benefit was identified over Samgraha's already-working shape, and reinventing it would break the "reuse over reinvent" principle this entire proposal set has applied to crate structure, database split, and config conventions.
- **Architectural Goal:** Consistency with validated prior art (same principle as proposal 08's crate split).

### Fine-Grained Tools, One Per Operation
- **Context:** Samgraha exposes 18 separate tools for its scope rather than a handful of action-multiplexed ones.
- **Decision:** Dharma follows the same convention — 28 tools, one per operation, none accepting an internal `action` enum that changes its parameter shape.
- **Alternatives Considered:** Group by concern into ~5 tools, each taking an `action` parameter.
- **Rejection Reason:** Action-multiplexed tools lose per-operation JSON-schema precision (a client can't know which fields apply without first knowing the action), and Samgraha's own working design already answers this question the other way.
- **Architectural Goal:** Every tool's parameters are fully described by its own schema, no conditional-on-action fields.

### Gate Actions Are MCP Tools, Not CLI-Only — Explicit Choice
- **Context:** Two designs were considered for where human-authorization actions (Capability Manifest approval, Task proposal approval, Proposal Lifecycle advancement) live: CLI-only (stronger structural guarantee, matches Philosophy(10) principle 1 most literally) or MCP tools gated by a required field (more flexible for MCP clients with an inline human-approval UI, weaker structural guarantee).
- **Decision:** MCP tools, gated by mandatory `human_approved: true` + `reviewed_by`.
- **Alternatives Considered:** CLI-only (the initially recommended option).
- **Rejection Reason:** None on architectural grounds — this was a deliberate preference for flexibility (an MCP client that itself surfaces an approval UI to a human, e.g. an IDE integration, can drive the whole flow without shelling out to a separate CLI), accepted with the tradeoff named in Security above.
- **Architectural Goal:** Support MCP clients that provide their own human-in-the-loop UI, at the cost of a structurally weaker (though still fully attributable) gate.

### Sync Runs Automatically on Approval
- **Context:** Requiring a separate `sync_repo` call after every approval is an easy step to forget, leaving an approved-but-unsynced repository.
- **Decision:** `review_capability_manifest` triggers the same sync `sync_repo` performs, automatically, on every call that results in `approved`.
- **Alternatives Considered:** Require an explicit `sync_repo` call as a separate step.
- **Rejection Reason:** Approval is already the human gate; nothing is gained by adding a second manual step before the repository is actually usable, and something is lost (an easy-to-forget manual step with no safety benefit).
- **Architectural Goal:** A repository is immediately usable the moment its Capability Manifest is approved; `sync_repo` remains available for the one case automatic sync doesn't cover — re-syncing after a Domain/Agent System update.

## Constraints

### Hard Constraints
- **Every gate tool requires `human_approved: true` and a non-empty `reviewed_by`** (source: Rationale above) — `services` rejects the call otherwise, before any write.
- **No tool spans two of the five concern groups** (source: Structural Approach above) — a multi-concern operation is a client composing multiple calls, never one tool doing both.
- **`repo_path` is explicit on every repo-scoped call** (source: System Overview) — no session-bound "current repo" state.
- **Approval triggers sync automatically** (source: Rationale above) — `review_capability_manifest` must not return `approved` without also completing the sync it implies.

### Soft Constraints
- Prefer naming new tools `verb_noun` matching Samgraha's existing convention (`register_x`, `list_x`, `get_x_info`, `x_status`) so the two MCP servers read as one family.

## Lifecycle

> Status: draft
> Draft commit: not yet committed
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means: the tool list in Component Model is reviewed against the actual `services` operations named in proposals 06/07/08/11/12 (no operation missing a tool, no tool without a backing operation), and the two open decisions this document makes (gate exposure, sync trigger) are not revisited without a new proposal.

## Traceability

### Derivation Chain

```text
04-agent-system-registry, 05-domain-system-registration, 06-mcp-registration-bootstrap,
07-proposal-execution-protocol, 08-schema-and-crate-architecture,
11-provider-config-and-repo-sync, 12-proposal-lifecycle-and-archival
    │
    ▼
MCP Tool Contract (this document) — the last open item from 00-overview's
What Is Still Open
    │
    ▼
(implementation — the `mcp` crate is built directly against this tool list)
```

### Non-Contradiction Rule

No downstream document may add a tool spanning two concern groups, remove the `human_approved`/`reviewed_by` requirement from a gate tool, make sync a separate required step after approval, or introduce session-bound repo context in place of explicit `repo_path`, without revising this document first.

# Dharma Agent Platform — Proposal Set Overview

> Status: Draft. 01-07 are design-only (no schema/code); 08 introduces the concrete reference schema (`schema/`) and crate boundaries that gate implementation. Index for `docs/proposal/`.

## Reading Order

| # | Document | Defines |
|---|---|---|
| 01 | [Agent Model](01-agent-model.md) | Agent: identity, ≤8 goals, backstory, Skill Bindings, Handoff Policy, Proposal Responsibility |
| 02 | [Epic/Usecase/Task Model](02-task-model.md) | Epic → Usecase → Task hierarchy; Task's Input/Output Contract, Step Sequence, Acceptance Criteria |
| 03 | [Skill Model](03-skill-model.md) | Skill: atomic single-responsibility, prompt+script dual path, analysis-only vs effect-capable |
| 04 | [Agent System Registry](04-agent-system-registry.md) | Open, pluggable Agent Systems (e.g. `documentation-management`, `rust-development`), not a fixed taxonomy |
| 05 | [Domain System Registration](05-domain-system-registration.md) | Domain System (e.g. `rust-dev-domain`, `electron-dev-domain`): Section Map + Section Profiles + Epic/Usecase/Task set, selected not authored |
| 06 | [MCP Registration & Bootstrap](06-mcp-registration-bootstrap.md) | Ordered sequence: MCP exists → repo registers → repo selects Domain System → default Agent System resolves capability |
| 07 | [Proposal & Execution Protocol](07-proposal-execution-protocol.md) | Mandatory Propose → Review → Approve gate, then handoff-chain execution |
| 08 | [Schema & Crate Architecture](08-schema-and-crate-architecture.md) | Concrete SQLite schema for 01-07, grouped by physical database — `mcp.db` (global registries + content) and `repo.db` (per-repo runtime state), see `schema/` — + six-crate Rust workspace (`common`/`schemas`/`registry`/`services`/`cli`/`mcp`); per-crate docs follow [docs/raw/crates.md](../raw/crates.md) |
| 09 | [Vision](09-vision.md) | The pivot (Electron app → agent platform) and Dharma's role as MCP infrastructure; sources the "(source: Vision)" constraints in 01-08 |
| 10 | [Philosophy](10-philosophy.md) | The principles under which agents act with real effect — human authorization as a structural gate, verifiable restraint, no self-certification, least privilege, traceability, open registries; guides Architecture and Security |
| 11 | [Provider Config & Repo Sync](11-provider-config-and-repo-sync.md) | Four TOML roles (`dharma-build`/`dharma-domain`/`dharma-agent`/`dharma-repo`); full Domain System copy + filtered Agent System copy into a consuming repo's `.dharma/repo.db`, every row also materialized to a real file under `.dharma/assets/` so execution never reaches outside the repo; `repo_config` caches resolved toml values; generated `domain-summary.md`/`agent-summary.md` with a Missing Coverage check |
| 12 | [Proposal Lifecycle & Archival](12-proposal-lifecycle-and-archival.md) | New [`docs/raw/proposal.md`](../raw/proposal.md) doc type (Architecture + a tracked Lifecycle section); `repo.db`'s `proposal_lifecycle`/`proposal_commit_log` pin every draft/finalized/implementing/verified/archived transition to a git commit — distinct from the Task-level Proposal Loop (07) |
| 13 | [Bodha Section Format Reference](13-bodha-section-format-reference.md) | Section Map/Profile/Profile-Default format verified against Bodha's actual files, frozen as observed; resolves 05's External Context gap; names (doesn't fix) 4 unparsed fields |
| 14 | [MCP Tool Contract](14-mcp-tool-contract.md) | 28 fine-grained tools (Samgraha's proven wire shape reused), grouped into 5 concerns; gate actions (Capability Manifest, Task proposal, Proposal Lifecycle) are MCP tools requiring `human_approved`+`reviewed_by`, not CLI-only; sync runs automatically on approval |

## Build Order

Schema is expensive to fix once data exists against it. Build order is therefore:

1. **Review and fix 08 and `schema/` first**, before writing any implementation code. Treat the schema as the gate — a wrong table shape caught now costs an edit; caught after repositories and Task Instances hold real rows, it costs a migration.
2. Implement the `registry` crate against the reviewed `schema/`.
3. Only then implement 01-07's behavior in `services`, `cli`, and `mcp`.
4. Config parsing (`dharma-build.toml`/`dharma-domain.toml`/`dharma-agent.toml`/`dharma-repo.toml`, see [11](11-provider-config-and-repo-sync.md) and `config/`) and the sync/summary flow can be implemented alongside 3 — they consume the same `services` operations, not a separate schema.
5. Every proposal from this point forward, including this set's own remaining drafts, gets a `proposal_lifecycle` row per [12](12-proposal-lifecycle-and-archival.md) once `repo.db` exists — this proposal set predates that tracking, so 00-11 have no such row retroactively; only 12 onward are expected to.

## Supersession Note

This proposal set was revised mid-stream. A later, higher-priority instruction corrected several structural decisions made in the first draft. Where a document is marked "Supersedes" or "Amended," that document's current content is authoritative — do not read the first-draft assumptions back in from memory or prior context.

Corrections applied, in summary:

1. **Task is not standalone.** It sits under Usecase, which sits under Epic. This hierarchy belongs to the Domain System, not to an individual repository. (02)
2. **Agent Systems are not a fixed set of five groups.** They are an open registry of Agent Systems — pluggable, named by concern (a technology stack or a cross-cutting function), symmetric with Domain Systems. (04)
3. **A repository does not author its own domain shape.** It selects an existing, registered Domain System (e.g. `rust-dev-domain`, `electron-dev-domain`). The Domain System owns the Section Map, Section Profiles, and Epic/Usecase/Task set; the repository only inherits them. (05)
4. **MCP comes first.** The build order is: MCP exists → repository registers → repository selects a Domain System → MCP's default/bootstrap Agent System analyzes and proposes which other Agent Systems and Section Profiles apply, gated by human approval. (06)
5. **Nothing executes without an approved proposal.** For every Task, an Agent must draft a proposed solution, the user reviews and may request changes, the Agent revises, and only after explicit approval does execution begin. Execution itself keeps the original handoff-chain design: one Agent acts (using one or more Skills), then transfers control to the next Agent, until the Task completes and an independent Completion Validator checks the result. (07)

## What Is Still Open

These proposals are structural (architecture-level) only, per `docs/raw/architecture.md`. Not yet addressed, and intentionally out of scope until these are approved:

- ~~The MCP transport/tool surface itself (tool names, request/response shapes).~~ **Resolved by [14](14-mcp-tool-contract.md)** — 28 tools reusing Samgraha's wire shape; two deliberate decisions made explicitly (gate actions are MCP tools with a `human_approved`/`reviewed_by` requirement rather than CLI-only; sync is automatic on Capability Manifest approval).
- Engineering, Build, Implementation, and QA documentation for any of the above — those follow their own `docs/raw` standards once the architecture here is settled.
- **Vision(09) and Philosophy(10) resolve the raw-standard cross-references.** `docs/raw/architecture.md` expects Architecture to cross-reference Vision(01) and Security to reference Philosophy(02). The Vision document ([09](09-vision.md)) supplies the pivot decision and the "(source: Vision)" constraints cited in 01-08; the Philosophy document ([10](10-philosophy.md)) supplies the principles the threat models in 04, 06, 07, and 08 rest on. Both are drafted; neither is treated as final until this proposal set is approved.
- ~~Bodha's `.bodha-structure/section` and `profile-default` are cited but not vendored or linked as an External Context doc (05).~~ **Resolved by [13](13-bodha-section-format-reference.md)** — verified directly against Bodha's actual files, frozen as observed. Three fields it names as not yet parsed into rows (`paper_type`/`supported_types`, map-level `validation`, profile `trigger`, and the entire Profile Default rule-group structure) are deferred to a later phase, not fixed in this revision — still preserved losslessly via `content_asset` in the meantime.
- **The audit subsystem (deterministic + per-model semantic scoring, weights, report templates, per-commit executions) is specified only in 08**, not as its own architecture-level proposal the way Agent/Task/Skill/Domain System/Agent System each got one. Acceptable for now — it is genuinely schema/engineering-level detail, not a new top-level entity — but worth a dedicated proposal if the audit model grows more structural decisions of its own before implementation.
- **Independent review pass (this revision) found 02, 03, and 05 had drifted from what 08's schema actually implements** — 05 was missing the `domain` tier entirely (a Domain System carries many Domains, each with its own Section Map), 03 stated Prompt and Script were symmetric when the schema (and 08) already made Prompt mandatory and Script optional and didn't mention the optional Template asset at all, and 02 didn't mention Epic self-nesting or a Task's optional `template_ref`. All three are now fixed to match 08/`schema/`. This is the kind of drift that should be checked again before implementation begins, not assumed fixed forever.

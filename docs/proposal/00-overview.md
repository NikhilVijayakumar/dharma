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

## Build Order

Schema is expensive to fix once data exists against it. Build order is therefore:

1. **Review and fix 08 and `schema/` first**, before writing any implementation code. Treat the schema as the gate — a wrong table shape caught now costs an edit; caught after repositories and Task Instances hold real rows, it costs a migration.
2. Implement the `registry` crate against the reviewed `schema/`.
3. Only then implement 01-07's behavior in `services`, `cli`, and `mcp`.

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

- The MCP transport/tool surface itself (tool names, request/response shapes) — 08 defines the `mcp` crate's role, not its wire-level tool contract.
- Engineering, Build, Implementation, and QA documentation for any of the above — those follow their own `docs/raw` standards once the architecture here is settled.
- **Vision(09) and Philosophy(10) exist.** `docs/raw/architecture.md` expects Architecture to cross-reference Vision(01) and Security to reference Philosophy(02). The Vision document ([09](09-vision.md)) supplies the pivot decision and the "(source: Vision)" constraints cited in 01-08; the Philosophy document ([10](10-philosophy.md)) supplies the principles the threat models in 04, 06, 07, and 08 rest on. Both were written before these proposals were treated as final.
- **Bodha's `.bodha-structure/section` and `profile-default` are cited but not vendored or linked as an External Context doc** (05). The Section Map / Section Profile shape in this proposal set assumes that structure is stable and reusable as-is; that assumption is unverified against Bodha's own docs and should become an explicit External Context reference before implementation.

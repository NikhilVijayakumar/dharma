# Proposal: Kriti — Bootstrap Domain System (Verify Domain System, Provision Capability, Verify Agent System)

> Status: Draft — design-only, no Kriti content changed yet. Conforms to `docs/raw/proposal.md` standard.
> **Status interpretation:** the single header status is a document-level label only; actual readiness/finalization is tracked per Epic in Lifecycle below.
> **Scope note:** like `docs/proposal/Kriti/01-analyse-concern-split-and-agent-system-evaluation.md`, this document lives in `dharma` (`docs/proposal/Kriti/`) but specifies work for the Kriti repository — a separate git repo this proposal does not write into.

## Purpose

Dharma's "verify a Domain System / find the Agents and Skills to execute it / verify an Agent System" workflow exists today only as prose in Kriti's `SYSTEM.md` files (the Scenario A/B pipeline description) — it has never been expressed as real `epic`/`usecase`/`task`/`task_step` rows, the exact data model Dharma already runs every consumer Domain System through (`docs/proposal/archive/02-task-model.md`). This proposal defines a **bootstrap Domain System**, authored and owned by Kriti like every other Domain System (proposal 05's "selected not authored"), whose Epics/Usecases/Tasks *are* that workflow — so Dharma's own self-analysis runs through the identical Task/Agent/Skill/Handoff machinery a user's own project would, with no special-cased Rust code path for "analyzing ourselves."

It covers three Epics, one per concern already split in `docs/proposal/16-agent-system-concern-split-and-release-bundling.md`:

1. **Verify a Domain System** — structural checks (does the declared tree exist, is it well-formed) *and* deep/qualitative analysis (gaps, improvement points, optimization) — routed to `domain-system-evaluation`.
2. **Provision capability for a Domain System** — map its Tasks to Agents/Skills, report gaps — routed to `capability-provisioning`.
3. **Verify an Agent System** — the same structural-and-deep-analysis treatment, but for an Agent System as the artifact — routed to `agent-system-evaluation`.

A key finding shapes Epic 1's design: Kriti already has mature, real audit content for exactly "deep analysis / gaps / improvement / optimization" of a Domain System — `Kriti/samgraha/system/dev/base_dev/audit/{deterministic,semantic}/{section,document}/` — and Dharma already has a schema built specifically to consume it (`audit_definition`/`audit_rule`/`audit_semantic`/`audit_calculation`/`audit_weights`/`audit_template`, `schema/mcp/20-25`). Epic 1's "deep analysis" Usecases should route through *that* existing mechanism, not reinvent scoring as more Task content. Epic 3 has no equivalent: `audit_definition` scopes only to `domain_system_id` (`schema/mcp/20-audit_definition.sql:11`, no `agent_system_id` column) — Dharma has no audit apparatus for Agent Systems today. This proposal names that gap rather than working around it silently (see Constraints).

## System Overview

### Overview

Three things verified directly (2026-08-06) ground this proposal:

1. **The bootstrap workflow itself has never been formalized as data — even though the format it needs already has a real precedent.** Kriti's `dharma/agent/system/{capability-provisioning,domain-system-evaluation}/SYSTEM.md` describe the Scenario A/B pipeline in prose only (`## Scenario B pipeline`, a text diagram); no `epic`/`usecase`/`task` content exists anywhere for *this* workflow (verify/provision/verify). This is different from claiming Kriti has no epic/usecase/task content at all — it does: `dharma/domain/system/dev/rust_dev/task.yaml:555-580` has a real `epics: [{name, objective, sort_order, usecases: [{name, description, sort_order, tasks: [{name, description, input_contract, output_contract, acceptance_criteria}]}]}]` tree, for `rust_dev`'s own documentation-generation domain (`propose-and-generate-vision`, `reconcile-vision`, etc.) — unrelated to this proposal's workflow, but a real file-shape precedent (see Data Flow). One thing that shape does *not* demonstrate: `task_step`/`required_capability` appears nowhere in `rust_dev/task.yaml` (verified — zero matches). This proposal's design depends on `task_step.required_capability` to route Tasks to the three concerns; the bootstrap content would be the first real Kriti content to use it, not an established pattern (`issue/domain-system/001` already covers why none of this reaches `mcp.db`'s structured tables regardless).
2. **Rich audit content already exists, but only on the `samgraha/` side, and isn't captured by anything.** `Kriti/samgraha/system/dev/base_dev/audit/deterministic/section/12-qa/02-unit_testing.yaml` is a real file: `id`, `description`, `condition`, `message`, `severity`, `weight`, `mandatory`, `evidence.type` — mapping near-exactly onto `audit_rule`'s columns. `Kriti/dharma/domain/system/dev/rust_dev/` (the copy Dharma already registers as `rust_dev`) has no `audit/` folder — that content was never projected across. Separately, `crates/services/src/audit.rs` (verified) has functions only for *running* audits (`start_audit_run`/`complete_audit_run`/`get_audit_run`/`override_audit_result`) — nothing that parses provider audit content into `audit_definition`/`audit_rule`/`audit_semantic`. This is the same class of gap as `issue/agentic-system/001` (capture without parse), one level further down the stack.
3. **No equivalent audit mechanism exists for Agent Systems.** `audit_definition` (`schema/mcp/20-audit_definition.sql`) has `domain_system_id NOT NULL` and an optional `domain_id`; there is no `agent_system_id` anywhere in the audit tables (20-25). Whatever "deep analysis of an Agent System" means today, it cannot be a weighted/scored audit — only Task/Skill-based heuristic checks (the `agent-system-evaluation` Skills already specified in proposal 16's Required Content Shape: binding-completeness checks, pass/fail-with-reasons reports).

### Structural Approach

Two content types, not one: (a) the bootstrap Domain System's `epic`/`usecase`/`task`/`task_step` tree (all three Epics), authored fresh in Kriti; (b) for Epic 1 only, projecting Kriti's existing `base_dev/audit/` content into the `dharma/domain/system/dev/rust_dev/` folder Dharma already registers, plus a new Dharma-side capture/parse step for it (out of this document's scope — Kriti-side work only; the Dharma-side parse step is a dependency, tracked separately, same pattern as proposals 16/Kriti-01).

### Diagram

```text
Kriti (new bootstrap Domain System, name TBD e.g. "dharma-bootstrap"):
  epic: Verify a Domain System         ──required_capability──▶ domain-system-evaluation
    usecase: Structural verification     (existing Kriti agents: domain-system-verifier, domain-verifier, hierarchy-verifier, section-verifier)
    usecase: Deep analysis / gaps / optimization  ──routes through──▶ audit_definition/audit_rule/audit_semantic
                                                                        (sourced from Kriti's existing samgraha/system/dev/base_dev/audit/)
  epic: Provision capability for a Domain System  ──required_capability──▶ capability-provisioning
    usecase: Map tasks to capabilities, assign, design workflow, report gaps
  epic: Verify an Agent System          ──required_capability──▶ agent-system-evaluation
    usecase: Structural + heuristic checks only — no audit_definition equivalent exists (gap, see Constraints)
```

## Component Model

### Epic: Verify a Domain System
- **Responsibility:** Given a target Domain System, produce both a structural verification report and a deep-analysis (gap/improvement/optimization) score.
- **Ownership:**
  - **Usecase: Structural verification** — Tasks/task_steps naming `required_capability = "domain-system-evaluation"`, executed by the existing (already split, already verified clean) `domain-system-verifier`/`domain-verifier`/`hierarchy-verifier`/`section-verifier` chain. This is the formalization of the existing Scenario A prose pipeline — no new Agent/Skill content needed, only the Task/Usecase/task_step rows that name it.
  - **Usecase: Deep analysis / gaps / improvement / optimization** — routes to Dharma's existing Audit subsystem rather than new Task content, generically: its Task takes `repo_path`/`commit_hash` as input (Communication) and calls `run_audit` per domain in whatever Domain System that repo is registered against — it is not written against `rust_dev` specifically. `rust_dev` is named here only because it is the *only* Domain System with real audit content available to project today (`Kriti/samgraha/system/dev/base_dev/audit/{deterministic,semantic}/` → `Kriti/dharma/domain/system/dev/rust_dev/audit/`, mirroring however the rest of `rust_dev`'s projection from `base_dev` already happened). A repo registered against any other Domain System runs the identical Usecase; it just finds no `audit_definition` yet (Security, "fail loudly, don't silently no-op") until that Domain System gets equivalent audit content of its own.
- **Interfaces:** The structural Usecase's Tasks are ordinary `run_skill`-driven Task Instances (proposal 07's Proposal & Execution Protocol); the deep-analysis Usecase's Tasks invoke `run_audit`/`get_audit_result` (existing MCP tools) once the Dharma-side audit-content parse step exists.

### Epic: Provision capability for a Domain System
- **Responsibility:** Given a target Domain System, map every Task to a capable Agent/Skill, design the workflow/handoff chain, report gaps.
- **Ownership:** Tasks/task_steps naming `required_capability = "capability-provisioning"`, executed by the existing `capability-analyser` → `assignment-planner` → `workflow-designer` → `gap-analyser` → `orchestrator` chain (already split, already verified clean, per `docs/proposal/Kriti/01-...md`).
- **Interfaces:** Formalizes the existing Scenario B prose pipeline as real Task rows — no new Agent/Skill content needed.

### Epic: Verify an Agent System
- **Responsibility:** Given a target Agent System, judge its Agents/Skills/bindings for completeness — coverage of its declared concern, no orphaned Skills, bindings that resolve.
- **Ownership:** Tasks/task_steps naming `required_capability = "agent-system-evaluation"`, executed by whatever Agents/Skills that (still unauthored, per proposal 16's Required Content Shape) concern eventually provides.
- **Interfaces:** Structural/heuristic only — **no** `run_audit` path, since `audit_definition` has no `agent_system_id` (see Constraints, "Agent System Audit Gap").

### Component Diagram

```text
Bootstrap Domain System (Kriti-owned)
    ├─ Epic: Verify a Domain System
    │    ├─ Usecase: structural  ──Tasks──▶ domain-system-evaluation Agents (existing)
    │    └─ Usecase: deep analysis  ──Tasks──▶ run_audit ──▶ audit_definition (needs: audit content capture step + base_dev/audit/ projected into rust_dev/)
    ├─ Epic: Provision capability
    │    └─ Tasks ──▶ capability-provisioning Agents (existing)
    └─ Epic: Verify an Agent System
         └─ Tasks ──▶ agent-system-evaluation Agents (not yet authored, proposal 16)
```

## Communication

### Communication Paths

**Task Runtime → Agent (any Epic)**
- **Pattern:** Unchanged mechanism, proposal 07's Proposal & Execution Protocol — a Task's `task_step.required_capability` resolves to an Agent System by concern, then to a specific Agent via `agent_skill_binding`.
- **Contract:** Identical whether the target being analyzed is Dharma's own bootstrap content or a consumer's Domain System — the Task Runtime does not distinguish.

**Deep-analysis Usecase → Audit subsystem**
- **Pattern:** Synchronous, via existing MCP tools `run_audit`/`get_audit_result`.
- **Contract:** `tool_run_audit` (`crates/mcp/src/adapter.rs:812-840`, verified) does not take a Domain System name — it requires `repo_path` (an already-`approved` registered repo, `require_approved_registration`), `commit_hash`, `domain` (one specific domain's name within whatever Domain System that repo is registered against — not a domain-system-wide call), and `kind` (`'deterministic'` or `'semantic'`, `audit_definition.kind`'s two values, called separately). It resolves `domain_id` by joining `repo_registration.domain_system_id` (from `repo_path`'s own registration) against `domain.name = domain`. This proposal's Task therefore does **not** target "a Domain System" in the abstract — it targets whichever repo is passed as the Task's own `input_contract` `repo_path`/`commit_hash`, one call per (domain, kind) pair in that repo's registered Domain System, supplied by whoever executes the Task Instance (the same way any other Task's `input_contract` is populated at assignment time — nothing bootstrap-specific). This is what keeps the workflow generic: it runs identically for any repo registered against any Domain System. It only *produces a non-empty result* today for a repo registered against `rust_dev` (see System Overview and Data Flow) — any other target correctly reports "no `audit_definition` found" (Security) rather than a hardcoded assumption that `rust_dev` is the only valid target.

### Communication Diagram

```text
Task Runtime → capability-provisioning/domain-system-evaluation/agent-system-evaluation : resolve(required_capability) → Agent
Deep-analysis Task → run_audit(repo_path, commit_hash, domain, kind) : per (domain, kind) — deterministic + semantic scores, merged
```

## Data Flow

### Data Paths

**Bootstrap Content Path**
- **Entry point:** Kriti authors the bootstrap Domain System's `epic`/`usecase`/`task` YAML in the shape `rust_dev/task.yaml:555-580` already demonstrates (`epics: [{name, objective, sort_order, usecases: [{name, description, sort_order, tasks: [{name, description, input_contract, output_contract, acceptance_criteria}]}]}]`) — this document does not invent a new file format for that part. `task_step`/`required_capability` has no existing precedent to follow (see System Overview) — Kriti will need to define how a `task_step` entry is expressed in this file shape, since no current content shows it.
- **Transformations:** Captured into `content_asset` by the existing `capture_bundle` mechanism (unchanged); structured `domain`/`epic`/`usecase`/`task`/`task_step` rows depend on the still-open parse gap (`issue/domain-system/001`) — this proposal's content is bounded by that gap exactly like `rust_dev`'s content already is.
- **Exit point:** A registered Domain System (name TBD, e.g. `dharma-bootstrap`) whose Epics drive the same three concerns already split.

**Audit Content Path** (Epic 1's deep-analysis Usecase only)
- **Entry point:** `Kriti/samgraha/system/dev/base_dev/audit/` — existing, real, already-authored content.
- **Transformations:** Projected into `Kriti/dharma/domain/system/dev/rust_dev/audit/` (Kriti-side work, mirroring the existing `base_dev`→`rust_dev` projection); captured by Dharma via a **new** parse step (`content_asset` → `audit_definition`/`audit_rule`/`audit_semantic`/`audit_calculation`/`audit_weights`/`audit_template`) that does not exist today — Dharma-side, out of this document's scope, tracked as a dependency.
- **Exit point:** A queryable `audit_definition` for `rust_dev` — the first Domain System to have one, not the only one the mechanism supports. Any other Domain System gets the same `run_audit` path once someone projects/authors equivalent audit content for it; nothing in the Usecase's Task or in `run_audit` itself names `rust_dev`.

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Bootstrap Domain System content (epic/usecase/task/task_step YAML) | Kriti, authored fresh |
| `base_dev/audit/` content | Kriti, already exists — projected, not re-authored |
| The still-missing `content_asset`→structured-row parse steps (domain tree, agent/skill, audit content) | Dharma, tracked dependencies this proposal's content is bounded by |

## Security

### Trust Boundaries

- Unchanged from proposals 05/16 — Kriti-authored content, captured and structurally validated before use; no new privilege granted by this document.

### Threat Model

- **Deep-analysis Usecase silently no-ops if the audit capture step never ships:** Mitigation — the Usecase's Task should fail loudly (an `audit_definition` not found for the target) rather than reporting a false "no findings" pass, mirroring the Release Bundling Step's own "fail loudly on a real error, skip only on genuinely absent content" rule (proposal 16, Security).

## Lifecycle

> Status: draft
> Draft commit: not yet committed
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means, per Epic — deliberately not one blanket criterion, since Epic 3 depends on content that does not exist yet and this document must not read as implementable end-to-end when it isn't:

- **Epic 1 (Verify a Domain System) and Epic 2 (Provision capability):** content authored and end-to-end operable — Tasks resolve `required_capability` to `domain-system-evaluation`/`capability-provisioning` and reach agents that already exist and are already verified clean (`docs/proposal/Kriti/01-...md`). Epic 1's deep-analysis Usecase additionally requires `base_dev/audit/` projected into `rust_dev/audit/` and the Dharma-side audit-content parse step to exist.
- **Epic 3 (Verify an Agent System):** content authored (Tasks exist, `required_capability = "agent-system-evaluation"`) is **not sufficient** to call this Epic finalized — `agent-system-evaluation` has no Agents/Skills yet (proposal 16). Epic 3 is only finalized once that concern's content exists and a Task actually resolves to a real Agent. Until then, this document's own status may read "draft" or "finalized" for Epics 1-2 while Epic 3 stays explicitly "content authored, not operable" — that distinction must be stated wherever this proposal's status is reported, not collapsed into one status line.

## Rationale

### Route Domain-System Deep Analysis Through the Existing Audit Subsystem, Not New Task Content
- **Context:** "Deep analysis, gaps, improvement points, optimization" could be built either as more Agent/Skill/Task content, or as Audit content — Dharma already has both mechanisms.
- **Decision:** Use the Audit subsystem (`audit_definition`/`audit_rule`/`audit_semantic`/`audit_calculation`/`audit_weights`/`audit_template`) — it already exists, is purpose-built for exactly this (weighted deterministic rules + semantic per-model scoring + merge formulas), and Kriti already has mature real content for it (`base_dev/audit/`) that has simply never been projected or captured.
- **Alternatives Considered:** Model "deep analysis" as more Usecases/Tasks/Skills under `domain-system-evaluation`, duplicating what Audit already does.
- **Rejection Reason:** Would reinvent weighted scoring, severity/mandatory failure policy, and report templating that already exist as schema and as real Kriti content — pure duplication for no benefit.
- **Architectural Goal:** Reuse validated, already-built mechanism; only add what's genuinely missing (the capture/parse step and the projection).

## Constraints

### Hard Constraints
- **Agent System Audit Gap:** `audit_definition` has no `agent_system_id` column (`schema/mcp/20-audit_definition.sql`) — Epic 3 ("Verify an Agent System") cannot use `run_audit`/weighted scoring today. Its deep-analysis Usecase, if authored, must stay Task/Skill-based (heuristic checks, pass/fail-with-reasons — proposal 16's Required Content Shape), not claim audit-grade scoring it cannot back. A Dharma-side schema change to add agent-system-scoped audits is a separate, future proposal — not in scope here.
- This document's structured content is bounded by three separate, still-open Dharma-side parse gaps (domain tree, agent/skill, audit content) — none of which this proposal implements.

### Soft Constraints
- Prefer the bootstrap Domain System's name make clear it's Dharma's own self-analysis content, not a consumer-facing standard (e.g. `dharma-bootstrap`, not something that could be mistaken for a general-purpose domain like `rust-dev-domain`).

## Traceability

### Derivation Chain

```text
Task/Epic/Usecase Model (02) — the data model this document formalizes the workflow into
Kriti — analyse/ Concern Split (Kriti/01) — the three concerns this bootstrap content routes to
    │
    ▼
Kriti — Bootstrap Domain System & Audit Content (this document)
    │
    ▼
Kriti: new bootstrap Domain System content; base_dev/audit/ → rust_dev/audit/ projection
Dharma (tracked dependencies, not this document's scope): domain-tree parse step, agent/skill parse step, audit-content parse step
```

### Non-Contradiction Rule

No downstream Kriti or Dharma change may claim Agent System audit scoring without first resolving the Agent System Audit Gap (Constraints), or reinvent Audit-subsystem functionality as new Task/Skill content for a Domain System that already has real audit content available.

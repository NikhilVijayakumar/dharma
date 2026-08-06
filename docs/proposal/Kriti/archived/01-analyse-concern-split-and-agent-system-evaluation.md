# Proposal: Kriti — `analyse` Concern Split & `agent-system-evaluation` Authoring

> Status: Draft — design-only, no Kriti content changed yet. Conforms to `docs/raw/proposal.md` standard.
> **Scope note:** this document lives in the `dharma` repository (`docs/proposal/Kriti/`) but specifies work for the **Kriti** repository (`/home/dell/PycharmProjects/Kriti`) — Kriti is a separate git repository this proposal does not write into. It exists so Dharma's proposal 16 (`docs/proposal/16-agent-system-concern-split-and-release-bundling.md`) has a concrete, reviewable specification of the precondition it depends on, without dharma authoring or owning Kriti's content (proposal 16, "selected not authored").

## Purpose

Dharma's proposal 16 requires two things from Kriti (or a successor provider) that do not exist yet:

1. Kriti's combined `dharma/agent/system/analyse/` folder — one Agent System serving two concerns — split into `capability-provisioning/` and `domain-system-evaluation/`, each a self-contained Agent System folder.
2. A new `agent-system-evaluation/` folder, authored from scratch, judging whether a registered Agent System's own Agents/Skills/bindings are complete and good.

This document specifies exactly what changes in Kriti's existing 9 Agents / 13 Skills to make (1) true, and what minimum content (2) requires — precise enough for a Kriti-repository author to implement directly.

## System Overview

### Overview

Verified directly against `Kriti/dharma/agent/system/analyse/` (2026-08-06): `SYSTEM.md` declares `concern = "analysis"` and two Scenarios — A (verify the Domain System) and B (provision the agent capability) — in one Agent System. Its 9 Agents split cleanly along that line **except** for two agents whose Skill lists cross both scenarios:

- **`orchestrator`** (skills: `analyse-domain-system`, `render-verification-report`, `map-task-to-capability`, `render-provisioning-report`, `verify-epic-completion`) — by role, literally coordinates *both* scenarios: enumerate/dispatch/merge/report for A, then sequence/merge/emit for B.
- **`workflow-designer`** (skills: `propose-agent-assignment`, `design-handoff-workflow`, `verify-task-completion`, `verify-usecase-completion`, `verify-epic-completion`) — a Scenario B agent whose goal 3 ("pair every level with its verifier") currently holds three Scenario-A verification Skills directly.

This is not just a style problem. `schema/mcp/19-agent_skill_binding.sql`'s own header states the invariant this violates once split: *"the Agent-Management Agent System must only ever pair an Agent with a Skill from the SAME `agent_system_id`."* Once `capability-provisioning` and `domain-system-evaluation` are two separate registrations (two separate `agent_system_id` values), neither `orchestrator` nor `workflow-designer` can keep a Skill list that spans both — the binding would be structurally invalid, not just conceptually messy. Every other Agent (`capability-analyser`, `assignment-planner`, `gap-analyser` on the provisioning side; `domain-system-verifier`, `domain-verifier`, `hierarchy-verifier`, `section-verifier` on the evaluation side) is already single-concern as captured today and needs no Skill-list change.

### Structural Approach

Two edits (trim `orchestrator` and `workflow-designer`), one role reassignment (`render-verification-report` moves from `orchestrator` to `domain-system-verifier`, which already aggregates the system-level verdict), and one new folder (`agent-system-evaluation/`, content authored fresh — Dharma's proposal 16 does not require this folder to exist before its own Release Bundling Step ships; that entry simply stays skipped until it does).

### Diagram

```text
Before: Kriti/dharma/agent/system/analyse/
  agent/  {assignment-planner, capability-analyser, domain-system-verifier,
           domain-verifier, gap-analyser, hierarchy-verifier, orchestrator,
           section-verifier, workflow-designer}          (9, 2 cross-concern)
  skill/  {13 skills, one bag}

After:
  Kriti/dharma/agent/system/capability-provisioning/
    agent/ {assignment-planner, capability-analyser, gap-analyser,
             orchestrator (trimmed), workflow-designer (trimmed)}   (5)
    skill/ {map-task-to-capability, propose-agent-assignment,
             identify-agent-gaps, design-handoff-workflow,
             render-provisioning-report}                            (5)

  Kriti/dharma/agent/system/domain-system-evaluation/
    agent/ {domain-system-verifier (extended), domain-verifier,
             hierarchy-verifier, section-verifier}                  (4)
    skill/ {verify-epic-usecase-task, analyse-domain-system,
             render-verification-report, verify-section-map,
             verify-section-profile, verify-task-completion,
             verify-usecase-completion, verify-epic-completion}     (8)

  Kriti/dharma/agent/system/agent-system-evaluation/   (new, authored per Required Content Shape below)
```

## Component Model

### `capability-provisioning/` folder
- **Responsibility:** Scenario B only — find every Agent/Skill able to execute a Domain System's Tasks, assign, design workflow/handoff, report gaps.
- **Ownership:** `assignment-planner`, `capability-analyser`, `gap-analyser` — unchanged, verified already single-concern.
  - `workflow-designer` — **trim** `skills:` from `[propose-agent-assignment, design-handoff-workflow, verify-task-completion, verify-usecase-completion, verify-epic-completion]` to `[propose-agent-assignment, design-handoff-workflow]`. Reword goal 3 ("Pair every level with its verifier") from *performing* verification to *declaring* the requirement: the workflow names which level needs a verifier and leaves the verification act to `domain-system-evaluation`'s own Agents, reached at Task-execution time via `task_step.required_capability` naming that concern — never via a direct Skill binding, since that binding could not resolve across two `agent_system_id`s.
  - `orchestrator` — **trim and rescope** to Scenario B only. `skills:` becomes `[map-task-to-capability, render-provisioning-report]` (drop `analyse-domain-system`, `render-verification-report`, `verify-epic-completion`). Role text drops the original goals 1-4 (Scenario A enumerate/dispatch/merge/report) entirely, keeps goals 5-8 (renumbered 1-4: sequence the B pipeline, merge per-epic blueprints, emit the blueprint) but rewords goal 8 ("hand the element list, findings, and gaps forward between the two scenarios") — Scenario A's gap findings now arrive as this Agent System's external Task input (from a separate registration), not an internal handoff between two scenarios of one Agent System. `handoff_candidate_role` changes from `domain-system-verifier` (now in the other folder — cross-concern, invalid) to empty (`""`), matching the existing pattern `hierarchy-verifier` already uses for "nothing further to hand off to within this concern."
- **Interfaces:** Registered as `capability-provisioning`/concern `capability-provisioning` (matches Dharma proposal 16's Component Model and `config/providers/agent-capability-provisioning.toml`).

### `domain-system-evaluation/` folder
- **Responsibility:** Scenario A only — verify a Domain System end to end and render the verification report.
- **Ownership:** `domain-verifier`, `hierarchy-verifier`, `section-verifier` — unchanged, verified already single-concern.
  - `domain-system-verifier` — **extend**, absorbing the reporting/top-level-coordination role `orchestrator` no longer covers on this side (it already aggregates per-domain verdicts into the system-level verdict — the natural owner, rather than cloning a second `orchestrator`-named agent into this folder). Add `render-verification-report` to `skills:` (now `[verify-epic-usecase-task, analyse-domain-system, render-verification-report]`). Role text gains the enumerate/dispatch responsibility the original `orchestrator` goals 1-4 covered, scoped to Scenario A only.
- **Interfaces:** Registered as `domain-system-evaluation`/concern `domain-system-evaluation` (matches Dharma proposal 16's Component Model and `config/providers/agent-domain-system-evaluation.toml`).

### `agent-system-evaluation/` folder (new)
- **Responsibility:** Per Dharma proposal 16's Required Content Shape — judge a registered Agent System's own Agents/Skills/bindings for completeness and quality.
- **Ownership:** Authored fresh in Kriti; not derived from `analyse/`'s existing content (no existing Agent or Skill here judges an Agent System as an artifact — everything in `analyse/` today judges a *Domain* System, or provisions capability *for* one). Minimum content, per proposal 16: at least one Agent whose role is judging another Agent System's completeness against its declared `concern`; at least one Skill checking Agent↔Skill binding completeness; at least one Skill rendering a pass/fail-with-reasons report.
- **Interfaces:** Registered as `agent-system-evaluation`/concern `agent-system-evaluation`, matching `config/providers/agent-agent-system-evaluation.toml` — that entry stays skipped by Dharma's Release Bundling Step until this folder exists.

### Component Diagram

```text
capability-provisioning/orchestrator (trimmed)
    ├─ capability-analyser ──map-task-to-capability──▶ assignment-planner
    │                                                       │
    │                        ──propose-agent-assignment──▶ workflow-designer (trimmed)
    │                                                       │  ──design-handoff-workflow──▶
    │                                                       ▼
    │                                                 gap-analyser ──identify-agent-gaps──▶
    └────────────────────────render-provisioning-report──────────────────────────────────▶ blueprint

domain-system-evaluation/domain-system-verifier (extended)
    ├─ domain-verifier ──▶ section-verifier ──▶ hierarchy-verifier   (unchanged chain)
    └──render-verification-report──▶ verification report
```

## Communication

### Communication Paths

**Scenario A output → Scenario B input** (unchanged relationship, now cross-Agent-System instead of cross-Scenario-within-one-system)
- **Pattern:** Asynchronous — Scenario A's gap findings (unresolved `required_capability`, empty Task contracts, 0-Task usecases) feed Scenario B's starting input, as `SYSTEM.md` already documents.
- **Contract:** Unchanged in substance; the mechanism by which the finding set crosses from `domain-system-evaluation` to `capability-provisioning` is now the same cross-Agent-System Task/handoff mechanism Dharma's Handoff Broker already provides (proposal 04/07), not an internal handoff between two scenarios of one registration.

### Communication Diagram

```text
domain-system-evaluation (Agent System) → capability-provisioning (Agent System) : gap findings (via Task input, not intra-system handoff)
```

## Data Flow

### Data Paths

**Split Path**
- **Entry point:** `Kriti/dharma/agent/system/analyse/{agent,skill}/`.
- **Transformations:** Files partitioned per Component Model above; `workflow-designer` and `orchestrator` edited (not just moved) per the trims specified; `render-verification-report` reassigned from `orchestrator` to `domain-system-verifier`; each folder gets its own `SYSTEM.md` (see Constraints).
- **Ownership boundary:** Entirely inside Kriti; Dharma never receives or stores these source files, only whatever it captures via `content_root` once Kriti's split lands (proposal 16).
- **Exit point:** Two self-contained folders, each independently registrable as a Dharma Agent System with no cross-folder Skill reference remaining.

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| All Agent/Skill source content (both existing and new `agent-system-evaluation/`) | Kriti, entirely |
| The resulting `content_asset`/`agent`/`skill` rows once captured | Dharma's `mcp.db`, per proposal 16 — out of this document's scope |

## Security

### Trust Boundaries

- Unchanged from Dharma proposal 16 — this document only reshapes Kriti's own content; it grants no new privilege and does not touch `is_privileged`/`is_privileged_request` on any of the three concerns.

### Threat Model

- **A trimmed `workflow-designer` silently drops verification instead of deferring it:** Mitigation — the reworded goal 3 must explicitly state the workflow *names* the required verifier role per level (task/usecase/epic) without performing verification itself; an implementer omitting that naming step would silently reintroduce the "verifier is the worker" failure mode `SYSTEM.md`'s own Completion/verification semantics section already forbids.

## Lifecycle

> Status: draft
> Draft commit: not yet committed
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means: `capability-provisioning/` and `domain-system-evaluation/` exist as separate folders in Kriti with the exact Agent/Skill membership and edits specified in Component Model above, each with its own `SYSTEM.md` (`id`/`concern`/`scenarios` updated per Constraints), and no Agent in either folder lists a Skill that lives in the other folder.

## Rationale

### Reuse `domain-system-verifier` Rather Than Cloning a Second `orchestrator`
- **Context:** Splitting `orchestrator`'s two-scenario role naively suggests two `orchestrator`-named agents, one per folder.
- **Decision:** Only `capability-provisioning` keeps an `orchestrator`; `domain-system-evaluation` extends its existing system-level aggregator (`domain-system-verifier`) with the enumerate/dispatch/report responsibility instead.
- **Alternatives Considered:** Clone `orchestrator` into both folders, trimmed differently in each.
- **Rejection Reason:** `domain-system-verifier` already does the system-level aggregation `orchestrator`'s Scenario-A goals required; a second orchestrator-shaped agent would duplicate that responsibility rather than extend it, adding an agent with no distinct job.
- **Architectural Goal:** Minimum agent count per folder that still covers every original responsibility exactly once.

## Constraints

### Hard Constraints
- No Agent in `capability-provisioning/` or `domain-system-evaluation/` may list a Skill filed under the other folder (source: `schema/mcp/19-agent_skill_binding.sql`'s same-`agent_system_id` invariant, System Overview above).
- Each split folder needs its own `SYSTEM.md`: `capability-provisioning/SYSTEM.md` sets `id = "capability-provisioning"`, `concern = "capability-provisioning"`, `scenarios = ["provision-agent-capability"]`; `domain-system-evaluation/SYSTEM.md` sets `id = "domain-system-evaluation"`, `concern = "domain-system-evaluation"`, `scenarios = ["verify-domain-system"]` — each drops the other's Scenario from its own file and its own "Scenario B pipeline"/"Scenario A" prose sections accordingly.
- `agent-system-evaluation/` content must satisfy Dharma proposal 16's "Required Content Shape" (Component Model, "`agent-system-evaluation` Agent System" section) — this document does not restate that spec, only points to it.

### Soft Constraints
- Prefer keeping every unchanged Agent's `handoff_candidate_role` pointing within its own folder as already verified today (`assignment-planner`→`gap-analyser`, `capability-analyser`→`assignment-planner`, `gap-analyser`→`orchestrator`, `section-verifier`→`hierarchy-verifier`, `domain-verifier`→`section-verifier`) — only `orchestrator`'s and `workflow-designer`'s cross-folder pointers need review (see Component Model).

## Traceability

### Derivation Chain

```text
Dharma proposal 16 (Agent System Concern Split & Release Bundling) — the precondition this document satisfies
    │
    ▼
Kriti — analyse/ concern split & agent-system-evaluation/ authoring (this document)
    │
    ▼
config/providers/agent-capability-provisioning.toml, agent-domain-system-evaluation.toml,
agent-agent-system-evaluation.toml (dharma repo) — start resolving content once this lands
```

### Non-Contradiction Rule

No downstream Kriti change may reintroduce a Skill reference crossing `capability-provisioning`/`domain-system-evaluation`/`agent-system-evaluation`, or let one of these three folders self-certify another's completeness (Dharma proposal 16's no-self-certification constraint applies identically here).

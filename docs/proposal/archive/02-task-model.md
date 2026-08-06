# Proposal: Epic / Usecase / Task Model

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Supersedes** the flat Task Model in the original draft of this document: Task is no longer a standalone top-level entity. It is now the innermost unit of a three-tier hierarchy — Epic → Usecase → Task — owned and defined by a Domain System (see Domain System Registration proposal), not authored per repository.

## Purpose

This document defines the standard for the Epic, Usecase, and Task entities within Dharma's agent-based execution architecture. Together they describe the full unit-of-work hierarchy that a Domain System defines once and every repository registered against that Domain System inherits.

Unlike Samgraha, where a Standard defines Usecases directly (a two-tier model: Usecase → Step), Dharma uses a three-tier hierarchy: an Epic groups related Usecases under one domain-wide objective, a Usecase groups related Tasks under one user-facing capability, and a Task remains the executable unit with an explicit input/output contract, an ordered step sequence, and acceptance criteria spanning happy path, corner case, and edge case.

## System Overview

### Overview

A Domain System (e.g. `rust-dev-domain`, `electron-dev-domain`) defines the complete Epic set for its domain. Each Epic defines the Usecases that realize it. Each Usecase defines the Tasks that realize it. A Task carries the same contract shape as before — name, description, Input Contract, Output Contract, Step Sequence, Acceptance Criteria Set — but its existence and scope are always traced upward to a Usecase and an Epic, and both are supplied by the chosen Domain System rather than authored ad hoc per repository.

### Structural Approach

The hierarchy is strictly one-directional for definition (Epic defines Usecase defines Task) and strictly one-directional for execution (a Task is what actually runs; Epic and Usecase are structural/organizational, not executable). A repository does not invent its own Epics — it inherits the Epic/Usecase/Task set of the Domain System it registers against (see MCP Registration & Bootstrap proposal).

### Diagram

```text
Domain System (e.g. rust-dev-domain)
   │
   ▼
 Epic  ──────────────────────────────────┐  (domain-wide objective)
   │                                     │
   ▼                                     ▼
Usecase ── Usecase ── Usecase        Usecase  (user-facing capability)
   │           │
   ▼           ▼
 Task        Task ── Task              (executable unit)
```

## Component Model

### Epic
- **Responsibility:** States one domain-wide objective the Domain System exists to satisfy (e.g., for `rust-dev-domain`: "produce a reviewable crate architecture"). An Epic may itself contain another Epic, for objectives that decompose into sub-objectives before reaching Usecases.
- **Ownership:** Name, objective statement, an optional parent Epic reference, ordered list of Usecases that realize it.
- **Interfaces:** Read by the Domain System registry when a repository registers; read by the default Agent System during shape analysis (see MCP Registration & Bootstrap proposal).

### Usecase
- **Responsibility:** States one user-facing (or system-facing) capability that contributes to its parent Epic.
- **Ownership:** Name, description, ordered list of Tasks that realize it, reference to parent Epic.
- **Interfaces:** Read by Task Runtime to group related Tasks; read by Report Agent Systems to summarize progress at the capability level rather than the individual-Task level.

### Task
- **Responsibility:** Executable unit of work: explicit input, explicit output, an ordered Step Sequence, and an Acceptance Criteria Set spanning happy path, corner case, and edge case.
- **Ownership:** Input Contract, Output Contract, Step Sequence, Acceptance Criteria Set, reference to parent Usecase, an optional template reference the Domain System suggests as a starting point.
- **Interfaces:** Assigned to an initiating Agent by the Task Runtime; validated by the Completion Validator (see Proposal & Execution Protocol proposal) against its Acceptance Criteria Set. An Agent may substitute one of its own Skill's Templates (see Skill Model proposal) for the Task's suggested template when it judges that a better fit.

### Component Diagram

```text
Domain System ──defines──▶ Epic ──defines──▶ Usecase ──defines──▶ Task
                                                                     │
                                                     Input/Output Contract, Step Sequence,
                                                     Acceptance Criteria (happy/corner/edge)
```

## Communication

### Communication Paths

**Domain System Registry → Repo Registration Record**
- **Pattern:** Synchronous read at registration time.
- **Contract:** The chosen Domain System's Epic/Usecase/Task set is bound to the registering repository (see MCP Registration & Bootstrap proposal); the repository does not modify this set directly.

**Task Runtime → Usecase / Epic**
- **Pattern:** Synchronous read.
- **Contract:** Task Runtime resolves a Task's parent Usecase and Epic to provide progress context to Report Agent Systems, but executes only at the Task level.

### Communication Diagram

```text
Repo → Domain System Registry : register(chosenDomainSystem)
Domain System Registry → Repo Registration Record : bind(epicSet, usecaseSet, taskSet)
Task Runtime → Usecase : resolveParent(task)
Usecase → Epic : resolveParent(usecase)
```

## Data Flow

### Data Paths

**Definition Inheritance Path**
- **Entry point:** Repository selects a Domain System at registration.
- **Transformations:** None — Epic/Usecase/Task definitions are inherited verbatim from the Domain System, not transformed per repository.
- **Ownership boundary:** The Domain System owns the definitions; the repository owns only its Repo Registration Record binding to them.
- **Exit point:** A concrete Task instance, ready for Task Runtime assignment.

**Execution Path**
- **Entry point:** External input conforming to a Task's Input Contract.
- **Transformations:** Step Sequence entries transform input toward the Output Contract shape (see Agent Model, Skill Model, Proposal & Execution Protocol proposals for how).
- **Ownership boundary:** Execution State (per Task) tracks the current owning Agent.
- **Exit point:** Output Contract data, validated by the Completion Validator.

### Data Flow Diagram

```text
Domain System ──inherit──▶ Epic ──▶ Usecase ──▶ Task (Input Contract)
                                                    │
                                          Step Sequence (per step)
                                                    │
                                                    ▼
                                    Task (Output Contract) ──▶ Completion Validator
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Epic, Usecase, Task definitions | Domain System (authored once, reused across repositories) |
| Repo Registration Record | Repository ↔ Domain System binding |
| In-flight Task execution data | Currently-assigned Agent, per Task's Execution State |

## Security

### Trust Boundaries

- **Domain System definitions → Repository:** Trusted, read-only from the repository's perspective — a repository cannot alter an Epic/Usecase/Task definition it inherits, only request a different Domain System or propose a change through the Agent-Management Agent System.
- **External input → Task Input Contract:** Untrusted, validated at Task start as before.

### Threat Model

- **Silent hierarchy drift:** A repository's local tooling caches an outdated Epic/Usecase/Task set after the Domain System is updated upstream. Mitigation: Repo Registration Record stores a version reference to the Domain System; a version mismatch blocks Task assignment until re-synced.
- **Unauthorized Epic/Usecase authoring:** A repository attempts to define its own Epic instead of inheriting one. Mitigation: only the Agent-Management Agent System (see Agent System Registry proposal) may write Epic/Usecase/Task definitions into a Domain System.

## Rationale

### Three-Tier Hierarchy Instead of Flat Task List
- **Context:** A flat list of Tasks per repository does not express which Tasks belong together under one user-facing capability, or which capabilities together satisfy one domain-wide objective.
- **Decision:** Introduce Epic (domain-wide objective) and Usecase (user-facing capability) as structural parents of Task.
- **Alternatives Considered:** Keep Task as the only tier, with a free-text "category" tag.
- **Rejection Reason:** A free-text tag is not queryable or auditable the way an explicit parent reference is, and does not let a Domain System declare its Epic set completely.
- **Architectural Goal:** Traceable, complete domain coverage per Domain System.

### Hierarchy Owned by Domain System, Not Repository
- **Context:** If each repository authored its own Epic/Usecase/Task set, two repositories using the same Domain System (e.g. two Rust services) would duplicate the same structural work.
- **Decision:** Epic/Usecase/Task definitions belong to the Domain System and are inherited by every repository that registers against it.
- **Alternatives Considered:** Per-repository authoring with an optional "import from Domain System" convenience.
- **Rejection Reason:** Making inheritance optional reintroduces the duplication the three-tier model exists to prevent.
- **Architectural Goal:** Reusable domain-shape definitions (see Domain System Registration proposal).

## Constraints

### Hard Constraints
- **Task always has a parent Usecase, Usecase always has a parent Epic** (source: Component Model above) — no orphaned Tasks or Usecases.
- **Definitions inherited, not authored per repository** (source: Rationale above) — a repository cannot introduce a new Epic/Usecase/Task outside the Agent-Management Agent System's write path.
- **Typed Input/Output Contracts and minimum Acceptance Criteria** (carried over from the original Task Model) — unchanged.

### Soft Constraints
- Prefer a Usecase with a small number of Tasks; split a Usecase whose Task list grows large.

## Traceability

### Derivation Chain

```text
Agent Model
    │
    ▼
Epic / Usecase / Task Model (this document)
    │
    ├──▶ Domain System Registration (Epic/Usecase/Task are authored inside a Domain System)
    └──▶ Proposal & Execution Protocol (Tasks are executed via propose-review-approve-execute)
```

### Non-Contradiction Rule

No downstream proposal may let a repository author its own Epic/Usecase/Task set outside a registered Domain System, or let a Task exist without a parent Usecase and Epic, without revising this document first.

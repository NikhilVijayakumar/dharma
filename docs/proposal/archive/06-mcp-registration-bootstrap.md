# Proposal: MCP Registration & Bootstrap

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **New in this revision.** Establishes MCP as the first thing that must exist, and defines the registration sequence: create MCP → register repository → choose a Domain System → default Agent System resolves capability. This document has priority over any conflicting registration flow implied by earlier proposals.
> **Extended by Provider Config & Repo Sync (11):** the repository-side of step 2 is `dharma-repo.toml`; step 6's approval triggers a sync that copies the Domain System in full and the approved Agent Systems filtered into the repository's own `.dharma/` directory. See 11 for the config surface and sync/summary detail this document doesn't itself specify.

## Purpose

This document defines the standard for the MCP platform's registration and bootstrap sequence within Dharma's agent-based execution architecture. It describes the order of existence — MCP first, then repository registration, then Domain System selection, then capability resolution — and the component that performs that resolution.

Unlike Samgraha, where a repository registers and directly becomes (or registers as) a Standard, in Dharma a repository registers with MCP and then selects an existing Domain System; MCP's own default Agent System then determines, through analysis rather than direct execution, which other Agent Systems and Section Profiles the repository's work will draw on.

## System Overview

### Overview

MCP is the platform boundary: it is created first, before any repository exists within the system. Registering a repository is the second step and requires naming a Domain System (see Domain System Registration proposal) to register against. Once bound, MCP invokes its Default/Bootstrap Agent System (see Agent System Registry proposal), which reads the repository and its chosen Domain System's Section Map, Section Profiles, and Epic/Usecase/Task set, and proposes a Capability Manifest — the set of Agent Systems and Section Profiles applicable to that repository.

### Structural Approach

The sequence is strictly ordered and each step is a precondition for the next: MCP existing is a precondition for repository registration; repository registration with a chosen Domain System is a precondition for capability resolution; capability resolution is a precondition for any Task being assigned to that repository. No step may be skipped or reordered.

### Diagram

```text
1. MCP exists
        │
        ▼
2. Repository registers with MCP
        │
        ▼
3. Repository selects a Domain System (e.g. rust-dev-domain)
        │
        ▼
4. Default/Bootstrap Agent System analyzes repo + Domain System
        │
        ▼
5. Proposed Capability Manifest (Agent Systems + Section Profiles)
        │
        ▼
6. Human approval ──▶ Repo Registration Record finalized
        │
        ▼
7. Sync: full Domain System copy + filtered Agent System copy → repository's .dharma/
   (see Provider Config & Repo Sync, 11)
```

## Component Model

### MCP Platform
- **Responsibility:** The root system that holds the Domain System Registry, the Agent System Registry, and every Repo Registration Record.
- **Ownership:** Both registries and all registration records; exists prior to any repository.
- **Interfaces:** Exposes the registration entry point a repository uses to join the system.

### Repository Registration Entry Point
- **Responsibility:** Accepts a repository's request to join MCP and requires a Domain System selection as part of that request.
- **Ownership:** The registration request lifecycle, from submission to either rejection (unregistered Domain System) or a bound Repo Registration Record.
- **Interfaces:** Reads from the Domain System Registry (see Domain System Registration proposal) to validate the selection; writes an initial, unresolved Repo Registration Record.

### Default/Bootstrap Agent System
- **Responsibility:** Analyzes a newly-registered repository and its chosen Domain System to propose which other Agent Systems and Section Profiles apply.
- **Ownership:** Its own analysis Agents/Skills (see Agent System Registry proposal); does not own the repository's or Domain System's content.
- **Interfaces:** Reads the Domain System's Section Map, Section Profiles, and Epic/Usecase/Task set; queries the Agent System Registry for candidates; writes a proposed Capability Manifest.

### Capability Manifest
- **Responsibility:** Lists the Agent Systems and Section Profiles resolved as applicable to a specific repository.
- **Ownership:** Produced by the Default/Bootstrap Agent System; finalized only after human approval.
- **Interfaces:** Read by the Task Runtime and Handoff Broker (see Proposal & Execution Protocol proposal) once approved, to determine which Agent Systems may be assigned to that repository's Tasks.

### Component Diagram

```text
MCP Platform ──hosts──▶ Domain System Registry, Agent System Registry, Repo Registration Records
        ▲
        │ registers(repo, chosenDomainSystem)
   Repository
        │
        ▼
Default/Bootstrap Agent System ──analyzes──▶ Capability Manifest ──(human approval)──▶ Repo Registration Record
```

## Communication

### Communication Paths

**Repository → MCP Platform (Registration Entry Point)**
- **Pattern:** Synchronous request.
- **Contract:** Repository submits itself plus a Domain System name; MCP validates the name against the Domain System Registry and either rejects or creates an initial Repo Registration Record.

**MCP Platform → Default/Bootstrap Agent System**
- **Pattern:** Synchronous invocation, immediately following successful registration.
- **Contract:** MCP hands the bound Domain System's shape (Section Map, Section Profiles, Epic/Usecase/Task set) to the Default/Bootstrap Agent System; it returns a proposed Capability Manifest.

**Default/Bootstrap Agent System → Human Reviewer**
- **Pattern:** Asynchronous, gated.
- **Contract:** The proposed Capability Manifest is presented for review; the reviewer approves, modifies, or rejects it before it is finalized (see Proposal & Execution Protocol proposal for the general propose-review-approve pattern this follows).

### Communication Diagram

```text
Repo → MCP Platform : register(repo, domainSystemName)
MCP Platform → Domain System Registry : validate(domainSystemName)
MCP Platform → Default/Bootstrap Agent System : analyze(boundDomainSystemShape)
Default/Bootstrap Agent System → (human reviewer) : proposedCapabilityManifest
(human reviewer) → MCP Platform : approve | modify | reject
MCP Platform → Repo Registration Record : finalize(approvedManifest)
```

## Data Flow

### Data Paths

**Bootstrap Path**
- **Entry point:** Repository submits a registration request naming a Domain System.
- **Transformations:** Domain System's shape is bound to the repository; Default/Bootstrap Agent System maps that shape to candidate Agent Systems and Section Profiles.
- **Ownership boundary:** MCP owns the registries; the repository owns only its registration request and, once finalized, its Repo Registration Record.
- **Exit point:** A finalized, human-approved Repo Registration Record, ready for Task assignment.

### Data Flow Diagram

```text
Repo ──register(name, domainSystem)──▶ MCP Platform ──▶ Default/Bootstrap Agent System
                                                                  │
                                                     proposed Capability Manifest
                                                                  │
                                                       (human approval) ──▶ Repo Registration Record
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Domain System Registry, Agent System Registry | MCP Platform |
| Registration request | Repository (until accepted) |
| Proposed Capability Manifest | Default/Bootstrap Agent System |
| Finalized Repo Registration Record | MCP Platform, bound to the repository |

## Security

### Trust Boundaries

- **Repository → MCP Registration Entry Point:** Untrusted request — validated against the Domain System Registry before anything is bound.
- **Default/Bootstrap Agent System → Human Reviewer:** Every Capability Manifest is treated as a proposal, never auto-finalized, regardless of what it contains.

### Threat Model

- **Skipping the ordering:** A repository or client attempts to request Task assignment before registration, or before Domain System selection, or before Capability Manifest approval. Mitigation: Task Runtime refuses to assign any Task to a repository without a finalized Repo Registration Record; each precondition in the sequence is enforced structurally, not just by convention.
- **Auto-approval drift:** Operational pressure leads to treating Capability Manifest approval as a rubber stamp. Mitigation: approval naming the Agent-Management Agent System requires explicit justification (carried over from the Agent System Registry proposal); routine manifests may be approved quickly, but the gate itself is never removed.
- **Unregistered Domain System injection:** A registration request supplies Domain System shape content directly instead of a registered name. Mitigation: the Registration Entry Point only accepts a name, resolved against the Domain System Registry (carried over from the Domain System Registration proposal).

## Rationale

### Strict Ordering: MCP, Then Repository, Then Domain System, Then Capability
- **Context:** Samgraha's model lets a repository register and immediately act as a Standard, with capability implicitly bundled in.
- **Decision:** Dharma enforces a strict four-step sequence, with capability resolution always the last, analysis-driven step.
- **Alternatives Considered:** Allow a repository to register and select a Domain System and request specific Agent Systems in a single combined step.
- **Rejection Reason:** Combining the steps removes the analysis checkpoint the Default/Bootstrap Agent System provides, and makes it harder to audit which decision happened at which point.
- **Architectural Goal:** Auditable, ordered bootstrap.

### Capability Resolution Is Analysis, Not Direct Binding
- **Context:** The repository names only a Domain System; it does not directly request specific Agent Systems.
- **Decision:** The Default/Bootstrap Agent System performs the mapping from Domain System shape to candidate Agent Systems, as a proposal.
- **Alternatives Considered:** Let the repository directly list the Agent Systems it wants.
- **Rejection Reason:** Direct listing bypasses the same analysis-then-approval discipline this whole system otherwise requires of every Task (see Proposal & Execution Protocol proposal), and risks a repository requesting capability it does not actually need or should not have.
- **Architectural Goal:** Consistent propose-review-approve discipline, applied even at bootstrap.

## Constraints

### Hard Constraints
- **Strict step ordering** (source: Rationale above) — MCP existence, repository registration, Domain System selection, and Capability Manifest approval must occur in that order with no step skipped.
- **No Task assignment before finalized registration** (source: Threat Model above) — Task Runtime enforces this structurally.
- **Domain System selection by name only** (source: Domain System Registration proposal) — no inline shape content accepted at registration.

### Soft Constraints
- Prefer fast-tracking Capability Manifest approval when it contains only non-privileged, previously-approved-pattern Agent Systems, while still requiring a human action.

## Traceability

### Derivation Chain

```text
Domain System Registration, Agent System Registry
    │
    ▼
MCP Registration & Bootstrap (this document)
    │
    ▼
Proposal & Execution Protocol (Task Runtime uses the finalized Repo Registration Record to assign Tasks)
```

### Non-Contradiction Rule

No downstream proposal may permit Task assignment before a repository's registration sequence (MCP → repository → Domain System → approved Capability Manifest) is fully complete, without revising this document first.

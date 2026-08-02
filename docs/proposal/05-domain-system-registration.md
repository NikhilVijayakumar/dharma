# Proposal: Domain System Registration

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Supersedes** the original Domain Shape Integration draft. A repository does not author a bespoke Domain Definition Set — it **chooses** a registered Domain System (e.g. `rust-dev-domain`, `electron-dev-domain`), the same way it would have chosen a Standard in Samgraha, but the Domain System itself is a reusable registry asset, not a per-repository bundle.
> **Extended by Provider Config & Repo Sync (11):** a Domain System provider declares itself via `dharma-domain.toml`; a consuming repository's selection syncs that Domain System's content in full into its own `.dharma/repo.db` — see 11 for both.

## Purpose

This document defines the standard for the Domain System entity within Dharma's agent-based execution architecture. Domain System Documentation describes how a technology or subject-matter domain (Rust development, Electron development, and so on) is captured once as a reusable, registrable asset that any matching repository can select at registration.

Unlike Samgraha, where each repository registers as its own Standard and must independently supply data for every schema under `knowledge`, a repository in Dharma registers against one existing Domain System. The Domain System — not the repository — owns the Section Map, the Section Profiles, and the Epic/Usecase/Task set (see Epic / Usecase / Task Model proposal). A repository contributes nothing to the Domain System's shape; it only inherits it.

## System Overview

### Overview

A Domain System is a named, registered entry (e.g. `rust-dev-domain`, `electron-dev-domain`) that bundles three things: a Section Map (which sections a document of this domain needs, and why — following the shape of Bodha's `.bodha-structure/section`), a Section Profile per section (what data belongs in that section and how it is filled — following Bodha's `profile-default`), and the full Epic/Usecase/Task set for the domain (see Epic / Usecase / Task Model proposal). A repository registers by selecting one existing Domain System; it does not define a new one itself.

### Structural Approach

Domain Systems live in a registry parallel to the Agent System Registry. Authoring or revising a Domain System is restricted to the Agent-Management Agent System (see Agent System Registry proposal); a repository's only action is selection, at registration time, brokered by MCP (see MCP Registration & Bootstrap proposal).

### Diagram

```text
┌─────────────────────────── MCP Domain System Registry ───────────────────────────┐
│                                                                                    │
│   [ rust-dev-domain        ]     [ electron-dev-domain    ]     [ ... other Domain Systems ]    │
│   Section Map              Section Map                                            │
│   Section Profiles         Section Profiles                                       │
│   Epic/Usecase/Task set    Epic/Usecase/Task set                                  │
│                                                                                    │
└────────────────────────────────────────────────────────────────────────────────────┘
                     ▲
                     │ selects (does not author)
                Repository
```

## Component Model

### Domain System Entry
- **Responsibility:** Names one domain (e.g. `rust-dev-domain`) and bundles its Section Map, Section Profiles, and Epic/Usecase/Task set.
- **Ownership:** All three sub-artifacts below.
- **Interfaces:** Selected, not authored, by a registering repository; read by the default Agent System during shape analysis.

### Section Map
- **Responsibility:** States which sections a document of this domain needs and why each is required, mirroring Bodha's `.bodha-structure/section` map.
- **Ownership:** Section list with purpose/rationale per section.
- **Interfaces:** Read by any Agent System producing or reviewing a document for a repository registered against this Domain System.

### Section Profile
- **Responsibility:** States what data a given section can be filled with and how, one profile per section, mirroring Bodha's `profile-default`.
- **Ownership:** Fill rules per section.
- **Interfaces:** Read by Documentation-concern Agent Systems when producing a proposed solution for a documentation Task.

### Epic/Usecase/Task Set
- **Responsibility:** The complete Epic → Usecase → Task hierarchy for this domain (see Epic / Usecase / Task Model proposal).
- **Ownership:** All Epics, Usecases, and Tasks defined for this Domain System.
- **Interfaces:** Inherited in full by every repository registered against this Domain System; read by Task Runtime to assign work.

### Component Diagram

```text
Domain System Entry ──owns──▶ Section Map
                     ──owns──▶ Section Profile (one per section)
                     ──owns──▶ Epic/Usecase/Task Set
                                          │
                     Repository ──selects──┘ (read-only inheritance)
```

## Communication

### Communication Paths

**Repository → Domain System Registry**
- **Pattern:** Synchronous selection, at registration time.
- **Contract:** Repository submits a chosen Domain System name; Registry returns the bound Section Map, Section Profiles, and Epic/Usecase/Task set, or an error if the name is unregistered.

**Agent-Management Agent System → Domain System Registry**
- **Pattern:** Synchronous write, gated to Agent-Management only.
- **Contract:** Agent-Management submits a new or revised Domain System entry; Registry accepts after validating no conflicting entry exists for the same domain name.

### Communication Diagram

```text
Repo → Domain System Registry : select(domainSystemName)
Domain System Registry → Repo : sectionMap, sectionProfiles, epicUsecaseTaskSet | error(unregistered)
Agent-Management Agent System → Domain System Registry : write(domainSystemEntry)
```

## Data Flow

### Data Paths

**Selection Path**
- **Entry point:** Repository names a Domain System at registration.
- **Transformations:** None — the Domain System's Section Map, Section Profiles, and Epic/Usecase/Task set are inherited verbatim.
- **Ownership boundary:** The Domain System Registry owns all shape content; the repository owns only the fact of its selection (the Repo Registration Record, see MCP Registration & Bootstrap proposal).
- **Exit point:** A bound Repo Registration Record, ready for the default Agent System to run shape analysis against.

### Data Flow Diagram

```text
Repo ──selects(name)──▶ Domain System Registry ──▶ Section Map + Section Profiles + Epic/Usecase/Task Set
                                                          │
                                                          ▼
                                              Repo Registration Record (bound)
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Section Map, Section Profiles, Epic/Usecase/Task set | Domain System Registry (authored by Agent-Management Agent System) |
| Domain System selection | Repo Registration Record |

## Security

### Trust Boundaries

- **Agent-Management Agent System → Domain System Registry:** The only trusted write path.
- **Repository → Domain System Registry:** Read/select-only boundary — a repository cannot alter a Domain System entry through the selection action.

### Threat Model

- **Unregistered or spoofed Domain System selection:** A repository names a Domain System that does not exist, or attempts to smuggle inline shape content instead of selecting a registered entry. Mitigation: Domain System Registry rejects any registration attempt that is not a lookup by registered name; no inline Section Map/Profile content is accepted from a repository.
- **Version drift:** A Domain System is revised after a repository has already registered against it, leaving that repository's Epic/Usecase/Task set stale. Mitigation: Repo Registration Record stores a version reference; Task Runtime blocks new Task assignment on a version mismatch until re-sync (carried over from the Epic/Usecase/Task Model proposal).

## Rationale

### Domain System as Selected Asset, Not Authored Bundle
- **Context:** Samgraha requires each repository to independently populate every schema under `knowledge` for its Standard, which is repeated work for every repository of the same technology.
- **Decision:** A Domain System is authored once and selected by any number of repositories.
- **Alternatives Considered:** Allow a repository to author its own Domain System inline at registration, with reuse as an optional convenience.
- **Rejection Reason:** Making authoring the default and reuse optional reproduces Samgraha's per-repository duplication in practice, even if reuse is technically possible.
- **Architectural Goal:** True reuse across repositories sharing a technology or domain.

### Reuse Bodha's Section Map / Section Profile Shape
- **Context:** Bodha already expresses "what sections exist and why" and "how each section is filled" as proven, working formats.
- **Decision:** Domain System Section Map and Section Profile reuse Bodha's existing shape rather than a new format.
- **Alternatives Considered:** Design a Dharma-specific shape format.
- **Rejection Reason:** No architectural benefit was identified over the existing, working Bodha format; reinventing it would fragment tooling.
- **Architectural Goal:** Reuse over reinvention (carried over from the original Domain Shape Integration proposal).

## Constraints

### Hard Constraints
- **Selection only, no inline authoring** (source: Threat Model above) — a repository registers by name against an existing Domain System; it cannot submit inline shape content in place of a selection.
- **Exclusive write access** (source: Agent System Registry proposal) — only the Agent-Management Agent System may create or revise a Domain System entry.
- **Version-bound inheritance** (source: Threat Model above) — a Repo Registration Record tracks the Domain System version it was bound at.

### Soft Constraints
- Prefer extending an existing Domain System (e.g. adding a Usecase) over registering a near-duplicate Domain System for a closely related technology.

## Traceability

### Derivation Chain

```text
Agent System Registry, Epic/Usecase/Task Model
    │
    ▼
Domain System Registration (this document)
    │
    ▼
MCP Registration & Bootstrap (repository selects a Domain System as the first registration step)
```

### Non-Contradiction Rule

No downstream proposal may let a repository author inline Domain System shape content in place of selecting a registered entry, or grant write access to the Domain System Registry outside the Agent-Management Agent System, without revising this document first.

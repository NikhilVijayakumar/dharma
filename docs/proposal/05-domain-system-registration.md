# Proposal: Domain System Registration

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Supersedes** the original Domain Shape Integration draft. A repository does not author a bespoke Domain Definition Set — it **chooses** a registered Domain System (e.g. `rust-dev-domain`, `electron-dev-domain`), the same way it would have chosen a Standard in Samgraha, but the Domain System itself is a reusable registry asset, not a per-repository bundle.
> **Extended by Provider Config & Repo Sync (11):** a Domain System provider declares itself via `dharma-domain.toml`; a consuming repository's selection syncs that Domain System's content in full into its own `.dharma/repo.db` — see 11 for both.
> **Section Map/Profile format pinned by [13](13-bodha-section-format-reference.md):** the format cited below is now verified against Bodha's actual files, frozen as observed, with named gaps deferred rather than assumed away.

## Purpose

This document defines the standard for the Domain System entity within Dharma's agent-based execution architecture. Domain System Documentation describes how a technology or subject-matter domain (Rust development, Electron development, and so on) is captured once as a reusable, registrable asset that any matching repository can select at registration.

Unlike Samgraha, where each repository registers as its own Standard and must independently supply data for every schema under `knowledge`, a repository in Dharma registers against one existing Domain System. The Domain System — not the repository — owns the Section Map, the Section Profiles, and the Epic/Usecase/Task set (see Epic / Usecase / Task Model proposal). A repository contributes nothing to the Domain System's shape; it only inherits it.

## System Overview

### Overview

A Domain System is a named, registered entry (e.g. `rust-dev-domain`, `electron-dev-domain`) that carries a set of **domains** — individual documents such as `vision`, `philosophy`, `architecture`, `engineering`, `qa` (a base domain set carries 16; a domain like `rust-dev-domain` extends that set and drops a few that don't apply, per `docs/raw`'s own precedent of a repository maintaining a subset of doc-types). Each domain owns its own Section Map (which sections that document needs, and why — a self-referencing tree of sections and optional subsections, following the shape of Bodha's `section-map.yaml`) and a Section Profile per section (what data belongs in that section and how it is filled — following Bodha's `profile-default`). The Domain System as a whole also owns the full Epic/Usecase/Task set for the domain (see Epic / Usecase / Task Model proposal). A repository registers by selecting one existing Domain System; it does not define a new one itself.

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
- **Responsibility:** Names one Domain System (e.g. `rust-dev-domain`) and bundles the Domain set, the Epic/Usecase/Task set, and (see proposal 08) its audit definitions.
- **Ownership:** The Domain set below, plus the Epic/Usecase/Task Set.
- **Interfaces:** Selected, not authored, by a registering repository; read by the default Agent System during shape analysis.

### Domain
- **Responsibility:** Represents one document type this Domain System carries (e.g. `vision`, `architecture`, `qa`) — the unit a Section Map and its Section Profiles are scoped to.
- **Ownership:** Name, an optional tier/relationship position, and the Section Map + Section Profiles below.
- **Interfaces:** A Domain System owns any number of Domains; read by any Agent System producing or reviewing a document of this type.

### Section Map
- **Responsibility:** States which sections a document of this Domain needs and why each is required, as a self-referencing tree — subsections nest under sections — mirroring Bodha's `section-map.yaml`.
- **Ownership:** The section tree, with purpose/rationale and a required/optional flag per entry.
- **Interfaces:** Read by any Agent System producing or reviewing a document of this Domain for a repository registered against this Domain System.

### Section Profile
- **Responsibility:** States what data a given section (or subsection) can be filled with and how, one profile per section, mirroring Bodha's `profile-default`.
- **Ownership:** Fill rules per section, inheriting a default profile unless overridden.
- **Interfaces:** Read by Documentation-concern Agent Systems when producing a proposed solution for a documentation Task.

### Epic/Usecase/Task Set
- **Responsibility:** The complete Epic → Usecase → Task hierarchy for this domain (see Epic / Usecase / Task Model proposal).
- **Ownership:** All Epics, Usecases, and Tasks defined for this Domain System.
- **Interfaces:** Inherited in full by every repository registered against this Domain System; read by Task Runtime to assign work.

### Component Diagram

```text
Domain System Entry ──owns──▶ Domain (× N, e.g. vision, architecture, qa, ...)
                                  ──owns──▶ Section Map (self-referencing tree)
                                  ──owns──▶ Section Profile (one per section)
                     ──owns──▶ Epic/Usecase/Task Set
                                          │
                     Repository ──selects──┘ (read-only inheritance)
```

## Communication

### Communication Paths

**Repository → Domain System Registry**
- **Pattern:** Synchronous selection, at registration time.
- **Contract:** Repository submits a chosen Domain System name; Registry returns the bound Domain set (each with its Section Map and Section Profiles) and Epic/Usecase/Task set, or an error if the name is unregistered.

**Agent-Management Agent System → Domain System Registry**
- **Pattern:** Synchronous write, gated to Agent-Management only.
- **Contract:** Agent-Management submits a new or revised Domain System entry; Registry accepts after validating no conflicting entry exists for the same domain name.

### Communication Diagram

```text
Repo → Domain System Registry : select(domainSystemName)
Domain System Registry → Repo : domainSet (sectionMaps + sectionProfiles per domain), epicUsecaseTaskSet | error(unregistered)
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
Repo ──selects(name)──▶ Domain System Registry ──▶ Domain set (Section Maps + Section Profiles) + Epic/Usecase/Task Set
                                                          │
                                                          ▼
                                              Repo Registration Record (bound)
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Domain set, Section Maps, Section Profiles, Epic/Usecase/Task set | Domain System Registry (authored by Agent-Management Agent System) |
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

### Domain as Its Own Tier, Not a Flat Section Map Per System
- **Context:** A Domain System doesn't carry one document — it carries many (e.g. `vision`, `philosophy`, `architecture`, `qa`, and others), each needing its own Section Map, the same way this very proposal set spans `docs/raw/architecture.md`, `docs/raw/qa.md`, and others as separate documents.
- **Decision:** `domain` is its own entity between the Domain System and its Section Map/Section Profiles — a Domain System owns any number of Domains, each scoped to one document type.
- **Alternatives Considered:** One Section Map per Domain System, covering every document type in a single flat tree.
- **Rejection Reason:** A single flat tree cannot express that `rust-dev-domain` extends a base document set but drops a few types that don't apply to it — that's a per-document-type decision, not a per-system one.
- **Architectural Goal:** A Domain System can extend or narrow a base document set per Domain, without forcing every Domain System to define every document type from scratch.

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

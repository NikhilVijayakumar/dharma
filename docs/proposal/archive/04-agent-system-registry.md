# Proposal: Agent System Registry

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Supersedes** the fixed five-group taxonomy in the original Agent-Group Taxonomy draft. Agent Systems are pluggable and registrable, parallel in kind to Domain Systems — not a closed set of five built-in groups.
> **Extended by Provider Config & Repo Sync (11):** an Agent System provider declares itself via `dharma-agent.toml`; only the subset a repository's Capability Manifest approves is synced (filtered, not full) into its `.dharma/repo.db` — see 11 for both.

## Purpose

This document defines the standard for the Agent System entity within Dharma's agent-based execution architecture. Agent System Documentation describes how Agents and Skills are packaged into a named, registrable, reusable unit of capability that MCP can offer to any repository, regardless of that repository's Domain System.

Unlike Samgraha's Standard (one bundle scoped to a single repository's domain) and unlike a fixed internal taxonomy, an Agent System is an open registry entry: `documentation-management`, `rust-development`, `electron-development`, and others are all Agent Systems that MCP can hold side by side, each contributed independently and matched to a repository's needs by the default Agent System at registration time.

## System Overview

### Overview

MCP maintains a registry of Agent Systems. Each Agent System packages a set of Agents (see Agent Model) and the Skills those Agents are bound to (see Skill Model), organized around one coherent concern — a technology stack (`rust-development`), a cross-cutting function (`documentation-management`, `audit`), or any other concern a domain expert defines. A repository is not limited to one Agent System: the default Agent System resolves which registered Agent Systems apply, based on the repository's chosen Domain System (see Domain System Registration and MCP Registration & Bootstrap proposals).

### Structural Approach

The registry is open and flat: any number of Agent Systems may be registered, and a repository may draw on several at once. One Agent System — the default/bootstrap Agent System — is privileged: it is the one MCP invokes first, before any Domain System-specific Agent System is resolved, to perform shape analysis and propose which other Agent Systems apply.

### Diagram

```text
┌──────────────────── MCP Agent System Registry ────────────────────┐
│                                                                     │
│  [ Default/Bootstrap Agent System (priv.) ]                        │
│  [ documentation-management Agent System  ]                        │
│  [ rust-development Agent System          ]                        │
│  [ ... other registered Agent Systems     ]                        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
        │ resolves applicable systems for
        ▼
   Repository + chosen Domain System
```

## Component Model

### Agent System Entry
- **Responsibility:** Names one coherent capability concern and packages the Agents and Skills that serve it.
- **Ownership:** Name, concern statement, the set of Agents it registers, the set of Skills those Agents are bound to.
- **Interfaces:** Read by the default Agent System during shape analysis; read by the Handoff Broker (see Proposal & Execution Protocol proposal) when resolving a handoff target.

### Default/Bootstrap Agent System
- **Responsibility:** The one Agent System MCP invokes at repository registration to analyze the repository and its chosen Domain System, and to propose which other Agent Systems, Agents, and Section Profiles apply.
- **Ownership:** Its own Agents/Skills, specialized in shape analysis rather than in any single domain or function.
- **Interfaces:** Reads the Domain System's Section Map/Section Profile and Epic/Usecase/Task set; writes a proposed Capability Manifest (see MCP Registration & Bootstrap proposal); is itself just another Agent System entry, privileged only by being invoked first.

### Agent-Management Agent System
- **Responsibility:** Creates and modifies Agent, Skill, Domain System, and Agent System definitions themselves.
- **Ownership:** The only Agent System permitted to write to these registries; every other Agent System reads them.
- **Interfaces:** Invoked only with explicit authorization (see Security below).

### Concern-Specific Agent Systems
- **Responsibility:** Serve one named concern — a technology stack or cross-cutting function — with a focused Agent/Skill set (e.g. `rust-development`, `documentation-management`, `electron-development`).
- **Ownership:** Their own Agents and Skills, scoped to their declared concern.
- **Interfaces:** Resolved into a repository's available capability by the default Agent System; participate in handoffs like any other Agent System's Agents.

### Component Diagram

```text
Repo + Domain System ──▶ Default Agent System ──analyzes──▶ Agent System Registry
                                                                   │
                                     ┌─────────────────────────────┼────────────────────┐
                                     ▼                              ▼                    ▼
                          documentation-management        rust-development      Agent-Management
                               Agent System                 Agent System        Agent System (priv.)
```

## Communication

### Communication Paths

**Default Agent System → Agent System Registry**
- **Pattern:** Synchronous query, at repository registration.
- **Contract:** Default Agent System submits the repository's Domain System concern signals; Registry returns matching Agent System candidates.

**Handoff Broker → Agent System Entry**
- **Pattern:** Synchronous resolution, at Task execution time.
- **Contract:** Handoff Broker submits a required capability; the matching Agent System Entry returns a specific Agent to receive control.

### Communication Diagram

```text
Default Agent System → Agent System Registry : query(domainSystemConcern)
Agent System Registry → Default Agent System : candidateAgentSystems
Handoff Broker → Agent System Registry : resolve(requiredCapability)
Agent System Registry → Handoff Broker : targetAgent
```

## Data Flow

### Data Paths

**Resolution Path**
- **Entry point:** Repository registers with a chosen Domain System.
- **Transformations:** Default Agent System maps Domain System concern to candidate Agent Systems; result becomes a proposed Capability Manifest entry.
- **Ownership boundary:** The registry owns Agent System definitions; the repository owns only its resolved manifest.
- **Exit point:** Approved Capability Manifest naming the Agent Systems available to that repository's Tasks.

### Data Flow Diagram

```text
Repo + Domain System ──▶ Default Agent System ──▶ Agent System Registry
                                                          │
                                              candidate Agent Systems
                                                          │
                                                          ▼
                                              proposed Capability Manifest
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Agent System definitions (Agents, Skills, concern) | Agent System Registry (authored by Agent-Management Agent System) |
| Proposed/approved Capability Manifest | Default Agent System (proposed) → Repo Registration Record (approved) |

## Security

### Trust Boundaries

- **Agent-Management Agent System ↔ all other Agent Systems:** Privileged boundary — only Agent-Management may write Agent/Skill/Domain System/Agent System definitions.
- **Default Agent System ↔ Repository input:** The repository's Domain System choice and content are semi-trusted signals for resolution, not directly executable instructions.

### Threat Model

- **Unauthorized privileged resolution:** The default Agent System's analysis names Agent-Management as a required Agent System without justification. Mitigation: any Capability Manifest entry naming Agent-Management requires explicit human approval before being written to the Repo Registration Record (carried over from the original Domain Shape Integration proposal).
- **Concern overlap collision:** Two registered Agent Systems both claim to serve the same concern, causing ambiguous resolution. Mitigation: Agent System registration is itself gated by the Agent-Management Agent System, which checks for concern overlap before accepting a new entry.

## Rationale

### Open Registry Instead of Fixed Taxonomy
- **Context:** A fixed set of five built-in groups (documentation, management, code-generation, audit, report) cannot express domain-specific concerns like `rust-development` or `electron-development` without stretching one of the five categories.
- **Decision:** Agent Systems are an open, named registry, parallel in kind to the Domain System registry.
- **Alternatives Considered:** Keep the fixed five-group taxonomy and treat technology-specific capability as a sub-classification within one group.
- **Rejection Reason:** Forcing technology-specific capability into a generic function bucket loses the ability to reason about "does this repository have a `rust-development` Agent System available" directly.
- **Architectural Goal:** Symmetry between Domain Systems and Agent Systems as equally first-class, pluggable registry entries.

### Default Agent System Is Just Another Agent System
- **Context:** Shape analysis (matching a repository to capability) is itself a task an Agent performs, not a special platform-only mechanism.
- **Decision:** The default/bootstrap Agent System is an ordinary registry entry, privileged only by being invoked first at registration.
- **Alternatives Considered:** A bespoke, non-agent shape-analysis component outside the Agent System model.
- **Rejection Reason:** A bespoke component would be an exception to the "everything is Agents and Skills" principle, adding a second execution model to reason about.
- **Architectural Goal:** Uniformity — the platform dogfoods its own Agent/Skill model for its own bootstrap step.

## Constraints

### Hard Constraints
- **Exclusive write access** (carried over from the original Agent-Group Taxonomy) — only the Agent-Management Agent System may create or modify Agent/Skill/Domain System/Agent System definitions.
- **Concern uniqueness at registration** (source: Security threat model above) — a new Agent System must not claim a concern already served by an existing entry without explicit review.

### Soft Constraints
- Prefer resolving to an existing Agent System over registering a new one for a narrowly overlapping concern.

## Traceability

### Derivation Chain

```text
Agent Model, Skill Model
    │
    ▼
Agent System Registry (this document)
    │
    ├──▶ MCP Registration & Bootstrap (default Agent System resolves capability at registration)
    └──▶ Proposal & Execution Protocol (Agent Systems are handoff targets)
```

### Non-Contradiction Rule

No downstream proposal may reintroduce a closed, fixed set of Agent Systems, or grant a non-Agent-Management Agent System write access to Agent/Skill/Domain System/Agent System definitions, without revising this document first.

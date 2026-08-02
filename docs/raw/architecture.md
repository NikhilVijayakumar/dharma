# Architecture Standard

> *Deterministic rules for this domain: `common/tier2/audit/deterministic/document/architecture.yaml`*

## Table of Contents
- [Purpose](#purpose)
- [Crate Architecture](#crate-architecture)
- [Trait Design](#trait-design)
- [System Overview](#system-overview)
- [Component Model](#component-model)
- [Communication](#communication)
- [Data Flow](#data-flow)
- [Security](#security)
- [Rationale](#rationale)
- [Constraints](#constraints)
- [Required Sections](#required-sections)
- [Goals](#goals)
- [Non-Goals](#non-goals)
- [Success Criteria](#success-criteria)
- [Responsibilities](#responsibilities)
- [Scope](#scope)
- [Out of Scope](#out-of-scope)
- [Inputs](#inputs)
- [Outputs](#outputs)
- [Traceability](#traceability)
- [Relationships](#relationships)
- [Required Characteristics](#required-characteristics)
- [Audit Rules](#audit-rules)
- [Validation Rules](#validation-rules)
- [Generation Rules](#generation-rules)
- [Enhancement Rules](#enhancement-rules)
- [Summary](#summary)
- [Common Mistakes](#common-mistakes)
- [Documentation Folder](#documentation-folder)
- [Usage](#usage)
- [Related](#related)
- [Architecture as a Documentation Collection](#architecture-as-a-documentation-collection)
- [Single Responsibility](#single-responsibility)
- [Architectural Boundaries](#architectural-boundaries)
- [Architectural Principles](#architectural-principles)
- [Technology Independence](#technology-independence)
- [Cross-Repository Architecture](#cross-repository-architecture)
- [Quality Requirements](#quality-requirements)

---


## Purpose

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/01-purpose.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
This document defines the standard for [Documentation Type] within the [ecosystem name] documentation ecosystem.

[Documentation Type] describes [what it describes].

Unlike [related type], [distinctive characteristic].

[Core purpose statement.]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> This document defines the standard for Architecture Documentation within the engineering documentation ecosystem. Architecture Documentation describes the structural organization of a system — how responsibilities are divided among components and how those components relate. Unlike a single Vision or Feature document, Architecture is a collection of focused documents, each covering one structural concern.

**Incorrect:**
> Architecture Documentation covers the microservices layout, the React component tree, the PostgreSQL schema, and the Kubernetes deployment manifests used by the system.
> *Why wrong: names specific technologies and implementation artifacts instead of stating the document type's role and boundary — that belongs in the sections themselves, not the Purpose statement.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** architect
- **Do:** State what Architecture Documentation is and what it is not in the same breath; describe it as a collection rather than a single artifact; keep the boundary with Engineering and Feature Technical Design explicit
- **Don't:** Name specific components, technologies, or frameworks; describe how any particular system is organized; list features or capabilities

This document defines the standard for Architecture Documentation within the engineering documentation ecosystem.

Architecture Documentation describes the structural organization of a system.

Unlike other documentation types, Architecture is not expected to be represented by a single document.

Instead, it is a structured collection of related documents that collectively describe the responsibilities, boundaries, interactions, and organization of the system.

Architecture explains **how responsibilities are organized**.

It does not explain implementation details.

---

## System Overview

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/02-system_overview.yaml`*

### Template

> **minimum_content:** 2 subsections
> **length_guidance:** moderate
> **diagram_requirements:** component

```markdown
## System Overview

> [metadata block]

### Overview
[1-2 paragraphs: system purpose, primary capabilities, high-level approach]

### Structural Approach
[1 paragraph: how the system is organized at the top level]

### Diagram
[High-level component or system context diagram]
```

### Examples

**Correct:**
> DataSync is a distributed data synchronization platform that coordinates data exchange between enterprise systems. It provides reliable, ordered delivery of data changes across heterogeneous datastores, supporting both real-time and batch synchronization modes. The system is organized into an ingestion layer, a transformation engine, and a distribution layer, each with distinct ownership and scaling characteristics.

**Incorrect:**
> DataSync uses Apache Kafka 3.4 with Spring Boot 3.1 for event streaming, PostgreSQL 15 for metadata storage, and Redis 7 for caching. The backend runs on AWS EKS with Kubernetes 1.27.
> *Why wrong: names specific library versions, frameworks, and cloud infrastructure — this is Engineering detail, not architectural overview.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** new contributor
- **Do:** Open with the system's purpose in one sentence; describe capabilities before structure; use a diagram to anchor the overview
- **Don't:** Name specific libraries or frameworks; describe component internals; assume the reader knows the codebase

**Required subsections:** Overview, Diagram
**Optional subsections:** Structural Approach, Key Capabilities
**Required diagrams:** system context or component overview diagram
**Required cross-references:** Vision(01)

*(To be written by the domain expert. This section defines the high-level system description and structural approach.)*

---

## Component Model

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/03-component_model.yaml`*

### Template

> **minimum_content:** 1 subsection per component
> **length_guidance:** extensive
> **diagram_requirements:** component

```markdown
## Component Model

> [metadata block]

### Components

#### [Component Name]
- **Responsibility:** [what this component owns]
- **Ownership:** [data/processes owned]
- **Interfaces:** [how other components interact with it]

#### [Component Name]
- **Responsibility:** [what this component owns]
- **Ownership:** [data/processes owned]
- **Interfaces:** [how other components interact with it]

### Component Diagram
[Diagram showing all components and their relationships]
```

### Examples

**Correct:**
> **Ingestion Service**
> - **Responsibility:** Accepts data changes from external systems and validates their structure before passing them downstream.
> - **Ownership:** Raw incoming change events, ingestion queues.
> - **Interfaces:** Exposes a submission endpoint; publishes validated events to the Transform Engine.
>
> **Transform Engine**
> - **Responsibility:** Applies mapping rules to convert incoming data formats into the canonical system model.
> - **Ownership:** Mapping rules, transformation state, intermediate representations.
> - **Interfaces:** Consumes validated events from Ingestion; publishes canonical records to Distribution.

**Incorrect:**
> The Ingestion Service is implemented as a Node.js 20 Express app with 4 REST endpoints (`/api/v1/ingest`, `/api/v1/batch`, `/api/v1/status`, `/api/v1/health`). It uses Bull queues backed by Redis and calls `validateSchema()` from the shared `@datasync/validation` package.
> *Why wrong: describes implementation details (runtime, endpoints, package names, function signatures) instead of responsibility and ownership boundaries.*

### Writing Guidance

- **Tone:** structural
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** architect
- **Do:** State each component's single responsibility; define ownership boundaries explicitly; include a component relationship diagram
- **Don't:** Describe class hierarchies or function signatures; conflate responsibility with implementation; list components without defining boundaries

**Required subsections:** Components (with one entry per component), Component Diagram
**Optional subsections:** Component Relationships, Boundary Definitions
**Required diagrams:** component relationship diagram
**Required cross-references:** System Overview, Communication, Data Flow

*(To be written by the domain expert. This section defines the system's components, their responsibilities, and how they relate.)*

---

## Communication

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/04-communication_paths.yaml`*

### Template

> **minimum_content:** 2 subsections
> **length_guidance:** moderate
> **diagram_requirements:** sequence

```markdown
## Communication

> [metadata block]

### Communication Paths
[For each communication path: source, destination, pattern (sync/async/event), contract]

### Interaction Patterns
[Description of communication patterns used across the system]

### Communication Diagram
[Sequence or flow diagram showing inter-component communication]
```

### Examples

**Correct:**
> **Ingestion → Transform Engine**
> - **Pattern:** Asynchronous, event-driven.
> - **Contract:** Ingestion publishes a validated event; Transform Engine acknowledges receipt and processes independently. Events are idempotent and ordered within a single source.
>
> **Transform Engine → Distribution**
> - **Pattern:** Asynchronous, queue-based.
> - **Contract:** Transform publishes canonical records with a unique identifier. Distribution guarantees at-least-once delivery and deduplicates on the identifier.

**Incorrect:**
> Ingestion calls Transform via HTTP POST to `http://transform:8080/process` with a JSON body. It uses Axios with a 5-second timeout and retries 3 times with exponential backoff. Responses are validated against the OpenAPI schema in `transform-api.yaml`.
> *Why wrong: specifies network protocols, library choices, timeout values, and implementation-level retry logic — all of which belong in Engineering, not Architecture.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** tables
- **Audience:** engineer
- **Do:** Define a contract for every component-to-component path; classify each interaction pattern (sync, async, event-driven); include a sequence diagram
- **Don't:** Specify network protocols or transport details; describe retry logic or timeout values; conflate communication paths with data paths

**Required subsections:** Communication Paths, Communication Diagram
**Optional subsections:** Interaction Patterns, Contract Definitions
**Required diagrams:** sequence diagram of primary communication paths
**Required cross-references:** Component Model, Data Flow

*(To be written by the domain expert. This section defines how system components communicate and interact.)*

---

## Data Flow

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/05-data_flow.yaml`*

### Template

> **minimum_content:** 2 subsections
> **length_guidance:** moderate
> **diagram_requirements:** flowchart

```markdown
## Data Flow

> [metadata block]

### Data Paths
[For each major data path: entry point, transformations, ownership boundaries, exit point]

### Data Ownership
[Table or list mapping data entities to owning components]

### Data Flow Diagram
[Flowchart showing data movement through the system]
```

### Examples

**Correct:**
> **Inbound Data Path**
> - **Entry point:** External system submits data changes.
> - **Transformations:** Schema validation and format normalization.
> - **Ownership boundary:** Ingestion Service owns raw events until transformation completes.
> - **Exit point:** Canonical records delivered to Distribution.
>
> **Data Ownership**
> | Data Entity | Owning Component |
> |---|---|
> | Raw incoming events | Ingestion Service |
> | Canonical records | Transform Engine |
> | Delivery confirmations | Distribution Service |

**Incorrect:**
> Data flows through a PostgreSQL table called `raw_events` with columns `id`, `payload`, `created_at`. The Transform Engine runs a SQL query `SELECT * FROM raw_events WHERE processed = false`, deserializes the JSONB payload using `JSON.parse()`, and inserts into `canonical_records` via an ORM bulk insert.
> *Why wrong: describes database schemas, SQL queries, and code-level operations — these are implementation details, not architectural data flow.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** tables
- **Audience:** architect
- **Do:** Trace every major data path from entry to exit; assign ownership to a specific component at each boundary; include a data flow diagram
- **Don't:** Describe database schemas or SQL queries; reference ORM methods or serialization code; document data paths that bypass component boundaries

**Required subsections:** Data Paths, Data Flow Diagram
**Optional subsections:** Data Ownership, Data Transformations
**Required diagrams:** data flow diagram covering all major paths
**Required cross-references:** Component Model, Communication, Security

*(To be written by the domain expert. This section defines how data moves through the system and who owns it.)*

---

## Security

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/06-security_considerations.yaml`*

### Template

> **minimum_content:** 2 subsections
> **length_guidance:** moderate
> **diagram_requirements:** component

```markdown
## Security

> [metadata block]

### Trust Boundaries
[Description of where trust changes — external/internal, component-to-component]

### Threat Model
[Key threats, attack vectors, and mitigations at the architectural level]

### Security Controls
[Architectural security measures — access control model, data protection requirements]
```

### Examples

**Correct:**
> **Trust Boundaries**
> - **External → Ingestion:** Untrusted external systems submit data; Ingestion validates all inputs before internal processing.
> - **Ingestion → Transform:** Trusted boundary — both are internal components communicating over an internal network.
>
> **Threat Model**
> - **Spoofing:** External systems may impersonate legitimate data sources. Mitigation: authenticated submission with signed payloads.
> - **Data tampering:** Malicious payloads may attempt to exploit downstream processing. Mitigation: schema validation at the Ingestion boundary.

**Incorrect:**
> We use JWT tokens signed with RS256 via the `jsonwebtoken` library. Passwords are hashed with bcrypt (12 rounds). The API gateway uses Kong 3.4 with rate limiting of 100 req/min. All traffic is encrypted with TLS 1.3.
> *Why wrong: specifies concrete libraries, library versions, configuration values, and protocol versions — these are Engineering implementation details, not architectural security controls.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** mixed
- **Audience:** architect
- **Do:** Define every trust boundary with source and destination; document the threat model before controls; tie each control to a specific threat
- **Don't:** Name specific security libraries or libraries' configuration values; describe implementation of encryption or authentication; omit a threat that has no documented mitigation

**Required subsections:** Trust Boundaries, Threat Model
**Optional subsections:** Security Controls, Access Control Model
**Required diagrams:** trust boundary diagram
**Required cross-references:** Component Model, Data Flow, Philosophy(02)

*(To be written by the domain expert. This section defines the architectural security posture, boundaries, and threat model.)*

---

## Rationale

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/07-rationale.yaml`*

### Template

> **minimum_content:** 1 entry per significant decision
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Rationale

> [metadata block]

### [Decision Name]
- **Context:** [what prompted this decision]
- **Decision:** [what was decided]
- **Alternatives Considered:** [what else was evaluated]
- **Rejection Reason:** [why alternatives were rejected]
- **Architectural Goal:** [which goal this serves]
```

**Required subsections:** one entry per significant architectural decision
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Vision(01), Philosophy(02)

### Examples

**Correct:**
> **Event-Driven Ingestion**
> - **Context:** Multiple external systems submit data at unpredictable rates and volumes.
> - **Decision:** Ingestion publishes events asynchronously rather than processing synchronously.
> - **Alternatives Considered:** Synchronous request/response ingestion with backpressure.
> - **Rejection Reason:** Synchronous processing would couple external system availability to ingestion availability, violating the reliability pillar.
> - **Architectural Goal:** Resilient Connections.

**Incorrect:**
> We chose Kafka over RabbitMQ because it has better throughput benchmarks and our team already knows the Java client library.
> *Why wrong: justifies a specific technology choice by implementation-level benchmarks and team familiarity — that belongs in Engineering's rationale, not Architecture's. Architecture rationale should justify structural decisions (sync vs async, ownership boundaries) against architectural goals, not product-specific tooling.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** architect
- **Do:** Record the context that prompted each decision; name the alternatives that were actually considered; tie every decision back to an architectural goal or pillar
- **Don't:** Justify decisions by technology benchmarks, licensing, or team familiarity; record decisions without a rejected alternative; let rationale entries go stale once a decision is superseded

*(To be written by the domain expert. This section defines the reasoning behind architectural decisions.)*

---

## Constraints

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/08-constraints.yaml`*

### Template

> **minimum_content:** 1 list of constraints
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Constraints

> [metadata block]

### Hard Constraints
[Constraints that cannot be violated — with source and reason for each]

### Soft Constraints
[Preferences and guidelines that should be followed unless justified]

### Platform Constraints
[Hardware, OS, or runtime constraints that shape architecture]
```

**Required subsections:** Hard Constraints
**Optional subsections:** Soft Constraints, Platform Constraints
**Required diagrams:** none
**Required cross-references:** External Context, Platform Pillars(01)

### Examples

**Correct:**
> **Hard Constraints**
> - **Offline-first operation** (source: Platform Pillars) — the system must remain functional with no network connection; no component may assume live connectivity.
> - **Single-writer data ownership** (source: External Context) — the upstream partner system requires exactly one writer per record to avoid conflict resolution on their side.
>
> **Soft Constraints**
> - Prefer components that can be tested in isolation, unless a hard constraint makes isolation impractical.

**Incorrect:**
> The system must use Rust 1.75+ and target a minimum of 4GB RAM.
> *Why wrong: states a language version and hardware minimum as if they were architectural constraints, without a source or architectural reason — these are either Engineering-level technology decisions or unsourced assertions, not sourced structural bounds.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** architect
- **Do:** Attribute every constraint to its source (External Context, Platform Pillars, organizational rule); separate hard constraints from soft preferences explicitly; state the consequence of violating a hard constraint
- **Don't:** List a constraint without a source; mix implementation-level limits (language versions, dependency pins) into architectural constraints; present preferences as if they were immovable

*(To be written by the domain expert. This section defines the non-functional requirements and constraints that bound the architecture.)*

---

## Required Sections

Every Architecture document must contain the following sections.
Sections are identified by heading text; the compiler maps each to a semantic type.

| Section | semantic_type | Required | Aliases | Content Requirements |
|---------|--------------|----------|---------|---------------------|
| System Overview | `system_overview` | ✓ | Overview, Architecture Overview | System purpose, primary capabilities, structural approach; entry point for new contributors |
| Component Model | `component_model` | ✓ | Components, Component Architecture | Component names, responsibilities, ownership boundaries, relationships |
| Communication | `communication_paths` | ✓ | Communication Paths, Component Communication | Communication paths, interaction patterns, contracts between components |
| Data Flow | `data_flow` | ✓ | Data Movement, Information Flow | Data entry/exit points, movement paths, ownership boundaries, transformations |
| Security | `security_considerations` | ✓ | Security Architecture, Security Model | Trust boundaries, threat model, access control model, data protection requirements |
| Purpose | `purpose` | | Overview, Summary | Root intent, why Architecture Documentation exists, scope boundaries |
| Rationale | `rationale` | | Decision Rationale, Architectural Decisions, Why | Decision reasoning, alternatives considered, trade-offs, rejection criteria |
| Constraints | `constraints` | | Limitations, Non-Functional Requirements | Non-functional requirements, platform limitations, organizational rules with source attribution |
| Traceability | `traceability` | | Traces To, Derived From | Tier model, derivation chain, downstream standards, non-contradiction rule |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

Architecture Documentation aims to:

* Give the system a single authoritative structural description.
* Make component boundaries and responsibilities explicit.
* Let Feature Technical Design constrain itself against one source of truth.
* Keep structure stable while individual features change.

---

## Non-Goals

Architecture Documentation does not attempt to define:

* Product purpose
* Feature specifications
* Engineering decisions
* Technology selection
* Source code
* Algorithms
* Build configuration
* Library usage

These responsibilities belong to other documentation standards.

---

## Success Criteria

Architecture Documentation is successful when:

* Engineers understand how the system is organized.
* Responsibilities are unambiguous.
* New contributors can locate architectural information easily.
* Components can evolve independently.
* Implementation decisions remain consistent with architectural intent.
* AI systems can understand the system organization without reading source code.

---

## Responsibilities

Architecture Documentation is responsible for describing:

* System structure
* Components
* Responsibilities
* Ownership boundaries
* Communication
* Dependencies
* Data flow
* Runtime organization
* Deployment organization
* Architectural constraints
* System invariants

Architecture defines how the system is organized.

It does not define how it is implemented.

---

## Scope

Architecture Documentation may include:

* System Overview
* Component Model
* Module Boundaries
* Runtime Boundaries
* Communication Architecture
* Data Flow
* Security Architecture
* Deployment Architecture
* Persistence Architecture
* Plugin Architecture
* Integration Architecture
* Lifecycle Documentation
* Architectural Invariants

Projects should include only the architectural topics relevant to the repository.

Architecture documentation is intentionally modular.

---

## Out of Scope

Architecture Documentation must not describe:

* Product vision
* Feature requirements
* User experience
* UI design
* Source code
* Algorithms
* Class implementations
* Function implementations
* Programming syntax
* Library APIs
* Configuration files
* Build scripts

These belong to downstream documentation.

---

## Inputs

Architecture Documentation derives from:

* Vision
* Technology Decisions
* Platform Constraints
* Security Threat Model
* Engineering Principles

Architecture is independent of Feature Documentation.

Architecture should not derive from implementation.

---

## Outputs

Architecture provides direction for:

* Feature Technical Design
* Engineering Decisions
* Implementation
* Testing Standards
* Validation
* Documentation Audits

Implementation should conform to Architecture.

---

## Traceability

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/09-traceability.yaml`*

### Template

> **minimum_content:** 1 diagram + derivation list
> **length_guidance:** concise
> **diagram_requirements:** flowchart

```markdown
## Traceability

> [metadata block]

### Derivation Chain
[Diagram showing Architecture's position in the documentation tier model]

### Downstream Impact
[List of standards that Architecture constrains or feeds into]

### Non-Contradiction Rule
[Statement that no downstream document may contradict Architecture]
```

**Required subsections:** Derivation Chain, Non-Contradiction Rule
**Optional subsections:** Downstream Impact
**Required diagrams:** tier model diagram
**Required cross-references:** Vision(01), Feature Technical Design(10), Engineering(07)

### Examples

**Correct:**
> Tier 2: Architecture (System Overview, Component Model, Security)
>     ├──→ Tier 3: Feature Technical Design
>     └──→ Tier 5: Engineering (soft, non-mandatory)
>
> **Non-contradiction rule:** No downstream document may describe a component boundary, ownership assignment, or communication path that contradicts this Architecture. When a Feature Technical Design needs a boundary Architecture doesn't define, Architecture is updated first.

**Incorrect:**
> Architecture traces to the `src/components/` directory and the deployment YAML files.
> *Why wrong: references source code and deployment configuration instead of the documentation hierarchy — Traceability connects Architecture to other standards, not to the codebase it eventually governs.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** diagrams
- **Audience:** architect
- **Do:** Show Architecture's tier position and every standard it feeds; state the non-contradiction rule explicitly; keep the diagram in sync when new standards derive from Architecture
- **Don't:** Reference source code, deployment artifacts, or CI/CD configuration; omit downstream standards from the diagram; leave the non-contradiction rule implicit

Architecture should remain traceable.

```text
Vision
    │
    ├──────────────────────────┐
    ↓                          ↓
Feature chain            Architecture Documentation
(Feature)                    (technology decisions,
    │                          platform constraints,
    │                          engineering principles)
    └──────────────────────────┘
                ↓
      Feature Technical Design
      (Feature + Architecture + optional External Context)
                ↓
          Engineering
                ↓
          Implementation
```

Architecture provides the structural foundation that Feature Technical Design applies to specific features.

---

## Relationships

| Document | Relationship |
|---|---|
| Philosophy | Architecture is guided by Philosophy |
| Security | Architecture is guided by Security's threat model (once registered) |
| Engineering | Soft, non-mandatory alignment — most frameworks expect an architecture, none require one first |
| Feature Technical Design | Architecture constrains it |
| External Context | May be referenced when integration constraints affect structure |

---

## Required Characteristics

Architecture Documentation should be:

* Consistent across components
* Traceable to Philosophy and Security
* Technology-independent at the system level
* Boundary-respecting
* Stable
* Reviewable independent of any single feature

---

## Audit Rules

An audit should verify:

* Architecture is modular.
* Responsibilities are clearly separated.
* Ownership is explicit.
* Boundaries are documented.
* Communication paths are understandable.
* Documents do not duplicate one another.
* Architecture supports Feature Technical Design without depending on Feature Documentation.
* Architecture avoids implementation detail.
* Cross-repository references are used instead of duplication.

Architecture quality is evaluated across the complete documentation collection.

---

## Validation Rules

Architecture Documentation is considered valid when:

* Responsibilities are clearly defined.
* Every architectural concern has a documented owner.
* Boundaries are explicit.
* Communication paths are documented.
* Component responsibilities do not overlap.
* Architectural documents remain modular.
* Architecture remains traceable to Features.
* No implementation details dominate architectural descriptions.

Validation applies to the architecture collection rather than individual files.

---

## Generation Rules

When generating Architecture Documentation:

* Decompose by architectural responsibility.
* Prefer multiple focused documents over one large document.
* Describe responsibilities before technologies.
* Define boundaries explicitly.
* Reference external architectures rather than duplicating them.
* Maintain traceability to Features.

Architecture generation should optimize for maintainability rather than document count.

---

## Enhancement Rules

When enhancing Architecture Documentation:

* Improve separation of concerns.
* Split oversized documents.
* Remove duplicated responsibilities.
* Strengthen ownership definitions.
* Clarify communication.
* Improve traceability.
* Remove implementation leakage.
* Preserve architectural intent.

Architecture should become clearer through refinement.

---

## Summary

Architecture Documentation is the structural specification of the system.

It is a modular collection of focused documents that collectively describe system organization, responsibilities, ownership, boundaries, and communication.

The objective is not to document every implementation detail, but to provide a clear, maintainable, and traceable architectural model that guides engineering and implementation throughout the lifecycle of the project.

---

## Common Mistakes

Examples include:

* Mixing architecture with implementation.
* Large monolithic architecture documents.
* Undefined ownership.
* Hidden communication paths.
* Missing boundaries.
* Technology-driven architecture.
* Duplicated architectural responsibilities.
* Documenting source code instead of architecture.

These should be reported during audits.

---

## Documentation Folder

Architecture documents live under:

```text
docs/raw/architecture/
```

---

## Usage

Written by architects/senior engineers when a system's structure changes; read by anyone writing Feature Technical Design (which constrains itself to what Architecture permits). Use `samgraha compile --domain architecture` after adding a new architecture document, and `samgraha audit --domain architecture` to catch missing System Overview / Component Model / Security sections before review.

## Related

- [Feature Technical Standard](feature-technical.md) — constrained by Architecture
- [Engineering Standard](engineering.md) — guided by Architecture


## Architecture as a Documentation Collection

Architecture is a collection of related documents.

Example:

```text
architecture/

    system-overview.md

    component-model.md

    runtime-boundary.md

    communication.md

    persistence.md

    deployment.md
```

Each document should describe one architectural concern.

Responsibilities should not overlap.

---

## Single Responsibility

Every architecture document should have one primary responsibility.

Examples:

* Runtime Boundary
* Plugin Lifecycle
* Persistence
* Security
* Communication

Large architecture documents should be decomposed into smaller focused documents.

---

## Architectural Boundaries

Architecture should define:

* Responsibility ownership
* Component boundaries
* Communication boundaries
* Data ownership
* Runtime ownership
* Security boundaries
* Extension boundaries

Boundaries should be explicit.

Implicit architecture should be avoided.

---

## Architectural Principles

Architecture should promote:

* Separation of concerns
* High cohesion
* Low coupling
* Explicit ownership
* Stable interfaces
* Predictable communication
* Clear dependencies
* Replaceable components

Projects may define additional architectural principles.

---

## Technology Independence

Architecture should remain implementation independent whenever practical.

Architecture should describe:

* responsibilities

instead of

* frameworks

Architecture may reference technologies only when they are architecturally significant.

Example:

Acceptable

* Electron Main Process
* Browser Process
* Plugin Runtime

Not acceptable

* React Hooks
* Axios
* SQLite API
* Rust syntax

Technology selection belongs in Engineering Documentation.

---

## Cross-Repository Architecture

If a repository depends on another repository:

Architecture should define:

* ownership boundaries
* interaction contracts
* communication model

Architecture should not duplicate another repository's architecture.

Instead, reference the relevant documentation.

---

## Quality Requirements

Architecture Documentation should be:

* Modular
* Cohesive
* Traceable
* Technology appropriate
* Responsibility driven
* Consistent
* Maintainable
* Scalable
* Reviewable

Architecture should evolve through decomposition rather than document growth.

---


## Crate Architecture

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/12-crate_architecture.yaml`*

### Template

> **minimum_content:** 2 paragraphs + 1 diagram
> **length_guidance:** moderate
> **diagram_requirements:** Architecture diagram showing Cargo workspace and crates

```markdown
This system is organized as a Cargo workspace with distinct crates enforcing architectural boundaries.

[Describe the workspace structure and the responsibility of each crate.]

```mermaid
graph TD
    App[app_runtime] --> Core[core_domain]
    Infrastructure[infra_db] --> Core
    API[api_server] --> Core
```

## Dependency Direction

[Specify the rules for dependency direction between crates (e.g., infrastructural crates depend on core, never the reverse).]
```

**Required subsections:** Dependency Direction
**Optional subsections:** none
**Required diagrams:** Workspace Architecture Diagram
**Required cross-references:** Component Model(03)

### Examples

**Correct:**
> The `core` crate contains all business logic and traits, and has zero dependencies. The `postgres_impl` crate implements those traits and depends on `core`.

**Incorrect:**
> We just put everything in one crate.
> *Why wrong: Violates the modular crate architecture requirement for systems engineering.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs and mermaid diagrams
- **Audience:** systems engineer
- **Do:** Enforce strict dependency direction.
- **Don't:** Allow circular dependencies or tightly coupled crates.

---

## Trait Design

> *Structural rules: `common/tier2/audit/deterministic/section/architecture/13-trait_design.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
The system relies on trait-based abstraction to decouple implementation details from business logic.

[Describe the core traits that define the system's external interfaces (e.g., repositories, external services).]

## Generic Constraints

[Explain how generic bounds are used in the core domain to depend on traits rather than concrete types.]
```

**Required subsections:** Generic Constraints
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> The `UserRepository` trait defines the data access contract. The `UserService<R: UserRepository>` struct uses this trait, allowing the Postgres implementation to be injected at runtime.

**Incorrect:**
> The `UserService` uses the `PostgresUserRepository` struct directly.
> *Why wrong: Fails to use trait-based abstraction.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** systems engineer
- **Do:** Mandate trait usage for all external dependencies.
- **Don't:** Use concrete types in core domain signatures.

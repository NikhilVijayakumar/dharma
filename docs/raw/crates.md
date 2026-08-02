# Crate Standard

> *New doc type for this repository — no upstream Kriti `rust_dev` domain file numbers this separately; it fills a gap the existing 5-file subset (`architecture.md`, `build.md`, `engineering.md`, `implementation.md`, `qa.md`) left uncovered for a Cargo workspace made of more than one crate.*

## Table of Contents
- [Purpose](#purpose)
- [Crate Overview](#crate-overview)
- [Responsibility](#responsibility)
- [Dependencies](#dependencies)
- [Public Interface](#public-interface)
- [Constraints](#constraints)
- [Required Sections](#required-sections)
- [Goals](#goals)
- [Non-Goals](#non-goals)
- [Success Criteria](#success-criteria)
- [Scope](#scope)
- [Out of Scope](#out-of-scope)
- [Traceability](#traceability)
- [Relationships](#relationships)
- [Audit Rules](#audit-rules)
- [Common Mistakes](#common-mistakes)
- [Documentation Folder](#documentation-folder)
- [Usage](#usage)
- [Related](#related)

---

## Purpose

This document defines the standard for Crate Documentation within the engineering documentation ecosystem. Crate Documentation describes a single Cargo workspace member — its responsibility, its dependency boundary, and the interface it exposes to the rest of the workspace.

Unlike Architecture Documentation, which describes the whole workspace's crate graph in one collection (see `architecture.md`'s "Crate Architecture" section), a Crate document covers exactly one crate. Unlike Engineering Documentation, it does not justify technology choices; unlike Implementation Documentation, it does not describe function bodies.

### Examples

**Correct:**
> This document defines the standard for Crate Documentation within the engineering documentation ecosystem. Crate Documentation describes one Cargo workspace member's responsibility, dependency boundary, and public interface. Unlike Architecture Documentation, which describes the full crate graph as a collection, a Crate document is scoped to a single crate.

**Incorrect:**
> This document explains what the `registry` crate does, listing its structs, functions, and the SQL it runs.
> *Why wrong: describes one crate's content instead of stating what the Crate document type covers and how it differs from Architecture Documentation — that belongs in the sections below, not the Purpose statement.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** systems engineer
- **Do:** State that a Crate document covers exactly one workspace member; keep the boundary with Architecture (whole-graph) and Implementation (function bodies) explicit
- **Don't:** Name a specific crate's contents; describe function signatures or struct fields

---

## Crate Overview

> *Structural rules for this section mirror `architecture.md`'s System Overview.*

### Template

> **minimum_content:** 1 paragraph
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Crate Overview

> [metadata block]

`<crate-name>` is [one-sentence role in the workspace]. It sits at [layer] in
the dependency direction defined by the workspace's Crate Architecture (see
architecture.md).
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Architecture(05), specifically its Crate Architecture section

### Examples

**Correct:**
> `registry` owns SQLite migrations and typed access for both physical databases (`mcp.db`, `repo.db`). It sits between `schemas`/`common` (below) and `services` (above) in the workspace's dependency direction.

**Incorrect:**
> `registry` has a `RegistryError` enum, a `Connection` struct with a `pool: r2d2::Pool` field, and twelve public functions.
> *Why wrong: lists implementation members instead of stating the crate's one-sentence role and its position in the dependency direction — that belongs in Responsibility and Dependencies, or in Implementation Documentation, not here.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** systems engineer
- **Do:** Open with the crate's one-sentence role; state its layer in the dependency direction
- **Don't:** List structs, functions, or types; restate the whole workspace's Crate Architecture

---

## Responsibility

### Template

> **minimum_content:** 1 paragraph + 1 bullet list
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Responsibility

> [metadata block]

`<crate-name>` owns:
- [thing 1 it owns]
- [thing 2 it owns]

`<crate-name>` explicitly does not own:
- [thing owned by a different crate instead]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> `registry` owns:
> - SQLite migrations for `mcp.db` and `repo.db`
> - Every cross-database logical-reference validation named in `schema/`'s comments
>
> `registry` explicitly does not own:
> - Business logic (owned by `services`)
> - JSON Schema validation rules (owned by `schemas`, though `registry` calls into it)

**Incorrect:**
> `registry` does database stuff.
> *Why wrong: too vague to audit against — doesn't name what it owns or what it explicitly excludes.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** systems engineer
- **Do:** Name what the crate owns and, at least once, what it explicitly does not own
- **Don't:** Describe how ownership is implemented internally

---

## Dependencies

### Template

> **minimum_content:** 1 list + Dependency Direction statement
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Dependencies

> [metadata block]

`<crate-name>` depends on: [internal workspace crates only — external crates.io
dependencies belong in Engineering/Build Documentation, not here]

### Dependency Direction

[State which crates may depend on this one, and which this one may never
depend on, per the workspace's Crate Architecture.]
```

**Required subsections:** Dependency Direction
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Architecture(05)'s Crate Architecture section

### Examples

**Correct:**
> `registry` depends on: `common`, `schemas`.
>
> **Dependency Direction:** `services` may depend on `registry`; `registry` may never depend on `services`, `cli`, or `mcp`. This crate has no reverse dependency.

**Incorrect:**
> `registry` uses `rusqlite = "0.31"` and `serde_json = "1"`.
> *Why wrong: lists external crates.io dependencies and version pins — that's Build Documentation's concern (`Cargo.toml`), not this crate's workspace-internal dependency boundary.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** systems engineer
- **Do:** List only internal workspace crates; state the dependency direction both ways (who this depends on, who may depend on this)
- **Don't:** List crates.io dependencies or version numbers; leave the direction implicit

---

## Public Interface

### Template

> **minimum_content:** 1 list
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Public Interface

> [metadata block]

`<crate-name>` exposes, to the crates that depend on it:
- [capability 1, described by role not by function signature]
- [capability 2]

`<crate-name>` does not re-export: [anything a dependent must not reach past this crate to get]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> `registry` exposes: typed read/write operations per table, one store trait per schema area (e.g. `DomainSystemRegistryStore`, `ExecutionStore`).
>
> `registry` does not re-export: the underlying SQLite connection type — `services` never constructs or holds one directly.

**Incorrect:**
> `pub fn insert_domain_system(conn: &Connection, name: &str, version: &str) -> Result<i64, RegistryError>`
> *Why wrong: this is a function signature — Implementation Documentation's concern. Public Interface states what capability is exposed, not its exact signature.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** systems engineer
- **Do:** Describe exposed capability by role; state what is deliberately not re-exported
- **Don't:** Paste function signatures or type definitions

---

## Constraints

### Template

> **minimum_content:** 1 list of constraints
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Constraints

> [metadata block]

### Hard Constraints
- [constraint] (source: [Architecture / Security / this crate's own Rationale])
```

**Required subsections:** Hard Constraints
**Optional subsections:** Soft Constraints
**Required diagrams:** none
**Required cross-references:** Architecture(05)

### Examples

**Correct:**
> **Hard Constraints**
> - **Sole SQLite access** (source: Architecture, Trust Boundaries) — no other crate may open a SQLite connection.

**Incorrect:**
> Try to keep queries fast.
> *Why wrong: unsourced, unattributed preference — not a constraint an audit can check.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** bullet lists
- **Audience:** systems engineer
- **Do:** Attribute every constraint to its source document
- **Don't:** State an unsourced preference as if it were a hard constraint

---

## Required Sections

Every Crate document must contain the following sections. Sections are identified by heading text.

| Section | semantic_type | Required | Content Requirements |
|---------|--------------|----------|---------------------|
| Crate Overview | `crate_overview` | ✓ | One-sentence role, position in dependency direction |
| Responsibility | `crate_responsibility` | ✓ | What this crate owns, what it explicitly does not own |
| Dependencies | `crate_dependencies` | ✓ | Internal workspace dependencies, Dependency Direction statement |
| Public Interface | `crate_public_interface` | | Capability exposed to dependents, by role not signature |
| Constraints | `crate_constraints` | | Sourced hard/soft constraints |

Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

Crate Documentation aims to:

* Give each workspace member a single authoritative responsibility statement.
* Make the dependency direction between crates explicit and checkable.
* Let a reader decide "which crate should this new code go in" without reading source.

---

## Non-Goals

Crate Documentation does not attempt to define:

* The whole workspace's crate graph (that is Architecture's Crate Architecture section)
* Function signatures or struct/enum definitions (Implementation)
* crates.io dependency choices or version pins (Build/Engineering)
* Test strategy (QA)

---

## Success Criteria

Crate Documentation is successful when:

* A new contributor can tell which crate owns a given responsibility without reading source.
* Dependency-direction violations are checkable against this document, not just against `Cargo.toml`.
* No two crates' Responsibility sections claim the same ownership.

---

## Scope

A Crate document may include: Crate Overview, Responsibility, Dependencies, Public Interface, Constraints. Projects should write one Crate document per workspace member once that crate's responsibility has stabilized — not before, and not retrofitted from source code line-by-line.

---

## Out of Scope

A Crate document must not describe: function bodies, struct/enum field lists, external dependency version pins, build scripts, test cases.

---

## Traceability

### Derivation Chain

```text
Architecture (Crate Architecture section: the whole workspace's crate graph)
    │
    ▼
Crate Documentation (this standard) — one document per workspace member
    │
    ▼
Implementation (function-level detail for that crate)
```

**Non-contradiction rule:** No Crate document may describe a dependency direction that contradicts Architecture's Crate Architecture section. When a Crate document needs a dependency Architecture doesn't already permit, Architecture is updated first.

**Required cross-references:** Architecture(05)

---

## Relationships

| Document | Relationship |
|---|---|
| Architecture | A Crate document is constrained by Architecture's Crate Architecture section; it may not introduce a dependency direction Architecture doesn't permit |
| Implementation | Implementation covers function-level detail for a crate already described here |
| Engineering | Engineering justifies crates.io dependency choices; Crate Documentation only lists which internal crates are used |

---

## Audit Rules

An audit should verify:

* Every workspace member (`Cargo.toml` `[workspace] members`) has a Crate document, or is explicitly marked exempt.
* No two Crate documents claim the same Responsibility ownership.
* Every Dependencies section's Dependency Direction matches Architecture's Crate Architecture graph.
* No Crate document contains a function signature, struct field list, or crates.io version pin.

---

## Common Mistakes

* Restating source code (struct fields, function signatures) instead of responsibility and interface.
* Listing crates.io dependencies instead of workspace-internal ones.
* Writing one Crate document that actually covers two crates, or duplicating Architecture's whole-graph diagram inside a single-crate document.
* Leaving Dependency Direction implicit instead of stating both directions (who this depends on, who may depend on this).

---

## Documentation Folder

Crate documents live under:

```text
docs/raw/crates/
```

One file per workspace member, named after the crate (e.g. `docs/raw/crates/registry.md`).

---

## Usage

Written by the engineer who stabilizes a crate's responsibility boundary; read by anyone deciding which crate new code belongs in, and by anyone reviewing whether a proposed change crosses a dependency-direction boundary this document forbids.

## Related

- [Architecture Standard](architecture.md) — Crate Architecture section constrains every Crate document
- [Implementation Standard](implementation.md) — covers function-level detail for a crate already described here
- [Engineering Standard](engineering.md) — justifies crates.io dependency choices this document only lists by name

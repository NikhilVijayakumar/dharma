# Proposal Standard

> *New doc type for this repository, in the same spirit as `crates.md` — no upstream Kriti `rust_dev` domain file numbers a "proposal" type separately. A Proposal is an Architecture-standard document (see `architecture.md`) that additionally carries a tracked lifecycle: draft → finalized → implementing → verified → archived, each transition pinned to a git commit, recorded in `repo.db`'s `proposal_lifecycle` (see `docs/proposal/12-proposal-lifecycle-and-archival.md`).*

## Table of Contents
- [Purpose](#purpose)
- [Lifecycle](#lifecycle)
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

This document defines the standard for Proposal Documentation: a design document that gates implementation of one coherent unit of work, and whose lifecycle — draft, finalized, implementing, verified, archived — is tracked outside the document itself, pinned to git commits, so its history can always be replayed.

A Proposal is architecturally identical to an Architecture document (it must satisfy every section `architecture.md` requires — Purpose, System Overview, Component Model, Communication, Data Flow, Security, Rationale, Constraints, Traceability) plus one thing Architecture documents don't need: a place to state where this specific document currently stands in its own lifecycle, and what commits mark each transition.

### Examples

**Correct:**
> This document defines the standard for Proposal Documentation: an Architecture-standard document whose lifecycle — draft, finalized, implementing, verified, archived — is tracked and pinned to git commits, so a Proposal's history can always be replayed from stored rows plus the commits they point to.

**Incorrect:**
> This proposal describes the schema for the new caching layer and why we need it.
> *Why wrong: describes one specific proposal's content instead of stating what the Proposal document type is and what makes it different from a plain Architecture document — that belongs in the proposal's own Purpose section, not this standard's.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** architect
- **Do:** State that a Proposal is an Architecture document plus a tracked lifecycle; name every required Architecture section this standard inherits
- **Don't:** Describe any single proposal's subject matter; restate Architecture's own Purpose section content

---

## Lifecycle

### Template

> **minimum_content:** 1 paragraph + status line
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Lifecycle

> Status: draft | finalized | implementing | verified | archived
> Draft commit: <hash or "not yet committed">
> Finalized commit: <hash or "not yet finalized">
> Implementation commit (final, verified): <hash or "not yet implemented">
> Archive commit: <hash or "not yet archived">

[1 short paragraph: what "finalized" means for THIS proposal — i.e. what
must be true before implementation may begin against it.]
```

**Required subsections:** none — the status line block itself is the required content
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> Status: implementing
> Draft commit: `a1b2c3d`
> Finalized commit: `e4f5a6b`
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived
>
> Finalized means: the schema changes this proposal specifies exist in `schema/` and have been reviewed for gaps at least once.

**Incorrect:**
> This proposal is basically done, just need to write the code.
> *Why wrong: no status line, no commit hashes — unreplayable and unqueryable. The Lifecycle section exists specifically so a reader (or a tool) never has to guess.*

### Writing Guidance

- **Tone:** factual
- **Voice:** third person
- **Structure:** a status line block, then prose
- **Audience:** implementer, auditor
- **Do:** Keep the status line block's four fields present even when a value is "not yet X"; state what "finalized" concretely means for this proposal
- **Don't:** Omit a field because it's not yet filled in; describe implementation progress in prose instead of updating the status line

This section is a human-readable mirror of the same proposal's row in `repo.db`'s `proposal_lifecycle` table (see `docs/proposal/12-proposal-lifecycle-and-archival.md`) — the row is authoritative once implementation begins; this section is what a reader sees without querying the database.

---

## Required Sections

Every Proposal document must contain the following sections. Sections are identified by heading text.

| Section | semantic_type | Required | Content Requirements |
|---------|--------------|----------|---------------------|
| Purpose | `purpose` | ✓ | Root intent, why this proposal exists, its scope boundary — inherited from `architecture.md` |
| System Overview | `system_overview` | ✓ | Inherited from `architecture.md` |
| Component Model | `component_model` | ✓ | Inherited from `architecture.md` |
| Communication | `communication_paths` | ✓ | Inherited from `architecture.md` |
| Data Flow | `data_flow` | ✓ | Inherited from `architecture.md` |
| Security | `security_considerations` | ✓ | Inherited from `architecture.md` |
| Lifecycle | `proposal_lifecycle` | ✓ | Status line (draft/finalized/implementing/verified/archived) + commit hashes per transition |
| Rationale | `rationale` | | Inherited from `architecture.md` |
| Constraints | `constraints` | | Inherited from `architecture.md` |
| Traceability | `traceability` | | Inherited from `architecture.md` |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

Proposal Documentation aims to:

* Give every gated unit of implementation work a single authoritative design document, exactly as rigorous as an Architecture document.
* Make a proposal's current lifecycle state answerable without archaeology — from one status line, or one DB row.
* Let a finished proposal be archived without losing its history: draft, finalized, and final-implementation commits all remain queryable after the file moves to `archive/`.

---

## Non-Goals

Proposal Documentation does not attempt to define:

* Implementation code or function-level detail (Implementation Documentation's concern)
* The Task-level Proposal Loop's runtime drafts/revisions (`repo.db`'s `proposal_revision`/`proposal_approval`, see Proposal & Execution Protocol, 07) — that is a *different*, already-existing mechanism for one Agent proposing a solution to one Task at runtime; this standard governs *design* proposals like the documents in `docs/proposal/`, not that runtime loop.
* Build tooling or CI configuration.

---

## Success Criteria

Proposal Documentation is successful when:

* Every proposal that gated real implementation has a `proposal_lifecycle` row with all four commit fields eventually filled.
* No proposal is archived without a recorded, verified implementation commit.
* A reader can determine a proposal's exact current state from its Lifecycle section alone, without asking anyone.

---

## Scope

A Proposal document is the full Architecture document set (per `architecture.md`'s Scope) plus the Lifecycle section this standard adds. Projects should write one Proposal per coherent, gate-worthy unit of work — not one per file change, not one covering unrelated concerns.

---

## Out of Scope

A Proposal document must not describe: source code, function signatures, CI/build scripts, or the Task-level Proposal Loop's runtime state (see Non-Goals).

---

## Traceability

### Derivation Chain

```text
Architecture (docs/raw/architecture.md) — every required section this standard inherits
    │
    ▼
Proposal Standard (this document) — adds the Lifecycle section + repo.db tracking
    │
    ▼
Individual Proposal documents (docs/proposal/*.md) — one per gated unit of work
    │
    ▼
Implementation, gated by each proposal's "finalized" commit;
archived (docs/proposal/archive/) once its implementation commit is verified
```

**Non-contradiction rule:** No Proposal document may be archived without a `proposal_lifecycle` row whose implementation commit is marked verified. No downstream tooling may treat a proposal's Lifecycle section as authoritative once implementation has begun — the `repo.db` row is authoritative from that point on.

**Required cross-references:** Architecture(05, in the tier sense used by `architecture.md`)

---

## Relationships

| Document | Relationship |
|---|---|
| Architecture | A Proposal document must satisfy every section Architecture requires; Proposal adds only the Lifecycle section |
| Implementation | Implementation work is gated by a Proposal's "finalized" commit and closes it out with the "verified" implementation commit |
| Proposal & Execution Protocol (07) | Governs a *different* mechanism — the Task-level Proposal Loop — not to be confused with this document-level standard (see Non-Goals) |

---

## Audit Rules

An audit should verify:

* Every file under `docs/proposal/` numbered 12 or later (excluding `00-overview.md` and `archive/`) satisfies Architecture's Required Sections plus this standard's Lifecycle section.
* Proposals 01-11 predate this standard's adoption (`docs/proposal/00-overview.md`, Build Order step 5): grandfathered — no retroactive `proposal_lifecycle` rows, missing Lifecycle sections are not audit failures.
* Every proposal with a `repo.db` `proposal_lifecycle` row past `draft` has a non-null `draft_commit_hash`.
* No `proposal_lifecycle` row is `archived` without a verified `implementation_commit_hash`.
* No archived proposal's file is missing from `docs/proposal/archive/`.

---

## Common Mistakes

* Treating the Lifecycle section as optional or as free-form prose instead of the required status-line-plus-commits block.
* Confusing this standard with the Task-level Proposal Loop (07) — they share a name, not a mechanism.
* Archiving a proposal file without a corresponding `proposal_lifecycle` row recording the archive commit.
* Letting the Lifecycle section's status line drift out of sync with the actual `repo.db` row once implementation has begun (the DB row is authoritative from that point).

---

## Documentation Folder

Proposal documents live under:

```text
docs/proposal/
```

Archived proposals move to `docs/proposal/archive/`, keeping their original filename, once their `proposal_lifecycle` row reaches `archived`.

The standard itself (this file) lives at `docs/raw/proposal.md` — a root raw standard, next to `architecture.md` and `crates.md`. There is no `docs/raw/proposal/` directory: "proposal" is a document type (instances live in `docs/proposal/`), unlike "crates" which is a doc-family folder (`docs/raw/crates/`).

---

## Usage

Written by whoever is designing a gated unit of work, before implementation starts; read by whoever implements it (to know what "finalized" requires), by whoever verifies it (to know what "done" means), and by anyone later auditing what happened (via the Lifecycle section or the `proposal_lifecycle` row it mirrors).

## Related

- [Architecture Standard](architecture.md) — every section this standard requires
- [Crates Standard](crates.md) — the other doc type this repository added beyond the inherited five, same house style
- [Proposal 12 — Proposal Lifecycle & Archival](../proposal/12-proposal-lifecycle-and-archival.md) — the `repo.db` schema this standard's Lifecycle section mirrors

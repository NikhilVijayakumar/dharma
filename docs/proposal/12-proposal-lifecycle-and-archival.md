# Proposal: Proposal Lifecycle & Archival

> Status: Draft — schema is concrete (see `schema/repo/15-proposal_lifecycle.sql`, `16-proposal_commit_log.sql`), no implementation code yet. Conforms to `docs/raw/proposal.md` standard (itself new in this revision) — this is the first document required to carry a Lifecycle section, and it does (below).
> **New in this revision.** Formalizes "proposal" as a first-class, lifecycle-tracked document type: draft → finalized → implementing → verified → archived, every transition pinned to a git commit, recorded in `repo.db` — not the Task-level Proposal Loop (07), a separate, already-existing mechanism this document does not touch.

## Purpose

This document defines the standard for how a Proposal document (per `docs/raw/proposal.md`) moves through its lifecycle in a tracked, git-commit-pinned, backtrackable way, and how a finished proposal is archived without losing that history.

Unlike leaving a proposal's status as tribal knowledge ("I think we implemented that one"), Dharma records it: which commit first introduced the proposal, which commit finalized its design for implementation, which commit completed and verified the implementation, and which commit archived the file — each queryable from `repo.db`, independent of anyone's memory of what happened.

## System Overview

### Overview

Every Proposal document (`docs/proposal/NN-name.md`) gets exactly one `proposal_lifecycle` row in the repository's own `repo.db` — the same database that already holds Task Instance runtime state and audit executions, because a proposal's lifecycle is a per-repo, per-commit observation, not global platform content (mirroring the audit-execution split in proposal 08). The row's `status` only moves forward — draft, finalized, implementing, verified, archived — and each forward move requires the commit hash that justifies it. A full commit history (every commit touched along the way, not just the milestones) is kept in `proposal_commit_log`.

### Structural Approach

Two tables, one relationship: `proposal_lifecycle` is the one-row-per-proposal milestone snapshot; `proposal_commit_log` is its append-only detail. Neither table knows anything about a proposal's *content* — that lives in the Markdown file itself, governed by `docs/raw/proposal.md`. The two are linked only by the proposal's `name` (matching its filename slug) and, once implementation begins, by commit hashes a human or a CI hook records.

### Diagram

```text
docs/proposal/12-name.md  (content, governed by docs/raw/proposal.md)
        │
        │ same `name` slug
        ▼
repo.db: proposal_lifecycle (1 row)  ──milestones──▶  draft / finalized / implementing / verified / archived
        │
        │ 1:N
        ▼
repo.db: proposal_commit_log  (every commit touched, phase-tagged)
```

## Component Model

### `proposal_lifecycle` Row
- **Responsibility:** Tracks one proposal's current status and the four milestone commit hashes (draft, finalized, implementation/final, archive).
- **Ownership:** `status`, the four commit-hash columns, `verified_at`, `archived_at`, `doc_path` (which moves once archived).
- **Interfaces:** Written by whoever advances a proposal's status (a human, or a tool acting on their instruction); read by anyone auditing where a proposal stands, and by the Proposal document's own Lifecycle section (kept in sync by hand or by tooling, but the DB row is authoritative once implementation begins, per `docs/raw/proposal.md`).

### `proposal_commit_log` Entry
- **Responsibility:** Records one commit touched during a proposal's draft, finalized, or implementation phase, phase-tagged.
- **Ownership:** `commit_hash`, `phase`, `message`, `recorded_at`.
- **Interfaces:** Appended to on every commit relevant to the proposal; read when replaying "what actually happened" beyond the four milestone snapshots.

### Archival Step
- **Responsibility:** Once a proposal reaches `verified`, moves its file from `docs/proposal/` to `docs/proposal/archive/`, records the moving commit as `archive_commit_hash`, and flips `status` to `archived`.
- **Ownership:** No schema of its own — a `services`-layer operation (per proposal 08's crate split) that updates `proposal_lifecycle.doc_path`, `archive_commit_hash`, `archived_at`, and `status` together.
- **Interfaces:** Invoked once, manually or via a tool, after verification; irreversible in the sense that `status` does not move backward from `archived`.

### Component Diagram

```text
proposal_lifecycle ──1:N──▶ proposal_commit_log
        │
        │ status transition (forward only)
        ▼
Archival Step ──moves file──▶ docs/proposal/archive/ ──records──▶ archive_commit_hash, archived_at
```

## Communication

### Communication Paths

**Author → `proposal_lifecycle`**
- **Pattern:** Synchronous write, at each lifecycle transition.
- **Contract:** Author (or a tool acting on their instruction) advances `status` and supplies the commit hash the new status requires; a transition missing its required commit hash is rejected by the table's own `CHECK` constraint.

**Author → `proposal_commit_log`**
- **Pattern:** Synchronous append, on every relevant commit.
- **Contract:** Each commit touched during the `draft`, `finalized`, or `implementing` phase is appended as one row, phase-tagged; never edited or deleted afterward.

### Communication Diagram

```text
Author → proposal_lifecycle : advance(status, commitHash)
proposal_lifecycle → Author : accepted | rejected(missingCommitHash)
Author → proposal_commit_log : append(commitHash, phase, message)
```

## Data Flow

### Data Paths

**Lifecycle Advancement Path**
- **Entry point:** A proposal document exists (at minimum, in `draft` status with no commit yet).
- **Transformations:** Each forward status move (draft→finalized→implementing→verified→archived) writes the milestone commit hash the `CHECK` constraint requires for that status.
- **Ownership boundary:** `proposal_lifecycle` owns the milestone snapshot; `proposal_commit_log` owns the full detail underneath it.
- **Exit point:** An `archived` row with all four commit hashes populated, and the file moved to `docs/proposal/archive/`.

### Data Flow Diagram

```text
draft ──(draft_commit_hash)──▶ finalized ──(finalized_commit_hash)──▶ implementing
                                                                          │
                                                          (implementation_commit_hash, verified_at)
                                                                          ▼
                                                                      verified
                                                                          │
                                                          (archive_commit_hash, archived_at)
                                                                          ▼
                                                                      archived
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Proposal document content | The Markdown file itself, governed by `docs/raw/proposal.md` |
| `proposal_lifecycle` row | `repo.db`, one per proposal, written at each transition |
| `proposal_commit_log` rows | `repo.db`, append-only, one per relevant commit |

## Security

### Trust Boundaries

- **Author → `proposal_lifecycle`:** Trusted — whoever can commit to this repository can advance a proposal's lifecycle; there is no separate authorization layer beyond the repository's own commit access.
- **Forward-only status:** A structural boundary, not a permission boundary — the `CHECK` constraint prevents skipping a required commit hash, but does not prevent an authorized author from advancing status prematurely in good faith.

### Threat Model

- **Status advanced without the commit that justifies it:** Someone marks a proposal `verified` without an `implementation_commit_hash`. Mitigation: the table's own `CHECK` constraint rejects this at the database layer, not just by convention.
- **Archiving before verification:** A proposal's file is moved to `archive/` while still `implementing`. Mitigation: the Archival Step only runs against rows already in `verified` status (per `docs/raw/proposal.md`'s Non-Contradiction Rule); the `CHECK` constraint additionally requires the full milestone chain — `draft_commit_hash`, `finalized_commit_hash`, `implementation_commit_hash`, and `verified_at` — before `archived` can be set.
- **Milestone snapshot drifting from actual history:** `proposal_lifecycle`'s four commit hashes look plausible but don't match what `proposal_commit_log` actually recorded. Mitigation: an audit can cross-check that each milestone hash in `proposal_lifecycle` also appears as a row in `proposal_commit_log` with the matching `phase` (`draft` / `finalized` / `implementation` / `archive`).

## Lifecycle

> Status: draft
> Draft commit: `6cc5919`
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means: `schema/repo/15-proposal_lifecycle.sql` and `16-proposal_commit_log.sql` exist and have been reviewed for gaps at least once (this document's own review pass), and `docs/raw/proposal.md` exists as the governing standard — both true as of this draft.

## Rationale

### `repo.db`, Not `mcp.db`
- **Context:** A proposal's lifecycle is tied to one repository's own git commits — the same shape as an audit execution (proposal 08: "Executions... are per-repo, per-commit observations... live in `repo.db`").
- **Decision:** `proposal_lifecycle` and `proposal_commit_log` live in `repo.db`, not `mcp.db`.
- **Alternatives Considered:** A global proposal registry in `mcp.db`, tracking every repository's proposals in one place.
- **Rejection Reason:** A proposal's commits only mean something relative to the one repository they were made in; a global table would need a repository reference on every row anyway, reproducing the same per-repo scoping `repo.db` already provides for free by being its own file. This also reuses a decision already made and reviewed for audit executions, rather than inventing a new split for a structurally identical case.
- **Architectural Goal:** Consistency with the established `mcp.db` (global platform content) vs. `repo.db` (per-repo, per-commit observation) split.

### Forward-Only Status, Enforced by `CHECK`
- **Context:** A proposal's lifecycle should not silently un-verify or un-finalize; the milestone commit hashes should not exist for a status that hasn't actually been reached.
- **Decision:** `status` transitions are validated by a `CHECK` constraint requiring the commit hash(es) each status implies.
- **Alternatives Considered:** Leave ordering and commit-hash presence to convention / application-level checks only.
- **Rejection Reason:** The same principle Philosophy(10) states generally — voluntary restraint is not verifiable — applies here: a convention can be forgotten under time pressure; a `CHECK` constraint cannot.
- **Architectural Goal:** A proposal's recorded status is always backed by the commit that justifies it.

### Milestones Plus Full Log, Not Either Alone
- **Context:** A quick "what state is this proposal in" check needs one row; a full audit of "what actually happened" needs every commit, not just the endpoints.
- **Decision:** `proposal_lifecycle` keeps the four-milestone snapshot; `proposal_commit_log` keeps the full append-only detail.
- **Alternatives Considered:** Only the milestone snapshot (lose intermediate history); only the full log (lose the fast "current state" answer).
- **Rejection Reason:** Either alone forces a tradeoff between quick status checks and full backtrackability that keeping both avoids entirely, at the cost of one small extra table.
- **Architectural Goal:** Both a fast status answer and full backtrackability, from the same feature.

## Constraints

### Hard Constraints
- **Status transitions require their milestone commit hash** (source: Rationale above) — enforced by `proposal_lifecycle`'s `CHECK` constraint, not by convention.
- **No archival before verification** (source: Threat Model above) — the Archival Step only runs against `verified` rows; `archived` additionally requires the full chain: draft, finalized, and implementation commit hashes plus `verified_at`.
- **`proposal_commit_log` is append-only** (source: Component Model above) — a recorded commit is never edited or deleted.

### Soft Constraints
- Prefer recording a `proposal_commit_log` entry for every commit that meaningfully advances a proposal, not just the four milestones, so later replay has real texture rather than four bare hashes.

## Traceability

### Derivation Chain

```text
docs/raw/proposal.md (the doc-type standard this document conforms to)
    │
    ▼
Proposal Lifecycle & Archival (this document)
    │
    ▼
(terminal proposal — every future docs/proposal/*.md document gets a
 proposal_lifecycle row once this is implemented)
```

### Non-Contradiction Rule

No downstream proposal may archive a document without a `verified` `proposal_lifecycle` row backing it, advance `status` without its required commit hash, or track proposal lifecycle state in `mcp.db` instead of `repo.db`, without revising this document first.

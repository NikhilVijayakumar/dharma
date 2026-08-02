# Crate: schemas

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`schemas` is the workspace's validation layer: JSON Schema definitions and the single validation entry point for every JSON-shaped column in `schema/`. It sits between `common` (below) and `registry`/`services` (above) in the workspace's dependency direction.

## Responsibility

`schemas` owns:
- JSON Schema documents for every JSON-shaped column in `schema/` — Task Input/Output Contracts, Acceptance Criteria, Skill Invocation Contracts, proposal drafts, Context Envelope payloads, Section Map/Profile JSON payloads, and audit evidence JSON — plus captured-YAML structure validation at capture time
- The validation entry point applied before any JSON-shaped write is committed

`schemas` explicitly does not own:
- SQLite migrations or table definitions (owned by `registry`; `schema/` is the canonical reference copy)
- Business logic (owned by `services`)

## Dependencies

`schemas` depends on: `common`.

### Dependency Direction

`registry` may depend on `schemas` — its enforcement boundary, where every write is validated before commit regardless of caller. `services` may depend on `schemas` as defense-in-depth only, for early feedback before a call even reaches `registry`; that use is not itself a security boundary. `schemas` may never depend on `registry`, `services`, `cli`, or `mcp`.

## Public Interface

`schemas` exposes, to the crates that depend on it:
- JSON Schema documents per JSON-shaped column, and a validation entry point that rejects non-conforming payloads

`schemas` does not re-export: any storage or migration type — validation is reached as a function call, not through schema internals.

## Constraints

### Hard Constraints
- **Validation before commit** (source: 08, registry is the single enforcement point) — `registry` must validate every JSON-shaped payload against `schemas` before committing a row.
- **Defense-in-depth is not enforcement** (source: 08, Threat Model) — `services`' use of `schemas` is early feedback only; it never replaces `registry`'s enforcement call.

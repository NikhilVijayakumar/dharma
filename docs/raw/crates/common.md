# Crate: common

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`common` is the workspace's foundation layer: the zero-dependency primitives every other crate builds on. It sits at the bottom of the dependency direction defined by the workspace's Crate Architecture — the only crate nothing may depend below.

## Responsibility

`common` owns:
- Error types shared across the workspace
- Environment / configuration resolution
- Filesystem helpers
- ID generation
- Shared traits

`common` explicitly does not own:
- JSON Schema definitions or validation (owned by `schemas`)
- SQLite migrations and typed storage access (owned by `registry`)
- Business logic (owned by `services`)

## Dependencies

`common` depends on: none — it is the workspace's only zero-dependency crate.

### Dependency Direction

Every internal crate may depend on `common`; `common` may never depend on any internal crate. It has no reverse dependency.

## Public Interface

`common` exposes, to the crates that depend on it:
- Shared error and configuration types used across crate boundaries
- Filesystem and ID-generation helpers

`common` does not re-export: nothing — it is the workspace leaf, so there is nothing reachable past it.

## Constraints

### Hard Constraints
- **Zero internal dependencies** (source: Architecture, Crate Architecture section) — `common` must never gain a dependency on another workspace crate, or every crate's dependency graph grows by that edge.
- **No storage access** (source: 08, Trust Boundaries) — `common` never opens a SQLite connection, directly or transitively; storage is `registry`'s alone.

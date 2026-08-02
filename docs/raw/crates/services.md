# Crate: services

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`services` is the workspace's business-logic layer: it implements proposals 01-07's behavior by orchestrating `registry` calls. It sits between `registry`/`schemas`/`common` (below) and `cli`/`mcp` (above) in the workspace's dependency direction.

## Responsibility

`services` owns:
- Content capture from provider files (recording every captured file in `content_asset` and parsing its shape into rows)
- Repository registration
- Domain/Agent System resolution (Default/Bootstrap Agent System logic), backed by `analysis_cache`
- Sync-to-repo (seeding): copying the required Domain/Agent content into a repo's `repo.db` and running the seeder scripts
- Proposal Loop drafting and revision
- Handoff Broker resolution
- Completion Validator checks
- Audit orchestration: deterministic rule runs, per-model semantic runs, aggregation by weights, override/cancel, evidence persistence, and report rendering

`services` explicitly does not own:
- SQLite migrations and typed storage access (owned by `registry`)
- Entry points — CLI or MCP protocol adaptation (owned by `cli` and `mcp`)
- JSON Schema document definitions (owned by `schemas`)

## Dependencies

`services` depends on: `registry`, `common`, and (defense-in-depth only, not an enforcement boundary) `schemas`.

### Dependency Direction

`cli` and `mcp` may depend on `services`; `services` may never depend on `cli` or `mcp`. Neither entry point may call `registry` directly, bypassing `services`.

## Public Interface

`services` exposes, to the crates that depend on it:
- The operations both `cli` and `mcp` call — content capture, repository registration, Domain/Agent System resolution, sync/seed, Task assignment, proposal submission/approval, handoff, audit invocation
- Orchestration functions generic over the store traits, so a test can inject an in-memory fake store without spinning up SQLite, and the production binary injects the real `registry` implementation at startup

`services` does not re-export: `registry`'s store traits as part of its public surface — entry points reach storage only through `services`.

## Constraints

### Hard Constraints
- **Entry points share `services`** (source: 08, Crate Architecture) — `cli` and `mcp` both call the same `services` functions; neither re-implements business logic, so behavior cannot drift between the two entry points.
- **Never opens a SQLite connection** (source: 08, Trust Boundaries) — `services` calls `registry`'s typed functions; it never constructs or holds a connection.

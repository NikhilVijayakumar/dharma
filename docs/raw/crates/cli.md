# Crate: cli

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`cli` is one of the workspace's two entry points: a command-line interface for administrative operations. It sits at the top of the workspace's dependency direction, depending on `services` and nothing above it.

## Responsibility

`cli` owns:
- The command-line entry point for administrative operations — registering a Domain System or Agent System, capturing provider files, inspecting registries, running migrations, running seeders

`cli` explicitly does not own:
- Business logic (owned by `services`)
- SQLite migrations and typed storage access (owned by `registry`)
- The MCP protocol server (owned by `mcp`)

## Dependencies

`cli` depends on: `services` (transitively on everything below).

### Dependency Direction

Nothing may depend on `cli`. `cli` may never depend on `mcp`, and may never call `registry` directly, bypassing `services`.

## Public Interface

`cli` exposes, to the operator running it:
- A command-line surface for administration, backed by `services` operations

`cli` does not re-export: `services` or `registry` types — it is a terminal entry point, not a library.

## Constraints

### Hard Constraints
- **`main` used for administration only** (source: 08, Component Model) — `cli` is the only crate with a `main` used for administrative operations.
- **No direct `registry` access** (source: 08, Trust Boundaries) — `cli` may not depend on `registry`, which would skip `services`' validation and the Agent-Management authorization check.

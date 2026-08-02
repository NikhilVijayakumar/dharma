# Crate: mcp

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`mcp` is one of the workspace's two entry points: the MCP protocol server exposing Dharma's operations as tools. It sits at the top of the workspace's dependency direction, depending on `services` and nothing above it.

## Responsibility

`mcp` owns:
- The MCP protocol server exposing Dharma's operations as tools — repository registration, Task assignment, proposal submission/approval, handoff

`mcp` explicitly does not own:
- Business logic (owned by `services`)
- SQLite migrations and typed storage access (owned by `registry`)
- The administrative command-line surface (owned by `cli`)

## Dependencies

`mcp` depends on: `services` (transitively on everything below).

### Dependency Direction

Nothing may depend on `mcp`. `mcp` may never depend on `cli`, and may never call `registry` directly, bypassing `services`.

## Public Interface

`mcp` exposes, to external MCP clients:
- A tool surface for Dharma's operations, backed by `services` operations

`mcp` does not re-export: `services` or `registry` types — it is a terminal entry point, not a library.

## Constraints

### Hard Constraints
- **`main` used at runtime by MCP clients** (source: 08, Component Model) — `mcp` is the only crate with a `main` used at runtime.
- **Inputs are untrusted until validated** (source: 08, Trust Boundaries) — `mcp` sits at the boundary to external MCP clients; its inputs are untrusted until `services`/`schemas` validate them.
- **No direct `registry` access** (source: 08, Trust Boundaries) — `mcp` may not depend on `registry`, which would skip `services`' validation and the Agent-Management authorization check.

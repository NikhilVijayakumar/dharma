# Crate: registry

> Status: Draft — crate boundary is structural design only, no implementation code. Conforms to `docs/raw/crates.md` standard. Crate graph per `docs/proposal/08-schema-and-crate-architecture.md`.

## Crate Overview

`registry` is the workspace's storage layer: it owns SQLite migrations and typed access for both physical databases (`mcp.db`, `repo.db`). It sits between `schemas`/`common` (below) and `services` (above) in the workspace's dependency direction.

## Responsibility

`registry` owns:
- The `.sql` migration constants, mirrored from `schema/` — the canonical reference copy (see `schema/README.md`)
- Typed read/write operations per table, one store trait per schema area: `DomainSystemRegistryStore` (`domain_system_registry`), `DomainContentStore` (`section`/`section_profile`/`epic`/`usecase`/`task`/`task_step`, scoped by `domain_system_id`), `AgentSystemRegistryStore` (`agent_system_registry`), `AgentContentStore` (`agent`/`agent_goal`/`skill`/`skill_prompt`/`skill_script`/`skill_example`/`agent_skill_binding`, scoped by `agent_system_id`), `RegistrationStore` (`repo_registration`/`capability_manifest`), and `ExecutionStore` (all seven `repo.db` tables)
- The `repo.db` → `mcp.db` logical-reference validation named in `schema/`'s comments — the only cross-database boundary in this schema; every reference within `mcp.db` itself is a real `FOREIGN KEY`, needing no validation at this layer beyond what SQLite already enforces

`registry` explicitly does not own:
- Business logic (owned by `services`)
- JSON Schema validation rules (owned by `schemas`, though `registry` calls into it)
- Entry points (owned by `cli` and `mcp`)

## Dependencies

`registry` depends on: `common`, `schemas`.

### Dependency Direction

`services` may depend on `registry`; `registry` may never depend on `services`, `cli`, or `mcp`. It has no reverse dependency.

## Public Interface

`registry` exposes, to the crates that depend on it:
- Typed read/write operations per table
- One store trait per schema area, per the Trait Design section of 08

`registry` does not re-export: the underlying SQLite connection type — `services` never constructs or holds one directly.

## Constraints

### Hard Constraints
- **Sole SQLite access** (source: 08, Trust Boundaries) — `registry` is the only crate that opens a SQLite connection; `cli` and `mcp` may not depend on `registry` directly.
- **`schema/` is the canonical reference copy** (source: `schema/README.md`) — the `registry` crate's actual migrations must match it; divergence is a defect, not an acceptable variance.
- **Logical references validated before commit** (source: 08, Threat Model) — no `repo.db` write may reference an `mcp.db` row (`task`, `agent_system_registry`, `agent`) that doesn't exist.

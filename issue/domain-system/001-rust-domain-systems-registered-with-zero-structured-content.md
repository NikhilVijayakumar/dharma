# Issue: Both registered Rust Domain Systems have zero structured content

**Found:** 2026-08-04, verifying `mcp.db` (`/home/dell/.dharma/mcp.db`) directly (MCP `list_domain_systems` currently fails schema validation — see `docs/proposal/15-mcp-list-tool-result-shape-fix.md` — so this was checked via direct DB query instead).

## Claim being checked

User-stated assumption: a Domain System for Rust is available in Dharma. Confirmed true at the registry level — two rows exist:

| id | name | version | description |
|---|---|---|---|
| 1 | `rust_dev` | 1.0.0 | "Rust/Cargo documentation domain system projected from base_dev" |
| 2 | `rust-dev-domain` | 0.1.0 | "Rust systems-engineering documentation domain: 13 domains with deterministic+semantic audit content modeled as ordinary Usecases/Tasks, a greenfield/brownfield propose-then-execute workflow (82 usecases across 4 Epics), projected from Kriti's samgraha rust_dev standard." |

## What's actually broken

Both rows are registry-only. Every table that should hold the structured tree the description promises is **empty** for both `domain_system_id`s:

| table | rows |
|---|---|
| `domain` | 0 |
| `section` | 0 |
| `section_profile` | 0 |
| `epic` | 0 |
| `usecase` | 0 |
| `task` | 0 |
| `task_step` | 0 |

`rust-dev-domain`'s own description claims "13 domains ... 82 usecases across 4 Epics" — none of that exists in the database right now. There is no domain, no epic, no usecase, no task to run, verify, or hand a Task Instance against.

What *does* exist: 402 rows in `content_asset` — raw captured files (`domain/map/*.yaml`, `domain/profile/*/*.yaml`, `SYSTEM.md`, a generator script) from whatever `recapture_domain_system` run produced them. These look like the *source* the structured tree should have been parsed from, but the parse/populate step into `domain`/`epic`/`usecase`/`task` never ran (or ran and silently produced nothing).

## Downstream consequence

`repo_registration` id 1 (`mock-repo-new`, at `/home/dell/PycharmProjects/Kriti/test/dharma/domain-system/mock-repo-new`) is registered against `domain_system_id = 2` (`rust-dev-domain`) with `status = 'approved'`, and has an **approved** `capability_manifest` (id 1, reviewed by `nikhil` on 2026-08-03). That approval was granted against a Domain System with no actual content — there is nothing yet for that repo to be audited or worked against.

## Suggested next step

Before trusting `rust-dev-domain` (or `rust_dev`) as usable: run whatever parse/ingest step is supposed to turn the 402 captured `content_asset` rows into `domain`/`section`/`epic`/`usecase`/`task` rows, then re-check row counts. If no such step exists yet, that's the actual gap — the domain system registration flow captures raw files but has no implemented step to structure them.

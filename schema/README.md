# Dharma Schema — reference copy

Two physical SQLite databases, mirroring samgraha's global-vs-per-repo split
(`standards.db`/`registry.db` in `mcp_dir()` vs. each repo's own `knowledge.db`),
not five concern-folders as an earlier draft of this schema had it. Design
rationale lives in `docs/proposal/08-schema-and-crate-architecture.md`; the
entities themselves are specified in `docs/proposal/01` through `07`.

| Directory | DB file | Where it lives | Scope | Written by |
|---|---|---|---|---|
| `mcp/` | `mcp.db` (one, global) | MCP's own data directory (e.g. `~/.dharma/mcp.db`), never inside a repository | registries — `domain_system_registry` (00), `agent_system_registry` (01); capture layer — `content_asset` (02), `yaml_template` (03), `seeder` (04); Domain System content — `domain` (05), `section` (06), `section_profile` (07), `epic` (08), `usecase` (09), `task` (10), `task_step` (11); Agent System content — `agent` (12), `agent_goal` (13), `skill` (14), `skill_prompt` (15), `skill_script` (16), `skill_example` (17), `skill_template` (18), `agent_skill_binding` (19); audit definitions — `audit_definition` (20), `audit_rule` (21), `audit_semantic` (22), `audit_calculation` (23), `audit_weights` (24), `audit_template` (25); `analysis_cache` (26); registration — `repo_registration` (27), `capability_manifest` (28) | capture/registration flows through `services`; content rows written only at capture time from provider files |
| `repo/` | one `repo.db` per registered repository | inside that repository (path recorded in `mcp.db`'s `repo_registration.repo_db_path`) | the Propose→Review→Approve→Execute runtime state for every Task Instance this repo runs — `task_instance`, `proposal_revision`, `proposal_approval`, `execution_state`, `handoff_log`, `context_envelope`, `completion_validation`; plus synced content (`synced_content`, 07) and audit executions (`audit_run` 08, `audit_deterministic_result` 09, `audit_semantic_run` 10, `audit_semantic_dimension` 11, `audit_finding` 12, `audit_override` 13) | Task Runtime, Proposal Loop, Handoff Broker, Completion Validator; sync/seed and audit flows through `services` |

## Why two databases, not five

Every table inside `mcp/` shares one file, so every reference between
`domain_system_registry` → `domain`/`section`/`section_profile`/`epic`/
`usecase`/`task`/`task_step`, `agent_system_registry` → `agent`/`skill`, the
content tables → `content_asset` (the capture ledger every content row
traces to), and `repo_registration` → `capability_manifest` is a real,
enforced `FOREIGN KEY` — an earlier draft of this schema split those into
one file per registered Domain System / Agent System, which turned every
one of those references into an unenforced cross-database logical reference
for no isolation benefit, since they're all platform-owned, global,
provider-captured content either way.

`repo/` stays a separate file per repository — mirroring samgraha's per-repo
`knowledge.db` — because it genuinely is different data: repo-local runtime
state, not global platform content. Because each `repo.db` already scopes to
exactly one repository by being its own file, `task_instance` carries no
`repo_registration_id` column; that ambiguity only existed under the earlier
shared-file design.

## Cross-database references

SQLite has no foreign keys across separate database files. `repo.db` →
`mcp.db` is the **only** remaining cross-database boundary in this schema:
columns like `task_id`, `initiating_agent_system_id`/`initiating_agent_id`,
the `handoff_log` `from`/`to` pairs, `synced_content.mcp_row_id`, and
`audit_run.domain_id` in `repo/` are **logical references**, validated by
the `registry` crate at write time, not enforced by `REFERENCES`. Every such
column is commented at its declaration with which table in `mcp.db` it
logically points to. Nothing inside `mcp/` needs this treatment — see "Why
two databases, not five" above.

## Status

Concrete reference copy, not yet loaded by any runtime. These `.sql` files are
the canonical reference copy of the schema — same role samgraha's `schema/`
plays relative to its Rust `const` migrations (see `crates/registry` in
samgraha). Review this schema before any data is populated against it: fixing
a table shape after rows exist is materially more expensive than fixing it now.

## Load order

Within each directory, files are numbered in dependency order — a table only
references tables from lower-numbered files in the same directory (or a
logical, cross-db reference into `mcp.db`, noted in a comment, from `repo/`).
Apply `mcp/` in numeric order first; each `repo.db` (numeric order within
`repo/`) assumes the `mcp.db` it logically references already exists.

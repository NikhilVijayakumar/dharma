# Dharma Schema — reference copy

Two physical SQLite databases, mirroring samgraha's global-vs-per-repo split
(`standards.db`/`registry.db` in `mcp_dir()` vs. each repo's own `knowledge.db`),
not five concern-folders as an earlier draft of this schema had it. Design
rationale lives in `docs/proposal/08-schema-and-crate-architecture.md`; the
entities themselves are specified in `docs/proposal/01` through `07`.

| Directory | DB file | Where it lives | Scope | Written by |
|---|---|---|---|---|
| `mcp/` | `mcp.db` (one, global) | MCP's own data directory (e.g. `~/.dharma/mcp.db`), never inside a repository | registries — `domain_system_registry` (00), `agent_system_registry` (01); capture layer — `content_asset` (02), `yaml_template` (03), `seeder` (04); Domain System content — `domain` (05), `section` (06), `section_profile` (07), `epic` (08), `usecase` (09), `task` (10), `task_step` (11); Agent System content — `agent` (12), `agent_goal` (13), `skill` (14), `skill_prompt` (15), `skill_script` (16), `skill_example` (17), `skill_template` (18), `agent_skill_binding` (19); audit definitions — `audit_definition` (20), `audit_rule` (21), `audit_semantic` (22), `audit_calculation` (23), `audit_weights` (24), `audit_template` (25); `analysis_cache` (26); registration — `repo_registration` (27), `capability_manifest` (28) | capture/registration flows through `services`; content rows written only at capture time from provider files |
| `repo/` | one `repo.db` per registered repository | inside that repository's own `.dharma/` directory (path recorded in `mcp.db`'s `repo_registration.repo_db_path`) | the Propose→Review→Approve→Execute runtime state for every Task Instance this repo runs — `task_instance`, `proposal_revision`, `proposal_approval`, `execution_state`, `handoff_log`, `context_envelope`, `completion_validation`; plus synced content (`synced_content`, 07 — every row tagged with its owning `domain_system_id`/`agent_system_id`, and every row also materialized to a real file under `.dharma/assets/`, see below) and audit executions (`audit_run` 08, `audit_deterministic_result` 09, `audit_semantic_run` 10, `audit_semantic_dimension` 11, `audit_finding` 12, `audit_override` 13); plus this repo's own resolved config (`repo_config`, 14); plus this repo's Proposal document lifecycle tracking (`proposal_lifecycle` 15, `proposal_commit_log` 16 — see `docs/raw/proposal.md`) | Task Runtime, Proposal Loop, Handoff Broker, Completion Validator; sync/seed and audit flows through `services`; proposal lifecycle advanced by whoever authors/implements/verifies a proposal |

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

## Why synced content is also materialized to real files

`mcp.db` lives in MCP's own data directory — outside every repository. A
naive sync that only wrote DB rows into `repo.db` would still leave the
actual script/skill/template bytes reachable only by reading back through
`content_asset`/`skill_script` etc. in that external directory — meaning
every single script execution or template render would need filesystem
permission to reach outside the repository, not just once at sync time.

`synced_content.local_path` (see `repo/07-synced_content.sql`) fixes this:
every synced row, regardless of `kind`, is also written out as a real file
under the repository's own `.dharma/assets/`. A Script Runtime execs
`local_path`; nothing at execution time ever resolves back to mcp.db's
provider-relative `content_asset.file_path` or `skill_script.script_ref` —
those remain meaningful only to the capture/sync step itself, never to
Task/Skill execution.

Every `synced_content` row additionally carries a `domain_system_id` or
`agent_system_id` (logical refs, exactly one set) so "which rows belong to
which registered system" is answerable from repo.db alone — a re-sync after
a Domain System version bump invalidates exactly that system's rows without
a mid-session reach back into mcp.db. The sync includes the Domain System's
audit definitions (so `audit_run` executions are renderable locally) and
each system's provider-declared seeders; the generic seeder ships in the
runtime, not in `synced_content`.

`repo_config` (`repo/14-repo_config.sql`) applies the same principle to
configuration: `dharma-repo.toml`'s resolved values (docs/implementation/
tests/scripts/report directories, selected Domain System) are materialized
into one row in `repo.db` at sync time, so any tool needing that
information queries the repo's own local db — it does not re-parse the toml
file or query `mcp.db` on every call. `mcp_dir` is the one exception: it is
read only when a re-sync is explicitly requested.

## Cross-database references

SQLite has no foreign keys across separate database files. `repo.db` →
`mcp.db` is the **only** remaining cross-database boundary in this schema:
columns like `task_id`, `initiating_agent_system_id`/`initiating_agent_id`,
the `handoff_log` `from`/`to` pairs, `synced_content.mcp_row_id`,
`synced_content.domain_system_id`/`agent_system_id`, `audit_run.domain_id`,
and `repo_config.repo_uuid`/`domain_system_name` in
`repo/` are **logical references**, validated by the `registry` crate at
write time, not enforced by `REFERENCES`. Every such column is commented at
its declaration with which table in `mcp.db` it logically points to.
`synced_content.local_path` is not one of these — it is a plain repository-
relative filesystem path, not a reference into `mcp.db` at all (see "Why
synced content is also materialized to real files" above). Nothing inside
`mcp/` needs this treatment — see "Why two databases, not five" above.

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

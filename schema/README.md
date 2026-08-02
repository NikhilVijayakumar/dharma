# Dharma Schema — reference copy

Two physical SQLite databases, mirroring samgraha's global-vs-per-repo split
(`standards.db`/`registry.db` in `mcp_dir()` vs. each repo's own `knowledge.db`),
not five concern-folders as an earlier draft of this schema had it. Design
rationale lives in `docs/proposal/08-schema-and-crate-architecture.md`; the
entities themselves are specified in `docs/proposal/01` through `07`.

| Directory | DB file | Where it lives | Scope | Written by |
|---|---|---|---|---|
| `mcp/` | `mcp.db` (one, global) | MCP's own data directory (e.g. `~/.dharma/mcp.db`), never inside a repository | `domain_system_registry` (00), `agent_system_registry` (01); one file per Domain System content table — `section` (02), `section_profile` (03), `epic` (04), `usecase` (05), `task` (06), `task_step` (07); one file per Agent System content table — `agent` (08), `agent_goal` (09), `skill` (10), `skill_prompt` (11), `skill_script` (12), `skill_example` (13), `agent_skill_binding` (14); `repo_registration` (15), `capability_manifest` (16) | Agent-Management Agent System (00-14); MCP registration flow + human approval (15, 16) |
| `repo/` | one `repo.db` per registered repository | inside that repository (path recorded in `mcp.db`'s `repo_registration.repo_db_path`) | `task_instance`, `proposal_revision`, `proposal_approval`, `execution_state`, `handoff_log`, `context_envelope`, `completion_validation` — the Propose→Review→Approve→Execute runtime state for every Task Instance this repo runs | Task Runtime, Proposal Loop, Handoff Broker, Completion Validator |

## Why two databases, not five

Every table inside `mcp/` shares one file, so every reference between
`domain_system_registry` → `section`/`epic`/`usecase`/`task`/`task_step`,
`agent_system_registry` → `agent`/`skill`, and `repo_registration` →
`capability_manifest` is a real, enforced `FOREIGN KEY` — an earlier draft of
this schema split those into one file per registered Domain System / Agent
System, which turned every one of those references into an unenforced
cross-database logical reference for no isolation benefit, since they're all
platform-owned, global, Agent-Management-Agent-System-authored content either
way.

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
and the `handoff_log` `from`/`to` pairs in `repo/` are **logical references**,
validated by the `registry` crate at write time, not enforced by
`REFERENCES`. Every such column is commented at its declaration with which
table in `mcp.db` it logically points to. Nothing inside `mcp/` needs this
treatment — see "Why two databases, not five" above.

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

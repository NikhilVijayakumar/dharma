# Proposal: Provider Config & Repo Sync

> Status: Draft — config surface is example-only (`config/`, `env/`), not loaded by any runtime; the schema this proposal specifies already exists in `schema/` (`repo/07-synced_content.sql` with `local_path` and the `domain_system_id`/`agent_system_id` tags, `repo/14-repo_config.sql`). Conforms to `docs/raw/architecture.md` standard.
> **Extends** MCP Registration & Bootstrap (06) with the step it left unspecified: how a repository actually configures itself for each of the three roles it can play (Domain System provider, Agent System provider, consuming repository), and what MCP writes into a consuming repository once its Capability Manifest is approved — a **copy**, never a live execution channel, split into a full Domain System copy and a filtered Agent System copy, plus two generated human-readable summaries.
> **Amended:** a DB row alone is not enough — `mcp.db` lives outside every repository, so a sync that only wrote rows into `repo.db` would still make every script/skill execution reach across that boundary for the actual bytes, asking for external-folder filesystem permission on every single invocation instead of once at sync. Every synced row is now also materialized to a real file inside the repository's own `.dharma/assets/`, and a `repo_config` table caches this repository's own resolved `dharma-repo.toml` values so tools never need to re-parse that file or re-query `mcp.db` mid-session. See "Local Asset Materialization", "Repo Config Table", and "Repo Context Resolution" below.

## Purpose

This document defines the standard for the TOML configuration surface a repository uses to declare which role it plays toward MCP, and for the sync step that follows Capability Manifest approval (06): MCP copies the selected Domain System's content in full and the approved Agent Systems' content in filtered form into the repository's own `.dharma/` directory, then writes two generated summaries describing what arrived and what, if anything, is missing.

Unlike Samgraha's single `samgraha.toml` shape reused (with a `kind` field) across every repository role, Dharma gives each of its three MCP-facing roles its own config file — because a repository providing a Domain System, a repository providing an Agent System, and a repository merely consuming both need almost entirely non-overlapping fields, and a repository maintaining Dharma itself needs a fourth, MCP-unrelated config for its own build/docs tooling.

## System Overview

### Overview

Four TOML files exist: `dharma-build.toml` (Dharma's own repository, self-tooling — build, docs, audit; not part of the MCP protocol roles below), `dharma-domain.toml` (a Domain System provider, e.g. a repository playing the role Kriti plays for Samgraha), `dharma-agent.toml` (an Agent System provider, e.g. a repository supplying `rust-development` or `documentation-management` Agents/Skills), and `dharma-repo.toml` (a consuming repository — names itself, selects a Domain System, registers with MCP per proposal 06). A provider's TOML points MCP at the content Dharma should capture into `content_asset` (08); a consumer's TOML only selects, never authors, per Domain System Registration (05). Once a consuming repository's Capability Manifest is approved, MCP writes into that repository's `.dharma/` directory: `repo.db` (per schema/repo/), a materialized copy of every synced row under `assets/`, `repo_config` (this repository's own resolved settings), `domain-summary.md`, and `agent-summary.md`.

### Structural Approach

Config authoring and sync are one-directional: a provider's TOML feeds capture (provider → `mcp.db`, see proposal 08's capture ledger); a consumer's TOML feeds selection (consumer → `repo_registration`, see proposal 06); sync then flows the other way (`mcp.db` → consumer's `repo.db` and `.dharma/assets/`), never live-executing against the consumer, only copying — and copying twice: once as a `synced_content` DB row, once as a real file at `synced_content.local_path`, so nothing at execution time needs to resolve back through `mcp.db` to MCP's own (external, outside-the-repo) data directory. Domain content copies in full because a Domain System is chosen as one coherent whole; Agent content copies filtered to the approved Capability Manifest because Agent Systems are additive and a repository should carry only the capability it was actually granted.

### Diagram

```text
dharma-domain.toml ──registers──▶ Domain System Registry + domain content (mcp.db)
dharma-agent.toml  ──registers──▶ Agent System Registry + agent content (mcp.db)
dharma-repo.toml   ──registers, selects──▶ repo_registration (mcp.db)
                                                  │ (Capability Manifest approved, 06)
                                                  ▼
                                    ┌───────────────────────────┐
                                    │   Sync (full + filtered)   │
                                    └───────────────────────────┘
                                                  │
                                                  ▼
                              .dharma/  (inside the consuming repository)
                                ├── repo.db
                                ├── assets/              (every synced_content row, materialized to a real file)
                                │     ├── skill_script/*.py
                                │     ├── skill_prompt/*.md
                                │     └── ...             (one subfolder per `kind`)
                                ├── domain-summary.md    (full Domain System received)
                                └── agent-summary.md     (filtered Agent Systems + gaps)
```

## Component Model

### Domain System Provider Config (`dharma-domain.toml`)
- **Responsibility:** Names the Domain System a repository provides, and where its domain/section/epic/usecase/task source files live.
- **Ownership:** The provider repository, at its root — never inside `.dharma/`.
- **Interfaces:** Read by the capture flow (08) to walk the declared content root and populate `content_asset`; feeds `domain_system_registry` (mcp.db 00).

### Agent System Provider Config (`dharma-agent.toml`)
- **Responsibility:** Names the Agent System a repository provides (its concern, e.g. `rust-development`), whether it is privileged (Agent-Management / Default-Bootstrap), and where its agent/skill source files live.
- **Ownership:** The provider repository, at its root.
- **Interfaces:** Read by the capture flow to populate `content_asset` and `agent_system_registry` (mcp.db 01).

### Consuming Repo Config (`dharma-repo.toml`)
- **Responsibility:** Names the repository, selects the Domain System it registers against, and states where its `.dharma/` directory lives.
- **Ownership:** The consuming repository, at its root.
- **Interfaces:** Read by the MCP Registration Entry Point (06) at `register(repo, domainSystemName)`; does not name specific Agent Systems (06's Rationale: capability resolution is analysis, not direct binding).
- **Additional fields:** optional `[repository.documentation]` / `[repository.tests]` / `[repository.scripts]` / `[repository.implementation]` sections name where this repository keeps those artifacts — their resolved values land in `repo_config`'s `docs_dir`/`tests_dir`/`scripts_dir`/`implementation_dir`. `[repository.ignore]` lists path globs that scope file-walking operations (the capture walk on a provider's TOML, audit evidence file-glob checks on the consumer's TOML) so generated/vendor directories (`.git`, `target`, `node_modules`) are skipped. `[report]` names where `domain-summary.md`/`agent-summary.md` land, falling back to `.dharma/` if unset. None of these are materialized into `repo_config` as anything but the single resolved row it already owns.

### Dharma Build Config (`dharma-build.toml`)
- **Responsibility:** Dharma's own self-tooling config — documentation root/domain list (`docs/raw`), implementation dir (`crates/`), build/test pipelines, report output. Has no MCP protocol role; exists because Dharma is, itself, just another repository that follows its own documentation standard.
- **Ownership:** The `dharma` repository root.
- **Interfaces:** Read by whatever tooling audits Dharma's own `docs/raw` compliance and runs its build/test pipelines — out of scope for the MCP runtime described elsewhere in 01-08.

### Sync Engine
- **Responsibility:** After Capability Manifest approval, copies the selected Domain System's content in full and the approved Agent Systems' content filtered to only `capability_manifest.status = 'approved'` rows into the consuming repository's `repo.db` (`synced_content`, repo/07) — and, for every row, also writes the same bytes to a real file under `.dharma/assets/`, recording that path as `synced_content.local_path`. Each row is tagged with its owning `domain_system_id` or `agent_system_id` so re-sync/version-bump invalidation is answerable from repo.db alone. The Domain System's audit definitions and each system's provider-declared seeders sync with their owning system (see Data Paths below).
- **Ownership:** No schema of its own; writes `synced_content` rows (DB + file, together) and `repo_config` (see below).
- **Interfaces:** Reads `mcp.db` (domain content tables 05-11, agent content tables 12-19, the audit definition tables 20-25 the repo needs to run audits locally, and each system's provider-declared seeders from 04); writes `repo.db`'s `synced_content` and the corresponding files under `.dharma/assets/`.

### Local Asset Materialization
- **Responsibility:** Guarantees that every `synced_content` row has a corresponding real file at `local_path`, byte-identical to `content`, so a Script Runtime, Prompt Runtime, or template renderer only ever needs to open a path inside the repository — never `mcp.db`'s `content_asset.file_path` or `skill_script.script_ref`, both of which are meaningful only relative to MCP's own external data directory.
- **Ownership:** The `.dharma/assets/` directory tree; one subfolder per `kind`, one file per synced row.
- **Interfaces:** Written once by the Sync Engine per sync; read by whichever Skill invocation (see proposal 03) needs a script, prompt template, or audit template's actual bytes.

### Repo Config Table
- **Responsibility:** Caches this repository's own resolved `dharma-repo.toml` values (docs/implementation/tests/scripts/report directories, selected Domain System name+version) as a single row in `repo.db` (`repo_config`, repo/14), so that information is answerable from the repository's own local db without re-parsing the toml file or querying `mcp.db`.
- **Ownership:** Exactly one row per `repo.db`, written at registration/sync time and refreshed whenever `dharma-repo.toml` changes and a re-sync runs.
- **Interfaces:** Read by any MCP tool that needs "where does this repo keep its docs / tests / reports" (see Repo Context Resolution below); `mcp_dir` within it is read only when a re-sync is explicitly requested.

### Repo Context Resolution
- **Responsibility:** Given a repository identifier, resolves `mcp.db`'s `repo_registration.repo_db_path` exactly once per session/tool-call boundary, then routes every subsequent query for that repository to its local `repo.db` (`repo_config`, `synced_content`, execution/audit tables) — never back to `mcp.db` or MCP's external data directory mid-session.
- **Ownership:** No schema of its own; a resolution step in `services` (see proposal 08's crate split) that MCP tools call through.
- **Interfaces:** Reads `mcp.db`'s `repo_registration.repo_db_path` once; all further reads/writes in that session target the resolved `repo.db` path directly.

### Domain Summary Generator
- **Responsibility:** Writes `.dharma/domain-summary.md`: the selected Domain System's name/version, a domain graph (Epic → Usecase → Task tree, or Section Map tree), and a narrative of what was received. Always describes a full copy — there is nothing to omit.
- **Ownership:** Reads `synced_content` rows of domain-scoped `kind`s; writes the markdown file.
- **Interfaces:** Runs once per successful sync, after the Sync Engine completes the domain copy.

### Agent Summary Generator
- **Responsibility:** Writes `.dharma/agent-summary.md`: which Agent Systems were synced, which specific Agents/Skills came with them, and a **Missing Coverage** section listing any `task_step.required_capability` (from the synced domain content) whose concern has no corresponding `capability_manifest.status = 'approved'` row for this repository.
- **Ownership:** Reads `synced_content` rows of agent-scoped `kind`s plus the domain content's `task_step.required_capability` values; writes the markdown file.
- **Interfaces:** Runs once per successful sync, after the Sync Engine completes the agent copy; its Missing Coverage section is what a human reviewer (06) reads to decide whether to approve more Agent Systems. The approved `capability_manifest` rows are supplied from the sync context — the manifest lives in `mcp.db` and is not itself synced into `repo.db` — so the generator runs at sync time, when that state is in hand, never as a mid-session `mcp.db` read.

### Component Diagram

```text
dharma-domain.toml ──▶ Capture (08) ──▶ mcp.db domain content
dharma-agent.toml  ──▶ Capture (08) ──▶ mcp.db agent content
dharma-repo.toml   ──▶ Registration Entry Point (06) ──▶ repo_registration

repo_registration (approved) ──▶ Sync Engine ──┬──▶ repo.db synced_content (domain, full)   ──▶ .dharma/assets/*
                                                 ├──▶ repo.db synced_content (agent, filtered) ──▶ .dharma/assets/*
                                                 ├──▶ repo.db repo_config (resolved dharma-repo.toml)
                                                 ├──▶ .dharma/domain-summary.md
                                                 └──▶ .dharma/agent-summary.md

MCP tool call ──▶ Repo Context Resolution ──(once)──▶ mcp.db repo_registration.repo_db_path
                                                              │
                                                              ▼
                                              all further reads/writes ──▶ that repo.db directly
```

## Communication

### Communication Paths

**Provider repository → Capture flow**
- **Pattern:** Synchronous, at capture time (whenever the provider's declared content changes).
- **Contract:** `dharma-domain.toml` / `dharma-agent.toml` names a content root; Capture walks it, writes `content_asset` rows, and registers/updates the corresponding `domain_system_registry` / `agent_system_registry` entry.

**Consuming repository → MCP Registration Entry Point**
- **Pattern:** Synchronous request (06).
- **Contract:** `dharma-repo.toml` supplies repo identity and a Domain System name; unchanged from 06's `register(repo, domainSystemName)` contract.

**MCP → Sync Engine → consuming repository's `.dharma/`**
- **Pattern:** Synchronous, triggered once by Capability Manifest approval (06); re-run whenever the manifest changes thereafter.
- **Contract:** Sync Engine writes `repo.db` and both summaries; the consuming repository never pulls or executes against `mcp.db` directly.

### Communication Diagram

```text
Provider repo → Capture : declare(contentRoot)  [from dharma-domain.toml | dharma-agent.toml]
Capture → mcp.db : write(content_asset, registry entry)

Consumer repo → MCP Platform : register(repo, domainSystemName)  [from dharma-repo.toml, per 06]
MCP Platform → Consumer repo : approved(capabilityManifest)  [per 06]
MCP Platform → Sync Engine : sync(repoRegistrationId)
Sync Engine → repo.db : write(synced_content: domain, full)
Sync Engine → repo.db : write(synced_content: agent, filtered by approved manifest)
Sync Engine → .dharma/ : write(domain-summary.md, agent-summary.md)
```

## Data Flow

### Data Paths

**Full Domain Sync Path**
- **Entry point:** Capability Manifest reaches at least one `approved` Agent System entry, or a Domain-System-only sync is explicitly requested.
- **Transformations:** Every domain-scoped `mcp.db` row for the selected `domain_system_id` — domain (05), section (06), section_profile (07), epic (08), usecase (09), task (10), task_step (11), the Domain System's audit definitions (audit_definition 20, audit_rule 21, audit_semantic 22, audit_calculation 23, audit_weights 24, audit_template 25), and any provider-declared seeders (seeder 04) — is copied verbatim into `synced_content`, `kind`-tagged, no filtering, each row tagged with `domain_system_id`. (The generic seeder Dharma ships lives in the runtime; only provider-declared seeders are synced as `kind 'seeder'`.)
- **Ownership boundary:** `mcp.db` owns the source rows; `repo.db`'s `synced_content` owns the copy, independently updatable without touching `mcp.db`.
- **Exit point:** A complete domain content set (structure plus its audit definitions, so `audit_run` executions are locally renderable) inside the consuming repository's `repo.db`.

**Filtered Agent Sync Path**
- **Entry point:** Capability Manifest has one or more rows with `status = 'approved'`.
- **Transformations:** Only agent-scoped `mcp.db` rows (agent, agent_goal, skill, skill_prompt, skill_script, skill_example, skill_template, agent_skill_binding) whose `agent_system_id` matches an approved manifest row are copied into `synced_content`, each tagged with `agent_system_id`; a provider-declared seeder belonging to an approved Agent System syncs with it.
- **Ownership boundary:** Same as above; a rejected or not-yet-reviewed Agent System's content is never written to `repo.db`, not even in a disabled state.
- **Exit point:** A reduced Agent/Skill set inside the consuming repository's `repo.db` — only what was actually granted.

**Materialization Path**
- **Entry point:** A row is about to be written to `synced_content` (from either path above).
- **Transformations:** The same bytes are written both as `synced_content.content` and as a file at `synced_content.local_path` under `.dharma/assets/<kind>/`; the two must match, or the sync is incomplete.
- **Ownership boundary:** `.dharma/assets/` is owned by the Sync Engine — nothing else writes to it, and nothing reads a synced asset from anywhere else.
- **Exit point:** A real, repository-local file any Skill invocation can open directly.

**Repo Config Path**
- **Entry point:** Registration or re-sync reads this repository's `dharma-repo.toml`.
- **Transformations:** Resolved values (docs/implementation/tests/scripts/report dirs, selected Domain System name+version) are written into `repo_config`'s single row, overwriting the prior one.
- **Ownership boundary:** `repo_config` is owned by the Sync Engine; every other component reads it, none but the Sync Engine writes it.
- **Exit point:** A locally-queryable answer to "where does this repo keep X," with no toml re-parse and no `mcp.db` round-trip required afterward.

### Data Flow Diagram

```text
mcp.db domain content (domain_system_id = X) ──ALL rows──▶ repo.db synced_content (kind ∈ domain kinds) ──▶ .dharma/assets/*
mcp.db agent content (agent_system_id ∈ approved manifest) ──filtered──▶ repo.db synced_content (kind ∈ agent kinds) ──▶ .dharma/assets/*
dharma-repo.toml (resolved) ──▶ repo.db repo_config (single row)
                                                                              │
                                                              cross-reference: task_step.required_capability
                                                              vs. approved agent_system concerns
                                                                              │
                                                                              ▼
                                                      .dharma/agent-summary.md "Missing Coverage"
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| `dharma-domain.toml`, `dharma-agent.toml`, `dharma-repo.toml`, `dharma-build.toml` | Each repository, at its own root — never generated by MCP |
| `content_asset`, domain/agent content, `capability_manifest` | `mcp.db`, per proposal 08 |
| `repo.db` `synced_content` (DB rows) | Sync Engine, one repository's copy, independently stale-able from `mcp.db`; every row tagged with its owning `domain_system_id`/`agent_system_id` |
| `.dharma/assets/*` (materialized files) | Sync Engine, written together with each `synced_content` row, never read from elsewhere |
| `repo.db` `repo_config` | Sync Engine, single row, overwritten on every re-sync |
| `.dharma/domain-summary.md`, `.dharma/agent-summary.md` | Domain/Agent Summary Generators, regenerated (overwritten) on every sync |

## Security

### Trust Boundaries

- **Provider TOML → Capture:** Semi-trusted — structured, authored content, captured into the immutable-append `content_asset` ledger (08), never executed.
- **Consumer TOML → Registration Entry Point:** Untrusted request, as in 06 — a name and a selection, nothing else.
- **Sync Engine → consuming repository:** One-directional write into `.dharma/`; the consuming repository has no channel back into `mcp.db` other than through the standard registration/approval flow.
- **Execution → `.dharma/assets/`:** Every Skill invocation resolves its script/prompt/template from a path inside the repository's own tree; it never holds a path into MCP's external data directory, so it never needs permission to reach outside the repository.

### Threat Model

- **Repeated external-folder access on every execution:** Without materialization, a Script Runtime resolving a skill's script would read `mcp.db`'s `content_asset`/`skill_script` and open a file in MCP's own (external, outside-the-repo) data directory on every single invocation — meaning an OS/sandbox permission prompt, or a broadened trust boundary, per execution rather than once at sync. Mitigation: `synced_content.local_path` materializes every synced row to a real file under `.dharma/assets/` at sync time; execution never resolves through `mcp.db` at all.
- **Agent content leaking past filtering:** A bug in the Sync Engine copies an agent-scoped row whose `agent_system_id` has no `approved` manifest entry. Mitigation: the Filtered Agent Sync Path's query is scoped by an inner join against `capability_manifest WHERE status = 'approved'`, never a blocklist subtracted from the full agent catalog — the query can only ever add approved rows, not accidentally include unapproved ones by omission.
- **Stale `repo_config` after a `dharma-repo.toml` edit:** A repository's toml changes (e.g. a new `report_dir`) but no re-sync has run, so `repo_config` still answers with the old value. Mitigation: `repo_config.last_synced_at` records when it was last written; tooling that reads `repo_config` for a path also checks this timestamp isn't older than the toml file's own modification time, and prompts a re-sync if it is.
- **Provider TOML requesting privileged registration silently:** A `dharma-agent.toml` sets an unearned privileged flag. Mitigation: `is_privileged` on `agent_system_registry` is set by the Agent-Management Agent System at review time (proposal 04), never taken verbatim from provider-supplied TOML.
- **Stale `.dharma/` after Domain System version bump:** The provider's Domain System is revised after a consuming repository's last sync. Mitigation: unchanged from proposal 02/06 — `repo_registration.domain_system_version` is checked, and a mismatch blocks new Task assignment until re-sync.

## Rationale

### Full Domain Copy, Filtered Agent Copy
- **Context:** A Domain System is chosen as one coherent whole (proposal 05); Agent Systems are additive, resolved per repository by analysis (proposal 06).
- **Decision:** Sync copies 100% of the selected Domain System's content, but only the subset of Agent System content actually approved for this repository.
- **Alternatives Considered:** Copy both in full; filter both.
- **Rejection Reason:** Copying the Domain System partially would leave a repository with an incomplete Epic/Usecase/Task hierarchy it cannot make sense of. Filtering the Agent System catalog is the opposite case: copying it in full would give every repository every Agent System that exists anywhere, defeating the point of the Capability Manifest's per-repository approval gate.
- **Architectural Goal:** `repo.db` carries exactly the domain shape it needs, and exactly the capability it was granted — nothing more, nothing less.

### Four Config Files, Not One
- **Context:** Samgraha uses one `samgraha.toml` shape with a `kind` field distinguishing repository roles.
- **Decision:** Each of Dharma's three MCP-facing roles (Domain System provider, Agent System provider, consuming repository) gets its own TOML file, plus a fourth for Dharma's own build tooling.
- **Alternatives Considered:** One `dharma.toml` with a `kind` field, mirroring Samgraha exactly.
- **Rejection Reason:** The three MCP-facing roles have almost no field overlap (a provider names content roots and a concern/domain identity; a consumer only selects), so a single schema would carry mostly-irrelevant optional fields for whichever role a given repository plays. `dharma-build.toml` is not an MCP-facing role at all — folding it into the same file would blur "how does this repository interact with MCP" with "how is this repository's own codebase built."
- **Architectural Goal:** Each config file is readable on its own as "what this repository is, toward MCP" without cross-referencing which fields apply to which role.

### Generated Summaries With an Explicit Missing-Coverage Section
- **Context:** The Capability Manifest approval gate (06) already requires a human reviewer; a raw table dump of synced rows is not what that reviewer needs to decide confidently.
- **Decision:** Every sync produces `domain-summary.md` and `agent-summary.md`, the latter always including a Missing Coverage section — even when empty, stated as empty, not omitted.
- **Alternatives Considered:** No generated summary; rely on inspecting `repo.db` directly.
- **Rejection Reason:** Inspecting raw `synced_content` rows to determine whether a Domain System's every `required_capability` is actually covered is exactly the kind of check a human reviewer would otherwise have to reconstruct by hand, every time.
- **Architectural Goal:** The artifact a human actually reads to approve a Capability Manifest is the artifact Dharma actually generates.

### Every Synced Row Is Also a Real File
- **Context:** `mcp.db` and its capture ledger live in MCP's own external data directory; a DB-row-only sync would leave the actual script/prompt/template bytes reachable only by reading back through that external location.
- **Decision:** Every `synced_content` row is written twice — once as `content` in `repo.db`, once as a real file at `local_path` under `.dharma/assets/` — uniformly, regardless of `kind`, not just for kinds that are obviously executable.
- **Alternatives Considered:** Materialize only the kinds that are actually executed as scripts (`skill_script`), leaving prompts/templates as DB-only text.
- **Rejection Reason:** Special-casing which kinds get materialized adds a decision every new `kind` has to make correctly; a uniform rule ("every synced row gets a file") has no such failure mode and costs nothing extra once the Sync Engine already writes the DB row.
- **Architectural Goal:** No execution path ever needs permission to reach outside the repository, for any kind of synced content.

### Repo Config Materialized, Not Re-Parsed
- **Context:** `dharma-repo.toml` already states where this repository keeps its docs/tests/reports; without `repo_config`, every tool needing that answer would re-parse the toml file, or worse, query `mcp.db`.
- **Decision:** Resolved toml values are cached as a single row in `repo.db`, refreshed only at registration/re-sync.
- **Alternatives Considered:** Have every tool call read `dharma-repo.toml` directly each time.
- **Rejection Reason:** Repeated toml parsing scattered across every tool call is exactly the same class of problem as repeated external-folder access for scripts — a cost paid on every call that a single sync-time write avoids entirely.
- **Architectural Goal:** `repo.db` is self-sufficient for ordinary tool calls; `dharma-repo.toml` and `mcp.db` are touched only at registration and re-sync.

## Constraints

### Hard Constraints
- **Domain sync is all-or-nothing** (source: Rationale above) — no partial Domain System copy is ever written to a `repo.db`.
- **Agent sync is scoped to `capability_manifest.status = 'approved'` only** (source: Threat Model above) — never a blocklist-filtered full catalog.
- **Both summaries are regenerated on every sync** (source: Rationale above) — a stale summary describing a prior Capability Manifest state is a defect, not an acceptable variance.
- **Provider TOML never sets `is_privileged` directly** (source: Threat Model above) — that flag is Agent-Management-reviewed only.
- **Every `synced_content` row has a non-null `local_path`, and `content`/the file it names must match** (source: Rationale above) — a DB row with no materialized file is an incomplete sync.
- **`repo_config` has exactly one row, written only by the Sync Engine** (source: Rationale above) — no other component may write it, and ordinary tool calls read it instead of `dharma-repo.toml` or `mcp.db`.

### Soft Constraints
- Prefer overwriting `.dharma/domain-summary.md` / `agent-summary.md` in place rather than versioning them; they describe current state, not history (the append-only history already lives in `synced_content.synced_at` and `capability_manifest`).

## Traceability

### Derivation Chain

```text
04-agent-system-registry, 05-domain-system-registration, 06-mcp-registration-bootstrap,
08-schema-and-crate-architecture (content_asset, synced_content originally defined)
    │
    ▼
Provider Config & Repo Sync (this document) — extends schema/repo/07-synced_content.sql
with `local_path`, adds schema/repo/14-repo_config.sql
    │
    ▼
(terminal proposal — feeds Engineering/Implementation once the registry/services
 crates are built against the existing schema)
```

### Non-Contradiction Rule

No downstream proposal may let a consuming repository's TOML directly name specific Agent Systems (bypassing 06's analysis-then-approval discipline), let Agent sync include a row outside an approved Capability Manifest entry, omit the Missing Coverage section from `agent-summary.md`, write a `synced_content` row without a matching materialized file at `local_path`, or read repository configuration from anywhere other than `repo_config` outside of registration/re-sync, without revising this document first.

# Proposal: Agent System Concern Split & Release Bundling

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/proposal.md` standard.

## Purpose

Today exactly one Agent System is registered in `mcp.db`: `analyse` (id 1, concern `analysis`), whose own description admits it does two unrelated things — "(A) Domain-System verification and (B) agent-capability provisioning." This contradicts proposal 04's own constraint that one Agent System serves one coherent concern ("Concern uniqueness at registration," `docs/proposal/archive/04-agent-system-registry.md:153`). This proposal splits that single registration into three concerns for the upcoming MCP release:

1. **`capability-provisioning`** — finds every Agent/Skill combination able to execute a given Domain System's Tasks, and reports where none exists.
2. **`domain-system-evaluation`** — judges whether a registered Domain System's content is *good*, not merely structurally present.
3. **`agent-system-evaluation`** — judges whether a registered Agent System's own Agents/Skills/bindings are *good*. This concern's source content is explicitly **not authored, owned, or stored by Dharma** — it is provided by an external repository (today Kriti, at `/home/dell/PycharmProjects/Kriti/dharma/agent/system`), the same "selected not authored" relationship proposal 05 already established for Domain Systems. Dharma must be able to switch this to a different provider repository later without any Dharma code change.

This proposal also defines how a packaged Dharma release ships pre-populated: as many of these concerns as their provider has published content for get captured into the package at build time, so a fresh install needs no manual `register_*` call for whatever's already there — `agent-system-evaluation` stays absent from a given release until its provider publishes content, which is expected, not a defect (see Constraints). "Pre-populated" here means the capture step (`content_asset` rows) has already run; it does not by itself mean every concern's Agents/Skills are queryable end to end — see Data Flow.

## System Overview

### Overview

All 9 captured Agents and 13 captured Skills under the existing `analyse` registration originate from one Kriti folder, `Kriti/dharma/agent/system/analyse/{agent,skill}/`. Verified directly (2026-08-06): that folder has exactly one concern subfolder — no split exists in the provider today, and Dharma's capture mechanism has no filtering capability to manufacture one at registration time (`capture_bundle`, `crates/services/src/capture.rs:59-75`, walks every file under `root` unconditionally; `register_agent_system`, `crates/mcp/src/adapter.rs:244-268`, takes a single `content_root` with no subset/filter parameter). Splitting the registration into `capability-provisioning` and `domain-system-evaluation` therefore requires the split to exist as two physically separate directories before Dharma registers them — either Kriti restructures `analyse/` into two concern folders, or an intermediate step produces two filtered copies for Dharma to point at. This proposal requires the former (see Rationale): Kriti creates `capability-provisioning/` and `domain-system-evaluation/` folders, each containing only the Agents/Skills listed for it in Component Model below, before the corresponding `register_agent_system` call is made.

Separately, `crates/xtask/src/main.rs`'s `release()` (lines 24-104, verified) copies `bin/`, `config/*.toml` (examples), `env/*.env.example`, and `schema/` into the package — no `mcp.db`, no registration call, no `agent_system`/`domain_system`/`content_root` reference anywhere in the crate. The release ships **no** `mcp.db` at all; the runtime creates an empty one on first launch at `mcp_dir()/mcp.db` (`crates/registry/src/mcp_db.rs:380-391`), which resolves to `$HOME/.dharma` unless `DHARMA_MCP_DIR` is set (`crates/common/src/env.rs:44-59`).

This proposal adds a release step that captures every provider-configured Domain/Agent System into a packaged `pkg_dir/data/mcp.db`, and a matching runtime change so those pre-seeded rows actually get used. A launcher-script wrapper (exporting `DHARMA_MCP_DIR` before exec) was considered and rejected: `docs/release/mcp-configuration.md` — the single source every supported client is configured from — points Claude Code, OpenCode, Antigravity IDE, and Codex CLI at `bin/dharma-mcp[.exe]` directly (`mcp-configuration.md:48-55,98-105,171,219-232`), never at `run-mcp.sh`/`run-mcp.cmd`; a launcher-only fix would be dead code for every documented integration. The fix instead lives in `McpDb::open()` itself (`crates/registry/src/mcp_db.rs`): on first open, if the global `mcp_dir()/mcp.db` doesn't exist yet and this binary is running from a packaged release layout (`<exe_dir>/../data/mcp.db` present), that packaged db seeds the global one, once. This reaches every client regardless of how it invokes the binary, and — because it seeds the *same* global path `mcp-configuration.md:8-9` already documents as the one store, and only when that path is empty — it does not introduce a second, package-local database; the "single global server" model stays true, it just starts non-empty on a fresh install. Verified live: invoking a packaged `bin/dharma-mcp` directly (bypassing `run-mcp.sh` entirely) with a fresh `DHARMA_MCP_DIR` seeds and returns the packaged `rust_dev` registration; a second call with a row already added by the user is untouched by the seed (`seed_if_absent` only acts when the target path doesn't exist).

### Structural Approach

`register_agent_system` and `capture_bundle` are reused completely unchanged — no new capture-time filtering is introduced (an earlier draft of this proposal claimed subset-filtering without specifying it; that claim is corrected here). What's new is: (1) a Kriti-side folder split (out of this repository's scope to implement, but a stated precondition — see Constraints), (2) a small `dharma-build.toml` addition so the Release Bundling Step knows which provider config files to read, and (3) a seed-once check inside `McpDb::open()` so a pre-seeded package database is actually used at runtime by whichever client launches the binary directly (a launcher-script wrapper was tried first and rejected — see Purpose).

### Diagram

```text
Today:
  Kriti/dharma/agent/system/analyse/ ──capture_bundle──▶ mcp.db: agent_system "analyse" (concern: "analysis")
                                                            └─ 9 agents, 13 skills, one bag, two concerns mixed

This proposal (after Kriti splits its folder — precondition, not this proposal's own work):
  Kriti/.../capability-provisioning/  ──capture_bundle──▶ mcp.db: agent_system "capability-provisioning"
  Kriti/.../domain-system-evaluation/ ──capture_bundle──▶ mcp.db: agent_system "domain-system-evaluation"
  <provider>/.../agent-system-evaluation/ (does not exist yet) ──▶ mcp.db: agent_system "agent-system-evaluation"
                                                                      (provider-owned; Dharma never stores its source)

  xtask build-release ──registers all configured entries into pkg_dir/data/mcp.db──▶ release artifact
  McpDb::open() (any client, first launch) ──global mcp.db absent, packaged data/mcp.db present──▶ seed once, then open the (now non-empty) global mcp.db
```

## Component Model

### `capability-provisioning` Agent System
- **Responsibility:** Given a repository's chosen Domain System, find every Agent/Skill able to execute each of its Tasks; report Tasks with no matching Agent as a gap.
- **Ownership:** Once Kriti splits its folder (precondition), this concern's directory contains agents `assignment-planner`, `capability-analyser`, `gap-analyser`, `orchestrator`, `workflow-designer`; skills `map-task-to-capability`, `propose-agent-assignment`, `identify-agent-gaps`, `design-handoff-workflow`, `render-provisioning-report` — all verified present today under Kriti's combined `analyse/` folder.
- **Interfaces:** Registered via `register_agent_system(name="capability-provisioning", concern="capability-provisioning", content_root=<Kriti's split path>)`; consumed by the Default/Bootstrap Agent System's resolution step (proposal 04) when it proposes a Capability Manifest.

### `domain-system-evaluation` Agent System
- **Responsibility:** Judge a registered Domain System's content for quality — not just "does the `domain`/`epic`/`usecase`/`task` tree exist" (structural presence, the gap already filed in `issue/domain-system/001-rust-domain-systems-registered-with-zero-structured-content.md`) but "is it good": complete, internally consistent, matching its own Section Map.
- **Ownership:** Once Kriti splits its folder (precondition), this concern's directory contains agents `domain-system-verifier`, `domain-verifier`, `hierarchy-verifier`, `section-verifier`; skills `analyse-domain-system`, `verify-section-map`, `verify-section-profile`, `verify-epic-completion`, `verify-usecase-completion`, `verify-task-completion`, `verify-epic-usecase-task`, `render-verification-report` — all verified present today under Kriti's combined `analyse/` folder.
- **Interfaces:** Registered via `register_agent_system(name="domain-system-evaluation", concern="domain-system-evaluation", content_root=<Kriti's split path>)`; invoked whenever a repository registers or recaptures a Domain System, and on demand via the existing audit tools (`run_audit`/`get_audit_result`).

### `agent-system-evaluation` Agent System (provider-owned)
- **Responsibility:** Judge a registered Agent System's own Agents/Skills/bindings for completeness and quality — coverage of its declared concern, no orphaned Skills, bindings that actually resolve.
- **Ownership:** Source content (Agent YAML, Skill prompts/scripts/examples) is authored and owned **entirely by the provider repository** — Kriti today, at whatever path a future `dharma-agent.toml` names. Dharma owns nothing upstream of the `content_asset` rows its own capture flow produces once that content exists and is registered. This is the identical ownership split proposal 05 already defined for Domain Systems ("selected not authored").
- **Interfaces:** Registered the same way as any other Agent System, once content exists: `register_agent_system(name="agent-system-evaluation", concern="agent-system-evaluation", content_root=<provider path>)`. No Dharma code path is concern-specific to this one — swapping the provider means changing the `content_root` a config file points to, nothing else.
- **Required Content Shape:** To be capturable and useful, the provider's folder must follow the same `agent/*.yaml` + `skill/<name>/{skill.yaml,prompt.md,examples/}` layout `capture_bundle` already walks for the other two concerns (no schema beyond that is enforced by capture itself — `capture_bundle` accepts any file). At minimum it must include: at least one Agent whose stated role is judging another Agent System's completeness against its declared `concern`; at least one Skill that checks Agent↔Skill binding completeness (mirroring what `agent_skill_binding` should contain once populated); and at least one Skill that renders a pass/fail-with-reasons report, mirroring the existing `render-verification-report`/`render-provisioning-report` pattern.

### Release Bundling Step (new)
- **Responsibility:** At `xtask build-release` time, call `register_agent_system`/`register_domain_system` for every provider entry configured for this release, writing into `pkg_dir/data/mcp.db` — a database included in the package, not the runtime's default `~/.dharma/mcp.db`.
- **Ownership:** Reads a new `[[release.providers]]` array added to `dharma-build.toml` (Dharma's own build-time config, `crates/common/src/config.rs` `BuildConfig`) — each entry `{ kind = "agent_system" | "domain_system", config_path = "<path to a dharma-agent.toml- or dharma-domain.toml-shaped file>" }`. Each referenced file is parsed with the existing, unchanged `AgentSystemProviderConfig`/`DomainSystemProviderConfig` structs (`config.rs:286`, `:346`, single-block shape, already covered by an existing test at `config.rs:667-669`) — this proposal does not change those structs. Note: per both `config.example/dharma-agent.toml` and `dharma-domain.toml`'s own header comments ("proposal state, not yet loaded by any runtime"), the Release Bundling Step is the *first* code path that actually reads these files — they are fully specified but currently inert.
- **Interfaces:** New step in `crates/xtask/src/main.rs`'s release pipeline, upstream of packaging; provider paths are config values in `dharma-build.toml` and the files it references, never hardcoded, so a future non-Kriti provider is a config change only.

### First-Run Global-DB Seeding (new, in `registry`)
- **Responsibility:** Make the pre-seeded `pkg_dir/data/mcp.db` the source the shipped binary's *global* database starts from, on whichever machine and via whichever client actually launches it — without introducing a second, package-local database that would compete with `mcp_dir()`'s documented single global store.
- **Ownership:** `McpDb::open()` (`crates/registry/src/mcp_db.rs`) gains a `packaged_seed_db()` lookup (`<current_exe_dir>/../data/mcp.db`, `None` for a dev build or a binary run outside a packaged layout) and a `seed_if_absent(target, seed)` copy that only fires when `target` (the global `mcp_dir()/mcp.db`) does not exist yet — an existing global db, however it got there, is never touched. `open_at(path)` (used by `xtask`'s Release Bundling Step to write `pkg_dir/data/mcp.db` itself, and by tests) is unaffected — seeding is only in the no-argument `open()` path real clients call.
- **Interfaces:** No launcher script involvement — this fires inside the binary itself, so it reaches `bin/dharma-mcp`/`bin/dharma-mcp.exe` regardless of how a client invokes it. `DHARMA_MCP_DIR`'s existing precedence (`crates/common/src/env.rs:44-59`) is untouched; seeding only changes what's *already there* the first time that resolved path is opened.

### Component Diagram

```text
dharma-build.toml (Dharma's own build config)
    │ [[release.providers]]: { kind, config_path }
    ▼
xtask build-release ──parses each config_path with existing AgentSystemProviderConfig/DomainSystemProviderConfig──▶
    ──register_agent_system/register_domain_system──▶ pkg_dir/data/mcp.db
    ▼
released package: binary (bin/dharma-mcp) + pkg_dir/data/mcp.db, sibling directories
    │  any client launches bin/dharma-mcp directly (mcp-configuration.md's documented pattern)
    ▼
McpDb::open() ──global mcp.db absent, packaged data/mcp.db present──▶ seed once ──▶ single global mcp.db, now non-empty
```

## Communication

### Communication Paths

**Release Bundling Step → `pkg_dir/data/mcp.db`**
- **Pattern:** Synchronous, once per release build.
- **Contract:** For each `[[release.providers]]` entry, load its `config_path`, call the matching `register_agent_system`/`register_domain_system` + `capture_bundle` path a manual registration would use; a missing or empty `content_root` for `agent-system-evaluation` is not an error — that concern is simply omitted from this release's bundle until its provider publishes content, and the step logs which concerns it registered vs. skipped (see Constraints).

**`McpDb::open()` → global `mcp.db`**
- **Pattern:** Synchronous, at most once per machine (the seed only fires while the global db doesn't yet exist).
- **Contract:** If `packaged_seed_db()` finds `<exe_dir>/../data/mcp.db`, `seed_if_absent` copies it to `mcp_dir()/mcp.db` only when that path is absent; any existing global db, from any prior run, is left untouched. This is the path every documented client integration actually exercises, since none of them launch a launcher script (see System Overview).

**Default/Bootstrap Agent System → `capability-provisioning`**
- **Pattern:** Synchronous query, unchanged mechanism from proposal 04.
- **Contract:** Default Agent System resolves candidate Agent Systems by concern; `capability-provisioning` and `domain-system-evaluation` are now two distinct, independently resolvable candidates instead of one bundled `analysis` concern.

### Communication Diagram

```text
xtask build-release → dharma-build.toml : read [[release.providers]]
xtask build-release → <config_path>     : parse AgentSystemProviderConfig | DomainSystemProviderConfig
xtask build-release → pkg_dir/data/mcp.db : register_agent_system(name, concern, content_root)
client → bin/dharma-mcp : launch (direct — mcp-configuration.md's documented pattern for every client)
bin/dharma-mcp → McpDb::open() : packaged_seed_db() found, global mcp.db absent → seed_if_absent(global, packaged)
Default Agent System → mcp.db : resolve(domainSystemConcern) → candidateAgentSystems
```

## Data Flow

### Data Paths

**Provider Capture Path** (unchanged mechanism, new entries, new destination database)
- **Entry point:** A provider repository's `content_root`, named by a `dharma-build.toml`-referenced config file.
- **Transformations:** `capture_bundle` walks `content_root` into `content_asset` rows (proposal 08), same as any runtime registration, but writing into `pkg_dir/data/mcp.db` instead of `~/.dharma/mcp.db`. A separately tracked gap (`issue/agentic-system/001-analyse-agent-system-registered-with-zero-agents-or-skills.md`) means the further step from `content_asset` into structured `agent`/`skill` rows is not yet implemented for any Agent System. This proposal's Release Bundling Step produces the same `content_asset`-only state that gap already describes — it does not by itself close that gap.
- **Ownership boundary:** Everything upstream of `content_asset` belongs to the provider repository; `content_asset` onward belongs to the (packaged) `mcp.db`.
- **Exit point:** `content_asset` rows for each available concern, present in `pkg_dir/data/mcp.db`. On a fresh install, the first client to launch `bin/dharma-mcp` triggers `McpDb::open()`'s seed-once copy into `mcp_dir()/mcp.db`, making those rows part of the single global store from then on — not a second, package-local database.

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| `capability-provisioning` / `domain-system-evaluation` source content | Kriti, in its own split folders (precondition of this proposal) |
| `agent-system-evaluation` source content | The provider repository, entirely — never copied into `dharma` |
| Captured `content_asset` rows | `pkg_dir/data/mcp.db`, written by the existing capture flow at build time |
| `[[release.providers]]` entry list | `dharma-build.toml`, read by the new Release Bundling Step |
| Per-provider config (name, concern, content_root) | The individual `config_path` files it references — existing, unchanged struct shape |

## Security

### Trust Boundaries

- **Provider content ↔ Dharma capture:** Unchanged trust boundary from proposal 08/11 — captured content is data, not directly executable instruction, for all three concerns including the provider-owned `agent-system-evaluation`.
- **`agent-system-evaluation` self-certification:** A provider's own `agent-system-evaluation` Agent System must never be the sole gate that approves that same provider's other Agent Systems' `capability_manifest` — the existing human-approval gate (`review_capability_manifest`, requiring `human_approved`+`reviewed_by`) still applies unchanged, per Philosophy's "no self-certification" principle (`docs/proposal/archive/10-philosophy.md`).

### Threat Model

- **A future non-Kriti provider ships malicious `agent-system-evaluation` content:** Mitigation is unchanged from the existing provider trust model (proposal 08/11) — captured content only becomes live Agents/Skills through the same registration + human-reviewed capability manifest path every other Agent System already goes through; this proposal grants no new trust or privilege to this concern.
- **Concern-name collision at release-bundling time:** `agent_system_registry.concern` is `UNIQUE` at the schema level (`schema/mcp/01-agent_system_registry.sql:12`) — a second `register_agent_system` call on a taken concern already fails with a raw SQLite constraint error today. The Release Bundling Step must catch that error and report which provider entry collided, rather than letting a raw SQLite error abort the whole release build silently.

## Lifecycle

> Status: draft
> Draft commit: not yet committed
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means, concretely: (1) Kriti's `analyse/` folder is physically split into `capability-provisioning/` and `domain-system-evaluation/` subfolders, each containing the files listed under Component Model above; (2) `dharma-build.toml` supports a `[[release.providers]]` array and `crates/xtask`'s release step registers each entry into `pkg_dir/data/mcp.db`; (3) `McpDb::open()` seeds the global `mcp_dir()/mcp.db` from a packaged `data/mcp.db` on first open, only when the global path is absent, verified by launching a packaged `bin/dharma-mcp` directly (not `run-mcp.sh`) against a fresh `DHARMA_MCP_DIR`; (4) the disposition of the old `analyse`/`analysis` registration (see Constraints) has been carried out; and (5) a full `xtask build-release` run has been exercised at least once end-to-end against real (post-split) Kriti content — done: `capability-provisioning` (20 files) and `domain-system-evaluation` (28 files) both captured successfully once Kriti's split landed, `agent-system-evaluation` correctly skipped pending provider content.

## Rationale

### Require a Kriti-Side Folder Split, Not a Dharma-Side Filter
- **Context:** An earlier draft of this proposal claimed the split needed "no new capture mechanism" while also describing capture as if it accepted a subset filter — those two claims contradict each other, and no such filter exists in `capture_bundle` or `register_agent_system` today.
- **Decision:** Require Kriti to physically separate `analyse/` into two folders before Dharma registers them as two concerns. `capture_bundle`/`register_agent_system` stay genuinely unchanged; the split lives entirely in the provider's file layout, which is exactly where proposal 05's "selected not authored" model already says provider structure belongs.
- **Alternatives Considered:** (a) Add subset/filter capability to `capture_bundle` so one `content_root` can be captured twice with different inclusion rules; (b) accept full duplication — register both concerns against the same unfiltered folder, accepting that both get all 9 agents + 13 skills.
- **Rejection Reason:** (a) is real new product surface for a one-time, one-provider need — disproportionate, and the filter's inclusion rules would themselves need to live somewhere (this proposal, a config file, or Kriti) with no obviously better home than "Kriti's own folder structure." (b) defeats the entire purpose of the split — both registrations would still serve two concerns each, the exact problem this proposal exists to fix.
- **Architectural Goal:** Each Agent System registration serves exactly one concern, per proposal 04, using the capture mechanism exactly as it exists today.

### `agent-system-evaluation` Stays Entirely Provider-Owned
- **Context:** If Dharma authored or vendored a copy of this concern's content to ship a release faster, "swap to a different provider later" would require untangling a Dharma-owned copy from a provider-owned original.
- **Decision:** This concern's source content is never copied into or committed inside the `dharma` repository — only ever captured at release-build time from a configured external `content_root`, identical to how `rust-dev-domain` is captured from Kriti today rather than vendored into `dharma`.
- **Alternatives Considered:** Vendor a starter copy into `dharma` so the release has something even before Kriti publishes content.
- **Rejection Reason:** A vendored copy immediately becomes a second source of truth that drifts from the real provider content, and defeats the stated goal of a clean future provider swap.
- **Architectural Goal:** Provider swappability stays literal — changing one config value, not untangling ownership.

## Constraints

### Hard Constraints
- **Precondition, not this proposal's own deliverable:** Kriti's `analyse/` folder must be split into `capability-provisioning/` and `domain-system-evaluation/` subfolders (per Component Model) before either concern can be registered — this is Kriti-repository work, tracked as a dependency of this proposal, not performed by it.
- `agent-system-evaluation` source content must never be committed inside the `dharma` repository itself (source: Rationale above).
- The Release Bundling Step must catch and report a concern-name collision per provider entry, not let a raw SQLite constraint error abort the build without identifying which entry caused it (source: Security, Threat Model above).
- **Old `analyse`/`analysis` registration disposition:** No `unregister_agent_system` tool exists (only `unregister_repo`, `crates/mcp/src/adapter.rs:408`), and `capability_manifest.agent_system_id` has no `ON DELETE` cascade (`schema/mcp/28-capability_manifest.sql:13`) — so the existing approved `capability_manifest` (id 1, for `mock-repo-new`, against `analyse`) blocks any attempt to delete the old row outright, and this proposal does not introduce a new delete tool. Disposition: the old `analyse` row is **not deleted**; `capability_manifest` id 1 is superseded by proposing and approving two new capability manifests (for the same repo, against `capability-provisioning` and `domain-system-evaluation`) through the existing, unchanged `review_capability_manifest` flow. The old row remains in `agent_system_registry` as superseded history — consistent with this repository's append-only, no-silent-delete pattern elsewhere (e.g. `proposal_commit_log`, proposal 12).
- This proposal does not implement the `content_asset` → `agent`/`skill` structured-registry parse step; its output is bounded by whatever that step (tracked separately in `issue/agentic-system/001-...md`, which also notes each captured file is currently duplicated — a pre-existing bug this proposal does not fix) currently does or doesn't do.

### Soft Constraints
- Prefer the Release Bundling Step log which concerns it successfully registered and which it skipped for missing content, so an empty `agent-system-evaluation` bundle in a given release is visible, not silent.
- `task_step.required_capability` is a real `FOREIGN KEY` against `agent_system_registry.concern` (`schema/mcp/11-task_step.sql:16`). Currently moot — every registered Domain System has zero `task` rows (per `issue/domain-system/001-...md`) — but once Tasks exist, any `task_step` row referencing the `analysis` concern would need repointing to `capability-provisioning` or `domain-system-evaluation` before the old row could ever be removed in a future proposal.

## Traceability

### Derivation Chain

```text
Agent System Registry (04) — "one Agent System, one concern" constraint this proposal enforces
Domain System Registration (05) / Provider Config & Repo Sync (11) — the "selected not authored" provider pattern this proposal extends to a third Agent System
    │
    ▼
Agent System Concern Split & Release Bundling (this document)
    │
    ▼
Kriti: analyse/ → capability-provisioning/ + domain-system-evaluation/ (precondition)
crates/xtask: Release Bundling Step
crates/registry: McpDb::open() first-run global-DB seeding
dharma-build.toml: [[release.providers]] entries for the three concerns
```

### Non-Contradiction Rule

No downstream change may register more than one unrelated concern under a single Agent System row, copy `agent-system-evaluation` (or any future provider-owned concern) source content into the `dharma` repository itself, let a provider's own Agent System self-certify approval of that same provider's other Agent Systems without the existing human-approval gate, or claim a capture-time filtering mechanism exists in `capture_bundle`/`register_agent_system` without this document being revised to specify it.

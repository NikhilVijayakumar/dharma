# Issue: The only registered Agent System has zero Agents and zero Skills

**Found:** 2026-08-04, verifying `mcp.db` (`/home/dell/.dharma/mcp.db`) directly (MCP `list_agent_systems` currently fails schema validation — see `docs/proposal/15-mcp-list-tool-result-shape-fix.md` — so this was checked via direct DB query instead).

## Claim being checked

User-stated assumption: an Agent System exists with Agents/Skills able to (a) implement the Rust Domain System's content, and (b) validate Domain System / Agent System claims. Checked directly against the registry — **both parts are unverified, and the second is currently false in practice.**

Exactly one Agent System is registered:

| id | name | concern | is_privileged | description |
|---|---|---|---|---|
| 1 | `analyse` | analysis | false | "Domain-System verification (A) and agent-capability provisioning (B)" |

## What's actually broken

`agent`, `skill`, `agent_skill_binding`, and `agent_goal` — the tables that would hold this Agent System's actual Agents and Skills — are **all empty** (0 rows each). The `analyse` Agent System is a registry row with a description and nothing behind it: no Agent is queryable, no Skill is invocable, no binding exists between them.

Despite that, `capability_manifest` id 1 (agent_system_id 1, repo_registration_id 1 — `mock-repo-new`) is already **approved** (reviewed by `nikhil`, 2026-08-03). The capability manifest was approved for an Agent System that cannot currently do anything.

## What raw content exists (but isn't registered)

`content_asset` holds captured YAML/Markdown source for what this Agent System is *supposed* to contain — 9 Agent definitions and 13 Skills, each captured **twice** (every path below appears as two separate `content_asset` rows — a likely duplicate-capture bug in its own right, worth checking whatever `recapture_agent_system` run produced them):

**Agents** (`agent/*.yaml`): `assignment-planner`, `capability-analyser`, `domain-system-verifier`, `domain-verifier`, `gap-analyser`, `hierarchy-verifier`, `orchestrator`, `section-verifier`, `workflow-designer`

**Skills** (`skill/*/skill.yaml` + `prompt.md` + example): `analyse-domain-system`, `design-handoff-workflow`, `identify-agent-gaps`, `map-task-to-capability`, `propose-agent-assignment`, `render-provisioning-report`, `render-verification-report`, `verify-epic-completion`, `verify-epic-usecase-task`, `verify-section-map`, `verify-section-profile`, `verify-task-completion`, `verify-usecase-completion`

None of these are parsed into the `agent`/`skill` tables, so none are actually usable through the registry yet.

## Answering the user's question directly

- **Agent/Skill to *implement* (author) the Rust Domain System's content:** not present. Every one of the 9 Agents above is verification/orchestration/gap-analysis oriented (verifier, gap-analyser, capability-analyser, orchestrator, workflow-designer, assignment-planner) — none authors domain content. There is currently no Agent System in the registry whose job is to *write* `rust_dev` / `rust-dev-domain` material.
- **Agent/Skill to *validate* Domain System / Agent System claims:** this is exactly what `analyse`'s captured-but-unregistered content is for (`domain-system-verifier`, `hierarchy-verifier`, `section-verifier`, `analyse-domain-system`, `verify-epic-completion`, `verify-usecase-completion`, `verify-task-completion`, `capability-analyser`, `identify-agent-gaps`). The intent exists in the captured files. It is **not yet live** — zero rows in `agent`/`skill` — so right now nothing can actually run that validation.

## Suggested next step

Run whatever registration/parse step turns captured `agent/*.yaml` and `skill/*/skill.yaml` content into `agent`/`skill`/`agent_skill_binding` rows (mirrors the same missing-parse-step gap filed in `issue/domain-system/001-...md`), fix the duplicate-capture (each file currently captured twice), then re-verify `capability_manifest` id 1's approval actually corresponds to a populated Agent System.

# Vision: Dharma — the MCP layer of an open agent platform

> Status: Draft. Tier-1 vision document for the Dharma agent platform. Sources the constraints proposals 01-07 cite as "(source: Vision)", most concretely proposal 01's Repository-independence hard constraint and proposal 08's "Dharma Is Infrastructure, Not an Author" decision. Supersedes the earlier product framing recorded in `00-overview.md`'s Supersession Note (the pivot decision: Electron application → agent platform).

## The Pivot

Dharma began as an Electron application (a director's office suite backed by a TypeScript domain library). That framing is dead. Dharma is now the **MCP server layer** of an agent platform: the storage and serving shape that lets independently-authored Domain Systems and Agent Systems be registered once, captured faithfully, and served to every repository that needs them. Nothing in the proposals rebuilds the Electron application; the archived codebase in `archive/` is the record of the prior product, not the future of this one.

## The Problem

An agent platform has three hard problems that this vision addresses:

1. **Content is scattered and unauthored-anywhere.** Domains (Section Maps, Section Profiles), Agents, Skills, tasks, prompts, scripts, and audit definitions are authored by different providers in different formats. Without a canonical layer, every repository re-negotiates shape and version on its own — Samgraha's one-standard-per-repo coupling, reproduced.
2. **Reuse requires registration and capture.** Providers are the authors; nothing should copy their content by hand or re-derive it by convention. A repository should select an existing Domain System and Agent System, not author its own shape.
3. **Execution without governance is dangerous.** Agents act on real repositories. Nothing may execute until a proposal has been drafted, reviewed, revised, and explicitly approved — and every execution must be auditable against a captured standard, per commit.

## Dharma's Role

Dharma is the MCP infrastructure layer. It defines the storage and serving shape — nothing else. Providers (knowledge systems, agent-management systems) author Domain Systems and Agent Systems. Dharma:

1. **Registers** them.
2. **Captures** their files into Dharma's own data directory, recording every file in the `content_asset` ledger.
3. **Serves** the captured content to any registered repository: on registration a repo is matched to a Domain System and the applicable Agent Systems, and the required content is synced into that repo's own `repo.db`.
4. **Caches analysis** so a subsequently registered repo gets an already-computed resolution or audit instantly.

## Core Principles

- **Infrastructure, not an author.** Dharma never invents a domain, an agent, a skill, or a section profile. Every content row traces to a captured provider file or a provider-declared template/seeder. Bundling built-in content would make Dharma compete with the providers it serves.
- **Plug-in authorship.** The ecosystem's taxonomy is open and registrable — Domain Systems (proposal 05) and Agent Systems (proposal 04) are named, versioned, independently-authored assets, not fixed enumerations.
- **Repository-independence.** An Agent definition may not reference a specific repository, domain file, or path. Agents and Skills are authored once and reused across every repository that adopts the platform (source of proposal 01's hard constraint).
- **Selection, not authorship.** A repository selects an existing Domain System; it does not author its own domain shape.
- **Approval gates.** Repository registration and Capability Manifests are gated by human approval. No Task executes without an approved proposal (proposal 07).
- **Traceability.** Every content row, every audit evidence string, every proposal revision, every handoff hop traces to a captured source. Nothing is asserted without a source.
- **Per-repo execution, global reuse.** Audit definitions and analysis conclusions are global and reused; audit executions and runtime state are per-repository observations.

## Goals

- Give the ecosystem a single canonical registration, capture, and serving layer for Domain and Agent content.
- Let repositories adopt a Domain System and Agent System by selection, with a recorded, human-approved Capability Manifest.
- Make every execution auditable: deterministic rules plus a per-model semantic ensemble, evidence persisted per commit, human overrides and cancels recorded.
- Reuse prior art's validated shapes (Samgraha's six-crate Rust MCP server, its global-vs-per-repo database split) rather than inventing parallel ones.
- Make the platform bootstrap itself through its own Agent/Skill model (the Default/Bootstrap Agent System is an Agent System like any other).

## Non-Goals

- Dharma does not author Domain Systems, Agent Systems, domains, agents, skills, or section profiles.
- Dharma is not an execution engine for arbitrary code; scripts it captures execute only in a repo's own context and only when that repo selects the owning system.
- Dharma is not a fixed taxonomy. Proposals 04/05 made the registry open; no downstream document may reintroduce a closed set.
- The MCP wire-level tool contract is not defined by these proposals (08 defines the `mcp` crate's role, not its request/response shapes).

## Success Criteria

The platform is successful when:

- A new repository can register, select a Domain System, get a human-approved Capability Manifest, and begin proposing work — without hand-copying any content.
- Two repositories selecting the same Domain System resolve to the same audited outcome, retrieved from `analysis_cache` rather than re-derived.
- Every Task Instance's path from proposal draft to approved execution to handoff chain to completion verdict can be replayed from stored rows, with every hop's context intact.
- Every audit score can be justified: rules, evidence, model reasoning, and any human override are all recorded.
- Providers can update a Domain System or Agent System and have the change captured, versioned, and re-synced to the repositories that selected it.

## Traceability

```text
Vision (this document — the pivot: Electron app → agent platform)
    │
    ▼
Architecture (docs/proposal/08) — storage and crate shape, sourced by Vision
    │
    ├──▶ Agent Model (01) — Repository-independence sourced by Vision
    ├──▶ Domain System Registration (05) — open taxonomy, selected not authored
    ├──▶ Agent System Registry (04) — open taxonomy, plug-in authorship
    ├──▶ MCP Registration & Bootstrap (06) — approval-gated onboarding
    └──▶ Proposal & Execution Protocol (07) — nothing executes without approval
```

**Non-contradiction rule:** No downstream proposal or document may (a) assign repository-specific domain knowledge to an Agent definition, (b) close the Domain System or Agent System registries into a fixed set, (c) allow content rows without a captured source, or (d) allow execution without an approved proposal — without revising this document first.

## Related

- [Proposal 00 — Overview](00-overview.md) — the index, which now lists this document as resolved (it previously flagged it as a missing prerequisite).
- [Proposal 08 — Schema & Crate Architecture](08-schema-and-crate-architecture.md) — the concrete realization of this vision; "Dharma Is Infrastructure, Not an Author" restates Principle 1 in architectural terms.
- `docs/raw/architecture.md` — the Architecture standard that requires this Vision as a cross-reference.

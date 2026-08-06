# Philosophy: authority, restraint, and trust in an agent platform

> Status: Draft. Tier-1 philosophy document for the Dharma agent platform. Guides Architecture (per `docs/raw/architecture.md`'s Relationships: "Architecture is guided by Philosophy") and Security (which references Philosophy(02) in its cross-references). Supplies the first principles the threat models in proposals 04, 06, 07, and 08 rest on.

## Stance

Dharma is an agent platform whose agents act on real repositories. Its philosophy is the answer to one question: **under what conditions may an agent act with real effect?** The proposals answer it the same way every time — only after a human has authorized it, only through capability that was declared and recorded, and never unverifiably. This document states those conditions as principles so that Architecture, Security, and every downstream proposal cite one source rather than re-derive them.

## Principles

### 1. Human authorization is a structural gate, not a convention

An agent proposing work, a repository joining the platform, and a Capability Manifest granting capability are all proposals until a human approves them. This is not etiquette; it is structure. The Task Runtime separates Proposal State from Execution State, and nothing may enter execution without a recorded approval event (`repo.db`'s `proposal_approval`). Registration cannot be used until a finalized, human-approved Repo Registration Record exists. The gate is never removed — routine approvals may be fast, but the approval itself is mandatory.

### 2. Voluntary restraint is not verifiable

A rule an agent merely promises to follow is a wish, not a control. If effect-capable Skills could be reached during the Proposal Loop, "review before approval" would be meaningless — effects could already have happened. Restraint therefore lives in structure: Proposal Loop Skills must be analysis-only, and effect-capable Skills are inert until the Execution Loop. Whenever a principle can be enforced by shape rather than by promise, the shape wins.

### 3. Nobody self-certifies

The Agent that executes a Task does not get to declare it complete. An independent Completion Validator, checked against the Task's declared Acceptance Criteria, records the verdict. An audit's deterministic rules and per-model semantic scores are checked against captured evidence, and a same-model-same-commit re-run is de-duplicated rather than double-scored — so a result can always be replayed from stored rows.

### 4. Effect follows least privilege

An Agent invokes only the Skills on its declared binding allowlist. An Agent System's privileged status (Agent-Management) is the only boundary that may author definitions, and a Capability Manifest naming it requires explicit justification and explicit human approval. A repository receives only the capability it was resolved to need, never capability it requested on its own.

### 5. Everything is traceable to a source

Dharma is infrastructure, not an author: it registers, captures, and serves provider-authored content, and it invents none. Every content row, every audit evidence string, every proposal revision, every handoff hop, every override records what happened, who or what did it, and what it was based on. An assertion without a recorded source is treated as absent.

### 6. The registry is open; the gates are not

Domain Systems and Agent Systems are an open, pluggable registry — no fixed taxonomy, no closed set. Openness applies to what may be registered, not to what may act. Every new Agent System and Domain System still passes the same registration and approval discipline. Openness and control are complementary, not competing.

### 7. Integrity crosses database boundaries

Content that shares one store gets real foreign keys; the one boundary that is genuinely separate data (`repo.db` → `mcp.db`) keeps the logical-reference treatment, and forged references are rejected before commit. The integrity of the platform's records is not left to the well-behavedness of any single writer.

## Goals

- Make the conditions for agent action explicit, structural, and auditable.
- Give Security one set of principles to translate into threat models, and Architecture one set of principles to translate into structure.
- Keep human authority over the platform's irreversible events while letting the platform's routine work proceed without per-step friction.

## Non-Goals

- This document does not define specific threats or mitigations (those live in the proposals' Security sections and in Security documentation).
- It does not take a position on agent autonomy outside the boundaries of this platform.
- It does not prescribe the MCP wire contract, tool shapes, or implementation choices.

## Success Criteria

The philosophy holds when:

- No Task executes without a recorded, human-granted approval (proposal 07, enforced by `proposal_approval`).
- No repository is served without a human-approved registration and Capability Manifest (proposal 06).
- No effect-capable Skill runs during the Proposal Loop (proposal 07, enforced by the `is_analysis_only` flag).
- No executing Agent self-certifies completion (proposal 07, Completion Validator).
- Every content row and every audit score traces to a captured source or recorded evidence (`content_asset`, audit evidence rows).
- The registries accept new Domain/Agent Systems without exception, and no downstream document reintroduces a closed set.

## Traceability

```text
Philosophy (this document)
    │
    ▼
Security (proposals 04, 06, 07, 08 — threat models rest on these principles)
    │
    ▼
Architecture (docs/proposal/08 — structure realizes the principles)
    │
    ├──▶ Agent Model (01) — bindings, privileged boundary, ≤8 goals
    ├──▶ Agent System Registry (04) — open registry, privileged writes
    ├──▶ Domain System Registration (05) — selected not authored
    ├──▶ MCP Registration & Bootstrap (06) — approval-gated onboarding
    └──▶ Proposal & Execution Protocol (07) — no execution without approval
```

**Non-contradiction rule:** No downstream document may permit a Task to execute without recorded human approval, allow an effect-capable Skill during the Proposal Loop, let an executing Agent self-certify completion, allow an unprivileged Agent System to author definitions, or close the Domain/Agent System registries — without revising this document first.

## Related

- [Proposal 09 — Vision](09-vision.md) — the pivot and Dharma's role; Philosophy states the principles under which the vision's platform operates.
- [Proposal 00 — Overview](00-overview.md) — the index, which now lists this document as resolved (it previously tracked it as a missing prerequisite).
- `docs/raw/architecture.md` — the Architecture standard that requires Philosophy as a cross-reference (Security, Rationale) and declares "Architecture is guided by Philosophy".

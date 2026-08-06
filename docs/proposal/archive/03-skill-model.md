# Proposal: Skill Model

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Amended:** Every Skill declares whether it is analysis-only (safe to invoke while an Agent drafts a proposal) or effect-capable (reachable only once a proposal is approved). See Proposal & Execution Protocol proposal.
> **Corrected:** Prompt and Example Set are mandatory (not symmetric with Script, which is optional), and a fourth optional asset, Template, is added — both to match the schema and Domain/Agent Content Model already specified in proposal 08 (the earlier text in this document treated Prompt/Script as either-or-both and didn't mention Template at all).

## Purpose

This document defines the standard for the Skill entity within Dharma's agent-based execution architecture. Skill Documentation describes the smallest executable unit of capability — a mandatory prompt, an optional script, one or more worked examples, and an optional template, bound to exactly one responsibility — that an Agent composes to act.

Unlike Samgraha, where `step_script` and `step_prompt` are bound to a specific Usecase step, a Skill in Dharma is Task-independent and reusable: it is authored once, declares one responsibility, and any Agent whose Skill Bindings include it may invoke it from any Task.

## System Overview

### Overview

A Skill bundles up to four assets around one declared responsibility: a Prompt (semantic instruction, mandatory — every Skill is reachable through LLM reasoning even when a deterministic path also exists), a Script (deterministic executable, optional — used when the responsibility is mechanical), an Example Set (mandatory, one or more worked cases showing correct invocation and output), and a Template (optional — content an Agent may substitute when generating output for a Task). A Skill is invoked by an Agent within a Task step and returns output conforming to its declared Invocation Contract.

### Structural Approach

The Skill Model is the execution primitive beneath the Agent Model. Agents do not act directly; they act only through invoking Skills. Every Skill supports a Prompt path; a Skill may additionally support a Script path for the same responsibility, letting the runtime pick the deterministic path when available and fall back to the mandatory Prompt path otherwise — never the reverse, since the Prompt is the one path every Skill is guaranteed to have.

### Diagram

```text
┌───────────────────────────────────────────────┐
│                     Skill                      │
│                                                 │
│  Skill Definition (single responsibility)      │
│  Prompt (semantic path — mandatory)            │
│  Script (deterministic path — optional)        │
│  Example Set (worked cases — mandatory, ≥1)    │
│  Template (optional, for output generation)    │
│  Invocation Contract (input/output shape)      │
└───────────────────────────────────────────────┘
      ▲ invoked by                    │ runs on
      │                               ▼
   [ Agent ]              [ Script Runtime | Prompt/LLM Runtime ]
```

## Component Model

### Skill Definition
- **Responsibility:** States the Skill's single declared responsibility in one sentence.
- **Ownership:** Name, responsibility statement.
- **Interfaces:** Read by the Agent-Management Agent System during audit to confirm the Skill has not grown beyond one responsibility.

### Prompt
- **Responsibility:** Holds the semantic instruction template used to invoke this Skill through LLM reasoning. Mandatory — every Skill must have exactly one, even if a Script path also exists.
- **Ownership:** Prompt template text and variable slots.
- **Interfaces:** Consumed by the Prompt/LLM Runtime; receives the Skill's Invocation Contract input. Registration is rejected if this component is absent.

### Script
- **Responsibility:** Holds the deterministic executable used when this Skill's responsibility is fully mechanical. Optional — a Skill without one is invoked entirely through its Prompt.
- **Ownership:** Script logic.
- **Interfaces:** Consumed by the Script Runtime; receives the Skill's Invocation Contract input; preferred over the Prompt path when present (see Constraints).

### Example Set
- **Responsibility:** Demonstrates correct invocation and expected output for calibration. Mandatory — at least one worked example is required.
- **Ownership:** Worked input/output pairs, plus dos/don'ts, best practices, and common mistakes.
- **Interfaces:** Read by authors and reviewers; may be surfaced to the Prompt path as few-shot context. Registration is rejected if no example exists.

### Template
- **Responsibility:** Holds optional content an Agent may substitute when generating output for a Task that this Skill contributes to, in the same spirit as a Task's own optional `template_ref` (see Epic/Usecase/Task Model proposal).
- **Ownership:** Template content, scoped to this Skill.
- **Interfaces:** Read by an Agent during the Proposal or Execution Loop when it judges this Skill's template a better fit than the Task's own.

### Invocation Contract
- **Responsibility:** Declares the explicit input shape an Agent must supply, the output shape the Skill returns, and whether the Skill is analysis-only or effect-capable.
- **Ownership:** Input/output schema for this Skill, plus its analysis-only/effect-capable declaration.
- **Interfaces:** Enforced at invocation time by the Skill Registry, regardless of whether the Prompt or Script path executes. The Proposal Loop (see Proposal & Execution Protocol proposal) may only invoke Skills declared analysis-only; the Execution Loop may invoke either kind.

### Component Diagram

```text
Agent ──invoke(contract input)──▶ Skill Registry
                                       │
                         ┌─────────────┴─────────────┐
                         ▼                            ▼
                   Script Runtime               Prompt/LLM Runtime
                   (deterministic path)          (semantic path)
                         │                            │
                         └─────────────┬──────────────┘
                                       ▼
                              contract output ──▶ Agent
```

## Communication

### Communication Paths

**Agent → Skill Registry**
- **Pattern:** Synchronous invocation.
- **Contract:** Agent submits Invocation Contract input; Skill Registry checks the calling Agent's Skill Bindings allowlist before dispatch.

**Skill Registry → Script Runtime / Prompt-LLM Runtime**
- **Pattern:** Synchronous dispatch, path chosen by Skill capability (deterministic available vs. semantic-only).
- **Contract:** Runtime executes the Script or Prompt against the same Invocation Contract input and returns output in the declared output shape.

### Communication Diagram

```text
Agent → Skill Registry : invoke(skillRef, input)
Skill Registry → Script Runtime : run(script, input)        [if deterministic path exists]
Skill Registry → Prompt/LLM Runtime : run(prompt, input)     [otherwise]
Runtime → Skill Registry : output
Skill Registry → Agent : output
```

## Data Flow

### Data Paths

**Invocation Path**
- **Entry point:** Agent passes task-step data into the Skill's Invocation Contract.
- **Transformations:** Script Runtime or Prompt/LLM Runtime transforms input into output per the Skill's single responsibility.
- **Ownership boundary:** The Skill owns the transformation; it does not retain state between invocations.
- **Exit point:** Output returned to the invoking Agent, conforming to the Invocation Contract's output shape.

### Data Flow Diagram

```text
Agent ──input──▶ Invocation Contract ──▶ Script Runtime | Prompt/LLM Runtime ──▶ output ──▶ Agent
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Skill Definition, Prompt, Script, Example Set, Template | Skill authoring (fixed once approved) |
| Invocation input/output | Passes through the Skill statelessly per call |
| Skill allowlist per Agent | Agent definition (see Agent Model) |

## Security

### Trust Boundaries

- **Script path:** Can touch filesystem/network/process resources — treated as the higher-risk execution path and sandboxed accordingly.
- **Prompt path:** Runs against an LLM with no direct system access unless a tool is explicitly granted — lower default risk, but subject to prompt-injection from untrusted step input.

### Threat Model

- **Responsibility creep:** A Skill accumulates behavior beyond its declared single responsibility. Mitigation: Skill Definition audit checks scope at review time; a Skill doing more than one thing is split.
- **Script injection:** Untrusted Task input reaches the Script path unsanitized. Mitigation: Invocation Contract input is typed and validated before the Script Runtime executes.
- **Prompt injection:** Untrusted Task input manipulates the Prompt path into ignoring its Skill's declared responsibility. Mitigation: Prompt templates isolate variable slots from instruction text; the Prompt path has no filesystem/network access unless a tool is explicitly declared in the Invocation Contract.
- **Mislabeled effect capability:** A Skill with real side effects is declared analysis-only, making it reachable during proposal drafting. Mitigation: the analysis-only/effect-capable declaration is checked by the Agent-Management Agent System at registration, not left to the Skill author's self-assessment alone.

## Rationale

### One Skill, Two Execution Paths — Prompt Mandatory, Script Optional
- **Context:** Samgraha splits deterministic (script) and semantic (prompt) execution at the Usecase-step level, tying each to one specific step. A Skill also needs one path that always exists, so any Agent can invoke it even before a deterministic implementation has been authored.
- **Decision:** A single Skill may offer both a Prompt path and a Script path for the same declared responsibility, but the Prompt is mandatory and the Script is optional — never the reverse.
- **Alternatives Considered:** Separate Skill types for "deterministic skill" and "semantic skill"; treating Prompt and Script as symmetric, either-or-both.
- **Rejection Reason:** Splitting by execution mechanism instead of by responsibility fragments what is conceptually one capability and prevents reuse when a deterministic implementation later becomes available for a previously prompt-only Skill. Treating the two as symmetric would let a Script-only Skill exist with no LLM-reachable path at all, which every other part of this platform (the Proposal Loop's drafting, an Agent's judgment about which Skill to invoke) assumes is always available.
- **Architectural Goal:** Reusable, mechanism-agnostic capability, with one path — the Prompt — guaranteed present on every Skill.

### Template Is Optional, Not a Third Required Asset
- **Context:** Like a Task's own optional `template_ref`, a Skill may have output-generation content worth offering an Agent, but not every Skill produces output a template would help shape.
- **Decision:** Template is a fourth, optional asset — present only when the Skill's author judges it useful.
- **Alternatives Considered:** Make Template mandatory alongside Prompt and Example Set.
- **Rejection Reason:** Mandating a template for every Skill, including ones with no natural output shape to template, would force authors to write a template with nothing meaningful to say.
- **Architectural Goal:** Optional richness without a mandatory-asset tax on every Skill.

### Mandatory Single Responsibility
- **Context:** Capability bundles that grow to cover multiple concerns become hard to test, audit, and reuse.
- **Decision:** Every Skill must declare and be scoped to exactly one responsibility.
- **Alternatives Considered:** Multi-responsibility Skills scoped by Task instead of by function.
- **Rejection Reason:** Multi-responsibility Skills are the atomic-capability equivalent of monolithic components — they violate the composability the Agent/Task model depends on.
- **Architectural Goal:** Composable, independently testable Skills.

### Mandatory Example Set
- **Context:** Prompt-driven Skills drift in behavior without a calibration reference; humans reviewing a Skill need a concrete sense of correct output.
- **Decision:** Every Skill requires at least one worked example demonstrating correct invocation and output.
- **Alternatives Considered:** Examples optional, left to author discretion.
- **Rejection Reason:** Optional examples default to absent, reproducing the ambiguity Samgraha's prompt-only steps suffered from without MCP-side calibration.
- **Architectural Goal:** Predictable, reviewable Skill behavior.

## Constraints

### Hard Constraints
- **Single declared responsibility** (source: Rationale above) — a Skill covering more than one responsibility must be split before registration.
- **Domain-free naming** (source: Agent Model precedent) — a Skill must not reference a specific Task or repository domain by name.
- **Invocation Contract required** (source: Component Model) — a Skill without an explicit input/output shape cannot be registered.
- **Prompt and at least one Example are mandatory; Script and Template are optional** (source: Rationale above) — a Skill without a Prompt, or without at least one Example Set entry, cannot be registered.

### Soft Constraints
- Prefer the Script path over the Prompt path when the responsibility is fully deterministic, for cost and reliability.

## Traceability

### Derivation Chain

```text
Agent Model
    │
    ▼
Skill Model (this document)
    │
    ├──▶ Agent System Registry (Agent Systems bundle Skills by concern)
    └──▶ Proposal & Execution Protocol (analysis-only Skills drive proposal drafting; all Skills are the execution primitive within a step)
```

### Non-Contradiction Rule

No downstream proposal may bind a Skill to a specific Task or repository domain, or permit a Skill to cover more than one responsibility, without revising this document first.

# Proposal: Agent Model

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Amended:** Agent never executes a Task directly. Its first responsibility on assignment is to draft a proposed solution for user review; only after explicit approval does it execute using its Skill Bindings. See Proposal & Execution Protocol proposal.

## Purpose

This document defines the standard for the Agent entity within Dharma's agent-based execution architecture. Agent Documentation describes the structural shape of an autonomous actor — its identity, its goals, and the experience that justifies those goals — independent of any single repository's domain.

Unlike a Task (a unit of work) or a Skill (an atomic capability), an Agent is the actor that performs work. Unlike Samgraha's per-repository Standard, an Agent carries no embedded domain knowledge — it is authored once and reused across every repository that adopts Dharma's platform.

## System Overview

### Overview

An Agent is a named actor with a single role, up to eight goals, and a backstory that grounds each goal in claimed experience. Agents do not own domain knowledge or task-specific logic; they own judgment about which Skills to invoke and when to hand control to another Agent. An Agent is the unit that Task execution assigns work to, and the unit that an Agent System registers as a reusable capability.

### Structural Approach

The Agent Model sits below the Agent System Registry (which classifies Agents by concern) and above the Skill Model (which an Agent composes to act). It is deliberately thin: identity, goals, backstory, and a declared set of Skill bindings.

### Diagram

```text
┌───────────────────────────────────────────────┐
│                     Agent                      │
│                                                 │
│  Identity (name, role)                         │
│  Goal Set (≤ 8 goals, ordered)                 │
│  Backstory (experience per goal)               │
│  Skill Bindings (declared, allowlisted)        │
│  Handoff Policy (when/to whom control passes)  │
└───────────────────────────────────────────────┘
         │ invokes                 │ hands off to
         ▼                         ▼
   [ Skill Registry ]       [ another Agent ]
```

## Component Model

### Identity
- **Responsibility:** Names the actor and states its single role in one sentence.
- **Ownership:** Name, role string.
- **Interfaces:** Read by Task Runtime to display/attribute work; read by Handoff Broker to route control.

### Goal Set
- **Responsibility:** Enumerates up to eight goals the Agent pursues, ordered by priority.
- **Ownership:** The ordered goal list.
- **Interfaces:** Read by the Agent itself when selecting a Skill for a step; read by the Agent-Management Agent System when auditing role scope.

### Backstory
- **Responsibility:** Justifies each goal with a concrete claim of experience, so the goal is not an unsupported assertion.
- **Ownership:** One backstory entry per goal.
- **Interfaces:** Read during Agent authoring/review; not consumed at runtime by other components.

### Skill Bindings
- **Responsibility:** Declares the closed set of Skills this Agent is permitted to invoke.
- **Ownership:** The allowlist of Skill references.
- **Interfaces:** Checked by the Skill Registry on every invocation; violations are rejected before execution.

### Handoff Policy
- **Responsibility:** States the conditions under which this Agent transfers control to another Agent instead of continuing.
- **Ownership:** Handoff trigger conditions and candidate target roles (not specific Agent instances).
- **Interfaces:** Consulted by the Handoff Broker (see Proposal & Execution Protocol proposal) when a step exceeds this Agent's declared Skill Bindings.

### Proposal Responsibility
- **Responsibility:** On assignment, drafts a proposed solution for the Task — intended Skills, expected handoffs, and how the result will satisfy the Task's Acceptance Criteria Set — before any effect-capable Skill is invoked.
- **Ownership:** The current proposal draft, until the user approves it.
- **Interfaces:** Submits drafts to, and receives revision requests from, the Proposal Loop (see Proposal & Execution Protocol proposal). Only executes via Skill Bindings after that loop signals approval.

### Component Diagram

```text
Task Runtime ──assigns step──▶ Agent ──invokes──▶ Skill (via Skill Bindings)
                                  │
                                  └─ triggers Handoff Policy ──▶ Handoff Broker ──▶ next Agent
```

## Communication

### Communication Paths

**Task Runtime → Agent**
- **Pattern:** Synchronous assignment, asynchronous completion signal.
- **Contract:** Task Runtime assigns a Task (or, once past a handoff, the remaining Step Sequence of an in-progress Task) with its input contract; Agent acknowledges and later returns step output or a handoff request.

**Agent → Skill Registry**
- **Pattern:** Synchronous invocation.
- **Contract:** Agent submits a Skill Binding reference plus the current step's context; Skill Registry returns the Skill's output or rejects if the binding is not declared.

**Agent → Handoff Broker**
- **Pattern:** Asynchronous, event-driven.
- **Contract:** Agent submits a handoff request with the current Context Envelope; Handoff Broker acknowledges and takes ownership of routing (see Proposal & Execution Protocol proposal).

### Communication Diagram

```text
Task Runtime → Agent : assign(task)
Agent → Skill Registry : invoke(binding, stepContext)
Skill Registry → Agent : result
Agent → Handoff Broker : handoff(context)   [only if Handoff Policy triggers]
```

## Data Flow

### Data Paths

**Task Assignment Path**
- **Entry point:** Task Runtime assigns a Task to an Agent.
- **Transformations:** Agent maps each step in the Task's Step Sequence to one or more declared Skill Bindings, in order.
- **Ownership boundary:** Agent owns the Task's working data only for the duration of its turn (i.e., until it completes its steps or requests a handoff).
- **Exit point:** Step output returned to Task Runtime, or a handoff request with the Context Envelope.

### Data Flow Diagram

```text
Task Runtime ──assign(task)──▶ Agent ──per step──▶ Skill Bindings ──▶ step output
                                  │
                                  └─ handoff request + Context Envelope ──▶ Handoff Broker
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Agent identity, goals, backstory | Agent definition (authored, immutable at runtime) |
| Skill allowlist | Agent definition |
| Task working data | Currently-assigned Agent, for the duration of its turn |
| Context Envelope | Handoff Broker (see Proposal & Execution Protocol proposal) between hops |

## Security

### Trust Boundaries

- **Authoring → Runtime:** Agent definitions are authored/reviewed content (trusted once committed); runtime task input flowing into an Agent's steps is untrusted external data.
- **Agent → Skill Registry:** Trusted boundary, but bounded by the declared Skill Bindings allowlist — an Agent cannot invoke an undeclared Skill.

### Threat Model

- **Goal drift:** An Agent's behavior expands beyond its declared goals over time. Mitigation: goals and backstory are immutable at runtime; changes require Agent-Management Agent System review.
- **Skill scope escalation:** An Agent invokes a Skill outside its declared bindings. Mitigation: Skill Registry enforces the allowlist and rejects undeclared invocations.
- **Handoff impersonation:** A malicious step result claims a handoff to a privileged Agent. Mitigation: Handoff Broker resolves target Agents by role/policy, not by a caller-supplied identity.

## Rationale

### Goal Cap at Eight
- **Context:** Agents risk becoming unbounded catch-all actors if goals are unlimited.
- **Decision:** Cap the Goal Set at eight entries, ordered by priority.
- **Alternatives Considered:** Unlimited goals with a "primary goal" flag.
- **Rejection Reason:** Unlimited goals make an Agent's scope unauditable and encourage role sprawl.
- **Architectural Goal:** Bounded, auditable actor scope.

### Mandatory Backstory Per Goal
- **Context:** A goal stated without justification is indistinguishable from an aspirational claim.
- **Decision:** Every goal requires a backstory entry establishing the experience behind it.
- **Alternatives Considered:** A single free-text backstory for the whole Agent.
- **Rejection Reason:** A single backstory cannot be checked against individual goals during review.
- **Architectural Goal:** Reviewable, credible actor definitions.

### Domain-Free Agent Definitions
- **Context:** Samgraha's model binds capability to a per-repository Standard, making capability non-portable.
- **Decision:** Agent definitions must not embed repository-specific domain knowledge.
- **Alternatives Considered:** Domain-scoped Agents authored per repository.
- **Rejection Reason:** Domain-scoped Agents cannot be reused across repositories, recreating Samgraha's one-standard-per-repo coupling.
- **Architectural Goal:** Cross-repository reusability (see Domain System Registration proposal).

## Constraints

### Hard Constraints
- **Repository-independence** (source: Vision) — an Agent definition may not reference a specific repository, domain file, or path.
- **Declared Skill Bindings only** (source: Security threat model above) — an Agent may not invoke a Skill absent from its allowlist.
- **Goal Set ≤ 8** (source: Rationale) — enforced at authoring/validation time.

### Soft Constraints
- Prefer a narrow, single-role Agent over a broad, multi-purpose one unless the Agent System Registry explicitly calls for breadth.

## Traceability

### Derivation Chain

```text
Vision (Dharma pivot: Electron app → agent platform)
    │
    ▼
Agent Model (this document)
    │
    ├──▶ Epic/Usecase/Task Model (Agents are assigned Tasks)
    ├──▶ Skill Model (Agents invoke Skills)
    ├──▶ Agent System Registry (Agents are classified into Agent Systems)
    └──▶ Proposal & Execution Protocol (Agents propose, then hand off control to execute)
```

### Non-Contradiction Rule

No downstream proposal may assign domain-specific knowledge to an Agent definition, or exceed the eight-goal cap, without revising this document first.

# Proposal: Proposal & Execution Protocol

> Status: Draft — design-only, no schema/code. Conforms to `docs/raw/architecture.md` standard.
> **Supersedes** the original Orchestration & Handoff Protocol draft. The system never executes a Task directly. Every Task first goes through a mandatory Propose → Review → Approve gate; execution (the handoff chain across Agents/Skills) happens only after the user approves the proposed solution. This requirement has priority over any part of the original draft that implied direct execution.

## Purpose

This document defines the standard for how a Task moves from assignment to completion in Dharma: first through a proposal loop where Agents draft and refine a solution with the user, then — only after explicit approval — through an execution loop where Agents hand off control to one another using Skills until the Task's Output Contract is produced and validated.

Unlike Samgraha, where a deterministic step's script can run immediately and unreviewed, and where an optional proposal mode exists but is unreliable, Dharma makes the proposal step mandatory and universal: no Task is executed until an Agent has proposed a solution, the user has reviewed and, if needed, requested changes, and the user has explicitly approved the proposal.

## System Overview

### Overview

A Task assigned by the Task Runtime enters the Proposal Loop before any Skill executes with real effect. The initiating Agent — using its Skills for analysis, not for effect — drafts a proposed solution: what it intends to do, which Agents and Skills it expects to hand off to, and how the result will satisfy the Task's Acceptance Criteria Set. The user reviews, discusses, and may request changes; the Agent updates the proposal accordingly. Only once the user approves does the Task enter the Execution Loop, where the same handoff-chain model applies: one Agent acts using one or more Skills, then transfers control to the next Agent, until the Task completes and the Completion Validator checks the result.

### Structural Approach

Two loops, one Task: the Proposal Loop (iterative, user-in-the-loop, no side effects) always precedes the Execution Loop (handoff chain, real effects, gated only at entry by the approved proposal). A Task cannot enter the Execution Loop without an approved proposal on record.

### Diagram

```text
Task Runtime ──assign──▶ Agent
                            │
                            ▼
                     PROPOSAL LOOP
              ┌───────────────────────────┐
              │ Agent drafts proposal      │
              │ User reviews / discusses   │
              │ Agent revises              │◀── repeat until agreement
              └─────────────┬──────────────┘
                            │ user approves
                            ▼
                    EXECUTION LOOP
              ┌───────────────────────────┐
              │ Agent ──skill──▶ output    │
              │   │ handoff (if needed)    │
              │   ▼                        │
              │ next Agent ──skill──▶ ...  │◀── repeat until Task done
              └─────────────┬──────────────┘
                            ▼
                  Completion Validator
```

## Component Model

### Task Runtime
- **Responsibility:** Owns the Task's overall state — which loop it is in (Proposal or Execution), the current owning Agent, and the handoff/revision history.
- **Ownership:** Execution State, extended with a Proposal State (draft, under review, revised, approved).
- **Interfaces:** Moves a Task from Proposal State to Execution State only on an explicit user approval event; invokes the Completion Validator once Execution Loop steps report finished.

### Proposal Loop
- **Responsibility:** Coordinates the iterative draft/review/revise cycle between an Agent and the user before any Skill executes with real effect.
- **Ownership:** The current proposal draft, its revision history, and the review comments attached to each revision.
- **Interfaces:** Receives a draft from the assigned Agent; presents it to the user; routes user feedback back to the Agent; signals the Task Runtime on approval.

### Handoff Broker
- **Responsibility:** During the Execution Loop, resolves which Agent should receive control next when a handoff is requested, and confirms the receiving Agent accepts before transferring ownership. Unchanged in role from the original draft.
- **Ownership:** Handoff resolution logic, loop/depth detection.
- **Interfaces:** Receives handoff requests from the current executing Agent; queries the Agent System Registry (see Agent System Registry proposal) for a matching Agent; offers control to the resolved Agent.

### Context Envelope
- **Responsibility:** Carries the accumulated input, output, and history needed by the next Agent to continue the Task, across both loops.
- **Ownership:** Append-only record; a Proposal Loop revision is one entry, an Execution Loop handoff is another entry, in the same envelope.
- **Interfaces:** Written by the Proposal Loop on each revision and by the Handoff Broker on each hop; read by whichever Agent or the user reviews it next.

### Completion Validator
- **Responsibility:** Checks the Task's final Execution Loop output against its Acceptance Criteria Set before the Task Runtime marks the Task complete. Unchanged in role from the original draft.
- **Ownership:** Validation logic, independent of any executing Agent.
- **Interfaces:** Invoked once the Execution Loop reports finished; returns pass or fail with reason.

### Component Diagram

```text
Task Runtime ──assign──▶ Agent ──drafts──▶ Proposal Loop ──presents──▶ User
                                                 ▲                        │
                                                 └────── revise ──────────┘
                                                          │ approve
                                                          ▼
Task Runtime ──enters Execution Loop──▶ Agent ──invoke──▶ Skill
                                          │
                                          └─ handoff ──▶ Handoff Broker ──▶ next Agent
                                                          │
                                                          ▼
                                                Completion Validator
```

## Communication

### Communication Paths

**Agent → Proposal Loop**
- **Pattern:** Synchronous draft submission, repeated across revisions.
- **Contract:** Agent submits a proposed solution (intended Agents/Skills, expected handoffs, how Acceptance Criteria will be met); Proposal Loop records it and presents it to the user.

**Proposal Loop → User, User → Proposal Loop**
- **Pattern:** Asynchronous, human-paced, iterative.
- **Contract:** User reviews and either approves or returns feedback; Proposal Loop routes feedback back to the Agent for a revised draft.

**Proposal Loop → Task Runtime**
- **Pattern:** Synchronous, one-time per Task.
- **Contract:** On user approval, Proposal Loop signals the Task Runtime to transition the Task into the Execution Loop.

**Agent → Handoff Broker** (Execution Loop, unchanged from original draft)
- **Pattern:** Asynchronous, event-driven.
- **Contract:** Agent submits a handoff request with the current Context Envelope; Handoff Broker resolves a target Agent and confirms acceptance before completing the transfer.

**Task Runtime → Completion Validator** (unchanged from original draft)
- **Pattern:** Synchronous, invoked once at Execution Loop end.
- **Contract:** Task Runtime submits the accumulated output; Completion Validator returns pass or fail against the Acceptance Criteria Set.

### Communication Diagram

```text
Agent → Proposal Loop : draft(proposedSolution)
Proposal Loop → User : present(draft)
User → Proposal Loop : approve | feedback(comments)
Proposal Loop → Agent : revise(comments)               [repeat until approve]
Proposal Loop → Task Runtime : approved(finalProposal)
Task Runtime → Agent : enterExecutionLoop
Agent → Handoff Broker : handoff(contextEnvelope)        [if requested]
Task Runtime → Completion Validator : validate(output, acceptanceCriteria)
```

## Data Flow

### Data Paths

**Proposal Path**
- **Entry point:** Task Runtime assigns a Task to an initiating Agent.
- **Transformations:** Agent analyzes the Task (using Skills for analysis only, without side effects) and produces a draft solution; each user review cycle produces a revised draft.
- **Ownership boundary:** The Proposal Loop owns the draft and its revision history until user approval.
- **Exit point:** An approved proposal, handed to the Task Runtime to open the Execution Loop.

**Execution Path** (unchanged in structure from the original draft)
- **Entry point:** Approved proposal enters the Execution Loop.
- **Transformations:** Each Agent invokes Skills to transform step input toward step output; each handoff carries the Context Envelope forward.
- **Ownership boundary:** Execution State tracks the current owning Agent at every point.
- **Exit point:** Final output, checked by the Completion Validator, becomes the Task's Output Contract data or is returned for rework.

### Data Flow Diagram

```text
Agent ──draft──▶ Proposal Loop ──review/revise──▶ User ──approve──▶ Task Runtime
                                                                          │
                                                              Execution Loop (Agent ⇄ Skill ⇄ handoff)
                                                                          │
                                                                          ▼
                                                              Completion Validator
```

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Proposal draft and revision history | Proposal Loop, until user approval |
| Approved proposal | Task Runtime, as the entry condition for the Execution Loop |
| Execution State (current owner, handoff log) | Task Runtime, during the Execution Loop |
| Context Envelope | Proposal Loop (during drafting) and Handoff Broker (during execution) |
| Validation verdict | Completion Validator |

## Security

### Trust Boundaries

- **Proposal Loop ↔ Agent:** Any Agent's draft is treated as a suggestion, never as an instruction with effect — Skills invoked during proposal drafting must not perform actions with real-world side effects.
- **Task Runtime ↔ Execution Loop entry:** A hard gate — no Task may enter the Execution Loop without a recorded user approval event.
- **Handoff Broker ↔ Agents:** Trusted routing boundary — Agents request handoffs but do not choose their own successor (unchanged from the original draft).

### Threat Model

- **Silent execution without approval:** An Agent or a runtime bug causes a Task to begin executing with real effect before the user approves. Mitigation: Task Runtime structurally separates Proposal State from Execution State; Skills capable of real effect are only reachable from the Execution Loop, never from the Proposal Loop.
- **Proposal drafting with side effects:** An Agent uses an effect-capable Skill during drafting to "preview" a result, causing unintended real changes. Mitigation: Skills used during the Proposal Loop must be analysis-only (see Skill Model proposal); effect-capable Skills declare themselves as such and are unreachable until the Execution Loop.
- **Infinite handoff loop, Context Envelope tampering, premature completion:** Unchanged from the original draft — mitigated respectively by Handoff Broker depth/cycle limits, an append-only Envelope, and a Completion Validator structurally independent of every executing Agent.

## Rationale

### Mandatory Proposal Before Any Execution
- **Context:** Samgraha's optional proposal mode is inconsistently used and does not reliably know what to do, because it is a bolt-on rather than the default path.
- **Decision:** Every Task must pass through the Proposal Loop and receive explicit user approval before the Execution Loop begins; this is not optional for any Task.
- **Alternatives Considered:** Keep proposal as an opt-in mode, defaulting to direct execution.
- **Rejection Reason:** Opt-in proposal defaults to skipped under time pressure, exactly reproducing Samgraha's reliability gap.
- **Architectural Goal:** User stays in control of what changes before any change happens.

### Analysis-Only Skills During Drafting
- **Context:** If an Agent could use any Skill while drafting a proposal, "reviewing before approval" would be meaningless — effects could already have happened.
- **Decision:** Skills invoked during the Proposal Loop must be declared analysis-only; effect-capable Skills are reachable only from the Execution Loop.
- **Alternatives Considered:** Trust Agents to voluntarily avoid effectful Skills during drafting without a structural restriction.
- **Rejection Reason:** Voluntary restraint is not verifiable and would undermine the guarantee the approval gate is meant to provide.
- **Architectural Goal:** A proposal is genuinely inspectable before anything happens.

### Execution Loop Retains the Original Handoff-Chain Design
- **Context:** The one-agent-does-its-part-then-hands-off model, and the independent Completion Validator, already solve the problems they were designed for.
- **Decision:** Carry the Handoff Broker, Context Envelope, and Completion Validator forward unchanged into the Execution Loop; only the entry condition (approval) is new.
- **Alternatives Considered:** Redesign execution to be a single Agent per Task, now that a human has approved the plan.
- **Rejection Reason:** A single-Agent redesign would discard the composability the handoff chain provides — a task may still genuinely require several distinct capabilities in sequence.
- **Architectural Goal:** Reuse validated design where it still fits; change only what the new requirement (mandatory approval) actually demands.

## Constraints

### Hard Constraints
- **No Execution Loop entry without recorded approval** (source: Rationale above) — enforced structurally by the Task Runtime.
- **Proposal Loop Skills must be analysis-only** (source: Threat Model above) — effect-capable Skills are inert until the Execution Loop.
- **All handoffs via Handoff Broker, mandatory Completion Validator pass** (carried over from the original draft) — unchanged.

### Soft Constraints
- Prefer short proposal-revision cycles; an Agent should converge on an approvable draft within a small number of rounds, escalating to a different Agent (via the Agent-Management Agent System) if it cannot.

## Traceability

### Derivation Chain

```text
Epic/Usecase/Task Model, Agent Model, Skill Model, Agent System Registry, MCP Registration & Bootstrap
    │
    ▼
Proposal & Execution Protocol (this document)
    │
    ▼
(terminal proposal — feeds future Engineering/Implementation documentation, out of scope here)
```

### Non-Contradiction Rule

No downstream proposal or engineering document may permit a Task to enter the Execution Loop without a recorded user approval of its proposal, allow an effect-capable Skill to run during the Proposal Loop, or let an executing Agent self-certify Task completion, without revising this document first.

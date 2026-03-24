# Agent Communication Protocol

## Overview
Virtual Employees operate independently but must collaborate to ensure the "Antigravity" standard is met. We use a **Protocol-Based Handshake** system rather than direct inheritance method calls. This allows agents to be loosely coupled, dynamically communicating through the Global Sequential Queue.

## The Handshake Mechanism
When Agent A requires Agent B's domain expertise, it does not call `agent_b.run()`. Instead, Agent A submits a `Handshake Request` payload to the Global Queue.

### Types of Handshaking
1. **Support/Delegation Request**: Asking another agent to perform a sub-task.
   - *Example*: Julia (CTO) requests `VISUAL_AUDIT` from Sofia (Design).
2. **Clearance/Approval Request**: Asking another agent for permission to proceed.
   - *Example*: Maya (Funding) requests `BUDGET_APPROVED` from Nora (CFO).
   - *Example*: All agents request `COMPLIANCE_PASS` from Eva.
3. **Passive Broadcast (State Updates)**: Emitting a read-only variable for all agents to ingest during Stage 1 (`stage_init`).
   - *Example*: Lina emits `CONTEXT_LEVEL=FATIGUE_HIGH`.
   - *Example*: Nora emits `RUNWAY=CRITICAL`.

## Communication Permisibility Matrix
Not all agents can command all other agents. The strict command structure prevents infinite loops and organizational chaos.

| Agent | Can Request Support From (Handshake To) | Can Receive Requests From |
| :--- | :--- | :--- |
| **Mira** | All Agents (Initial Routing) | All Agents (Final Synthesis) |
| **Arya** | Nora (Finance Check), Julia (Tech Feasibility) | Dani (Feature Marketing), Julia (Major Pivots) |
| **Elina** | Mira (Force-clear Queue), Julia (System Fix) | All Agents (If task fails/times out) |
| **Nora** | Elina (Scale down Ops) | Maya (Funding checks), Arya (Pivot checks) |
| **Lina** | *None* (Passive Observer) | Mira (Context sync on init) |
| **Eva** | Originating Agent (If violation found) | All Agents (Mandatory prior to completion) |
| **Maya** | Nora (Budget constraint), Dani (Marketing metrics) | *None* |
| **Dani** | Julia (Feature logs), Sofia (Design collateral check) | Maya (Growth metrics), Arya (Brand alignment) |
| **Julia** | Sofia (Visual audit), Eva (Security check) | Dani (Feature requests), Elina (Bug fixes) |
| **Sofia** | *None* (Pure evaluator) | Julia (UI code), Dani (Marketing images) |

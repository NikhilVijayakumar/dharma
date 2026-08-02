# Stage Execution Protocol

## Overview
To maximize flexibility while maintaining a common architecture, all Virtual Employees in the Dhi system eschew rigid object-oriented inheritance. Instead, they implement the **Stage Execution Protocol**.

This protocol defines a mandatory, 4-stage lifecycle that every agent must execute when a slot in the Global Sequential Queue becomes available. While the stages are common across all agents, *what* happens in each stage is entirely determined by the specific Skills and Rules owned by that agent.

## The 4-Stage Lifecycle

### Stage 1: Initialization & Context Binding (`stage_init`)
Before starting any work, an agent must bind its local constraints and read the global queue state.
- **Mandatory Action**: Read the `Work Order` from Mira.
- **Mandatory Check**: Read the current `CONTEXT_LEVEL` broadcast by Lina (HR) to determine output verbosity.
- **Agent Action**: The agent loads its specific `Rule` files into its context.
  - *Example*: Eva loads `dcm-1-strict.md`. Nora loads `local-first-finance.md`.

### Stage 2: Planning & Validation (`stage_plan`)
The agent determines how it will solve the Work Order and validates the approach against its rules before executing.
- **Mandatory Action**: Select the model tier (local LM Studio, OpenRouter, or Gemini) based on the Director's active configuration via `ModelGatewayService.ts` and the agent's task requirements.
- **Agent Action**: The agent applies its specific `Skill` files to determine the methodology.
  - *Example*: Julia applies `clean-architecture-enforcement.md` to design a folder structure. Dani applies `narrative-design.md` to outline a campaign.
- **Handshake Check**: Does this plan require support from another agent? If yes, initiate a `Handshake Request` (see Agent Communication Protocol).

### Stage 3: Execution & Implementation (`stage_execute`)
The work is performed via TypeScript services in the Electron main process.
- **Mandatory Action**: Run the assigned task deterministically.
- **Agent Action**: The agent invokes its domain-specific TypeScript service via IPC.
  - *Example*: Sofia invokes `VisualAuditService.ts`. Elina invokes `WorkflowMonitorService.ts`.

### Stage 4: Synthesis & Output (`stage_synthesis`)
The service outputs are synthesized into the final result format and pushed back to the queue.
- **Mandatory Action**: Summarize the output according to the `CONTEXT_LEVEL` verbosity limitation.
- **Agent Action**: Submit output for Eva's final Compliance Audit. If Eva passes it, mark the Work Order as `COMPLETE` in the Global Queue so Mira can route it to the Director.

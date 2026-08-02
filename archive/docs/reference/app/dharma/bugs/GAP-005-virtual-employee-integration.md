# GAP-005: Virtual Employee Integration and Missing Core Services

**Severity**: HIGH  
**Category**: Agent Runtime / Service Layer  
**Date**: 2026-03-19  
**Status**: NOT STARTED ΓÇö Requires Agent Runtime Implementation  
**Affects**: All virtual employees, interaction backbone, core agent services  

---

## Summary

The `docs/virtual-employee/` specs define 10 virtual employees with specific TypeScript services, skills, rules, agentic assets, inter-agent handshakes, and protocol integrations. The actual implementation has **no agent runtime** ΓÇö no TypeScript services per agent, no agent-to-LLM communication, no Work Order system, no Stage Execution Protocol, and no Director Γåö Agent interaction surface.

---

## 1. Missing Agent TypeScript Services

Each virtual employee spec defines TypeScript services that should run in the Electron main process. None exist.

| Agent | Documented Service(s) | Exists? |
|:------|:---------------------|:--------|
| Mira (Secretary) | `QueueService.ts`, `CommandRouterService.ts` | Γ¥î |
| Arya (CEO) | `StrategyEvaluatorService.ts` | Γ¥î |
| Nora (CFO) | `VaultIngestService.ts`, `ForecastService.ts` | Γ¥î (vaultService.ts exists but is not Nora-specific) |
| Julia (CTO) | `GitGovernorService.ts`, `SystemHealthService.ts` | Γ¥î |
| Elina (COO) | `OpsMetricsService.ts` | Γ¥î |
| Eva (Compliance) | `ComplianceScanService.ts`, `AuditLogService.ts` | Γ¥î |
| Dani (CMO) | `AudienceAnalyticsService.ts` | Γ¥î |
| Sofia (Design) | `VisualAuditService.ts` | Γ¥î |
| Lina (HR) | `InteractionTrackingService.ts` | Γ¥î |
| Maya (Funding) | `OpportunityTrackerService.ts` | Γ¥î |

---

## 2. Missing Agent Work Order System

The triage and executive suites modules document a **Work Order** system where:
1. Director input is triaged by Mira using the Eisenhower Matrix
2. Mira creates structured `WorkOrder` intents
3. Work orders are dispatched to the appropriate agent via `CommandRouterService.ts`
4. Each agent processes the work order through a 4-stage lifecycle: **Init ΓåÆ Plan ΓåÆ Execute ΓåÆ Synthesis**
5. Agent outputs land in `agent-temp/` for Director approval

**None of this pipeline exists.** There is no:
- Work Order entity or type definition
- Eisenhower Matrix classification logic 
- Command routing from Mira to agents
- 4-stage execution lifecycle
- Agent output generation to `agent-temp/`

---

## 3. Missing Interaction Backbone (`interaction-backbone.md`)

This document defines a **mandatory** cross-module interaction contract:

### Expected
Every post-login page must show:
- Page owner identity (from virtual employee specs)
- Secretary identity (Mira)
- Quick actions: "Ask Owner" and "Ask Secretary"
- Free-text feedback input and send action

### Actual
- **No interaction strip/panel exists** on any page
- **No page owner identity** is shown anywhere
- **No Director Γåö Agent communication** is possible
- **No free-text request submission** functionality

The `DirectorFeedbackRequest` interface documented in the interaction backbone spec is not implemented anywhere:
```typescript
interface DirectorFeedbackRequest {
  moduleRoute: string;
  targetEmployeeId: string;
  message: string;
  timestampIso: string;
}
```

---

## 4. Missing Agent-to-LLM Communication Pipeline

The documentation specifies that agents communicate with LLMs via `ModelGatewayService.ts`:
- Each agent has a **model tier preference** (local LM Studio for privacy-sensitive, cloud for complex tasks)
- Agents use skills to generate structured outputs
- The context engine manages per-agent session tokens

**Current state**: 
- `modelGatewayService.ts` can probe providers and determine which is healthy
- But **no code** sends actual LLM requests through the gateway
- No prompt construction, no response parsing, no agent prompt templates
- The skill system has execution adapters but they don't call the model gateway with agent-specific prompts

---

## 5. Missing Inter-Agent Handshake Execution

Each virtual employee spec defines **inter-agent handshakes** ΓÇö which agents can request from which:

| Agent | Can Request From | Receives From |
|:------|:----------------|:-------------|
| Arya | Nora, Julia, Elina, Eva, Dani | Julia (pivots), Nora (finance) |
| Nora | None (CFO is endpoint) | Arya, Mira (ledger routing) |
| Julia | Nora (budget check) | Arya, Elina |
| ... | ... | ... |

**None of these handshakes are implemented.** The subagent system provides the infrastructure for delegation but no agent uses it for real inter-agent workflows.

---

## 6. Missing KPI Protocol Implementation

The KPI Protocol is a core system that:
1. **`registry.json`** stores KPIs per agent (created during onboarding)
2. Each agent monitors its own KPIs
3. KPI statuses drive the Daily Brief and Executive Suites
4. Stale data triggers re-upload requests

**Current state**:
- No `registry.json` file exists or is created
- No onboarding wizard generates KPIs
- No agent monitors any KPI
- KPI data in daily brief and suites is fabricated

---

## 7. Missing Queue System Protocol

Mira's queue system should implement:
- **10-slot queue buffer** with priority ordering
- **Crisis slot reservation** (1 always reserved for critical tasks)
- **Queue events**: `TASK_ENQUEUED`, `TASK_STARTED`, `TASK_FAILED`
- **Memory protection**: pause at 80% RAM, cooldown at 90% CPU

**Current state**:
- No `QueueService.ts` in main process
- Queue monitor displays fabricated tasks from `operationsService.ts`
- No real task enqueuing, execution, or lifecycle tracking
- No memory protection monitoring

---

## 8. Missing Document Management Protocol

The Document Management Protocol defines:
- Report lifecycle: `DRAFT ΓåÆ REVIEW ΓåÆ APPROVE ΓåÆ COMMIT`
- Schema validation against `schema_validation.json`
- Data classification (T1-T4) enforcement
- PII redaction before storage

**Partially implemented**:
- Γ£ô Schema validation exists in `vaultService.ts`
- Γ£ô Data classification exists (RESTRICTED/INTERNAL/CONFIDENTIAL/PUBLIC)
- Γ¥î No PII redaction/hashing before storage
- Γ¥î Report lifecycle only exists in governance as state transitions on seeded data
- Γ¥î No `ComplianceScanService.ts` for PII scanning

---

## Impact Summary

The virtual employee integration gaps mean:
1. **Dhi has no functioning AI agents** ΓÇö the core value proposition of 10 virtual executives is entirely unimplemented
2. **The Director cannot interact with any agent** ΓÇö no chat, no request submission, no feedback loop
3. **No real data processing** ΓÇö agents don't analyze, recommend, or produce deliverables
4. **The KPI and governance systems are cosmetic** ΓÇö they display fabricated data, not agent outputs
5. **The model gateway is unused** ΓÇö despite being one of the most complete features, nothing actually sends prompts to LLMs

---

## Resolution Roadmap

### Phase 1: Agent Service Scaffolding (Week 1-2)
Create stub TypeScript services for all 10 agents in `src/main/agents/`:
```
src/main/agents/
Γö£ΓöÇΓöÇ arya.ts          # Arya (CEO)
Γö£ΓöÇΓöÇ nora.ts          # Nora (CFO)
Γö£ΓöÇΓöÇ julia.ts         # Julia (CTO)
Γö£ΓöÇΓöÇ elina.ts         # Elina (COO)
Γö£ΓöÇΓöÇ eva.ts           # Eva (Compliance)
Γö£ΓöÇΓöÇ dani.ts          # Dani (CMO)
Γö£ΓöÇΓöÇ sofia.ts         # Sofia (Design)
Γö£ΓöÇΓöÇ lina.ts          # Lina (HR)
Γö£ΓöÇΓöÇ maya.ts          # Maya (Funding)
Γö£ΓöÇΓöÇ mira.ts          # Mira (Secretary) + QueueService
ΓööΓöÇΓöÇ core/
    Γö£ΓöÇΓöÇ WorkOrderService.ts      # Work Order lifecycle
    Γö£ΓöÇΓöÇ AgentProtocol.ts         # Base protocol for all agents
    ΓööΓöÇΓöÇ InteractionBackbone.ts   # Director Γåö Agent communication
```

### Phase 2: Work Order System Implementation (Week 2-3)
Implement the Work Order pipeline:
1. Create `WorkOrder` type definition with Eisenhower classification
2. Implement `CommandRouterService.ts` for routing work orders to agents
3. Implement 4-stage lifecycle: Init ΓåÆ Plan ΓåÆ Execute ΓåÆ Synthesis
4. Wire Triage UI to create work orders
5. Wire Executive Suites to display agent processing status

### Phase 3: Agent-to-LLM Communication (Week 3-4)
Wire agents to model gateway:
1. Create agent-specific prompt templates for each agent
2. Implement `sendPromptToAgent(agentName, context, prompt)` in each agent service
3. Add response parsing specific to each agent's output type
4. Integrate context engine for session-aware reasoning
5. Store agent outputs to `agent-temp/{agentId}/` for approval

### Phase 4: Directive Γåö Agent Interaction Backbone (Week 4-5)
Implement interaction surface on every module:
1. Create `InteractionPanel` component showing page owner + Mira
2. Add "Ask Owner" and "Ask Secretary" button handlers
3. Implement `DirectorFeedbackRequest` IPC flow
4. Integrate with notification center for agent responses
5. Wire all 21 modules to show interaction panel

### Phase 5: KPI & Queue Integration (Week 5-6)
Implement KPI and queue protocols:
1. Create `registry.json` in `.dhi/kpis/` during onboarding
2. Implement `KPIMonitorService.ts` for per-agent KPI tracking
3. Implement `QueueService.ts` with 10-slot buffer and memory protection
4. Integrate queue events into subagent telemetry
5. Wire daily brief and suites to read real KPI data

### Phase 6: End-to-End Agent Workflows (Week 6+)
Implement sample agent workflows:
1. **Governance**: Julia commits code on Director's behalf after Eva audit
2. **Triage**: Mira routes to Nora (finance) or Eva (compliance) based on classification
3. **Daily Brief**: Mira compiles brief using inputs from all agents at 8AM
4. **Executive Suites**: Per-agent workspace tabs show real agent outputs and metrics

---

## Current Workaround (Until Implementation)

- Virtual employees names exist in UI but don't perform any agent operations
- All agent outputs are seeded/hardcoded via `operationsService.ts`
- Director cannot request agent assistance on any module
- The 10 agents are cosmetic profiles, not functional AI entities

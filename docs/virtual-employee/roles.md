# Virtual Employee Roles Reference

## Overview

Dhi has **10 Virtual Employees**, each with a defined domain, authority boundary, model preference, and set of TypeScript services they own. All agents are orchestrated by Mira and run as TypeScript services in the Electron main process, invoked via IPC from the renderer.

---

## Role Summary Table

| Agent | Role | Model Preference | Crisis Status |
|:------|:-----|:----------------|:--------------|
| **Mira** | Secretary & Command Router | Dynamic (LM Studio → Gemini) | ACTIVE (restricted routing) |
| **Arya** | CEO — Strategic Alignment | Dynamic (LM Studio → Gemini) | INACTIVE |
| **Nora** | CFO — Financial Forecasting | LM Studio primary, Gemini for narrative | ACTIVE |
| **Elina** | COO — Operations & Queue | LM Studio primary, Gemini for analysis | ACTIVE |
| **Julia** | CTO — Engineering & Architecture | LM Studio primary, Gemini for complex arch | PARTIAL |
| **Eva** | Compliance Officer | LM Studio primary, Gemini for semantics | ACTIVE |
| **Dani** | CMO — Marketing & Narrative | Gemini primary, LM Studio offline | INACTIVE |
| **Sofia** | Head of Design | LM Studio primary, Gemini for creative | INACTIVE |
| **Lina** | HR — Director Context | Dynamic (LM Studio → Gemini) | INACTIVE |
| **Maya** | Funding & Resource Procurement | Gemini primary, LM Studio offline | INACTIVE |

---

## Authority Boundaries

Each agent operates strictly within their domain. Cross-domain actions require explicit handshakes.

| Domain | Owner | Cannot Do |
|:-------|:------|:----------|
| Strategy | Arya | Approve spend, execute code, publish comms |
| Finance | Nora | Approve hiring, execute code, make strategy |
| Operations | Elina | Make pivots, approve budgets, execute code |
| Technology | Julia | Approve budgets, make strategy, publish comms |
| Compliance | Eva | Provide legal advice, make decisions, approve spend |
| Marketing | Dani | Approve budgets, execute code, make strategy |
| Design | Sofia | Execute code, approve budgets, make strategy |
| HR | Lina | Approve hiring, make strategy, execute transactions |
| Funding | Maya | Approve spend, execute campaigns, make strategy |
| Routing | Mira | Make decisions, trigger financial actions, push to Git |

---

## TypeScript Services Ownership

All agent logic runs as Node.js services in the Electron main process. The renderer communicates via IPC channels.

| Service | Owner | Purpose |
|:--------|:------|:--------|
| `QueueService.ts` | Mira | 10-slot task queue, SQLite persistence |
| `ModelGatewayService.ts` | Mira | LLM routing: LM Studio → OpenRouter → Gemini |
| `FiscalForecastService.ts` | Nora | Runway projection, burn rate calculation |
| `VaultIngestService.ts` | Nora | CSV parsing, PII sanitization, local-only |
| `BudgetAuditService.ts` | Nora | API cost tracking, budget compliance |
| `WorkflowMonitorService.ts` | Elina | Queue health polling, latency tracking |
| `OpsReportService.ts` | Elina | Operational health report generation |
| `SystemHealthService.ts` | Julia | Test runner, uptime checks, latency measurement |
| `ArchitectureAuditService.ts` | Julia | Directory tree compliance scanning |
| `CodeGenService.ts` | Julia | Feature scaffolding from spec JSON |
| `GitGovernorService.ts` | Julia | Validated git commit bridge |
| `ComplianceScanService.ts` | Eva | Static artifact analysis, PII detection |
| `AuditLogService.ts` | Eva | Append-only JSONL audit trail |
| `AudienceAnalyzerService.ts` | Dani | Analytics CSV processing |
| `CampaignArchitectService.ts` | Dani | Multi-channel campaign plan generation |
| `VisualAuditService.ts` | Sofia | Token compliance, accessibility scanning |
| `ComponentScaffolderService.ts` | Sofia | MUI component boilerplate generation |
| `InteractionAnalyzerService.ts` | Lina | Director fatigue scoring, context level |
| `StrategyEvaluatorService.ts` | Arya | Cross-department alignment scoring |
| `PitchGeneratorService.ts` | Maya | Funding pitch assembly |
| `CronSchedulerService.ts` | Shared | Cron-based task scheduling (daily brief, weekly review) |
| `NotificationService.ts` | Shared | In-app notification routing by severity |

---

## Model Gateway — Provider Chain

All LLM calls route through `ModelGatewayService.ts`:

```
Agent Request
     │
     ▼
LM Studio (localhost:1234)
     │ Available?
     ├── YES → Execute locally
     └── NO
          │
          ▼
     OpenRouter (HTTPS fallback)
          │ Rate limited or unavailable?
          └── YES
               │
               ▼
          Gemini CLI (child_process.spawn)
```

**Privacy rule**: T4-classified data (raw ledger CSVs, PII) never leaves LM Studio. Only anonymized summaries may reach OpenRouter or Gemini.

---

## Queue Architecture

Managed by `QueueService.ts` with SQLite persistence:

```
Slots 1-8   → Standard execution (max 1 concurrent)
Slots 9-10  → Crisis Protocol reserved (Nora, Eva, Elina only)
```

Task lifecycle: `PENDING → IN_PROGRESS → AWAITING_AUDIT → COMPLETE`

Eva's `ComplianceScanService` must issue `AUDIT_PASS` before any task reaches `COMPLETE`.

---

## Crisis Protocol — Active Agents

When `CRISIS_ACTIVATED` is emitted (requires 2-of-3 from Nora, Eva, Elina):

| Agent | Status | Focus |
|:------|:-------|:------|
| Nora | ACTIVE | Runway monitoring, burn tracking, budget freeze |
| Eva | ACTIVE | Filing enforcement, compliance lock, double-audit |
| Elina | ACTIVE | Queue health, delivery stabilization |
| Julia | PARTIAL | Uptime only, no new features |
| All others | INACTIVE | Suspended until crisis resolved |

---

## Inter-Agent Handshake Map

```
Director
    │
    ▼
Mira ──────────────────────────────────────────────┐
    │                                               │
    ├──→ Arya (strategy)                            │
    │       └──→ Nora (financial viability)         │
    │       └──→ Julia (tech feasibility)           │
    │                                               │
    ├──→ Nora (finance)                             │
    │       └──→ Elina (budget overrun)             │
    │       └──→ Maya (runway math for pitch)       │
    │                                               │
    ├──→ Julia (tech)                               │
    │       └──→ Sofia (VISUAL_AUDIT)               │
    │       └──→ Eva (COMPLIANCE_PASS)              │
    │       └──→ Elina (SYSTEM_DEGRADATION)         │
    │                                               │
    ├──→ Eva (compliance gate — all agents)         │
    │                                               │
    ├──→ Dani (marketing)                           │
    │       └──→ Sofia (brand compliance)           │
    │       └──→ Maya (audience metrics)            │
    │                                               │
    ├──→ Lina (context level — passive broadcast)   │
    │                                               │
    └──→ Maya (funding)                             │
            └──→ Nora (runway math)                 │
            └──→ Arya (strategic alignment)         │
                                                    │
    ◄───────────────────────────────────────────────┘
    All agents report completions back to Mira
```

# KPI Protocol

## Overview
Every Virtual Employee in the Dhi system is measured by specific KPIs (Key Performance Indicators). This document defines how KPIs are discovered, registered, tracked, evaluated, and reported. It establishes the common protocol all agents must follow when dealing with metrics, while each agent applies their domain-specific skills and rules to calculate and evaluate their individual KPIs.

---

## 1. KPI Registry

### 1.1 Source of Truth
All KPIs are registered in a central file within the Governance Repository:
```
/governance/kpi/registry.json
```

This file is the **single source of truth** for what metrics exist, who owns them, and what their thresholds are.

### 1.2 Registry Schema
```json
{
  "kpis": [
    {
      "id": "KPI-NORA-001",
      "name": "Runway Months",
      "owner": "NORA_CFO",
      "type": "FINANCIAL",
      "unit": "months",
      "target": 12,
      "alert_threshold": 6,
      "crisis_threshold": 4,
      "data_source": "/governance/data/raw/ledger.csv",
      "evaluation_frequency": "DAILY",
      "last_evaluated": "2026-02-28T08:00:00Z",
      "last_value": 14,
      "status": "HEALTHY"
    },
    {
      "id": "KPI-JULIA-001",
      "name": "Platform Uptime",
      "owner": "JULIA_CTO",
      "type": "TECHNICAL",
      "unit": "percentage",
      "target": 99.5,
      "alert_threshold": 98.0,
      "crisis_threshold": 95.0,
      "data_source": "/governance/data/raw/health_logs.csv",
      "evaluation_frequency": "HOURLY",
      "last_evaluated": "2026-02-28T16:00:00Z",
      "last_value": 99.7,
      "status": "HEALTHY"
    }
  ]
}
```

### 1.3 KPI Status Values
| Status | Meaning |
|:-------|:--------|
| `HEALTHY` | Value meets or exceeds target. |
| `WARNING` | Value crossed `alert_threshold` but not `crisis_threshold`. |
| `CRITICAL` | Value crossed `crisis_threshold`. May trigger Crisis Protocol. |
| `STALE` | KPI has not been evaluated within its `evaluation_frequency`. |
| `MISSING` | Data source is unavailable or no data uploaded by Director. |

---

## 2. KPI Lifecycle

### 2.1 Discovery
KPIs are not invented by agents. They are derived from:
1. **Phase 1 Operations Playbook** (`docs/phase1/phase-1-operations-stabilization.md`) — which defines hard rules like "Runway <4 months → Crisis."
2. **Director Directives** — the Director may add custom KPIs via CSV upload.
3. **Agent Self-Registration** — when a new agent skill is added, it may register new KPIs into the registry.

### 2.2 Registration
To add a new KPI, an agent submits a Work Order to Mira containing:
```json
{
  "action": "KPI_REGISTER",
  "kpi": {
    "id": "KPI-DANI-002",
    "name": "Newsletter Open Rate",
    "owner": "DANI_CMO",
    "type": "MARKETING",
    "unit": "percentage",
    "target": 30,
    "alert_threshold": 20,
    "crisis_threshold": 10,
    "data_source": "/governance/data/raw/newsletter_metrics.csv",
    "evaluation_frequency": "WEEKLY"
  }
}
```
Eva must `AUDIT_PASS` the registration (ensuring the KPI doesn't duplicate an existing one and the thresholds are reasonable) before it is appended to `registry.json`.

### 2.3 Evaluation
Each agent evaluates their owned KPIs by invoking their domain-specific TypeScript service against data from their `data_source`.

**Evaluation Protocol** (every agent must follow):
1. Read `registry.json` to get their owned KPIs.
2. Load the `data_source` file using the **Document Management Protocol**.
3. Invoke their domain service to calculate the current value.
4. Compare against `target`, `alert_threshold`, and `crisis_threshold`.
5. Update `registry.json` with `last_evaluated`, `last_value`, and `status`.
6. If `status` changed to `WARNING` or `CRITICAL`, emit the appropriate event.

### 2.4 Reporting
KPI values flow into Mira's **Daily Brief** at 8AM. Mira reads `registry.json` and generates:
```
📊 FUNCTION STATUS SUMMARY
• NORA (CFO): Runway 14mo ✅ HEALTHY
• JULIA (CTO): Uptime 99.7% ✅ HEALTHY
• DANI (CMO): Open Rate 28% ⚠ WARNING
• ELINA (COO): Velocity +5% ✅ HEALTHY
```

---

## 3. Per-Agent KPI Assets

Each agent uses specific skills and rules to handle their KPI evaluation, invoking TypeScript services in the Electron main process.

### Nora (CFO) — Financial KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Runway Months | `FiscalForecastService.ts` | `financial-forecasting.md` | `burn-threshold.md` |
| Burn Rate | `IngestVaultService.ts` | `financial-forecasting.md` | `local-first-finance.md` |
| Budget Utilization | `BudgetAuditService.ts` | `budget-allocation.md` | `burn-threshold.md` |

**Evaluation Logic**: `IngestVaultService.ts` parses the ledger CSV → `FiscalForecastService.ts` calculates the rolling burn average → compares against `burn-threshold.md` thresholds → updates `registry.json`.

### Julia (CTO) — Technical KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Platform Uptime | `SystemHealthService.ts` | `code-forensics.md` | `test-first-mandate.md` |
| API Latency P95 | `SystemHealthService.ts` | `code-forensics.md` | `no-bloat-coding.md` |
| Test Pass Rate | `SystemHealthService.ts` | `clean-architecture-enforcement.md` | `test-first-mandate.md` |
| Dependency Freshness | `ArchitectureAuditService.ts` | `clean-architecture-enforcement.md` | `no-bloat-coding.md` |

**Evaluation Logic**: `SystemHealthService.ts` runs test suite and health metrics → `ArchitectureAuditService.ts` checks dependency versions → compares against targets → updates `registry.json`.

### Elina (COO) — Operational KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Velocity % | `WorkflowMonitorService.ts` | `process-optimization.md` | `operations-stabilization.md` |
| Backlog Size | `WorkflowMonitorService.ts` | `process-optimization.md` | `operations-stabilization.md` |
| Blocker Count | `WorkflowMonitorService.ts` | `bottleneck-resolution.md` | `operations-stabilization.md` |
| Queue Throughput | `WorkflowMonitorService.ts` | `process-optimization.md` | `operations-stabilization.md` |

**Evaluation Logic**: `WorkflowMonitorService.ts` continuously tracks queue metrics → summarizes → compares against thresholds → updates `registry.json`.

### Dani (CMO) — Marketing KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Channel ROI | `AudienceAnalyzerService.ts` | `market-sentiment.md` | `brand-tone-enforcement.md` |
| Reader Acquisition Cost | `AudienceAnalyzerService.ts` | `market-sentiment.md` | `brand-tone-enforcement.md` |
| Newsletter Open Rate | `AudienceAnalyzerService.ts` | `market-sentiment.md` | `brand-tone-enforcement.md` |

**Evaluation Logic**: `AudienceAnalyzerService.ts` parses CSV analytics exports → calculates engagement metrics → compares against targets → updates `registry.json`.

### Sofia (Design) — Design KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Token Violation Rate | `VisualAuditService.ts` | `ui-ux-analysis.md` | `perceptual-premium-enforcement.md` |
| Design Turnaround | `VisualAuditService.ts` | `ui-ux-analysis.md` | `perceptual-premium-enforcement.md` |
| Asset Reuse Ratio | `ComponentScaffolderService.ts` | `theme-orchestration.md` | `perceptual-premium-enforcement.md` |

**Evaluation Logic**: `VisualAuditService.ts` counts violations per commit → logs turnaround times → calculates reuse ratio from component library → updates `registry.json`.

### Eva (Compliance) — Governance KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Compliance Pass Rate | `ComplianceScanService.ts` | `security-audit.md` | `dcm-1-strict.md` |
| Audit Log Completeness | `AuditLogService.ts` | `governance-enforcement.md` | `git-lock-safety.md` |
| Filing Deadline Adherence | `ComplianceScanService.ts` | `governance-enforcement.md` | `dcm-1-strict.md` |

### Arya (CEO) — Strategic KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Strategic Alignment Score | `StrategyEvaluatorService.ts` | `strategic-alignment.md` | `antigravity-principles.md` |
| Cross-Module Cohesion | `StrategyEvaluatorService.ts` | `cross-module-cohesion.md` | `approval-matrix.md` |

### Lina (HR) — Human KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Director Session Complexity | `InteractionAnalyzerService.ts` | `context-management.md` | `notification-fatigue.md` |
| Context Level Accuracy | `InteractionAnalyzerService.ts` | `context-management.md` | `tone-alignment.md` |

### Maya (Funding) — Growth KPIs
| KPI | Service | Skill | Rule |
|:----|:--------|:------|:-----|
| Opportunities Identified | `PitchGeneratorService.ts` | `growth-modeling.md` | `brand-equity.md` |

---

## 4. KPI Alert Flow

```
Agent evaluates KPI
       │
       ├── HEALTHY → Update registry.json silently
       │
       ├── WARNING → Update registry + emit KPI_ALERT to Mira
       │              Mira includes in Daily Brief with ⚠ icon
       │
       └── CRITICAL → Update registry + emit CRISIS event
                       Nora/Eva/Elina evaluate if Crisis Protocol activation is warranted
                       If yes → CRISIS_ACTIVATED broadcast
```

---

## 5. Friday Weekly KPI Review

Every Friday, Mira compiles a comprehensive KPI review from `registry.json`:
```
📋 WEEKLY KPI REVIEW

FINANCIAL:
• Runway: 14mo (target: 12mo) ✅
• Burn Rate: $12k/mo (trend: -3%) ✅

TECHNICAL:
• Uptime: 99.7% (target: 99.5%) ✅
• Latency P95: 180ms (target: 200ms) ✅

OPERATIONS:
• Velocity: +5% ✅
• Blockers: 0 ✅

MARKETING:
• Open Rate: 28% (target: 30%) ⚠ WARNING
• Conversion: 4.2% ✅

COMPLIANCE:
• Pass Rate: 99% ✅
• Filing: 0 overdue ✅

DESIGN:
• Violations: 0 ✅
• Reuse Ratio: 65% ✅
```

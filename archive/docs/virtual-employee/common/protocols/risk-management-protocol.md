# Risk Management Protocol

## Overview
Every Virtual Employee in the Dhi system encounters risks specific to their domain — financial, technical, operational, legal, reputational, and strategic. This protocol defines the **common risk management baseline** that all agents must follow. Each agent then layers their own domain-specific skills, rules, and scripts on top to handle risks unique to their function.

This protocol-over-inheritance approach ensures a universal minimum standard while giving each agent the flexibility to extend risk handling for their specialization.

---

## 1. Risk Categories

| Category | Description | Primary Owner | Supporting Agents |
|:---------|:------------|:--------------|:------------------|
| **Financial** | Cash burn, runway collapse, budget overrun | Nora (CFO) | Maya, Arya |
| **Technical** | Downtime, data loss, security breach, dependency failure | Julia (CTO) | Elina, Eva |
| **Operational** | Queue deadlock, agent failure, delivery delays | Elina (COO) | Julia, Mira |
| **Compliance** | Filing overdue, PII leak, IP violation, RBAC breach | Eva (Compliance) | All agents |
| **Reputational** | Brand drift, tone violation, negative sentiment | Dani (CMO) | Sofia, Arya |
| **Strategic** | Market shift, competitive threat, product-market mismatch | Arya (CEO) | Dani, Julia, Nora |
| **Human** | Director fatigue, staffing gap, role ambiguity | Lina (HR) | Elina, Mira |

---

## 2. Common Risk Assessment (All Agents Must Follow)

### 2.1 Risk Scoring Matrix
Every risk identified by any agent must be scored using this common matrix before escalation.

| Dimension | Score 1 (Low) | Score 3 (Medium) | Score 5 (High) | Score 10 (Critical) |
|:----------|:-------------|:-----------------|:---------------|:--------------------|
| **Likelihood** | Unlikely (<10%) | Possible (10-40%) | Likely (40-70%) | Almost certain (>70%) |
| **Impact** | Minor inconvenience | Delays or budget variance | Major feature/revenue loss | Existential threat (crisis trigger) |

**Risk Priority = Likelihood × Impact**

| Score Range | Classification | Required Action |
|:------------|:---------------|:----------------|
| 1–5 | `LOW` | Log only. Include in Friday Weekly Review. |
| 6–15 | `MEDIUM` | Include in Mira's Daily Brief with ⚠ flag. |
| 16–30 | `HIGH` | Immediate Director notification. |
| 31–50+ | `CRITICAL` | Crisis Protocol trigger candidate. Evaluate with Nora/Eva/Elina. |

### 2.2 Mandatory Risk Payload
When any agent identifies a risk, they must emit a structured risk payload to the queue:
```json
{
  "protocol": "RISK_ASSESSMENT",
  "agent": "JULIA_CTO",
  "risk_id": "RISK-2026-0228-001",
  "category": "TECHNICAL",
  "description": "Primary API dependency has announced EOL in 60 days",
  "likelihood": 5,
  "impact": 5,
  "priority_score": 25,
  "classification": "HIGH",
  "mitigation_options": [
    "Migrate to alternative API within 30 days",
    "Build internal replacement in 45 days",
    "Negotiate extended support with vendor"
  ],
  "recommended_action": "Migrate to alternative API",
  "financial_impact": "Migration cost: ~$2000, No-action cost: Service outage",
  "compliance_impact": "None if migrated before EOL"
}
```

### 2.3 Minimum Risk Obligations Per Agent
Every agent, regardless of domain, must:
1. **Detect**: Scan their outputs and inputs for risks during Stage 2 (Plan) of the Stage Execution Protocol.
2. **Score**: Apply the Risk Scoring Matrix (Likelihood × Impact).
3. **Report**: Emit the risk payload to the queue if score ≥ 6.
4. **Mitigate**: Include at least 2 mitigation options in every risk payload.
5. **Track**: Risks are logged in `/governance/data/processed/risks/risk_register.json`.

---

## 3. Risk Register

All risks are tracked in a central register:
```
/governance/data/processed/risks/risk_register.json
```

```json
{
  "risks": [
    {
      "risk_id": "RISK-2026-0228-001",
      "raised_by": "JULIA_CTO",
      "raised_at": "2026-02-28T16:30:00Z",
      "category": "TECHNICAL",
      "priority_score": 25,
      "classification": "HIGH",
      "status": "OPEN",
      "mitigated_by": null,
      "resolved_at": null
    }
  ]
}
```

**Risk Status Values**: `OPEN` → `MITIGATING` → `RESOLVED` | `ACCEPTED` (Director chose to accept the risk).

---

## 4. Per-Agent Risk Layer (Domain-Specific Extensions)

Each agent applies the common risk baseline **plus** their own domain skills, rules, and TypeScript services for deeper risk analysis.

### Nora (CFO) — Financial Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Runway collapse detection | Skill: `financial-forecasting.md` | Scores runway risks using forecast models, not just manual assessment. Adds scenario analysis ("What if burn +10%?"). |
| Budget overrun prevention | Service: `BudgetAuditService.ts` | Automated scan of spend vs. allocation. Flags overrun before it becomes a scored risk. |
| Crisis threshold enforcement | Rule: `burn-threshold.md` | Automatically escalates Runway <4mo to `CRITICAL` (score ≥ 50) bypassing manual scoring. |

### Julia (CTO) — Technical Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Dependency vulnerability scan | Service: `ArchitectureAuditService.ts` | Scans dependency tree for known CVEs. Auto-scores as `HIGH` if CVE is critical. |
| Downtime risk prediction | Service: `SystemHealthService.ts` | Tracks uptime trends. If degradation >5% week-over-week, auto-generates risk payload. |
| Architecture drift detection | Skill: `clean-architecture-enforcement.md` | Identifies structural violations that increase technical debt risk over time. |

### Elina (COO) — Operational Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Queue deadlock detection | Skill: `bottleneck-resolution.md` | Identifies circular dependencies before they become deadlocks. Pre-emptive risk scoring. |
| Delivery delay prediction | Service: `WorkflowMonitorService.ts` | Tracks velocity trends. If velocity drops >20%, auto-generates `HIGH` risk. |
| Agent failure patterns | Rule: `operations-stabilization.md` | If an agent hits `DEGRADED` state 3+ times in 24h, escalates as operational risk. |

### Eva (Compliance) — Compliance Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Filing deadline enforcement | Skill: `governance-enforcement.md` | Maintains a regulatory calendar. Auto-raises `HIGH` risk 30 days before deadline. |
| PII exposure scanning | Service: `ComplianceScanService.ts` | Every processed output is scanned for PII. Any detection = `CRITICAL` immediate. |
| RBAC violation detection | Rule: `dcm-1-strict.md` | If any agent attempts to exceed its authority, auto-raises `HIGH` risk. |

### Dani (CMO) — Reputational Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Brand tone drift | Rule: `brand-tone-enforcement.md` | Scans all external copy for Antigravity tone violations. Auto-scores brand risk. |
| Negative sentiment spike | Service: `AudienceAnalyzerService.ts` | If sentiment drops to `NEGATIVE` for >48 hours, auto-raises `MEDIUM` risk. |

### Sofia (Design) — Visual Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Accessibility failure | Service: `VisualAuditService.ts` | WCAG AA violations auto-scored as `HIGH` reputational risk. |
| Theme structural flaw | Skill: `theme-orchestration.md` | If a token issue affects >3 components globally, auto-escalates to Director. |

### Arya (CEO) — Strategic Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Market shift detection | Skill: `strategic-alignment.md` | Evaluates competitive intelligence. Revenue shift >15% auto-scores `HIGH`. |
| Cross-module misalignment | Skill: `cross-module-cohesion.md` | Detects "promise drift" between marketing and engineering as strategic risk. |

### Lina (HR) — Human Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Director burnout | Service: `InteractionAnalyzerService.ts` | If fatigue score hits 8+ for 3 consecutive days, raises `MEDIUM` human risk. |
| Staffing gap | Skill: `context-management.md` | SLA drops indicating understaffing are scored as operational + human risk. |

### Maya (Funding) — Funding Risk
| Obligation | Asset | How It Extends the Common Protocol |
|:-----------|:------|:-----------------------------------|
| Funding gap | Skill: `growth-modeling.md` | If no viable opportunity identified for 30 days, raises `MEDIUM` risk. |
| Investor misalignment | Rule: `brand-equity.md` | If a proposed investor doesn't meet values criteria, flags as reputational risk. |

---

## 5. Risk Escalation Chain

```
Agent detects risk → Scores using Risk Matrix → Emits risk payload
       │
       ├── LOW (1-5) ──→ Log to risk_register.json silently
       │
       ├── MEDIUM (6-15) ──→ Log + Include in Mira's Daily Brief ⚠
       │
       ├── HIGH (16-30) ──→ Log + Immediate Director notification via Notification Centre
       │
       └── CRITICAL (31-50+) ──→ Log + Evaluate Crisis Protocol activation
                                   │
                                   ├── Nora: Is runway threatened?
                                   ├── Eva: Is compliance at risk?
                                   └── Elina: Is operations degraded?
                                   │
                                   ▼
                              2 of 3 confirm → CRISIS_ACTIVATED broadcast
```

---

## 6. Friday Weekly Risk Review

Every Friday, Mira compiles from `risk_register.json`:
```
⚠ WEEKLY RISK REGISTER

OPEN RISKS: 3
• [HIGH] RISK-001: API dependency EOL in 45 days (Julia)
• [MEDIUM] RISK-002: Newsletter open rate declining (Dani)
• [LOW] RISK-003: Design token minor inconsistency (Sofia)

RESOLVED THIS WEEK: 2
• RISK-004: Budget overrun Q1 (Nora) → RESOLVED
• RISK-005: Queue deadlock (Elina) → RESOLVED

RISK TREND: ↓ Decreasing (5 last week → 3 this week)
```

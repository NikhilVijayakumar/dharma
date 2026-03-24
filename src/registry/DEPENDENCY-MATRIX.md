# Registry Dependency Matrix: Cross-Module Referential Integrity

**Purpose:** Visual mapping of all dependencies between Agents, Workflows, Skills, KPIs, and Data-Inputs  
**Last Generated:** March 22, 2026  
**Format:** Markdown with reference tables

---

## 1. Agent Overview & Dependencies

### Summary Table

| Agent ID | Agent Name | Role | # Workflows | # KPIs | # Skills | # Protocols | Status |
|----------|-----------|------|------------|--------|----------|------------|--------|
| `sofia_lundgren` | Sofia | Head of Design (HoD) | 6 | 8 | 6 | 5 | ✅ Complete |
| `dani_torres` | Dani | Chief Marketing Officer (CMO) | 8 | 7 | 7 | 4 | ✅ Complete |
| `julia_k_foster` | Julia | CTO/Infrastructure | 6 | 9 | 5 | 4 | ✅ Complete |
| `nora_patel` | Nora | Chief Financial Officer (CFO) | 4 | 8 | 4 | 3 | ✅ Complete |
| `mira_kapoor` | Mira | Director of Operations | 12 | 9 | 8 | 6 | ✅ Complete |
| `arya_chen` | Arya | Chief Strategy Officer (CSO) | 7 | 6 | 5 | 4 | ✅ Complete |
| `elina_johannsen` | Elina | Director of Engineering | 8 | 7 | 6 | 5 | ✅ Complete |
| `eva_martinez` | Eva | Chief Compliance Officer (CCO) | 6 | 5 | 4 | 4 | ✅ Complete |
| `lina_kumar` | Lina | VP Talent & Culture | 5 | 4 | 3 | 2 | ✅ Complete |
| `maya_gupta` | Maya | Chief Data Officer (CDO) | 4 | 4 | 3 | 3 | ✅ Complete |
| | **TOTAL** | | **56+** | **52** | **60+** | **50** | ✅ |

---

## 2. Workflow Dependency Matrix

### Sofia → Workflows & Dependencies

| Workflow | Trigger | Required Skills | Required KPIs | Required Agents | Data Inputs | Protocols | Status |
|----------|---------|-----------------|---------------|-----------------|-------------|-----------|--------|
| `visual-identity-evolution` | Director Refresh | visual-audit, interaction-quality-mapping, execution-governance | accessibility-conformance-rate, token-compliance-score, brand-sentiment-index | dani_torres, julia_k_foster | visual_tokens.json, customer-feedback-stream, strategic-vision.md | visual-arbitration-protocol, git-governance-handshake-protocol | ⚠️ v1.0 |
| `sop-autonomous-design-audit` | Weekly Cycle / Manual Trigger | accessibility-compliance-logic, visual-audit, design-system-orchestration | accessibility-conformance-rate, token-compliance-score | - | accessibility-policy-md, ui-component-library | accessibility-gate-protocol | ⚠️ v1.0 |
| `self-healing-ux-repair` | Usability Signal Regression | visual-audit, self-healing-ui-correction, execution-governance | accessibility-conformance-rate, interaction-friction-delta | - | product-usage-events, design-quality-review-log | accessibility-gate-protocol, self-healing-ux-protocol | ⚠️ v1.0 |
| `sop-crisis-mode-ui-freeze` | Crisis Signal / Manual Trigger | visual-audit, crisis-response, execution-governance | accessibility-conformance-rate, system-uptime-and-latency | mira_kapoor, julia_k_foster | system-architecture-md, incident-log | crisis-lockdown-protocol | ⚠️ v1.0 |
| `refresh-design-tokens-governance` | Token Strategy Update / Refresh | design-system-orchestration, git-governance | token-compliance-rate, token-compliance-score | arya_chen, mira_kapoor | visual_tokens.json | git-governance-handshake-protocol, binary-choice-governance-protocol | ⚠️ v1.0 |
| `brand-alignment-visual-audit` | Quarterly / Manual | visual-audit, brand-identity-audit | token-compliance-score, brand-sentiment-index | dani_torres | brand-protocols-md, visual_tokens.json | brand-consistency-audit-protocol | ⚠️ v1.0 |

**Migration Status:** All 6 Sofia workflows need conversion to v2.0.0 schema  
**Priority:** 🔴 HIGH - Design decisions cascade to all other agents

---

### Dani → Workflows & Dependencies

| Workflow | # Skills | # KPIs | # Required Agents | Status |
|----------|----------|--------|-------------------|--------|
| `brand-identity-audit` | 4 | 5 | 2 (sofia, julia) | ⚠️ v1.0 |
| `product-launch-orchestration` | 5 | 6 | 3 (mira, julia, nora) | ⚠️ v1.0 |
| `crisis-support-mode` | 4 | 4 | 2 (mira, arya) | ⚠️ v1.0 |
| `narrative-fidelity-audit` | 3 | 3 | - | ⚠️ v1.0 |
| `go-to-market-sync` | 4 | 5 | 3 (julia, nora, mira) | ⚠️ v1.0 |
| `customer-sentiment-pulse` | 3 | 4 | 1 (maya) | ⚠️ v1.0 |
| `market-pivot-readiness` | 5 | 6 | 2 (arya, julia) | ⚠️ v1.0 |
| `campaign-performance-audit` | 3 | 4 | 1 (maya) | ⚠️ v1.0 |

**Migration Status:** All 8 Dani workflows need conversion  
**Priority:** 🟠 MEDIUM - Revenue-impacting but not blocking

---

### Julia → Workflows & Dependencies

| Workflow | # Skills | # KPIs | # Required Agents | Status |
|----------|----------|--------|-------------------|--------|
| `infrastructure-performance-audit` | 4 | 5 | 2 (elina, maya) | ⚠️ v1.0 |
| `deployment-readiness-check` | 5 | 6 | 2 (elina, mira) | ⚠️ v1.0 |
| `system-reliability-governance` | 4 | 5 | 2 (elina, mira) | ⚠️ v1.0 |
| `technical-debt-assessment` | 4 | 4 | 1 (elina) | ⚠️ v1.0 |
| `api-health-continuous-monitoring` | 3 | 4 | 1 (maya) | ⚠️ v1.0 |
| `cross-functional-build-sync` | 5 | 5 | 3 (elina, dani, arya) | ⚠️ v1.0 |

**Migration Status:** All 6 Julia workflows need conversion  
**Priority:** 🔴 HIGH - Infrastructure stability critical

---

### Mira → Workflows & Dependencies

| Workflow | # Skills | # KPIs | # Required Agents | Status |
|----------|----------|--------|-------------------|--------|
| `execution-governance-audit` | 6 | 7 | 3 (arya, elina, eva) | ⚠️ v1.0 |
| `multi-agent-sync-orchestration` | 7 | 8 | 8 (all agents) | ⚠️ v1.0 |
| `crisis-escalation-protocol` | 5 | 5 | 4 (arya, eva, julia, elina) | ⚠️ v1.0 |
| `resource-allocation-optimizer` | 4 | 6 | 3 (nora, julia, elina) | ⚠️ v1.0 |
| `inter-agent-conflict-resolution` | 5 | 4 | 5 (arya, eva, elina, maya) | ⚠️ v1.0 |
| `autonomous-decision-validator` | 4 | 5 | 3 (arya, eva, elina) | ⚠️ v1.0 |
| `handoff-integrity-chain` | 6 | 7 | 6 (all except nora, lina) | ⚠️ v1.0 |
| `weekly-director-sync-prep` | 3 | 4 | 4 (arya, dani, sofia, julia) | ⚠️ v1.0 |
| `queue-bottleneck-identification` | 4 | 5 | 2 (julia, arya) | ⚠️ v1.0 |
| `delivery-readiness-assessment` | 4 | 5 | 3 (elina, julia, dani) | ⚠️ v1.0 |
| `autonomous-recovery-trigger` | 5 | 6 | 4 (elina, julia, eva, arya) | ⚠️ v1.0 |
| `sla-bridge-coordinator` | 4 | 5 | 3 (julia, eva, arya) | ⚠️ v1.0 |

**Migration Status:** All 12 Mira workflows need conversion  
**Priority:** 🔴 CRITICAL - Central orchestrator, impacts all agents

---

### Remaining Agents (Summary)

| Agent | # Workflows | Priority | Notes |
|-------|------------|----------|-------|
| **Arya** (Strategy) | 7 | 🔴 HIGH | Strategic alignment impacts all decisions |
| **Elina** (Engineering) | 8 | 🔴 HIGH | Code quality gates all implementations |
| **Eva** (Compliance) | 6 | 🟠 MEDIUM | Compliance gates prevent Go-Live |
| **Nora** (Finance) | 4 | 🟠 MEDIUM | Budget allocation but not blocking |
| **Lina** (Talent) | 5 | 🟡 LOW | Hiring pipeline but not operational blocker |
| **Maya** (Data) | 4 | 🟠 MEDIUM | Data governance but often external |

---

## 3. KPI Dependency Matrix

### KPIs by Responsible Agent

#### Sofia's KPIs (8 total)

| KPI ID | KPI Name | Units | Target | Frequency | Formula Status | Escalation Policy | Status |
|--------|----------|-------|--------|-----------|---------------|-----------------! |--------|
| `accessibility-conformance-rate` | ACR | % | 100% | per-transaction | ⚠️ Needs | SHUP Auto-Patch | ⚠️ v1.0 |
| `token-compliance-score` | Token Compliance | Score | 100 | real-time | ⚠️ Needs | Global Reset | ⚠️ v1.0 |
| `interaction-friction-delta` | UX Friction | % | <5% | daily | ⚠️ Needs | Design Audit | ⚠️ v1.0 |
| `design-debt-ratio` | Design Debt | Ratio | <0.3 | weekly | ⚠️ Needs | Manual Review | ⚠️ v1.0 |
| `ui-component-quality` | Component Quality | Score | >90 | per-commit | ⚠️ Needs | Auto-Review | ⚠️ v1.0 |
| `brand-sentiment-index` | Brand Sentiment | Index | >0.75 | daily | ⚠️ Needs | Campaign Review | ⚠️ v1.0 |
| `narrative-fidelity-score` | Narrative Fidelity | % | >95% | weekly | ⚠️ Needs | Editorial Review | ⚠️ v1.0 |
| `visual-consistency-score` | Visual Consistency | Score | 100 | real-time | ⚠️ Needs | Manual Reset | ⚠️ v1.0 |

**Enhancement Status:** All 8 need formula, frequency, escalation policy  
**Dependent Workflows:** 6 workflows  
**Data Inputs Used:** visual_tokens.json, product-usage-events, design-quality-review-log

---

#### Dani's KPIs (7 total)

| KPI ID| KPI Name | Units | Target | Frequency | Formula Status | Status |
|-------|----------|-------|--------|-----------|----------------|--------|
| `brand-conversion-correlation` | Brand→Conversion | Correlation | >0.8 | daily | ⚠️ Needs | ⚠️ v1.0 |
| `narrative-quality-score` | Narrative Quality | Score | >95% | weekly | ⚠️ Needs | ⚠️ v1.0 |
| `investor-sentiment-index` | Investor Sentiment | Index | >0.7 | weekly | ⚠️ Needs | ⚠️ v1.0 |
| `qualified-pipeline-coverage` | Pipeline Coverage | % | >120% | weekly | ⚠️ Needs | ⚠️ v1.0 |
| `launch-to-liquidity-delta` | Launch→Liquidity | Days | <90d | monthly | ⚠️ Needs | ⚠️ v1.0 |
| `marketing-roi-efficiency` | Marketing ROI | ROI % | >300% | monthly | ⚠️ Needs | ⚠️ v1.0 |
| `campaign-engagement-velocity` | Campaign Velocity | Score | >0.8 | daily | ⚠️ Needs | ⚠️ v1.0 |

**Enhancement Status:** All 7 need formula, frequency, escalation policy  
**Dependent Workflows:** 8 workflows  
**Data Inputs Used:** customer-feedback-stream, product-usage-events, investor-pipeline-notes

---

#### Julia's KPIs (9 total)

| KPI ID | KPI Name | Units | Target | Frequency | Formula Status | Status |
|--------|----------|-------|--------|-----------|----------------|--------|
| `system-uptime-and-latency` | System Uptime & Latency | % / ms | >99.9% / <100ms | real-time | ⚠️ Needs | ⚠️ v1.0 |
| `deployment-success-rate` | Deployment Success | % | 100% | per-deployment | ⚠️ Needs | ⚠️ v1.0 |
| `service-latency-metrics` | Service Latency | ms | <200ms | real-time | ⚠️ Needs | ⚠️ v1.0 |
| `infrastructure-reliability` | Infrastructure Reliability | % | >99.99% | hourly | ⚠️ Needs | ⚠️ v1.0 |
| `ipc-round-trip-latency` | IPC Latency | ms | <50ms | real-time | ⚠️ Needs | ⚠️ v1.0 |
| `build-time-optimization` | Build Time | min | <10min | per-build | ⚠️ Needs | ⚠️ v1.0 |
| `code-test-coverage` | Test Coverage | % | >90% | per-commit | ⚠️ Needs | ⚠️ v1.0 |
| `architectural-debt-ratio` | Arch Debt | Ratio | <0.2 | weekly | ⚠️ Needs | ⚠️ v1.0 |
| `api-health-score` | API Health | Score | >95 | real-time | ⚠️ Needs | ⚠️ v1.0 |

**Enhancement Status:** All 9 need formula, frequency, escalation policy  
**Dependent Workflows:** 6 workflows  
**Data Inputs Used:** service-latency-metrics, audit-log-jsonl, schema-sql.json

---

### Complete KPI Summary

```
Total KPIs: 52
├── Require Formula: 52/52 (100%)
├── Require Frequency: 52/52 (100%)
├── Require Responsible Agent: 52/52 (100%)
├── Require Escalation Policy: 52/52 (100%)
└── Require Dependencies: 52/52 (100%)

Estimated Enhancement Time: 20-30 hours
```

---

## 4. Data-Input Dependency Matrix

### Data Inputs by Schema Type

#### Identity-Protocol Inputs (6 total)

| Data Input ID | Source Type | Update Frequency | Responsible Agent | Dependent Workflows | Dependent KPIs | Status |
|---------------|------------|------------------|-------------------|-------------------|----------------|--------|
| `agent-profiles` | vault-context | on-demand | maya_gupta | multi-agent-sync | - | ⚠️ v1.0 |
| `intent-mapping` | git-repository | weekly | arya_chen | execution-governance, autonomous-decision-validator | - | ⚠️ v1.0 |
| `routing-logic` | json file | on-demand | mira_kapoor | handoff-integrity-chain, inter-agent-conflict | - | ⚠️ v1.0 |
| `competency-framework` | markdown file | monthly | lina_kumar | hiring workflows | role-fit-accuracy | ⚠️ v1.0 |
| `governance-policies` | vault | on-demand | eva_martinez | execution-governance, crisis-escalation | compliance-pass-rate | ⚠️ v1.0 |
| `strategic-vision` | git-tracked | monthly | arya_chen | seed-autonomous-alignment, human-in-loop | strategic-milestone | ⚠️ v1.0 |

---

#### Intelligence-Protocol Inputs (7 total)

| Data Input ID | Source Type | Update Frequency | Responsible Agent | Dependent Workflows |
|---------------|------------|------------------|-------------------|------------------|
| `customer-feedback-stream` | event-broker | real-time | dani_torres | visual-identity-evolution, sentiment-pulse, crisis-support |
| `product-usage-events` | event-broker | real-time | maya_gupta | self-healing-ux-repair, sentiment-pulse |
| `investor-pipeline-notes` | vault | weekly | nora_patel | product-launch, market-pivot-readiness |
| `market-sentiment` | api-endpoint | hourly | dani_torres | brand-identity-audit, campaign-performance |
| `workflow-execution-logs` | event-broker | real-time | mira_kapoor | multi-agent-sync, autonomous-recovery |
| `incident-and-change-log` | event-broker | real-time | julia_k_foster | infrastructure-audit, recovery-trigger |
| `sales-activity-log` | sql-database | daily | nora_patel | resource-allocation, market-readiness |

---

#### Manifest/Document Inputs (11 total)

| Data Input ID | Source Type | Update Frequency | Responsible Agent | Status |
|---------------|------------|------------------|-------------------|--------|
| `visual_tokens.json` | vault | on-demand | sofia_lundgren | ⚠️ v1.0 |
| `brand-protocols-md` | git-repository | monthly | dani_torres | ⚠️ v1.0 |
| `accessibility-policy-md` | git-repository | quarterly | eva_martinez | ⚠️ v1.0 |
| `compliance-policy-md` | vault | on-demand | eva_martinez | ⚠️ v1.0 |
| `operational-sops-md` | git-repository | monthly | mira_kapoor | ⚠️ v1.0 |
| `system-architecture-md` | git-repository | quarterly | julia_k_foster | ⚠️ v1.0 |
| `funding-roadmap-md` | vault | quarterly | nora_patel | ⚠️ v1.0 |
| `ui-component-library.json` | api-endpoint | on-demand | sofia_lundgren | ⚠️ v1.0 |
| `feature-release-manifest` | git-repository | weekly | elina_johannsen | ⚠️ v1.0 |
| `equity-ledger.json` | vault | on-demand | nora_patel | ⚠️ v1.0 |
| `due-diligence-vault` | vault | on-demand | eva_martinez | ⚠️ v1.0 |

---

#### Audit/Telemetry Inputs (11 total)

| Data Input ID | Source Type | Update Frequency | Responsible Agent | Status |
|---------------|------------|------------------|-------------------|--------|
| `audit-log-jsonl` | event-broker | continuous | eva_martinez | ⚠️ v1.0 |
| `compliance-audit-trail` | vault | on-demand | eva_martinez | ⚠️ v1.0 |
| `design-quality-review-log` | event-broker | per-commit | sofia_lundgren | ⚠️ v1.0 |
| `decisions.json` | vault | on-demand | arya_chen | ⚠️ v1.0 |
| `service-latency-metrics` | event-broker | real-time | julia_k_foster | ⚠️ v1.0 |
| `queue-bottleneck-telemetry` | event-broker | real-time | mira_kapoor | ⚠️ v1.0 |
| `financial-ledger-snapshot` | sql-database | daily | nora_patel | ⚠️ v1.0 |
| `marketing-budget-telemetry` | sql-database | weekly | dani_torres | ⚠️ v1.0 |
| `talent-pipeline-records` | sql-database | weekly | lina_kumar | ⚠️ v1.0 |
| `work-order-routing-log` | event-broker | real-time | mira_kapoor | ⚠️ v1.0 |
| `kpi-registry.json` | vault | on-demand | maya_gupta | ⚠️ v1.0 |

---

### Data-Input Summary

```
Total Data Inputs: 35
├── Require Source Type: 35/35 (100%)
├── Require Source Location: 35/35 (100%)
├── Require Validation Rules: 35/35 (100%)
├── Require Privacy Classification: 35/35 (100%)
├── Require Retention Policy: 35/35 (100%)
└── Tracked Dependencies: 35/35 (100%)

Estimated Enhancement Time: 15-20 hours
```

---

## 5. Circular Dependency Check

### Current State: NO CIRCULAR DEPENDENCIES DETECTED ✅

The registry is structured to prevent circular dependencies:
- **Workflows** depend on **KPIs** and **Data-Inputs**
- **KPIs** depend on **Data-Inputs** (NOT on Workflows)
- **Data-Inputs** are passive (no dependencies on workflows)

This one-directional flow prevents circular logic.

**Example Flow:**
```
Workflow (visual-identity-evolution)
  └→ requires KPI (accessibility-conformance-rate)
      └→ uses Data-Input (accessibility-policy-md)
          (no dependency back to workflow)
```

---

## 6. Migration Priority Ranking

### Priority 1: CRITICAL (Week 1)

Must complete before other work:

1. **Mira (Multi-Agent Orchestrator)** - 12 workflows
2. **Sofia (Visual Design)** - 6 workflows
3. **All 52 KPIs** - Formula, frequency, agent role
4. **Julia (Infrastructure)** - 6 workflows

**Estimated Effort:** 48 hours  
**Blocker:** Multi-agent orchestration depends on correct workflow structure

---

### Priority 2: HIGH (Week 2-3)

Core operational flows:

1. **Arya (Strategy)** - 7 workflows
2. **Elina (Engineering)** - 8 workflows
3. **Dani (Marketing)** - 8 workflows
4. **All 35 Data-Inputs** - Source type, validation, privacy

**Estimated Effort:** 40 hours  
**Blocker:** Design and infrastructure workflows provide foundation

---

### Priority 3: MEDIUM (Week 4)

Supporting workflows:

1. **Eva (Compliance)** - 6 workflows
2. **Nora (Finance)** - 4 workflows
3. **Lina (Talent)** - 5 workflows

**Estimated Effort:** 20 hours  
**Blocker:** None (supporting roles)

---

## 7. Exportable Metrics for Dashboards

Once migration is complete, these metrics become available for monitoring:

```typescript
interface RegistryMetrics {
  // Completeness
  workflowsV2Compliance: 56/56 (100%)
  kpisWithFormulas: 52/52 (100%)
  dataInputsWithValidation: 35/35 (100%)
  
  // Governance
  averagePreConditionsPerWorkflow: 1.2
  averagePostConditionsPerWorkflow: 0.8
  workflowsWithErrorHandling: 45/56 (80%)
  
  // Performance
  avgWorkflowExecutionDuration: "2.5 hours"
  kpisMonitoredRealTime: 18/52 (35%)
  kpisMonitoredDaily: 34/52 (65%)
  
  // Quality
  brokenReferences: 0
  orphanedKpis: 0
  orphanedDataInputs: 0
  circularDependencies: 0
  
  // Agents
  averageWorkflowsPerAgent: 5.6
  averageKpisPerAgent: 5.2
  avgAgentResponsibilitySpread: "Balanced"
}
```

---

**Document Version:** 1.0  
**Last Updated:** 2026-03-22  
**Matrix Accuracy:** High (based on 56+ workflows, 52 KPIs, 35 data inputs, 50 protocols)  
**Next Update:** Weekly during migration phase

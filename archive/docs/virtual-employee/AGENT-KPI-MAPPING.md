# Agent KPI Mapping & Input Variables

**Date**: 2026-03-21  
**Purpose**: Define success metrics (KPIs) and required input data for each Virtual Employee (Agent)  
**Audience**: Platform Leadership, Agent Developers, Model Gateway

---

## 1. Overview

Each Virtual Employee operates within defined KPIs that measure both **operational efficiency** and **decision quality**. This document maps:
- **KPI Name**: Metric identifier
- **Measurement**: How the KPI is computed
- **Success Target**: The threshold for "acceptable" performance
- **Input Variables**: Data required from other systems or agents
- **Owner**: Which agent requires this input
- **Dependencies**: Upstream agents or services needed

---

## 2. Agent KPI Matrix

### Mira (Secretary & Command Router)
**Role**: Routes Director intent to correct owners, maintains coherent cross-module handoffs  
**Report Frequency**: Real-time per work order

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Routing Accuracy** | % of work orders routed to correct primary owner on first attempt | ≥95% | `workOrder.targetEmployeeId`, `workOrder.message`, `previousRoutingHistory` | Work Order Service |
| **Escalation Resolution Time** | Hours from escalation flag to resolution or director decision | ≤4h for critical, ≤24h for standard | `escalationTimestamp`, `resolutionTimestamp`, `context` | Audit Log |
| **Cross-Module Handoff Success** | % of delegations that complete without re-routing | ≥90% | `delegationChain`, `handoffContext`, `agentCapabilities` | Work Order Service |
| **Command Parser Accuracy** | % of director intent correctly parsed into actionable work order | ≥98% | `rawDirectorMessage`, `contextModules`, `previousIntents` | Model Gateway Output |
| **Response Synthesis Latency** | Minutes from last agent response to synthesized brief | ≤5min | `agentResponses`, `synthesisInputs` | Agent Execution Service |

**Required Input Variables**:
```json
{
  "workOrderService": {
    "submitDirectorRequest": {
      "message": "string",
      "moduleRoute": "string",
      "priority": "ROUTINE|URGENT|CRITICAL"
    },
    "queryRoutingHistory": {
      "targetEmployeeId": "string",
      "limit": "number"
    }
  },
  "auditLog": {
    "queryEscalations": {
      "timeRange": "[from, to]",
      "severity": "CRITICAL|HIGH|MEDIUM"
    }
  }
}
```

---

### Nora (Chief Financial Officer)
**Role**: Deterministic finance executive focused on burn discipline and audit-ready transparency  
**Report Frequency**: Daily cash position, Weekly variance analysis, Monthly strategic forecast

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Runway Projection Accuracy** | Variance between forecast and actual month-end position | ±3% of forecast | `monthlyBurnRate`, `revenue`, `expenses`, `contingencyReserve` | Ledger Sync |
| **Spend Guardrail Compliance** | % of transactions passing burn-discipline rules | ≥99% | `proposedExpense`, `spendCategory`, `runwayDays`, `policy` | Finance Service |
| **Audit Trail Completeness** | % of ledger entries with full provenance chain | 100% | `ledgerEntries`, `transactionSource`, `autorization` | Audit Log |
| **Variance Explanation Quality** | Director acceptance rate of monthly variance narratives | ≥90% acceptance | `actualBurn`, `budgetedBurn`, `contextualFactors` | Analysis Service |
| **Financial Risk Detection** | Hours from risk detection to escalation | ≤2h (critical), ≤8h (standard) | `runwayTrend`, `unexpectedExpense`, `revenueSlip` | Monitoring Service |

**Required Input Variables**:
```json
{
  "ledgerService": {
    "getRuntimeBurn": {
      "timeRange": "[startDate, endDate]",
      "detail": "daily|weekly|monthly"
    },
    "getSpendCategories": {},
    "getRevenueStreams": {}
  },
  "monitoringService": {
    "getCashPosition": {},
    "getRunwayDays": {}
  }
}
```

---

### Eva (Compliance Officer)
**Role**: Compliance authority enforcing policy, governance quality, and audit trail integrity  
**Report Frequency**: Real-time violations, Weekly compliance scorecard

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Policy Violation Detection** | % of violations flagged before execution | ≥100% of critical violations | `artifact`, `policy`, `controlPoints` | Audit Log, AI Analysis |
| **Compliance Scorecard** | % of all auditable events in full compliance | ≥98% | `auditEvents`, `policySet`, `exceptions` | Compliance Service |
| **Bypass Prevention** | # of attempted bypasses blocked | 100% block rate | `workOrder.requiresApproval`, `approval.granted` | Command Router |
| **Remediation Time** | Days from violation to resolution | ≤5 days | `violationDate`, `remediationDate` | Remediation Tracker |
| **Audit Export Readiness** | % of compliance snapshots exportable without manual curation | ≥95% | `complianceArtifacts`, `auditScope` | Compliance Service |

**Required Input Variables**:
```json
{
  "policyService": {
    "validateArtifact": {
      "artifact": "object",
      "policies": "[policyId]",
      "workOrderId": "string"
    },
    "getApplicablePolicies": {
      "workOrderType": "string",
      "context": "object"
    }
  },
  "auditService": {
    "logEvent": {
      "event": "object",
      "timestamp": "ISO8601",
      "provenance": "string"
    }
  }
}
```

---

### Julia (Chief Technology Officer)
**Role**: Technical feasibility, architecture integrity, and implementation sequencing  
**Report Frequency**: Per feature review, Weekly architecture health

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Feasibility Accuracy** | % of estimated features completed within ±2 day variance | ≥90% | `proposedFeature`, `architectureState`, `techDebt`, `teamCapacity` | Architecture Service |
| **Cycle Detection** | # of dependency cycles detected and prevented | 100% (zero cycles permitted) | `dependencyGraph`, `imports`, `exports` | Dependency Graph Analyzer |
| **Tech Debt Growth Rate** | Net change in tech debt score (lower is better) | ≤2% quarterly growth | `codeComplexity`, `testCoverage`, `buildTime`, `documentationGap` | Code Quality Service |
| **Build Stability** | % of builds passing without errors | ≥99% | `buildLogs`, `testResults`, `lintResults` | CI/CD Pipeline |
| **Architecture Alignment** | % of features adhering to clean architecture layers | ≥95% | `codeArtifacts`, `architectureLayer` | Code Review Bot |

**Required Input Variables**:
```json
{
  "architectureService": {
    "validateFeature": {
      "proposedFeature": "object",
      "architectureConstraints": "[constraint]"
    },
    "estimateEffort": {
      "featureScope": "object",
      "teamCapacity": "number"
    },
    "analyzeImpact": {
      "change": "object",
      "dependencyGraph": "object"
    }
  },
  "qualityService": {
    "getTechDebtScore": {},
    "getCyclomaticComplexity": {
      "module": "string"
    }
  }
}
```

---

### Elina (Chief Operating Officer)
**Role**: Orchestrates operations rhythm, delivery predictability, and queue health  
**Report Frequency**: Daily queue status, Weekly throughput report

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Queue SLA Compliance** | % of work orders meeting SLA targets | ≥98% | `workOrderId`, `createdAt`, `completedAt`, `slaTarget` | Queue Service |
| **Throughput Consistency** | Coefficient of variation in weekly throughput | ≤0.15 (15% variance) | `weeklyCompletedCount`, `weekNumber` | Work Order Service |
| **Blocked Path Resolution** | Hours from critical path block to resolution | ≤2h | `blockedDependency`, `blockDetectedAt`, `resolvedAt` | Dependency Tracker |
| **Capacity Utilization** | Queue occupancy ÷ capacity (optimal 0.65-0.75) | 0.65-0.75 | `queueSize`, `maxCapacity`, `processingSlotsUsed` | Queue Service |
| **Cross-Team Handoff Latency** | Average hours between on-time hand-off milestones | ≤4h per milestone | `handoffTimestamp`, `recipientAck` | Audit Log |

**Required Input Variables**:
```json
{
  "queueService": {
    "getQueueMetrics": {
      "timeRange": "[startDate, endDate]",
      "includeBlocked": true
    },
    "getThroughput": {
      "timeRange": "[startDate, endDate]",
      "periodUnit": "week"
    },
    "getCapacityStatus": {}
  },
  "dependencyTracker": {
    "getBlockedPaths": {
      "severity": "CRITICAL|HIGH"
    }
  }
}
```

---

### Nora → Maya (Funding & Resource Procurement) Handshake
**Role**: Manages capital strategy, funding opportunities, and resource procurement  
**Report Frequency**: Weekly opportunity scout, Monthly capital plan update

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Lead Generation Quality** | % of qualified leads that advance to LOI | ≥40% | `prospectProfile`, `fundingRound`, `targetRaise`, `previousPipelineData` | CRM, Investor DB |
| **Term Sheet Alignment** | % of terms matching pre-set criteria (dilution, valuation, governance) | ≥90% aligned | `termsTemplate`, `actualTerms`, `runwayConstraints` | Finance Service |
| **Capital Secured vs Target** | Actual capital raised ÷ quarterly target | ≥100% of target | `capitalRaised`, `quarterlyTarget`, `runway` | Finance Service |
| **Pipeline Health Score** | Weighted sum of lead stages and probabilities | ≥0.70 score (70% ready) | `leadStage`, `stageProbability`, `timeToDecision` | Pipeline Service |
| **Deck Effectiveness** | # of decks leading to investor meetings ÷ decks sent | ≥50% meeting rate | `decksSent`, `meetingsScheduled` | CRM |

**Required Input Variables**:
```json
{
  "financeService": {
    "getRunwayConstraints": {},
    "getCapitalRequirements": {
      "quarter": "string"
    },
    "getValidTermCriteria": {}
  },
  "investorDB": {
    "searchProspects": {
      "criteria": "object",
      "fundingStage": "string"
    }
  },
  "crm": {
    "leadHistory": {
      "prospectId": "string"
    }
  }
}
```

---

### Lina (Head of Human Resources)
**Role**: Owns talent quality, role-fit analysis, and hiring simulation integrity  
**Report Frequency**: Per hiring cycle (weekly), Monthly talent report

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Role-Fit Prediction Accuracy** | % of new hires rated ≥7/10 in role satisfaction after 90 days | ≥85% | `candidateProfile`, `roleRequirements`, `assessmentScores` | Hiring Service, 90-Day Review |
| **Bias Mitigation Score** | % of hiring decisions passing bias checks (demographics, backgrounds) | 100% PASS | `candidate`, `policyRules`, `historicalCohort` | Bias Detector |
| **Hiring Cycle Time** | Days from job posting to offer acceptance | ≤30 days | `jobPostedDate`, `offerAcceptedDate` | ATS |
| **Culture Fit Alignment** | % of hired candidates scoring ≥7/10 on culture assessment | ≥80% | `candidateCultureScore`, `teamCultureMatrix` | Hiring Service |
| **Hiring Quality Score** | Composite of role-fit, culture-fit, and retention @ 1 year | ≥8.0/10 composite | `roleFit`, `cultureFit`, `retentionStatus` | People Analytics |

**Required Input Variables**:
```json
{
  "hiringService": {
    "evaluateCandidate": {
      "candidateId": "string",
      "roleId": "string",
      "includeHistoricalComparisons": true
    },
    "runRoleFitSimulation": {
      "candidateProfile": "object",
      "roleRequirements": "object"
    }
  },
  "biasDetector": {
    "validateDecision": {
      "candidate": "object",
      "decision": "enum[HIRE|PASS]",
      "historicalCohort": "[candidate]"
    }
  }
}
```

---

### Arya (Chief Executive Officer)
**Role**: Sets strategic direction, approves major decisions, and reviews operational health  
**Report Frequency**: Quarterly strategic review, Monthly alignment check

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Strategic Alignment Score** | % of departmental initiatives scoring ≥6/10 against strategic goals | ≥85% aligned | `initiativeProfile`, `strategicGoals`, `marketContext` | Strategy Service |
| **Cross-Department Cohesion** | Promise-Delivery Gap (Marketing promises vs Engineering capacity) | ≤5% variance | `marketingCampaigns`, `engineeringCapacity`, `deliveryTimeline` | Delivery Service |
| **Strategic Risk Detection** | Days from risk emergence to escalation | ≤3 days for critical | `riskSignal`, `marketData`, `competitiveIntel` | Monitoring Service |
| **Approval Accuracy** | % of approved initiatives that deliver value within 6 months | ≥80% value delivery | `initiativeOutcome`, `estimatedValue`, `actualValue` | Outcome Tracker |
| **Quarterly Goal Achievement** | % of quarterly goals met or exceeded | ≥90% achievement | `quarterlyGoals`, `actualOutcomes` | Goal Tracker |

**Required Input Variables**:
```json
{
  "strategyService": {
    "evaluateInitiative": {
      "initiative": "object",
      "strategicGoals": "[goal]",
      "marketContext": "object"
    },
    "assessCohesion": {
      "marketingPlan": "object",
      "techCapacity": "object"
    }
  },
  "competitiveIntel": {
    "getMarketSignals": {
      "timeRange": "[startDate, endDate]"
    }
  }
}
```

---

### Dani (Chief Marketing Officer)
**Role**: Drives market visibility, brand strategy, and messaging consistency  
**Report Frequency**: Weekly campaign metrics, Monthly brand report

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **Brand Message Consistency** | % of external comms adhering to approved messaging | ≥98% consistent | `communication`, `brandGuidelines`, `previousMessaging` | Brand Policy Engine |
| **Campaign ROI** | Revenue influenced ÷ marketing spend | ≥3:1 ROI (3x return) | `campaignSpend`, `attributedRevenue`, `influncedOpportunities` | Revenue Attribution |
| **Brand Sentiment** | Net positive sentiment in social/news mentions | ≥0.70 (70% positive) | `mentions`, `sentiment`, `source` | Sentiment Analysis |
| **Channel Reach Efficiency** | Reach per dollar spent across all channels | ≥1000 reach/$ | `channelSpend`, `impressions`, `reachDeduped` | Ad Analytics |
| **Competitive Differentiation Score** | Unique messaging points vs competitor messaging | ≥3 distinct points | `ourMessaging`, `competitorMessaging`, `market` | Competitive Analysis |

**Required Input Variables**:
```json
{
  "brandService": {
    "validateMessaging": {
      "communication": "object",
      "guideline": "string"
    },
    "getApprovedMessages": {}
  },
  "revenueAttribution": {
    "getAttributedRevenue": {
      "campaign": "string",
      "timeRange": "[startDate, endDate]"
    }
  },
  "sentimentAnalysis": {
    "getBrandSentiment": {
      "timeRange": "[startDate, endDate]"
    }
  }
}
```

---

### Sofia (Chief Design Officer / Head of User Experience)
**Role**: Ensures user-centric design, accessibility compliance, and visual coherence  
**Report Frequency**: Per design review, Monthly accessibility audit

| KPI | Measurement | Success Target | Input Variables | Source |
|:----|:-----------|:----------------|:----------------|:-------|
| **WCAG 2.1 AA Compliance** | % of interface elements meeting WCAG AA standards | 100% | `uiElement`, `wcagCriteria`, `timestamp` | Accessibility Auditor |
| **Design System Reuse Rate** | % of components using approved design tokens | ≥90% reuse | `component`, `designSystem`, `codebase` | Design Audit |
| **User Test Validation** | % of major UX changes validated with user testing | ≥80% validated | `uxChange`, `testResults`, `userCohort` | User Research |
| **Color Contrast Compliance** | % of text passing AA contrast ratio (≥4.5:1) | 100% | `textColor`, `backgroundColor` | Contrast Checker |
| **Design-to-Code Accuracy** | % of delivered UI matching design specifications | ≥95% pixel-perfect | `designs`, `renderedUI` | Visual Regression Testing |

**Required Input Variables**:
```json
{
  "accessibilityService": {
    "auditInterface": {
      "uiHierarchy": "object",
      "wcagLevel": "AA|AAA"
    }
  },
  "designSystem": {
    "getTokens": {},
    "validateAlignment": {
      "component": "object"
    }
  },
  "userResearch": {
    "getTestResults": {
      "designChange": "object"
    }
  }
}
```

---

## 3. Cross-Agent Input Dependencies Map

```
Mira ← (work order context) ← Director
  ↓ (routing decision) 
  → Julia (technical feasibility) ← Architecture Service
  → Nora (financial impact) ← Finance Service
  → Elina (operational impact) ← Queue Service
  → Eva (compliance check) ← Compliance Service
  ↓ (synthesized response)
Director ← Mira
```

---

## 4. Real-Time Success Tracking

Each agent must provide:
1. **Current KPI Health** (dashboard view)
2. **Trend** (↑ improving, ↓ declining, → stable)
3. **Last Updated** (timestamp)
4. **Notes** (context for variance)

Example payload:
```json
{
  "agentId": "mira",
  "timestamp": "2026-03-21T10:30:00Z",
  "kpis": [
    {
      "name": "routingAccuracy",
      "value": 96.2,
      "unit": "%",
      "target": 95,
      "status": "HEALTHY",
      "trend": "STABLE",
      "lastUpdated": "2026-03-21T10:00:00Z"
    }
  ]
}
```

---

## 5. KPI Review Cadence

| Cadence | Owner | Action |
|:--------|:------|:-------|
| **Real-time** | Compliance (Eva), Operations (Elina) | Monitor thresholds; alert on violations |
| **Daily** | Finance (Nora), Compliance | Publish dashboard summaries |
| **Weekly** | All Agents | Submit KPI report; identify blockers |
| **Monthly** | CEO (Arya), Director | Strategic review; align priorities |
| **Quarterly** | CEO, Director, Board | Impact review; adjust targets |


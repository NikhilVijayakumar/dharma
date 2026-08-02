# GAP-002: operationsService.ts Is a 1143-Line Monolith Generating Mock/Hardcoded Data

**Severity**: CRITICAL  
**Category**: Backend / Data Integrity  
**Date**: 2026-03-19  
**Status**: FLAGGED FOR REFACTORING (Long-term)  
**Affects**: governance, triage, compliance, funding-digest, hiring-sim, design-audit, dashboard, infrastructure, daily-brief, weekly-review, suites  

---

## Summary

`src/main/services/operationsService.ts` (1143 lines) is a single monolithic service that generates **hardcoded seed data and mock payloads** for nearly every module in the application. Most of the data returned is not derived from real operations but is fabricated, seeded from constants, or assembled from cosmetic heuristics on unrelated telemetry signals.

## Evidence of Mock/Hardcoded Data

### Governance Module ΓÇö Hardcoded Decisions
```typescript
// Lines 290-359: Governance decisions are statically seeded
const seedOperationsState = (): PersistedOperationsState => {
  return {
    governanceDecisions: [
      { id: 'DEC-001', source: 'Nora (CFO)', title: 'Q3 Cost Guardrail Adjustment', status: 'DRAFT' },
      { id: 'DEC-002', source: 'Eva (Compliance)', title: 'Retention Policy Strict Mode', status: 'APPROVED' },
      ...
    ],
    triageItems: [
      { id: 'TRG-001', source: 'Secure Channel Alpha', topic: 'Q3 Financial Discrepancy Report', ... },
      ...
    ],
  };
};
```
These are never populated from real git operations, audit logs, or agent outputs.

### Funding Digest ΓÇö Hardcoded Financial Data
```typescript
// Lines 928-961: Financial figures are literal constants
const cashInBank = 786250;
const monthlyBurn = 42500;
leads: [
  { id: 'L-101', name: 'Sarah Jenkins', firm: 'Apex Capital', stage: 'Due Diligence', confidence: 75 },
  ...
],
```
No Maya (Funding) agent integration. No processed financial data from vault.

### Hiring Sim ΓÇö Hardcoded Candidates
```typescript
// Lines 977-1002: Candidate profiles are hardcoded
candidates: [
  { id: 'C-01', name: 'Alex Thompson', role: 'Staff Rust Engineer', matchScore: 93, ... },
  { id: 'C-02', name: 'Maria Garcia', role: 'Product Designer', matchScore: 86, ... },
  ...
],
```
No Lina (HR) agent integration. No model gateway calls for candidate generation.

### Executive Suites ΓÇö Generic Agent Profiles
```typescript
// Lines 885-910: Agent names don't match virtual employee specs
agents: [
  { id: 'AGT-OVR', name: 'The Overseer', role: 'Primary Orchestrator', ... },
  { id: 'AGT-SEC', name: 'Security Architect', role: 'Risk Assessment', ... },
  { id: 'AGT-OPS', name: 'Operations Coordinator', role: 'Queue and Triage Control', ... },
],
```
Should be the 10 virtual employees (Arya, Nora, Julia, etc.) from `docs/virtual-employee/*.md`, not generic placeholders.

### Design Audit ΓÇö Fabricated Metrics
```typescript
// Lines 1006-1048: Design audit metrics are cosmetic derivations
metrics: [
  { id: 'M-1', name: 'Contrast Ratios (WCAG AA)', value: '4.5:1 min met', ... },
  { id: 'M-2', name: 'Color Entropy', value: '12 unique hex', ... },
  // Not from actual visual audit - value is a constant
],
```
No `VisualAuditService.ts` or Niyama evaluator integration.

### Infrastructure ΓÇö Fake IPC Metrics
```typescript
// Lines 1119-1141: System metrics are manufactured
{ id: 'SYS-1', label: 'IPC Round Trip', value: `${12 + queue.pendingCount}ms`, ... },
{ id: 'SYS-2', label: 'Time to Interactive', value: '0.8s', ... },
```
These are not measured ΓÇö they're constant strings or simple arithmetic on unrelated counts.

### Compliance ΓÇö Derived from Cosmetic Heuristics
```typescript
// Lines 784-838: Violations count is sum of unrelated signals
const violationsCount = blockedSkills.length + providerWarnings.length + flaggedLogs.length;
const adherenceScore = Math.max(0, 100 - violationsCount * 4);
```
No `ComplianceScanService.ts`, no `AuditLogService.ts`, no audit_log.jsonl parsing.

### Daily Brief & Weekly Review ΓÇö Semi-Dynamic but No Agent AI
Both `getDailyBriefPayload()` and `getWeeklyReviewPayload()` construct payloads from gateway/skill/vault telemetry, but:
- No actual Mira compilation through model gateway
- No per-agent AI-generated report synthesis
- No Eisenhower Matrix classification
- Top requests are derived from system health, not agent work orders

### Subagent Seeding ΓÇö Demo Data
```typescript
// Lines 403-432: Demo subagents seeded on first call
const ensureDemoSubagents = (): void => {
  const parent = subagentService.spawn({ agentName: 'Mira Router', ... });
  const child = subagentService.spawn({ agentName: 'Eva Compliance', ... });
  subagentService.complete(child.id, 'Compliance pass completed for routing payload.');
};
```
Fake subagent tree created just for queue monitor display.

## What Should Happen

Each module documented in `docs/module/` specifies **specific data sources** and **agent integrations**:

| Module | Documented Data Source | Actual Data Source |
|:-------|:----------------------|:-------------------|
| Governance Lab | `audit_log.jsonl` + git operations + Eva AUDIT_PASS | Seeded JSON file |
| Triage | Queue System Protocol + Mira's Eisenhower Matrix | Seeded static items |
| Executive Suites | `/processed/{agent}/` output files per agent | 3 generic hardcoded agents |
| Funding Digest | `/processed/maya/opportunity_report.json` | Hardcoded dollar values |
| Hiring Sim | LLM-generated candidates via ModelGateway | Hardcoded profiles |
| Design Audit | `VisualAuditService.ts` static analysis | Fabricated metrics |
| Compliance | `ComplianceScanService.ts` + `AuditLogService.ts` | Heuristic from other signals |
| Infrastructure | `SystemHealthService.ts` RAM/CPU | Fake static values |

## Resolution

The `operationsService.ts` monolith needs to be decomposed into per-module services that:
1. Read from actual data sources documented in each module spec
2. Integrate with the correct virtual employee processing pipelines  
3. Use the model gateway for AI-generated content where specified
4. Read real system metrics for infrastructure monitoring

---

## Refactoring Roadmap (Phase-based)

### Phase 1: Extract Core Services from operationsService (Priority: HIGH)
Create the missing domain-specific services to replace hardcoded data:
- **ComplianceScanService.ts** ΓÇö Parse `audit_log.jsonl`, compute adherence scores (not cosmetic heuristics)
- **VisualAuditService.ts** ΓÇö Analyze UI components for WCAG compliance, measure color entropy, typography metrics
- **SystemHealthService.ts** ΓÇö Monitor actual RAM/CPU via Electron/Node APIs
- **DailyBriefCompilerService.ts** ΓÇö Orchestrate Mira to synthesize daily reports (not just aggregate telemetry)
- **HiringSimService.ts** ΓÇö Integrate with Lina (HR) agent to generate real candidate matches
- **FundingDigestService.ts** ΓÇö Integrate with Maya (Funding) to compute real runway/burn metrics

### Phase 2: Replace operationsService Mock Payloads (Priority: HIGH)
Update module repositories to call extracted services instead of `operationsService`:
- `src/renderer/src/features/compliance/repo/ComplianceRepo.ts` ΓåÆ call `ComplianceScanService`
- `src/renderer/src/features/design-audit/repo/DesignAuditRepo.ts` ΓåÆ call `VisualAuditService`
- `src/renderer/src/features/infrastructure/repo/InfrastructureRepo.ts` ΓåÆ call `SystemHealthService`
- `src/renderer/src/features/daily-brief/repo/DailyBriefRepo.ts` ΓåÆ call `DailyBriefCompilerService`
- `src/renderer/src/features/hiring-sim/repo/HiringSimRepo.ts` ΓåÆ call `HiringSimService`
- `src/renderer/src/features/funding-digest/repo/FundingDigestRepo.ts` ΓåÆ call `FundingDigestService`

### Phase 3: Virtual Employee Integration (Priority: CRITICAL)
- Implement **Work Order System** (see GAP-005)
- Implement **10 Virtual Employee TypeScript Services** (see GAP-005)
- Wire Agent ΓåÆ operationsService for real data generation
- Deprecate hardcoded agent profiles in suites module

### Phase 4: Decompose operationsService (Priority: MEDIUM)
- Retain ONLY: `getQueueMonitorPayload()`, `getSettingsPayload()`, governance decision persistence, triage persistence
- Move OR DELETE: All aggregate mock payloads (funding, hiring, design audit, infrastructure, compliance, daily/weekly briefs)
- Result: operationsService becomes <300 lines, focused on cross-cutting operational state

---

## Current Workaround (Until Refactoring)

Until the above services are implemented, users should understand:
- **All module displays show seeded/hardcoded data**, not real production state
- Financial metrics (funding), hiring candidates, design audit results, and compliance scores are **NOT real**
- Only **gateway status, vault files, triage items, governance decisions** are partly real
- Agent integrations do not exist; modules display demo data only

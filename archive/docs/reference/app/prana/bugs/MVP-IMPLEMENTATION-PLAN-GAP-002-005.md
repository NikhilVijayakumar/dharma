# MVP Implementation Plan (Functionality First)

Date: 2026-03-19
Scope: GAP-002, GAP-003, GAP-004, GAP-005
Architecture position: Keep current MVVM structure for MVP; defer Clean Architecture migration.

---

## 1) Objective

Deliver functional MVP behavior by fixing critical runtime and product gaps in this order:
1. GAP-004 (readiness integrity and release gating)
2. GAP-002 (real data/services instead of monolithic seeded payloads)
3. GAP-005 (virtual employee runtime and work-order flow)
4. GAP-003 (module-level UX/components powered by real data and agents)

Why this order:
- GAP-004 ensures honest tracking and gate discipline.
- GAP-002 creates real service/data foundations needed by all modules.
- GAP-005 enables core product value (agent execution) on top of real services.
- GAP-003 becomes meaningful only after data + agent runtime exist.

---

## 2) Priority Matrix

| Gap | Priority | MVP Criticality | Dependency | Exit Condition |
|:----|:---------|:----------------|:-----------|:---------------|
| GAP-004 Readiness Accuracy | P0 | Release governance | None | Feature gates reflect reality and are enforced |
| GAP-002 Operations Monolith | P0 | Data integrity | GAP-004 | Core modules use extracted real services |
| GAP-005 Agent Runtime | P0 | Core product capability | GAP-002 | Director -> Work Order -> Agent -> Output loop runs |
| GAP-003 Module Feature Completeness | P1 | UX/product completeness | GAP-002 + GAP-005 | Priority modules deliver documented functional flows |

---

## 3) Phase Plan (Functionality-First)

## Phase 0 - Governance Baseline (Week 0-1)
Focus: Lock realistic readiness and acceptance criteria.
Maps to: GAP-004

Deliverables:
- Keep corrected readiness snapshot (F1-F2 complete, F3-F8 partial) as release baseline.
- Define MVP "done" criteria per gap (functional behavior, not architecture purity).
- Add weekly gate review ritual (Gate D/F/G evidence refresh).

Checklist:
- [ ] Product + QA sign off on MVP scope excluding Clean Architecture migration.
- [ ] Checklist owner assigned (single accountable maintainer).
- [ ] Weekly review cadence scheduled.
- [ ] Release notes template includes known functional limitations.

Exit criteria:
- Team agrees on what can ship for MVP and what remains deferred.

---

## Phase 1 - Decompose operationsService by Capability (Week 1-3)
Focus: Remove fabricated outputs from critical modules.
Maps to: GAP-002

Implementation order inside Phase 1:
1. SystemHealthService (fastest, least coupled)
2. ComplianceScanService + AuditLogService
3. FundingDigestService
4. HiringSimService
5. VisualAuditService
6. DailyBriefCompilerService + WeeklyReviewCompilerService

Deliverables:
- Extract service modules from operations monolith.
- Replace seeded constants with real source readers/processors.
- Keep operationsService as thin orchestrator/facade only.

Checklist:
- [ ] Create src/main/services/SystemHealthService.ts with real RAM/CPU/process metrics.
- [ ] Create src/main/services/AuditLogService.ts to parse audit_log.jsonl.
- [ ] Create src/main/services/ComplianceScanService.ts with assertion outputs + SLA signals.
- [ ] Create src/main/services/FundingDigestService.ts sourcing real processed funding files.
- [ ] Create src/main/services/HiringSimService.ts using deterministic input + model gateway hook points.
- [ ] Create src/main/services/VisualAuditService.ts for measurable UI checks.
- [ ] Create src/main/services/DailyBriefCompilerService.ts and WeeklyReviewCompilerService.ts.
- [ ] Update IPC handlers to call extracted services, not hardcoded payload builders.
- [ ] Reduce operationsService responsibilities to shared state + aggregation only.
- [ ] Add unit tests for each new service (happy path + failure mode).

Exit criteria:
- Funding, hiring, design-audit, compliance, infrastructure, daily/weekly data are no longer hardcoded.
- operationsService no longer owns module-specific mock builders.

---

## Phase 2 - Agent Runtime MVP (Week 3-6)
Focus: Implement the minimal usable virtual employee loop.
Maps to: GAP-005

MVP runtime slice:
- Mira intake and routing
- WorkOrder lifecycle (Init -> Plan -> Execute -> Synthesis)
- 3 initial execution agents: Nora, Eva, Julia
- Agent output persistence and approval handoff

Deliverables:
- Agent core protocol and orchestration services.
- Director interaction submission path from UI to runtime.
- Basic agent-to-model gateway prompt/response flow.

Checklist:
- [ ] Define WorkOrder types (priority, owner, targetAgent, dueAt, state, artifacts).
- [ ] Implement QueueService (10-slot policy + crisis reserve behavior).
- [ ] Implement CommandRouterService (Mira routing with deterministic rules first).
- [ ] Implement AgentProtocol base contract + shared runtime utilities.
- [ ] Implement Nora, Eva, Julia agent services with bounded tool permissions.
- [ ] Implement interaction backbone IPC contract for DirectorFeedbackRequest.
- [ ] Implement stage transition auditing (who, when, result, artifact links).
- [ ] Persist outputs to app data processed folders with approval status.
- [ ] Add cancellation/retry/timeout handling for work orders.
- [ ] Add integration tests: Director request -> routed work order -> agent output.

Exit criteria:
- End-to-end functional loop works for at least 3 agent workflows.

---

## Phase 3 - High-Impact Module UX Completion (Week 6-9)
Focus: Build functional module surfaces powered by real runtime data.
Maps to: GAP-003

MVP module priority order:
1. Triage
2. Executive Suites
3. Governance
4. Compliance
5. Daily Brief + Weekly Review

Deliverables:
- Replace flat/demo displays in the above modules with functional widgets and actions.
- Wire module actions to real services and work orders.

Checklist:
- [ ] Triage: implement priority stack behavior + keyboard actions + real queue bindings.
- [ ] Suites: implement per-agent workspace tabs (start with Nora/Eva/Julia/Mira).
- [ ] Governance: show real policy decisions, audit stream, commit gate status.
- [ ] Compliance: SLA tracker and real violations feed from ComplianceScanService.
- [ ] Daily Brief: scheduled + on-demand compile from real agent outputs.
- [ ] Weekly Review: questionnaire flow with generated summaries from runtime outputs.
- [ ] Notification Centre: show hook/queue/agent events with actionable links.
- [ ] Add module-level integration tests for each priority module.

Exit criteria:
- Priority modules no longer depend on fabricated payloads and support functional operator workflows.

---

## Phase 4 - Stabilization and Release Hardening (Week 9-10)
Focus: Reliability and release confidence.
Maps to: GAP-002/003/005 hardening + GAP-004 verification

Deliverables:
- Test depth, observability, and fallback handling for degraded environments.
- Final MVP readiness review.

Checklist:
- [ ] Add fault-injection tests for provider outage, malformed files, queue overload.
- [ ] Add telemetry dashboards for work-order throughput, failure reasons, SLA drift.
- [ ] Validate cron-driven daily/weekly flows in time-shifted test runs.
- [ ] Security pass on tool-policy boundaries for agent actions.
- [ ] Update docs/bugs/FIX-SUMMARY.md with implemented (not planned) items.
- [ ] Freeze release candidate and run QA regression suite.

Exit criteria:
- MVP release sign-off with known limitations documented.

---

## 4) Parallelization Rules

Do in parallel:
- Service extraction streams in Phase 1 after shared contracts are defined.
- Agent implementation and UI wiring once WorkOrder contract is frozen.
- Test writing alongside implementation (no end-loaded test phase).

Do not parallelize:
- WorkOrder contract changes after integration starts.
- Queue policy changes during stabilization window.

---

## 5) Definition of Done (MVP)

A gap is considered functionally fixed for MVP when:
- Behavior is real (not seeded/mock constants for core flows).
- Trigger path exists from UI/IPC to runtime service.
- Result is persisted and observable in the target module.
- Automated tests cover happy path + at least one failure mode.
- Checklist gate evidence is updated and reviewable.

---

## 6) Risks and Mitigation

| Risk | Impact | Mitigation |
|:-----|:-------|:-----------|
| Agent runtime scope explosion | Timeline slip | Start with 3-agent slice; add remaining agents incrementally |
| Hidden coupling in operationsService | Refactor churn | Keep facade compatibility adapter during extraction |
| UI builds ahead of runtime | Rework | Enforce runtime-first milestones per module |
| Model provider instability | Unreliable workflows | Add deterministic fallback and retry budget |
| Over-optimistic schedule | Delivery pressure | Track optimistic vs realistic dates; review weekly |

---

## 7) Execution Checklist (Single View)

P0 (must complete for MVP value):
- [ ] Phase 0 governance baseline complete.
- [ ] Phase 1 service extraction complete for critical modules.
- [ ] Phase 2 3-agent runtime loop complete.
- [ ] Phase 3 priority modules functionalized (triage, suites, governance, compliance, briefs).
- [ ] Phase 4 stabilization complete and QA sign-off obtained.

P1 (next after MVP):
- [ ] Expand from 3 to 10 agents.
- [ ] Complete all documented specialized components across remaining modules.
- [ ] Revisit deferred architecture migration after MVP release.

---

## 8) Ownership Suggestion

- Runtime and services: Main process team
- Module UX and wiring: Renderer team
- Verification and gates: QA + engineering lead
- Scope control: Product + tech lead

Recommended weekly artifact updates:
- docs/bugs/FIX-SUMMARY.md
- docs/system/openclaw-feature-readiness-checklist.md
- docs/bugs/fix-plan-review.md

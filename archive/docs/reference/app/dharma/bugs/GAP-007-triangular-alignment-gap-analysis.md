# GAP-007: Full System Validation Gap Analysis

Date: 2026-03-22  
Role: Lead Quality Assurance Engineer and Principal Systems Architect  
Scope: End-to-end validation against docs/module, docs/system, and src/core/registry with zero-mock mandate

---

## 1. What Was Audited

### Runtime Surfaces
- Electron IPC handlers and main-process services (`auth`, `operations`, `registry`, `queue`, `work-orders`, `memory`, `subagents`)
- Renderer route guards and page routing
- Agent execution pipeline and model provider bridge
- Registry-facing UI modules for agent/skill/protocol/workflow/KPI/data input

### Contract Surfaces
- Atomic specs in `docs/module/`
- System architecture docs in `docs/system/`
- Registry contracts in `src/core/registry/`

---

## 2. Critical Findings (Current)

### C1. Agent execution used deterministic model stub
- Status: **Fixed in this pass**
- Prior behavior: `sharedPromptPipeline.callModel()` returned a hardcoded synthesis string.
- Remediation: Replaced stub with real provider execution through `localExecutionProviderService` using ordered fallback derived from gateway health.

### C2. Agent deep persona fields were not injected into prompts
- Status: **Fixed in this pass**
- Prior behavior: `objectives_long_term`, `interaction_style`, `personality_traits`, and registry backstory were not included in system prompt context.
- Remediation: Added registry lookup (`coreRegistryService.getAgentTemplate`) and prompt enrichment block in `sharedPromptPipeline.buildSystemPrompt()`.

### C3. Route protection violated volatile-session rule
- Status: **Partially fixed in this pass**
- Prior behavior: many non-auth pages were wrapped in `PublicOnlyGuard`, which permits unauthenticated access.
- Remediation: Converted those pages to `MainAppGuard` + authenticated layout, leaving only `/login`, `/forgot-password`, `/reset-password` as public.
- Remaining note: Review if `/access-denied` should be public for explicit pre-auth failure messaging; currently it is protected for strict interpretation.

---

## 3. High-Risk Gaps Still Open

### H1. Registry editor modules remain placeholder-only
- Evidence: `registry-agent-editor`, `registry-skill-editor`, `registry-protocol-editor`, `registry-workflow-editor`, `registry-kpi-editor`, `registry-data-input-editor` containers still show awaiting-bindings hero placeholders.
- Impact: No real edit cycle for current/improved/commit in these module-specific UIs.

### H2. Queue manager persistence is volatile
- Evidence: queue state is maintained in in-memory map; no SQLite persistence or restart recovery in queue service.
- Impact: queued/running task continuity is lost on process restart.

### H3. Onboarding staged state persistence is browser-local
- Evidence: onboarding draft/commit ledger path still relies on renderer local storage patterns, not durable SQLite staging tables.
- Impact: staged work can be lost on browser storage clear and is not centrally auditable.

### H4. Triple-engine runtime integration is incomplete
- Goose: workflow docs and registry references exist, but no verified execution bridge from live task processing.
- NemoClaw: no verified runtime bridge for UI anchor navigation from skill execution path.
- OpenCLAW: no verified runtime bridge for deep persona feature-analysis execution path.

### H5. Vector search quality remains heuristic
- Evidence: memory/vector behavior relies on lightweight token hashing and cosine similarity, not model embeddings.
- Impact: semantic recall quality can drift on long, nuanced requests.

---

## 4. Bug Sweep Decision

- The previous ΓÇ£no critical blocking gapsΓÇ¥ statement in this file was inaccurate under strict zero-mock criteria.
- This document now reflects runtime reality and distinguishes fixed vs still-open production blockers.

---

## 5. Remediation Priority (Next)

1. Implement real registry editor data/repo/viewmodel layers for all six registry editor modules.
2. Add durable queue persistence and startup recovery using SQLite-backed state.
3. Move onboarding staged state to SQLite with explicit transaction boundaries for Back/Next phase transitions.
4. Add verified execution bridges for Goose, NemoClaw, and OpenCLAW in the live work-order pipeline.
5. Upgrade vector indexing from heuristic hash vectors to embedding-backed retrieval.

---

## 6. Validation Outcome

- Zero-mock posture improved materially in this pass for agent execution and auth guard enforcement.
- Full production readiness is **not yet achieved** until open high-risk gaps are closed.

---

## 7. UI/UX Audit Addendum (2026-03-23)

### A1. Registry Operations UX was missing from onboarding lifecycle path
- Status: **Fixed in this pass**
- Prior behavior:
	- No keyword-based registry directory/file discovery for YAML/JSON/Markdown.
	- No markdown edit/save flow in lifecycle manager.
	- No full-file upload flow targeting registry directories.
	- No direct onboarding handoff to registry management tools.
- Remediation:
	- Added registry file explorer APIs and secure path handling in core/main IPC.
	- Added lifecycle UI for keyword filter, directory filter, file preview, markdown editing, and full-file uploads.
	- Added onboarding final-step quick links to lifecycle manager and registry viewer.

### A2. New data-input creation did not update registry schema/UI automatically
- Status: **Fixed in this pass**
- Prior behavior:
	- Only existing data-input overrides could be edited; new data-input records were not created in `src/core/registry/data-inputs`.
	- UI did not auto-refresh lifecycle snapshot after adding new data definitions.
- Remediation:
	- Added `createLifecycleDataInput` operation to write schema-valid data-input files directly into registry source.
	- Added lifecycle UI create form (id/name/description/schema/required fields/sample source + optional seed file/content).
	- Auto-refresh lifecycle snapshot so newly created data-inputs appear immediately in UI.

### A3. Cross-screen placeholder inventory
- Status: **Logged, partially open**
- Findings:
	- Multiple feature containers still show placeholder hero messages such as "Module awaiting data and logic bindings" (examples include `agent-deprecation`, `agent-performance-review`, `agent-provisioning`, `chat-external-routing`, `chat-internal-interface`, `virtual-employee-dashboard`, `viewer-markdown`, `viewer-pdf`, and others).
- Action:
	- Logged as remaining UI implementation debt. These are separate feature-delivery tasks beyond the registry/onboarding lifecycle scope addressed in this pass.

### A4. Hierarchical onboarding governance flow was missing dependency gates
- Status: **Fixed in this pass**
- Prior behavior:
	- Onboarding was a linear phase wizard without enforced step-level approvals.
	- Agent approval could progress without explicit company-core and global-asset approval gates.
	- No drill-down dashboard status model (`PENDING`/`DRAFT`/`APPROVED`) for each onboarding entity.
- Remediation:
	- Refactored onboarding flow into dependency-unlocked stages: company-core, global-assets, agent-deep-dive, channel-access, model-config, final-approval.
	- Added step approval state and drill-down navigation with return-to-previous-step support without draft loss.
	- Added final master-commit guard that blocks commit until prerequisite steps are approved.

### A5. Company registry root and agent persona-extension contract were absent
- Status: **Fixed in this pass**
- Prior behavior:
	- No dedicated company-core registry artifact under `src/core/registry/company`.
	- Agent schema did not require `core_objective`, `individual_vision`, and role-specific non-negotiables.
- Remediation:
	- Added `src/core/registry/company/company-core.json` and loaded it into onboarding blueprint snapshot.
	- Updated `agent-template.schema.json` and registry loader normalization to support required persona-extension fields.
	- Added onboarding validation for agent-vision alignment vs company vision using LLM check with deterministic fallback.

### A6. Remaining gap: true SQLite onboarding staging tables
- Status: **Open**
- Evidence:
	- Runtime still uses durable app state + vault projection; dedicated SQLite onboarding staging tables and transactional boundaries are not yet implemented.
- Impact:
	- Entity approvals are durable but not yet queryable through the full hybrid-schema table contract.
- Required follow-up:
	- Implement explicit SQLite onboarding tables for step drafts, entity statuses, and approval ledger with restart-safe transaction boundaries.

# Monorepo Split Analysis (Dhi, Prana, Dharma, Vidhan)

Baseline: restored monorepo docs from commit `c8f0a2617aab71add9f093044d0dda1c2854602e` under `docs/monorepo`.

Scope decision applied:
- Dhi: application (Director Office).
- Vidhan: application (Administration).
- Prana (Parana): shared core engine library used by both apps.
- Dharma: shared virtual-employee/schema library used by both apps.
- Astra: intentionally excluded from this split per request.

## Ownership Logic

1. If the module is currently routed/available in Dhi runtime, map to Dhi.
2. If it is cross-app engine/runtime/onboarding/auth/viewer infrastructure, map to Prana.
3. If it is virtual-employee/schema governance baseline, map to Dharma.
4. Remaining app-facing modules not present in Dhi are mapped to Vidhan.

## Availability Matrix

## Dhi (App)
Present modules in `docs/app/dhi/module`:
- chat-external-routing
- chat-internal-interface
- executive-suites
- kpi-breach-alerting
- kpi-verification-cron
- notification-centre
- report-daily-brief
- report-funding-digest
- report-weekly-review
- triage-incident-resolution
- triage-incident-routing
- virtual-employee-dashboard

## Prana / Parana (Library)
Present modules in `docs/app/prana/module`:
- login
- forgot-password
- reset-password
- infrastructure-layers
- onboarding-channel-configuration
- onboarding-model-configuration
- onboarding-registry-approval
- splash-system-initialization
- vault-folder-structure
- vault-knowledge-repository
- viewer-markdown
- viewer-pdf
- email-cron-heartbeat
- email-draft-sync
- email-management
- email-orchestrator-service
- google-ecosystem-integration

## Dharma (Library)
Present modules in `docs/app/dharma/module`:
- onboarding-hybrid-explorer-governance-lifecycle

Note:
- Dharma library scope is primarily represented via `core` and `system` documentation plus bug mapping for virtual-employee and business-alignment tracks.

## Vidhan (App)
Present modules in `docs/app/vidhan/module` (not available in Dhi mapping):
- administration-suite
- management-suite
- settings-application-rules
- settings-user-profile
- agent-deprecation
- agent-performance-review
- agent-provisioning
- compliance-audit-logger
- design-audit-verification
- governance-policy-editor
- hiring-sim
- protocol-enforcement-gate
- queue-execution-engine
- queue-visualization
- registry-agent-editor
- registry-data-input-editor
- registry-kpi-editor
- registry-protocol-editor
- registry-skill-editor
- registry-workflow-editor
- visual-identity-engine

## Bug Mapping Rule Used

Monorepo bug docs were mapped by primary ownership:
- Dhi: Director app gaps
- Prana: engine/runtime/service gaps
- Dharma: virtual employee/schema/business DNA gaps
- Vidhan: admin app and migration-related admin gaps

Astra-owned historical bug docs remain excluded from this split:
- GAP-009-registry-ui-contract-mismatch.md
- GAP-011-localization-and-hardcoded-ui-strings.md
- GAP-022-typography-api-limitations.md

## Final Validation (Deep Check)

### 1. Monorepo module coverage
- Baseline module docs in `docs/monorepo/module`: 51
- Split assignment in `docs/app/*/module` (excluding INDEX files): 51
- Missing modules from baseline: 0
- Duplicate ownership assignments: 0

Ownership counts:
- Dhi app modules: 12
- Prana library modules: 17
- Dharma library modules: 1
- Vidhan app modules: 21

### 2. Core/System/Bug split completeness
- Core baseline docs: 6 and represented across the split trees.
- System baseline docs: 11 and represented across the split trees.
- Bug baseline docs: 29 total.
- Mapped in this 4-target split: 26.
- Intentionally excluded as Astra-owned historical scope: 3 (`GAP-009`, `GAP-011`, `GAP-022`).

### 3. Shared library dependency isolation
- Prana and Dharma are documented as shared libraries consumed by both apps.
- Source-level import scan result used for this split: no `@dhi`/`@vidhan` import dependencies in `prana/src` and `dharma/src`.

### 4. Naming/route coverage gap closure
Monorepo module titles and runtime route names differ for several Dhi pages.
To avoid false gaps, a route-to-module reconciliation map was added:
- `docs/app/dhi/module/ROUTE-COVERAGE-MAP.md`

Result:
- The 4-target split (`dhi`, `prana`, `dharma`, `vidhan`) is complete for original monorepo documentation scope.

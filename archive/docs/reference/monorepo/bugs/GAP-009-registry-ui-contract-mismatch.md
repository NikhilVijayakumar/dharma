# GAP-009 Registry UI Contract Mismatch

Status: Open
Severity: Critical
Category: Registry UX contract
Scope: Onboarding registry approval + registry editors + lifecycle registry tooling

## Summary
Registry-related screens partially exist, but the implemented UI does not meet the schema-driven, side-by-side, and approval-gated contract defined in module and system docs.

## Expected Contract
- [docs/module/onboarding-registry-approval.md](docs/module/onboarding-registry-approval.md#L22) requires side-by-side default schema and editable user variant.
- [docs/module/onboarding-registry-approval.md](docs/module/onboarding-registry-approval.md#L35) requires staged SQLite state and explicit approval gate before final commit.
- [docs/module/infrastructure-layers.md](docs/module/infrastructure-layers.md#L33) requires dynamic UI from registry schemas and no hardcoded contracts.
- [docs/module/infrastructure-layers.md](docs/module/infrastructure-layers.md#L35) requires Current State and Proposed Improvement side-by-side before commit.

## Current Implementation Reality
- Registry lifecycle page has partial side-by-side only for profile/skill/KPI/data input fields:
  - [src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx](src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx#L365)
- Registry file explorer supports read and markdown save, plus file upload:
  - [src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx](src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx#L590)
- Dedicated registry editor pages are still placeholders:
  - [src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx](src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx#L9)
  - [src/renderer/src/features/registry-data-input-editor/view/RegistryDataInputEditorContainer.tsx](src/renderer/src/features/registry-data-input-editor/view/RegistryDataInputEditorContainer.tsx#L9)
  - [src/renderer/src/features/registry-kpi-editor/view/RegistryKpiEditorContainer.tsx](src/renderer/src/features/registry-kpi-editor/view/RegistryKpiEditorContainer.tsx#L9)
  - [src/renderer/src/features/registry-protocol-editor/view/RegistryProtocolEditorContainer.tsx](src/renderer/src/features/registry-protocol-editor/view/RegistryProtocolEditorContainer.tsx#L9)
  - [src/renderer/src/features/registry-skill-editor/view/RegistrySkillEditorContainer.tsx](src/renderer/src/features/registry-skill-editor/view/RegistrySkillEditorContainer.tsx#L9)
  - [src/renderer/src/features/registry-workflow-editor/view/RegistryWorkflowEditorContainer.tsx](src/renderer/src/features/registry-workflow-editor/view/RegistryWorkflowEditorContainer.tsx#L9)

## Additional Risk From Core Registry Layer
Registry snapshot intentionally does not hard-fail when validation errors exist:
- [src/core/registry/loader.ts](src/core/registry/loader.ts#L851)

This improves runtime resilience but allows UI editors to operate over drifted schemas without enforcing correction workflows.

## User Impact
- Operators cannot use dedicated editor routes promised by docs/module.
- Registry edits are centralized in one lifecycle page, reducing role-based clarity and discoverability.
- Approval semantics are not consistently enforced across registry editing surfaces.

## Required Fix Plan
1. Implement all dedicated registry editor routes using shared schema-driven form engine.
2. Add unified diff pane for Current vs Proposed across every registry entity type.
3. Add explicit staged state and approval workflow per editor route.
4. Add registry validation banner with actionable errors on each editor when snapshot has validation drift.

## Acceptance Criteria
- Every registry editor route renders functional entity-specific UI.
- Every write path shows Current and Proposed blocks before commit.
- Every commit path supports approval state and audit metadata.
- Registry validation drift is visible and blocks unsafe commits by policy.

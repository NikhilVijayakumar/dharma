# GAP-012 UI Plan Metadata Actionability Gap

Status: Open
Severity: Medium
Category: Planning-to-implementation fidelity
Scope: docs/plan structure used for runtime UI composition

## Summary
The plan corpus includes full page inventory and route styles, but deep-page metadata lacks actionable local navigation definitions. This creates an implementation blind spot: navigation and component composition are hand-coded instead of generated from plan data.

## Evidence
- Full page list exists in [docs/plan/page_index.json](docs/plan/page_index.json#L1).
- Deep page route model exists in [docs/plan/phase_1_approval.json](docs/plan/phase_1_approval.json#L2017).
- Local navigation repeatedly empty in deep pages:
  - [docs/plan/phase_1_approval.json](docs/plan/phase_1_approval.json#L604)
  - [docs/plan/phase_1_approval.json](docs/plan/phase_1_approval.json#L638)
  - [docs/plan/PG-DHI-028_architect.json](docs/plan/PG-DHI-028_architect.json#L16)

## Why It Matters To UI Delivery
Without local navigation and interaction metadata:
- Renderer cannot generate consistent subnavigation.
- Specialized pages become hard to discover.
- UI composition diverges from plan and module docs.

## Required Fix Plan
1. Add mandatory non-empty local_navigation schema for every deep page where page_type is standard or onboarding.
2. Add build-time validator for docs/plan that fails on empty navigation arrays where required.
3. Add generated TypeScript nav manifest output from docs/plan to replace manual route constant maintenance.
4. Add page-level interaction anchors in plan for editor and approval flows.

## Acceptance Criteria
- Every page in page index has actionably complete navigation metadata.
- A generated navigation artifact is consumed by renderer.
- Manual nav constants are reduced to fallback only.
- Plan changes propagate to runtime nav without manual route edits.

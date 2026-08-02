# GAP-010 Navigation Discoverability And Flow

Status: Open
Severity: High
Category: Navigation UX
Scope: Global and local navigation

## Summary
Route coverage is broad, but discoverable navigation is narrow. The product exposes many screens in routing, while primary navigation includes only a subset and planning metadata for local navigation remains empty.

## Evidence
- Extensive route map in [src/renderer/src/main.tsx](src/renderer/src/main.tsx#L98) includes onboarding, registry, reports, triage, viewers, and many advanced module pages.
- Primary nav definitions are a fixed subset in [src/renderer/src/common/constants/moduleRegistry.ts](src/renderer/src/common/constants/moduleRegistry.ts#L32).
- Planning metadata defines deep pages but leaves local navigation empty:
  - [docs/plan/PG-DHI-028_architect.json](docs/plan/PG-DHI-028_architect.json#L16)
  - [docs/plan/phase_1_approval.json](docs/plan/phase_1_approval.json#L604)

## Contract References
- [docs/system/openclaw-feature-readiness-checklist.md](docs/system/openclaw-feature-readiness-checklist.md#L34) requires no-dead-end navigation with Back and Home behavior.
- [docs/plan/page_index.json](docs/plan/page_index.json#L1) defines the full page inventory expected in the product flow.

## Impact
- Many pages are reachable only by direct route knowledge, not by guided UI flow.
- User journey from top-level modules to specialized pages is ambiguous.
- QA and product review cannot validate end-to-end navigation intent without manual route injection.

## Root Cause
- Sidebar is tied to manifest-backed core modules only.
- Local navigation layer from plan metadata is not implemented.
- No generated navigation graph from docs/plan into renderer routing helpers.

## Required Fix Plan
1. Build a route metadata adapter that imports page inventory and local navigation rules into runtime nav model.
2. Add contextual in-page links from top-level modules to specialized sub-pages.
3. Add universal breadcrumb component for non-top-level routes.
4. Add route health test to detect pages with zero incoming UI links.

## Acceptance Criteria
- Every non-auth page has at least one discoverable UI entry point.
- Every specialized page has breadcrumb and return path to parent context.
- Navigation graph is generated from plan metadata rather than manual constants.
- No page is reachable only by direct URL knowledge.

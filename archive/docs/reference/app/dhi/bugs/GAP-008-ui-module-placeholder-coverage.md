# GAP-008 UI Module Placeholder Coverage

Status: Open
Severity: Critical
Category: UI delivery completeness
Scope: Renderer feature containers

## Summary
A large portion of route-mapped screens are still scaffold placeholders instead of functional module views. This blocks parity with module specifications and prevents meaningful UI testing.

## Why This Is A Gap
The platform exposes extensive route coverage in [src/renderer/src/main.tsx](src/renderer/src/main.tsx#L98), but many mapped routes render static placeholder hero content only.

Representative placeholder implementation:
- [src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx](src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx#L9)

Representative placeholder siblings (same pattern at line 9):
- [src/renderer/src/features/queue-visualization/view/QueueVisualizationContainer.tsx](src/renderer/src/features/queue-visualization/view/QueueVisualizationContainer.tsx#L9)
- [src/renderer/src/features/registry-workflow-editor/view/RegistryWorkflowEditorContainer.tsx](src/renderer/src/features/registry-workflow-editor/view/RegistryWorkflowEditorContainer.tsx#L9)
- [src/renderer/src/features/registry-data-input-editor/view/RegistryDataInputEditorContainer.tsx](src/renderer/src/features/registry-data-input-editor/view/RegistryDataInputEditorContainer.tsx#L9)
- [src/renderer/src/features/viewer-markdown/view/ViewerMarkdownContainer.tsx](src/renderer/src/features/viewer-markdown/view/ViewerMarkdownContainer.tsx#L9)
- [src/renderer/src/features/virtual-employee-dashboard/view/VirtualEmployeeDashboardContainer.tsx](src/renderer/src/features/virtual-employee-dashboard/view/VirtualEmployeeDashboardContainer.tsx#L9)

Current audit count from source: 39 view containers still using "Module awaiting data and logic bindings".

## Contract References Violated
- [docs/system/openclaw-feature-readiness-checklist.md](docs/system/openclaw-feature-readiness-checklist.md#L33) requires schema-driven system and module screens.
- [docs/module/registry-agent-editor.md](docs/module/registry-agent-editor.md#L1) defines a concrete editor module, not a placeholder shell.
- [docs/module/registry-workflow-editor.md](docs/module/registry-workflow-editor.md#L1) defines workflow editing behavior and integration.

## User Impact
- Route reaches screen, but task cannot be executed.
- QA cannot verify feature behavior beyond navigation.
- "Implemented" screens are visually present but functionally absent, causing false readiness signals.

## Root Cause
- Scaffolding generated route-complete placeholders before data/viewmodel bindings.
- No enforcement gate that blocks route exposure until module exits placeholder state.

## Required Fix Plan
1. Add placeholder detection gate in CI: fail if any route-mapped view contains the placeholder marker string.
2. Prioritize module binding in this order:
   - Registry editors
   - Viewer screens
   - Incident and governance operational modules
3. For each module:
   - Add repository integration
   - Add viewmodel with loading and error states
   - Replace static hero shell with module-specific UI per docs/module contract

## Acceptance Criteria
- Zero route-mapped module views contain "Module awaiting data and logic bindings".
- Each screen has functional read path and at least one write/approval path where applicable.
- Each migrated screen shows deterministic loading, error, and empty states.
- Readiness checklist can mark corresponding feature as complete without documentation caveats.

# GAP-021: Administration Module Policy and Integration Feasibility

## Summary

Director Office coverage is broad, but there was no dedicated administration module blueprint that combines:
- policy lifecycle documentation in vault-friendly markdown units,
- staff administration datasets (registry, attendance, holiday calendar),
- employee feedback and wellbeing evaluation loops,
- external channel intelligence (mail and social trends),
- and practical integration pathways (Google Workspace, MCP, and webhook automation).

## Requested Capability

1. Create a dedicated Administration module that governs organization-wide administrative workflows and policy updates.
2. Create a vault policy folder structure where policy markdown files are logically separated and small enough for context-window-safe usage.
3. Maintain CSV-ready staff data and map attendance/holiday operations to Google Sheets when available.
4. Support document conversion workflows (Markdown <-> HTML <-> DOCX) with auditability.
5. Collect employee happiness/feedback via forms and route through administrative workflows.
6. Add dedicated operations account handling for email automations, meeting-note ingestion, daily/weekly reporting.
7. Include social trend intelligence (Twitter/X read-only) as an input channel for policy updates.
8. Assess feasibility using local reference repositories:
   - openclaw-main
   - goose-main
   - NemoClaw-main

## Feasibility Evidence

### Confirmed in Reference Repositories

- openclaw-main
  - Cron, webhook, and Gmail Pub/Sub automation are available in project docs.
  - File: openclaw-main/README.md
- openclaw-main
  - Feishu extension includes DOCX tooling and markdown/document handling patterns.
  - Files:
    - openclaw-main/extensions/feishu/src/docx.ts
    - openclaw-main/extensions/feishu/src/docx-batch-insert.ts
- goose-main
  - MCP-native architecture is present (rmcp dependency and MCP extension ecosystem).
  - Files:
    - goose-main/Cargo.toml
    - goose-main/CUSTOM_DISTROS.md
- goose-main
  - External MCP directory includes broad third-party connector options (including Rube MCP for multi-app integrations).
  - File: goose-main/documentation/static/servers.json

### Missing or Partial

1. No first-class Google Sheets/Docs/Forms integration implemented in this repository.
2. No first-class Twitter/X API connector implemented in this repository.
3. No end-to-end document conversion pipeline implemented in this repository.
4. No dedicated admin policy vault taxonomy seeded by onboarding flow (before this change).
5. No unified employee happiness feedback form template and processing contract in admin vault.

## Gap Impact

- Policy governance remained fragmented across docs and runtime state.
- Staff operations had no canonical admin module artifact structure.
- Integration intent existed, but there was no executable starter scaffold for operations teams.
- Director Office decision support lacked explicit channel intelligence and evaluation templates.

## Remediation Implemented in This Iteration

1. Added onboarding-time vault scaffold for Administration module artifacts:
   - policy folder taxonomy with atomic markdown files
   - staff registry CSV seed
   - attendance and holiday CSV templates
   - integration config and Google Sheets mapping templates
   - feedback and meeting-note templates
   - channel intelligence config (mail + social trend ingestion)
2. Added routing-map entries so administration assets are discoverable through existing vault routing contracts.
3. Added module and rollout docs for phased implementation and governance.

## Residual Gaps (Updated 2026-03-23)

### Resolved in This Iteration

1. ~~External connectors are template-configured but not yet wired to runtime API clients.~~
   - **Resolved**: `googleBridgeService.ts` provides protocol-interface-driven adapters for Google Sheets, Forms, and Docs/Drive. File-backed fallbacks work out of the box; live gateways activate when credentials are configured.
2. ~~Document conversion is represented as planned pipeline metadata; conversion services and tests are still pending.~~
   - **Previously resolved**: `documentConversionService.ts` implements full Markdown Γåö HTML Γåö DOCX pipeline with mammoth, turndown, marked, and html-to-docx.
3. ~~Google Forms ingestion handling and feedback scoring automation are still to be implemented.~~
   - **Resolved**: `feedbackSentimentService.ts` performs keyword-based sentiment analysis on feedback free-text fields with team aggregation and policy improvement suggestions.
4. **New**: `policyOrchestratorService.ts` implements recursive staff-vs-policy compliance evaluation with violation detection and improvement suggestion generation.
5. **New**: `meetingNoteActionItemService.ts` extracts structured action items from meeting note markdown with owner/due-date parsing and policy impact classification.
6. **New**: Policy registry sub-folder (`src/core/registry/policies/`) created with `administration-policy-schema.json` and `policy-areas.json`.

### Remaining Infrastructure Gaps

1. Live Google API adapters in `googleBridgeService.ts` require the `googleapis` npm package and production OAuth credentials. Currently placeholders that log and fall back to file-backed mode.
2. Social trend ingestion remains policy-modeled and read-only by configuration; no production Twitter/X API adapter is implemented.
3. New services (`policyOrchestratorService`, `feedbackSentimentService`, `meetingNoteActionItemService`, `googleBridgeService`) are not yet registered in `ipcService.ts` for renderer access.

## Recommended Next Steps

1. Install `googleapis` npm package and wire live Google API adapters in `googleBridgeService.ts`.
2. Register new administration services in `ipcService.ts` for frontend access.
3. Implement read-only social trend fetch pipeline with production Twitter/X adapter.
4. Add cron-scheduled weekly evaluator combining KPI + happiness + sentiment + compliance outputs.

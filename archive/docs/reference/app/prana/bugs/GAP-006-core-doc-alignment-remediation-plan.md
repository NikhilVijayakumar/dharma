# GAP-006 Remediation Plan: Core Docs Alignment (Full src)

## Objectives
1. Eliminate hardcoded user-facing strings from JSX in priority modules.
2. Reduce hardcoded pixel literals in feature UIs and use token/system spacing.
3. Move complex page orchestration into ViewModels where required.
4. Keep build green after each batch (`npm run typecheck`).

## Phases

### Phase 1 (Critical, immediate)
- Localize newly introduced lifecycle and dashboard/profile screens.
- Files:
  - `src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx`
  - `src/renderer/src/features/dashboard/view/DashboardContainer.tsx`
  - `src/renderer/src/features/employee-profiles/view/EmployeeProfileView.tsx`
  - `src/renderer/src/common/state/LifecycleProvider.tsx` (error/fallback copy)
  - `src/renderer/src/localization/i18n.ts` (add keys)
- Acceptance:
  - No hardcoded user-facing strings in these files.
  - Typecheck passes.

### Phase 2 (High)
- Localize operational monitor pages with repeated uppercase literals.
- Target groups:
  - Compliance, Design Audit, Infrastructure, Funding Digest, Hiring Sim, Daily Brief, Notification Centre, Queue Monitor, Governance.
- Acceptance:
  - Replace fixed labels/buttons with i18n keys.
  - Typecheck passes.

### Phase 3 (Medium)
- Theming/token cleanup for high-hit px files.
- Convert obvious px literals (`'2px'`, `'4px'`, `'8px'`, `'12px'`) to spacing/token expressions where practical.
- Acceptance:
  - Significant reduction of px literals in top-10 hit files.
  - No visual regressions in core screens.

### Phase 4 (Medium)
- MVVM refactor for heavy settings pages.
- Extract viewmodel hooks:
  - `useAgentLifecycleManagerViewModel`
  - `useProviderSettingsViewModel`
  - `useRegistryViewerViewModel`
- Acceptance:
  - Page components mostly declarative rendering + event delegation.
  - Async and orchestration logic moved to hooks.

## Execution Order
1. Phase 1 now.
2. Typecheck.
3. Phase 2 and 3 in bounded batches.
4. Phase 4 refactor after string/style stabilization.

## Validation Gates
- `npm run typecheck`
- Manual spot QA of edited screens.
- Re-run alignment scans and update `GAP-006-core-doc-alignment-src-audit.md` counts.

## Status
- [x] Audit documented.
- [x] Plan documented.
- [x] Phase 1 complete.
- [x] Phase 2 complete.
- [ ] Phase 3 complete.
- [ ] Phase 4 complete.

## Phase 2 Progress
- Localized high-hit operational screens:
  - `src/renderer/src/features/compliance/view/ComplianceView.tsx`
  - `src/renderer/src/features/design-audit/view/DesignAuditView.tsx`
  - `src/renderer/src/features/funding-digest/view/FundingDigestView.tsx`
  - `src/renderer/src/features/hiring-sim/view/HiringSimView.tsx`
  - `src/renderer/src/features/daily-brief/view/DailyBriefView.tsx`
  - `src/renderer/src/features/notification-centre/view/NotificationView.tsx`
- Localized additional high-priority screens:
  - `src/renderer/src/features/governance/view/GovernanceView.tsx`
  - `src/renderer/src/features/infrastructure/view/InfrastructureView.tsx`
  - `src/renderer/src/features/settings/pages/ProviderSettingsPage.tsx`
- Added corresponding translation keys in `src/renderer/src/localization/i18n.ts`.
- Validation: `npm run typecheck` PASS.

## Phase 3 Progress
- Batch 1 token/spacing substitutions completed in high-hit files:
  - `src/renderer/src/features/queue-monitor/view/QueueMonitorView.tsx`
  - `src/renderer/src/features/governance/view/GovernanceView.tsx`
  - `src/renderer/src/features/daily-brief/view/DailyBriefView.tsx`
- Applied safe replacements for obvious px literals (`2px`, `4px`, `8px`, `12px`) to spacing tokens where practical.
- Validation: `npm run typecheck` PASS.
- Batch 2 token/spacing substitutions completed in next high-hit files:
  - `src/renderer/src/features/vault-knowledge/view/VaultKnowledgeView.tsx`
  - `src/renderer/src/features/weekly-review/view/WeeklyReviewView.tsx`
  - `src/renderer/src/features/notification-centre/view/NotificationView.tsx`
  - `src/renderer/src/features/funding-digest/view/FundingDigestView.tsx`
- Applied additional safe replacements for border radius and micro spacing literals to spacing tokens/system values.
- Validation: `npm run typecheck` PASS.
- Batch 3 token/spacing substitutions completed in next high-hit files:
  - `src/renderer/src/features/suites/view/SuiteView.tsx`
  - `src/renderer/src/features/vault/view/VaultView.tsx`
  - `src/renderer/src/features/triage/view/TriageView.tsx`
  - `src/renderer/src/features/settings/view/SettingsView.tsx`
- Applied additional safe spacing substitutions and tokenized repeated border radius and micro-spacing literals.
- Validation: `npm run typecheck` PASS.

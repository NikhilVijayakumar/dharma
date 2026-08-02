# GAP-006: Core Docs Alignment Audit (Full src)

## Scope
- Audited `src/` with emphasis on `src/renderer/src` against:
  - `docs/core/localization.md`
  - `docs/core/theming.md`
  - `docs/core/mvvm-clean-architecture.md`
  - `docs/core/state.md`
  - `docs/core/hooks.md`

## Summary
- TypeScript compile status: PASS (`npm run typecheck`)
- Core-doc alignment status: NOT FULLY ALIGNED
- Primary gap classes:
  1. Hardcoded UI strings in JSX (localization rule violation)
  2. Hardcoded pixel values and inline style literals (token/theming rule drift)
  3. Stateful orchestration in `pages/containers` instead of ViewModels (MVVM drift)

## Findings

### 1) Localization Rule Violations (High)
Rule: all user-facing JSX text should use `literal['...']` keys.

Detection summary:
- Approximate hardcoded-text hits: **65**
- Top files by hit count:
  - 19: `src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx`
  - 9: `src/renderer/src/features/settings/pages/ProviderSettingsPage.tsx`
  - 7: `src/renderer/src/features/dashboard/view/DashboardContainer.tsx`
  - 5: `src/renderer/src/features/design-audit/view/DesignAuditView.tsx`
  - 4: `src/renderer/src/features/funding-digest/view/FundingDigestView.tsx`
  - 4: `src/renderer/src/features/hiring-sim/view/HiringSimView.tsx`
  - 3: `src/renderer/src/features/compliance/view/ComplianceView.tsx`
  - 3: `src/renderer/src/features/daily-brief/view/DailyBriefView.tsx`

Representative examples:
- `src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx`
- `src/renderer/src/features/dashboard/view/DashboardContainer.tsx`
- `src/renderer/src/features/file-viewers/FileViewerRouter.tsx`

### 2) Theming/Token Drift (Medium)
Rule: avoid hardcoded px values and prefer tokenized values/system spacing.

Detection summary:
- Approximate px-literal hits: **302**
- Top files by hit count:
  - 21: `src/renderer/src/features/queue-monitor/view/QueueMonitorView.tsx`
  - 16: `src/renderer/src/features/vault-knowledge/view/VaultKnowledgeView.tsx`
  - 15: `src/renderer/src/features/governance/view/GovernanceView.tsx`
  - 14: `src/renderer/src/features/daily-brief/view/DailyBriefView.tsx`
  - 13: `src/renderer/src/features/weekly-review/view/WeeklyReviewView.tsx`
  - 12: `src/renderer/src/features/notification-centre/view/NotificationView.tsx`
  - 12: `src/renderer/src/features/funding-digest/view/FundingDigestView.tsx`

Notes:
- Token files (`theme/tokens/*`) legitimately include px comments/definitions.
- Priority is UI feature files first.

### 3) MVVM/ViewModel Drift (Medium)
Rule: complex stateful orchestration should live in ViewModels; views/pages should stay presentation-centric.

Detection summary (state hooks inside non-viewmodel pages/containers):
- 7 hooks: `src/renderer/src/features/settings/pages/ProviderSettingsPage.tsx`
- 6 hooks: `src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx`
- 5 hooks: `src/renderer/src/features/settings/pages/RegistryViewerPage.tsx`

State-model mismatch example:
- `src/renderer/src/common/state/LifecycleProvider.tsx` manages async state manually; core docs recommend AppState/useDataState patterns for consistency.

## Risk
- UX language inconsistency and non-localizable UI.
- Increased maintenance cost due to style/value drift.
- Logic spread across pages slows testability and future refactors.

## Immediate Recommendation
- Execute phased remediation plan (see `docs/bugs/GAP-006-core-doc-alignment-remediation-plan.md`) and land changes in batches with typecheck after each batch.

## Progress Update
- Phase 1 completed:
  - Localized lifecycle manager, dashboard quick-edit, and employee profile edit labels/messages.
  - Added supporting i18n keys in `src/renderer/src/localization/i18n.ts`.
  - Validation: `npm run typecheck` PASS.
- Phase 2 batch (in progress) completed for high-hit operational screens:
  - Compliance, Design Audit, Funding Digest, Hiring Simulation, Daily Brief, Notification Centre.
  - Added translation coverage in `src/renderer/src/localization/i18n.ts`.
  - Validation: `npm run typecheck` PASS.
- Phase 2 follow-up batch completed for remaining high-priority screens:
  - Governance, Infrastructure Layers, Provider Settings.
  - Added translation coverage in `src/renderer/src/localization/i18n.ts`.
  - Validation: `npm run typecheck` PASS.
- Phase 3 batch 1 token cleanup completed:
  - Queue Monitor, Governance, Daily Brief.
  - Replaced safe high-frequency px literals with token/system spacing expressions.
  - Validation: `npm run typecheck` PASS.
- Phase 3 batch 2 token cleanup completed:
  - Vault Knowledge, Weekly Review, Notification Centre, Funding Digest.
  - Replaced additional safe border radius and micro-spacing literals with spacing tokens/system values.
  - Validation: `npm run typecheck` PASS.
- Phase 3 batch 3 token cleanup completed:
  - Suites, Vault, Triage, Settings view.
  - Replaced additional safe border radius and micro-spacing literals with spacing tokens/system values.
  - Validation: `npm run typecheck` PASS.

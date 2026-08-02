# DHI Implementation Violation Audit (2026-03-25)

## Scope
- Repository: Dhi app (`e:/Python/dhi`)
- Validation method: `npm run typecheck` from repository root
- Objective: detect implementation violations against current split documentation baseline

## Critical Findings

### 1. Build is currently broken due to API contract drift with dependency packages
Severity: Critical

Evidence:
- `src/renderer/src/features/daily-brief/view/DailyBriefContainer.tsx:5`
- `src/renderer/src/features/daily-brief/view/DailyBriefView.tsx:6`
- `src/renderer/src/features/dashboard/view/DashboardContainer.tsx:18`
- `src/renderer/src/features/employee-profiles/view/EmployeeProfileView.tsx:5`
- `src/renderer/src/features/employee-profiles/viewmodel/useProfileViewModel.ts:2`
- `src/renderer/src/features/executive-suites/view/ExecutiveSuitesContainer.tsx:3`
- `src/renderer/src/features/suites/repo/SuiteRepo.ts:3`
- `src/renderer/src/features/suites/view/SuiteView.tsx:6`

Symptoms from typecheck:
- Missing named exports from `astra` (e.g. `useOnboardingActionGate`, `OnboardingActionGate`, `useLifecycle`, `SkillEntry`, `SkillRepo`, `DynamicProfileRenderer`, `LifecycleGlobalSkill`, `LifecycleProfileDraft`, `getEmployeeAvatarPath`).

Impact:
- Documented Dhi feature set cannot be considered implementation-complete while compile contracts are failing.

Required Fix Direction:
- Align Dhi feature imports with the currently shipped `astra` package API.
- Or pin/upgrade `astra` dependency to a version that exports the required symbols.

### 2. Transitive dependency breakage from Prana surface leaks into Dhi build
Severity: High

Evidence:
- `node_modules/prana/src/ui/authentication/view/AccessDeniedView.tsx:9`
- `node_modules/prana/src/ui/onboarding/view/OnboardingView.tsx:22`
- `node_modules/prana/src/ui/viewer-pdf/view/ViewerPdfContainer.tsx:2`

Symptoms:
- Missing module path exports for `@astra/theme/tokens/spacing`, `@astra/components/ui/HeroSection`, and related subpaths.

Impact:
- Dhi compile reliability depends on Prana/Astra compatibility and currently fails.

Required Fix Direction:
- Ensure Prana imports only stable Astra public exports available to consumers.
- Avoid private/internal subpath assumptions unless explicitly exported by Astra package.

## Summary
- Typecheck status: Failed
- Total errors observed in run: 42 errors across 32 files
- Conclusion: Dhi currently violates implementation-readiness expectation despite documentation split completion.

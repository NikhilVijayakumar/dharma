# GAP-011 Localization And Hardcoded UI Strings

Status: Open
Severity: High
Category: UX consistency and i18n compliance
Scope: Onboarding and placeholder-heavy module screens

## Summary
Several screens still ship hardcoded English labels, placeholders, and helper copy instead of using the localization map. This violates localization-ready UX expectations and increases regression risk for copy updates.

## Evidence
Hardcoded onboarding strings in active UI:
- [src/renderer/src/features/onboarding-model-configuration/view/OnboardingModelConfigurationContainer.tsx](src/renderer/src/features/onboarding-model-configuration/view/OnboardingModelConfigurationContainer.tsx#L9)
- [src/renderer/src/features/onboarding-channel-configuration/view/OnboardingChannelConfigurationContainer.tsx](src/renderer/src/features/onboarding-channel-configuration/view/OnboardingChannelConfigurationContainer.tsx#L9)
- [src/renderer/src/features/onboarding-registry-approval/view/OnboardingRegistryApprovalContainer.tsx](src/renderer/src/features/onboarding-registry-approval/view/OnboardingRegistryApprovalContainer.tsx#L9)

Hardcoded placeholder copy in scaffold routes:
- [src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx](src/renderer/src/features/registry-agent-editor/view/RegistryAgentEditorContainer.tsx#L8)

## Contract References
- [docs/references/general.md](docs/references/general.md#L66) emphasizes multi-language readiness and no clipping behavior under text expansion.
- [docs/rules/Design Quality Checklist.md](docs/rules/Design Quality Checklist.md#L1) requires deliberate quality control, including readability and consistency.

## Impact
- Copy changes require code edits instead of locale updates.
- Non-English localization cannot be validated for these screens.
- UX wording diverges across modules because string governance is bypassed.

## Required Fix Plan
1. Move all user-visible strings in affected screens to localization keys.
2. Add lint rule to block literal strings in JSX text and input labels except approved constants.
3. Add i18n test snapshot for onboarding and registry shell routes.
4. Add translator audit checklist to release gate.

## Acceptance Criteria
- Affected screens contain no hardcoded UI labels, placeholders, or button text.
- All strings resolve through localization map.
- Non-default locale smoke test passes for onboarding and registry routes.

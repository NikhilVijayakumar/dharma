# Design & UI Gaps: External Integrations

## Onboarding and Settings UI

**Issue**: Missing Google Workspace & MCP specific configuration fields in `OnboardingChannelConfigurationContainer.tsx`.
**Details**: Currently, the UI is hard-coded to support Telegram (e.g., `telegramChannelId`, `webhookSubscriptionUri`). It lacks unified dynamic fields to configure Google Workspace paths (OAuth paths/Service account details) and general MCP sever connection strings.

**Issue**: Lack of Connection Validation (`Test Connection` button).
**Details**: The user is currently only able to input credentials and click "Save". There is no immediate visual feedback verifying that the Telegram API token is valid or that Google Auth succeeds. Users need a "Test Connection" button that returns a status indicator (≡ƒƒó Connected, ≡ƒö┤ Error) before saving.

**Issue**: Onboarding SetupWizard gating logic bypass.
**Details**: Currently, the SetupWizard phase containing Channel Setup does not require the connection to be "Tested Successful" to enable the "Next" button. This breaks the mandate that mandatory channels must pass verification before leaving the onboarding flow.

**Status**: Identified and pending implementation in Phase 3.

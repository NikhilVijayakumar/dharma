# Onboarding: Model Configuration - Infrastructure Stage Specification

## 1. Single Reason to Change (SRP)
This module governs model endpoint approval as part of Step 4 (Infrastructure & Access) in onboarding.

## 2. Input Data Required
- Endpoint URIs and model names for each provider.
- API keys/tokens (runtime-local and excluded from Vault payload).
- Provider enablement flags.

## 3. Pipeline Dependency
- This step unlocks only after Company Core, Global Assets, Agent Deep-Dive, and Channel Access are approved.
- Final master commit is blocked until model config step is approved.

## 4. Validation
- At least one enabled provider with non-empty endpoint and model is required.
- Provider config approval is explicit (`APPROVED`) and independent from draft state.

## 5. Storage Rules
- Model credentials remain runtime-local and are excluded from final Vault onboarding payload.
- Approval status is persisted with onboarding state to satisfy dependency checks.

## 6. Chat Scenarios
- Internal chat can receive validation logs for failed provider checks.
- No external channel dispatch is required for endpoint setup.

## 7. Navigation Guarantee
- Users can navigate back to Global Assets from this stage without losing in-progress drafts.

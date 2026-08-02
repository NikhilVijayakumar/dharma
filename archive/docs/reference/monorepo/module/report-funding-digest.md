# Analytics: Funding Digest - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to extracting, formatting, and delivering the Investor/Funding oriented intelligence dispatch. It sanitizes internal operations data into an external format.

## 2. Input Data Required
- **Financial Telemetry:** Runway coverage, burn rate variance, conversion correlations.

## 3. Registry Sub-Component Integration
- **Agents:** `julia` (finance).
- **Skills:** `pii-detection-logic`, `financial-forecasting`.
- **Workflows:** Invokes the delivery webhook graph.
- **Protocols:** Strictly enforces `privacy-by-design-protocol` to strip internal secrets.
- **KPIs:** Includes Investor-centric KPIs.
- **Data Inputs:** Financial schema inputs.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** The paramount security gate. Blocks the digest execution if any classified internal PII or secret is detected in the outgoing payload.
- **Goose:** Translates complex JSON charts into narrative investor summaries.
- **NemoClaw:** The Export/Preview UI button.

## 5. Hybrid DB & State Storage Flow
- **Generation:** Pulled from SQLite and Vault.
- **No Commitment:** The output digest is sent externally. It is *not* saved to the Vault memory to save disk space, though a tiny audit log `digest_sent: true` is committed to SQLite.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Pre-flight review requested from the human Admin before dispatch.
- **External Chat:** Final delivery is pushed explicitly via the external Telegram hook / Webhook channel configured during Phase 3 onboarding.

## 7. Cron & Queue Management
- **Cron Job:** Configured to push on the 1st of every month automatically *subject to approval*.
- **Failover / Catch-up Mechanic:** If offline, the queue detects a missed digest window. Since it requires human-in-the-loop review, it does *not* auto-fire; instead, it generates a persistent UI banner reminding the user that a digest is deeply overdue.

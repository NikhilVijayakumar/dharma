# Analytics: Weekly Review - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the high-level compilation of the strategic "Weekly Review" payload. It fundamentally differs from the Daily Brief as it measures week-over-week strategic drift.

## 2. Input Data Required
- **Time Window:** Monday to Sunday.
- **Dependencies:** Requires the 7 prior Daily Brief objects to exist in the Vault.

## 3. Registry Sub-Component Integration
- **Agents:** `arya` (strategic evaluator).
- **Skills:** `strategic-alignment`.
- **Workflows:** None.
- **Protocols:** None.
- **KPIs:** Compares week-over-week delta variants.
- **Data Inputs:** Vault Daily Briefs.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Identifies contradictory assertions across the 7 briefs.
- **Goose:** Concatenates and compresses the narratives into a high-level executive payload.
- **NemoClaw:** The Dashboard viewer.

## 5. Hybrid DB & State Storage Flow
- **Source:** Pulled directly from the **Vault** (reading previous commitments).
- **Target:** The new Weekly payload is appended to the **Vault Knowledge Repository**.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Broadcast to the `#Executive` channel.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** Runs Sunday at 23:59.
- **Failover / Catch-up Mechanic:** If offline during Sunday midnight, upon boot the system identifies a missing weekly boundary. The generation is shifted to the foreground queue to execute immediately.

# Analytics: KPI Breach Alerting - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the routing, escalation, and notification actions dispatched the exact moment a KPI mathematically crosses a `critical` or `warning` threshold.

## 2. Input Data Required
- **Breach Payload:** KPI Name, Delta Variance, Timestamp.

## 3. Registry Sub-Component Integration
- **Agents:** Wakes the designated `responsible_agent_role` defined in the KPI artifact.
- **Skills:** `governance-enforcement`.
- **Workflows:** Triggers the specific `escalation_policy` workflow (e.g., `trigger-stability-mode-protocol`).
- **Protocols:** Must pass external comms protocols.
- **KPIs:** The source of the trigger.
- **Data Inputs:** None directly.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Checks global throttling limits to prevent "alarm storms" (e.g., suppressing duplicate alerts if a KPI breaches 50 times in 1 minute).
- **Goose:** Extracts the mathematical error cluster into a human-readable alert sentence.
- **NemoClaw:** Controls UI toasts and flashing sidebar badges.

## 5. Hybrid DB & State Storage Flow
- **Incident Creation:** Creates an `active_incident` log in SQLite to track acknowledgment.
- **Commit:** The breach event creates a non-repudiable audit footprint in the **Vault**.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Dispatches a loud @here message containing the Goose summary to the primary incident response channel.
- **External Chat:** If the KPI SLA mandates a `critical` escalation, this feature hooks directly to the Telegram/Webhook configuration (from Phase 3 Onboarding) and fires a catastrophic payload structure to external admins immediately.

## 7. Cron & Queue Management
- **Queue Interactivity:** Webhook firings are pushed to a background task queue with automated exponential backoff retries if the Telegram network is unreachable.

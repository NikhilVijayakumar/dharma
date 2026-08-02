# Triage: Incident Resolution - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to executing the logic required to repair, close, or escalate a previously routed incident. 

## 2. Input Data Required
- **Investigation Artifacts:** Log selections, generated fixes, or user responses.

## 3. Registry Sub-Component Integration
- **Agents:** The fixer.
- **Skills:** Autonomous debugging.
- **Workflows:** Fix deployments.
- **Protocols:** `incident-escalation-protocol`.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Evaluates a proposed fix to ensure it doesn't violate governance rules.
- **Goose:** Extracts the final solution summary into an After-Action-Report narrative.
- **NemoClaw:** The resolution UI (code diffs, confirm buttons).

## 5. Hybrid DB & State Storage Flow
- **Execution:** Happens dynamically.
- **Commit:** If a system logic change occurs because of the resolution, the new state is committed to the **Vault** and Git. The Goose-generated After Action Report is permanently saved to the `Vault Knowledge Repository`.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** "Incident #123 Resolved by Mira. See diff".
- **External Chat:** None.

## 7. Cron & Queue Management
- **Queue Interactivity:** Once marked resolved, a queue task removes the item from the central visualization list.

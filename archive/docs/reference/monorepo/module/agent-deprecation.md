# Agent Lifecycle: Deprecation - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to retiring, archiving, or destroying a Virtual Employee. It encompasses the hard constraint checks preventing destruction of active entities.

## 2. Input Data Required
- **Target Selection:** The Agent ID to deprecate.
- **Rationale Input:** A mandatory justification string for audit purposes.

## 3. Registry Sub-Component Integration
- **Agents:** The target entity.
- **Skills:** None.
- **Workflows:** Deprecation checks if the agent is the *sole owner* of any critical workflow.
- **Protocols:** Must respect the `audit-trail-integrity-protocol`.
- **KPIs:** Warning if deprecation will drop KPI capacity.
- **Data Inputs:** None.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Executes a massive dependency graph check. If an agent is deprecated, OpenCLAW ensures no workflow will become un-routable. If a failure state is detected, deprecation is physically blocked.
- **Goose:** Not used.
- **NemoClaw:** The hard-confirmation UI anchors (e.g., typing "DEPRECATE" to confirm).

## 5. Hybrid DB & State Storage Flow
- **Execution:** Operations run via memory index in **SQLite**.
- **Final Commitment:** The agent's YAML file is removed from the active system schema, but its historical telemetry and deprecation rationale are permanently etched into the **Vault** as an archaeological record.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** System broadcast to Engineering logic channels indicating an agent has been decommissioned.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Queue Interactivity:** Triggering deprecation spawns a queue worker that systematically revokes any SQLite session tickets, API access tokens, or open thread subscriptions tied to the specific agent ID.

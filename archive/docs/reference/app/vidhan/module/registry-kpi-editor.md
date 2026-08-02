# Registry: KPI Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to creating or modifying Key Performance Indicators (the mathematical formulas, frequencies, and alert thresholds governing the system).

## 2. Input Data Required
- **KPI Metadata:** Name, Description.
- **Formulas:** Mathematical equations referencing specific Data Inputs.
- **Thresholds:** Critical, Warning, Optimal numerical bounds.
- **Schedules:** `frequency_of_check`.

## 3. Registry Sub-Component Integration
- **Agents:** Referenced as the "Responsible Agent" acting on failures.
- **Skills:** Not applicable.
- **Workflows:** Referenced for `escalation_policy` hooks.
- **Protocols:** Not applicable.
- **KPIs:** This is the core artifact being modified.
- **Data Inputs:** Heavily referenced as the raw variables utilized inside the formulas.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Parses the mathematical formula string to ensure syntax correctness against the selected Data Inputs.
- **Goose:** Translates business logic descriptions ("Alert when conversion drops 5%") into the structured threshold parameters.
- **NemoClaw:** Binds the complex formula input bounding boxes and variable autocomplete chips.

## 5. Hybrid DB & State Storage Flow
- **Drafting:** Stored safely in **SQLite DB**.
- **Approval Queue:** Sent to human review to prevent erroneous alert storms from bad threshold math.
- **Execution State:** Upon approval, it writes into the **Vault** and Git.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Creating a KPI does not trigger a chat interaction (unless an approval is required). 
- **External Chat:** The KPI Editor defines the bounds that *will* eventually trigger external Telegram alerts if a metric fails, but the editor itself operates silently.

## 7. Cron & Queue Management
- **Cron Configuration:** The KPI Editor directly writes the intervals (e.g., `frequency_of_check: daily`) which dynamically spawn new Cron validation jobs in the system core.
- **Failover / Catch-up Mechanic:** If the system is offline when a KPI check is due, upon startup the `kpi_last_eval` SQLite timestamp is checked. If the SLA is missed, an immediate check is pushed to the Queue Broker instantly before resuming regular cron scheduling.

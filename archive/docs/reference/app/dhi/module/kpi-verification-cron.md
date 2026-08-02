# Analytics: KPI Verification Engine - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the background mathematical engine that evaluates KPI health formulas against defined thresholds (`frequency_of_check`). It does NOT handle what happens *after* a breach (that belongs to alerting).

## 2. Input Data Required
- **Current Metric Values:** The literal numerical data returned by SQLite `data-inputs`.
- **Thresholds:** The boundaries from the Registry KPI YAML.

## 3. Registry Sub-Component Integration
- **Agents:** None involved directly in mathematics.
- **Skills:** None.
- **Workflows:** None.
- **Protocols:** None.
- **KPIs:** Reads the core objects continuously.
- **Data Inputs:** Uses these to satisfy the formula variables.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Executes the algebraic translation of the formula (e.g., parsing `(failed/total)*100` dynamically) and evaluates standard deviation limits safely.
- **Goose:** Not used.
- **NemoClaw:** Not used.

## 5. Hybrid DB & State Storage Flow
- **Read:** Evaluates numbers instantly from the **SQLite** buffer pool.
- **Write:** Updates the `current_kpi_value` cache column in **SQLite**. Does not commit historical values to the Vault directly.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** Spawns a dedicated micro-interval cron loop evaluating hundreds of KPIs asynchronously based on their `frequency_of_check` values (some real-time, some hourly).
- **Failover / Catch-up Mechanic:** On boot, checks `kpi_last_eval` for all KPIs. Any KPI missing its SLA window is immediately pushed to the top of the evaluation queue before normal polling begins to ensure no latent alarms were missed during downtime.

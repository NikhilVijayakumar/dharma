# Interface: Virtual Employee Dashboard - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the read-only, real-time visualization of a specific Agent's operational status (Live tasks, utilization rates, assigned SLAs).

## 2. Input Data Required
- **Target Selection:** The Agent ID.

## 3. Registry Sub-Component Integration
- **Agents:** The subject being observed.
- **Skills:** Displays active capabilities.
- **Workflows:** Displays the active queue stream.
- **Protocols:** None.
- **KPIs:** Visually binds the agent's SLA metrics onto a chart matrix.
- **Data Inputs:** Agent telemetry logs.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Provides dynamic 'Health Scoring' for the agent based on error frequencies.
- **Goose:** Not used.
- **NemoClaw:** The interactive grid anchoring live chart data and task queues.

## 5. Hybrid DB & State Storage Flow
- **Data Source:** Streams via websocket polling directly from the **SQLite** operational telemetry tables. It never queries the Vault because Vault data is historical and the Dashboard requires instant, ephemeral, live logic tracking.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Includes a "Ping Agent" quick-action button transitioning the UI into the Internal Chat view.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** None.

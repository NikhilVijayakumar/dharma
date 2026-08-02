# Agent Lifecycle: Performance Review - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the automated performance evaluations of a Virtual Employee against its assigned KPIs.

## 2. Input Data Required
- **Time Bounds:** Start date, End date.
- **Evaluation Criteria:** Targeted KPIs.

## 3. Registry Sub-Component Integration
- **Agents:** The subject (`eva` performs reviews on `mira`, etc).
- **Skills:** `strategic-alignment`, `behavioral-alignment-audit`.
- **Workflows:** Aggregates logs.
- **Protocols:** None.
- **KPIs:** Heavily utilized to calculate the grade.
- **Data Inputs:** Historical operational logs.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Scores the raw metric math against the acceptable thresholds.
- **Goose:** Harvests the massive raw reasoning traces from the month and structures a natural-language "Performance Summary / Areas of Improvement" paragraph for the human manager.
- **NemoClaw:** Renders the 360-degree review dashboard panel.

## 5. Hybrid DB & State Storage Flow
- **Processing:** Raw traces are read dynamically from **SQLite**.
- **Draft Review:** The Goose-generated review draft is saved back to **SQLite**.
- **Final Commitment:** Once the human manager signs off, the performance ticket acts as a permanent historical record and goes to **Vault**.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** The completed performance review is DM'd to the responsible Human Manager for required countersignature.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** Configured to automatically generate a skeletal review payload on the 1st of every month at `00:00`.
- **Failover / Catch-up Mechanic:** If the system is offline, the SQLite initialization detects `missed_monthly_reviews = true` and dispatches all agents to generate retrospective reviews immediately upon the engine starting, throttling the queue so as to not block priority live tasks.

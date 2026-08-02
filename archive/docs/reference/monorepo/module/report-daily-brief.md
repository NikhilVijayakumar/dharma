# Analytics: Daily Brief - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the automated aggregation, extraction, and generation of the 24-hour tactical "Daily Brief" report payload. It does not handle weekly logic or external funding digests.

## 2. Input Data Required
- **Time Window:** Midnight `T-1` to Midnight `T-0`.
- **Entity Scope:** All active task telemetry logs in SQLite for that window.

## 3. Registry Sub-Component Integration
- **Agents:** `mira` (aggregator).
- **Skills:** `traction-evidence-synthesis`, `narrative-fidelity-mapping`.
- **Workflows:** Not directly executed as a logic graph; handled via cron.
- **Protocols:** Must respect data visibility tiers.
- **KPIs:** Includes daily metric variances.
- **Data Inputs:** Operational SQLite logs.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Suppresses low-priority logs entirely to keep the brief tactical.
- **Goose:** Parses thousands of JSON execution logs and extracts them into a cohesive narrative Markdown text payload.
- **NemoClaw:** Renders the Brief Viewer UI (Markdown rendering block).

## 5. Hybrid DB & State Storage Flow
- **Generation:** Pulled and generated in memory from **SQLite**.
- **Commit:** The finalized Brief is saved directly to the **Vault Knowledge Repository** as a fixed daily record.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** DM'd dynamically to all users via standard notification queues at 08:00 AM.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** Runs at 06:00 AM server-time.
- **Failover / Catch-up Mechanic:** If the system is offline at 06:00 AM, upon boot, the `brief_last_generated` timestamp is evaluated. An immediate queue job is fired to compile the Brief retroactively before normal ops resume.

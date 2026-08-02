# Registry: Data Input Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to configuring Data Inputs (schemas defining event streams, privacy tiers, source endpoints, and retention policies).

## 2. Input Data Required
- **Source Configuration:** `source_type` (Kafka, CSV, API), `source_location`.
- **Validation Rules:** The exact fields required in the incoming payloads.
- **Data Governance:** Privacy classification and retention TTLS.

## 3. Registry Sub-Component Integration
- **Agents:** Not applicable.
- **Skills:** Not applicable.
- **Workflows:** Referenced by workflows as triggers.
- **Protocols:** Data inputs are strictly evaluated against privacy protocols.
- **KPIs:** KPIs consume data inputs.
- **Data Inputs:** This is the core artifact.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Dry-runs the requested endpoint `source_location` to verify cross-origin access and format correctness.
- **Goose:** Ingests a raw sample JSON object provided by the user and reverse-engineers a strict validation schema automatically.
- **NemoClaw:** Handles the Data Sample UI paste block and the subsequent generated schema mappings.

## 5. Hybrid DB & State Storage Flow
- **Drafting:** Configured entirely in SQLite.
- **Validation:** OpenCLAW validates connectivity.
- **Commit:** Once approved by the data architecture team, the Input schema is committed to Vault and pushed to Git.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** A payload schema validation report is dispatched for operator review.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Configuration:** The editor configures the `update_frequency` (e.g., hourly pulls from an API batch). This translates directly to a cron hook.
- **Failover / Catch-up Mechanic:** If the system is offline and misses a scheduled batch pull, upon next startup the sqlite log confirms exactly which chronological slices are missing. It fires an immediate queue sequence to pull all missing historical batches sequentially before resuming the live polling rate.

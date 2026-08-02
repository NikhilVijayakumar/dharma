# Setup & Config: Application Rules - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to editing global, system-wide constraints such as financial burn-rate caps, global data retention limits, and max concurrency caps. 

## 2. Input Data Required
- **Global Constraints:** `max_monthly_budget`, `audit_log_ttl_days`, `queue_concurrency`.

## 3. Registry Sub-Component Integration
- **Agents:** `eva` (adminises security overrides).
- **Skills:** `governance-enforcement`.
- **Workflows:** Modifying constraints can trigger the `trigger-stability-mode-protocol`.
- **Protocols:** Bounded by `privacy-by-design-protocol` (e.g. Cannot set TTL to infinite).
- **KPIs:** System-wide SLAs are affected.
- **Data Inputs:** None.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** The most critical step. If an Admin sets `max_monthly_budget` to $10M when the company only has a $1M runway, OpenCLAW halts the UI action with a deterministic logic failure.
- **Goose:** Not used.
- **NemoClaw:** The Global Settings UI wrapper.

## 5. Hybrid DB & State Storage Flow
- **Drafting:** Changes held in **SQLite**.
- **Execution Gate:** Requires Human-in-the-Loop multi-party authentication.
- **Commit:** Written permanently to the **Vault** as the definitive operational bounds document.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Loud broadcast to Operations chat: "Admin changed global log retention from 30 days to 90 days."
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** Application Rules often define the boundary variables that crons and queues use (e.g. configuring the queue polling frequency).

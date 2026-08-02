# Governance: Compliance Audit Logger - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the generation, encryption, and storage of chronological action footprints across the system.

## 2. Input Data Required
- **Trace Payload:** User ID, Action, Timestamp, Node ID, Success/Failure state, Payload hash.

## 3. Registry Sub-Component Integration
It monitors all components blindly:
- **Agents:** Yes.
- **Skills:** Yes.
- **Workflows:** Yes.
- **Protocols:** Yes.
- **KPIs:** Yes.
- **Data Inputs:** Yes.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Generates a lightweight SHA-256 hash of the trace payload to ensure the audit log cannot be tampered with mathematically.
- **Goose:** Not used.
- **NemoClaw:** Not used (Headless background worker).

## 5. Hybrid DB & State Storage Flow
- **Buffer:** To prevent disk IO thrashing, audit logs are buffered heavily in **SQLite**.
- **Commit:** Every 60 seconds, the SQLite buffer is flushed and permanently committed to the chronological append-only **Vault Audit Tree** and Git.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Queue Interactivity:** Handled via asynchronous micro-queues.
- **Failover / Catch-up Mechanic:** If the system is offline, the SQLite cache retains the un-flushed audit logs on disk. Upon boot, the initializer specifically checks the `audit_dirty_cache` table. Any unencrypted traces are immediately hashed and flushed to Vault before normal UI interaction unlocks, preserving sequential forensic integrity.

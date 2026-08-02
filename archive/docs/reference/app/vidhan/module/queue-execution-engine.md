# Queue: Execution Engine - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the background task consumption logic. It is the headless engine that pulls tasks, handles concurrency constraints, and routes payloads to agents.

## 2. Input Data Required
- **Task Payload:** JSON blob.
- **Limits:** Max concurrency variables, backoff timers.

## 3. Registry Sub-Component Integration
- **Agents:** Woken up by the queue to execute tasks.
- **Skills:** Fired during execution.
- **Workflows:** Provides the sequential logic.
- **Protocols:** Protocol breaches physically pause the queue execution.
- **KPIs:** `queue-throughput-velocity` is generated directly by this engine.
- **Data Inputs:** Raw triggers.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Enforces concurrency gates. Does the engine have enough memory budget to spawn a task?
- **Goose:** Triggers dynamically to parse unstructed task inputs.
- **NemoClaw:** Strictly absent (Headless background service).

## 5. Hybrid DB & State Storage Flow
- **Processing:** Leases a task lock in **SQLite** for `<timeout_ms>`.
- **Completion:** If successful, deletes task. If failed `< max_retries`, increments retry. If failed permanently, flags as `dead_letter` in SQLite.
- **Vault:** Successful heavy compliance tasks flush audit footprints to **Vault**.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** System errors like "Queue Worker Died" dispatch to admin chats.

## 7. Cron & Queue Management
- **Queue Execution:** This *is* the queue management broker.
- **Failover / Catch-up Mechanic:** Upon startup, the engine immediately runs a sweep for 'zombie' locks in SQLite (tasks grabbed right before a power failure) and clears the locks to resume processing accurately.

# Queue Orchestration

## Overview

Queue orchestration provides durable scheduling and recovery for cron-driven tasks.
Tasks are no longer transient in memory only. They are persisted in SQLite and replayable after restart.

## Components

- `cronSchedulerService`: Responsible for schedule timing, enqueue sweeps, and queue processing.
- `governanceLifecycleQueueStoreService`: SQLite store for task queue and task audit log.

## Flow

1. Scheduler tick identifies due jobs.
2. Due jobs are enqueued into SQLite task queue with source `SCHEDULED`.
3. On startup, scheduler recovers interrupted tasks and enqueues missed due jobs with source `MISSED`.
4. Pending/interrupted tasks are processed in order.
5. Task status transitions are written to queue table and audit table.

## Task Status Lifecycle

- `PENDING` -> task is queued.
- `RUNNING` -> task execution started.
- `COMPLETED` -> execution finished successfully or overlap skipped.
- `FAILED` -> execution failed.
- `INTERRUPTED` -> recovered from previous process crash/stop while running.

## Audit Events

The audit log records queue events such as:

- enqueue
- running
- completed
- failed
- recovery actions
- cron proposal review actions

Use `operations:get-task-audit-log` to retrieve latest events for governance and troubleshooting.

## Recovery Guarantees

- Interrupted running tasks are marked `INTERRUPTED` on initialization.
- Missed due schedules are re-enqueued after restart.
- Processing resumes from persisted queue state.

This design reduces lost automation work during application restarts and supports post-incident traceability.

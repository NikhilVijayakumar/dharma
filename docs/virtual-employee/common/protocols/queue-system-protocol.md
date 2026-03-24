# Queue System Protocol

## Overview
The Global Sequential Task Execution Queue is the backbone of the Dhi virtual employee architecture. It is an **event-driven, subscription-based, sequential queue** that enforces a strict one-task-at-a-time execution model to guarantee thread safety for the Governance Lab (Git) and prevent host system resource exhaustion.

This document defines the queue's internal mechanics: slot structure, subscription model, priority resolution, overflow handling, and failure recovery.

---

## 1. Queue Architecture

### 1.1 Queue Type
**Producer-Consumer Model**
- **Producers**: All Virtual Employees (Arya, Elina, Nora, Lina, Eva, Maya, Dani, Julia, Sofia).
- **Consumer**: The Execution Pipeline (`AgentRuntime` + `ModelGatewayService`) — the only component allowed to invoke LLM calls.
- **Controller**: Managed by `QueueService.ts` in the Electron main process, with Elina providing operational oversight.

### 1.2 Slot Structure
The queue is a **Fixed-Slot Buffer** with a hard maximum, persisted in SQLite (`dhi.db`).

| Slot Range | Purpose | Priority | Access |
|:-----------|:--------|:---------|:-------|
| **Slots 1–8** | Standard processing | Normal | All agents |
| **Slots 9–10** | Crisis Protocol reserved | Critical | Only CFO (Nora), Compliance (Eva), COO (Elina) |

- **Max Queue Length**: **10 tasks**.
- **Concurrent Execution**: **1 task at a time** (strictly sequential).
- **Hard Ceiling**: If all 10 slots are occupied, the queue is **FULL** and agents are blocked from submitting new Work Orders.

### 1.3 Task State Machine
Every Work Order in the queue transitions through these states:

```
PENDING → SLOT_ASSIGNED → IN_PROGRESS → AWAITING_AUDIT → COMPLETE
                                │                           │
                                ▼                           ▼
                            DEGRADED ──→ RESCHEDULED    VIOLATION ──→ REVISION
```

| State | Description |
|:------|:------------|
| `PENDING` | Agent has submitted Work Order, waiting for slot. |
| `SLOT_ASSIGNED` | Queue Controller has allocated a slot (1–10). |
| `IN_PROGRESS` | Agent Runtime is actively executing the task. |
| `AWAITING_AUDIT` | Execution complete, waiting for Eva's `COMPLIANCE_PASS`. |
| `COMPLETE` | Eva passed, result sent to Mira for Director synthesis. |
| `DEGRADED` | Task failed 3+ times. Marked for Elina's review. |
| `RESCHEDULED` | Elina decided to retry later, moved to back of queue. |
| `VIOLATION` | Eva rejected. Corrective feedback sent to originator. |
| `REVISION` | Agent is reworking the task based on Eva's feedback. |

---

## 2. Event-Driven Subscription Model

### 2.1 Why Subscription, Not Polling
Agents do **not** poll the queue. Polling wastes compute cycles and creates race conditions. Instead, the system uses a **Reactive Subscription** model where agents listen for broadcast events emitted by `QueueService.ts` via Electron IPC.

### 2.2 Event Types

| Event | Emitter | Listeners | Description |
|:------|:--------|:----------|:------------|
| `QUEUE_AVAILABLE` | Pipeline (after task completes) | All agents | A slot has opened. Agents with pending work may submit. |
| `SLOT_ASSIGNED` | Queue Controller | Specific agent | Your Work Order was accepted and assigned to slot N. |
| `QUEUE_FULL` | Queue Controller | Submitting agent | No slots available. Work Order is buffered externally. |
| `TASK_COMPLETE` | Queue Controller | Mira | A child task finished. Mira should aggregate results. |
| `DEGRADED_ALERT` | Queue Controller | Elina | A task has failed 3+ times and needs operational review. |
| `CRISIS_ACTIVATED` | Nora / Eva / Elina | All agents | System switching to Stability Mode. Non-critical agents freeze. |
| `CONTEXT_LEVEL` | Lina | All agents | Passive broadcast of Director's current cognitive load state. |
| `RUNWAY_STATUS` | Nora | All agents | Passive broadcast of current financial runway status. |

### 2.3 Subscription Flow
```
1. Pipeline finishes task → emits QUEUE_AVAILABLE
2. All agents with pending Work Orders receive the broadcast
3. Interested agents submit a Work Order (a "Bid")
4. Queue Controller evaluates all bids using Priority Resolution
5. Highest priority bid wins → SLOT_ASSIGNED
6. Remaining bids remain in PENDING state
7. Pipeline begins execution → task moves to IN_PROGRESS
8. Repeat
```

---

## 3. Priority Resolution

When multiple agents bid for the same open slot, the Queue Controller resolves priority using a weighted scoring system.

### 3.1 Priority Score Calculation
```
PRIORITY_SCORE = (URGENCY × 3) + (DIRECTOR_FOCUS × 2) + (DEPENDENCY_CHAIN × 2) + (AGENT_WEIGHT × 1)
```

| Factor | Weight | Description |
|:-------|:-------|:------------|
| `URGENCY` | ×3 | Is this a crisis trigger? (Runway <4mo = 10, Standard = 3) |
| `DIRECTOR_FOCUS` | ×2 | Does this align with the Director's current Briefing focus? |
| `DEPENDENCY_CHAIN` | ×2 | Is another agent blocked waiting for this result? |
| `AGENT_WEIGHT` | ×1 | Base priority by role (Eva=9, Nora=8, Elina=7, Julia=6, ...) |

### 3.2 Tie Breaking
If two agents have identical scores:
1. Agent with the older pending Work Order wins (FIFO).
2. If timestamps match, the agent with the shorter expected execution time wins.

### 3.3 Crisis Override
When `CRISIS_ACTIVATED` is emitted:
- All standard slots (1–8) are **frozen** (no new tasks accepted).
- Only active crisis agents (Nora, Eva, Elina) may submit to slots 9–10.
- When crisis resolves, frozen tasks resume in their original priority order.

---

## 4. Overflow Handling

### 4.1 External Backlog Buffer
When the queue reaches its 10-slot maximum, additional Work Orders are placed into an **External Backlog Buffer** (a holding area managed by `QueueService.ts` in SQLite, not a queue slot).

- **Max Backlog Size**: 20 additional Work Orders.
- **Backlog Behavior**: When a queue slot opens, the Queue Controller first checks the backlog before accepting new bids.
- **Backlog Expiry**: Work Orders in the backlog expire after **24 hours** if not processed. Mira is notified of expiry.

### 4.2 Absolute Overflow
If the backlog itself reaches 20 items (total system load = 30 Work Orders):
1. **Hard Block**: No agent can submit new Work Orders.
2. **Director Alert**: Mira pushes an in-app notification: `"QUEUE_OVERFLOW: 30 pending tasks. System at capacity."`.
3. **Elina Intervention**: Elina's `bottleneck-resolution.md` skill activates to analyze which tasks can be auto-cancelled or merged.

### 4.3 Memory Protection
- If host machine RAM usage exceeds **80%**, `QueueService.ts` stops accepting new Work Orders regardless of slot availability.
- If host CPU exceeds **90%** sustained for >5 minutes, the currently executing task is paused and the system enters a cooldown state.

---

## 5. Failure Recovery

### 5.1 Task Failure
If the Agent Runtime returns an error during execution:
1. **Retry**: The task is retried up to **3 times** with exponential backoff (1s, 5s, 15s).
2. **Degrade**: After 3 failures, the task is marked `DEGRADED` and moved out of the active slot.
3. **Alert**: `DEGRADED_ALERT` event is emitted to Elina for operational review.
4. **Reschedule or Cancel**: Elina decides to reschedule (back of queue) or cancel (with Director notification).

### 5.2 Queue Crash Recovery
If `QueueService.ts` itself crashes:
1. Queue state is persisted to `dhi.db` (SQLite) after every state transition.
2. On restart, the queue replays from `dhi.db` to restore exact state.
3. Any task that was `IN_PROGRESS` at crash time is moved back to `PENDING`.

### 5.3 Deadlock Detection
Elina's `bottleneck-resolution.md` skill continuously monitors for circular dependencies:
- If Agent A is waiting for Agent B, and Agent B is waiting for Agent A → **Deadlock**.
- Resolution: Force-timeout the oldest waiting task, reassign the Work Order, or escalate to Director.

---

## 6. Queue State Persistence

Queue state is stored in SQLite (`dhi.db`). The canonical in-memory representation:

```json
{
  "queue_id": "dhi-queue-001",
  "max_slots": 10,
  "active_task": {
    "slot": 3,
    "agent": "JULIA_CTO",
    "task_id": "TASK-2026-0228-001",
    "state": "IN_PROGRESS",
    "started_at": "2026-02-28T16:30:00Z"
  },
  "slots": [
    { "slot": 1, "state": "COMPLETE", "agent": "NORA_CFO" },
    { "slot": 2, "state": "AWAITING_AUDIT", "agent": "DANI_CMO" },
    { "slot": 3, "state": "IN_PROGRESS", "agent": "JULIA_CTO" },
    { "slot": 4, "state": "PENDING", "agent": "SOFIA_DESIGN" }
  ],
  "backlog": [],
  "crisis_mode": false,
  "system_health": {
    "ram_usage_pct": 62,
    "cpu_usage_pct": 45
  }
}
```

---

## 7. Relevant Agent Assets

| Agent | Relationship to Queue |
|:------|:----------------------|
| **Mira** | `QueueService.ts` routes Work Orders in. Aggregates results out. |
| **Elina** | Monitors queue health via `WorkflowMonitorService.ts`. Resolves deadlocks and overflows. |
| **Eva** | Acts as the `AWAITING_AUDIT` checkpoint. No task reaches `COMPLETE` without her pass. |
| **Lina** | Broadcasts `CONTEXT_LEVEL` as a passive queue state variable (no slot required). |
| **Nora** | Broadcasts `RUNWAY_STATUS` passively. Can trigger `CRISIS_ACTIVATED` event. |

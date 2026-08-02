# Work Order Runtime (Phase C1)

Date: 2026-03-19
Scope: Phase C1 of MVP zero-block plan

## 1. Purpose

Implement the core runtime flow for Director requests:
Director request -> routed work order -> queue -> execution lifecycle.

This is a main-process runtime feature and does not require Clean Architecture migration.

## 2. In Scope

- Work order schema and lifecycle states.
- Queue service with 10-slot capacity and 1-slot crisis reserve.
- Deterministic command router for Director requests.
- IPC and preload endpoints for submit/list/start/complete/fail flows.
- Unit tests for queue and routing lifecycle behavior.

## 3. Out of Scope

- Full 10-agent execution behavior (Phase C2/D).
- Renderer UX for interaction strip and ask-owner widgets.
- Advanced LLM prompt orchestration.

## 4. Runtime Components

- `src/main/services/workOrderService.ts`
- `src/main/services/queueService.ts`
- `src/main/services/commandRouterService.ts`

## 5. IPC Contracts

- `work-orders:submit-director-request`
- `work-orders:start-next`
- `work-orders:complete`
- `work-orders:fail`
- `work-orders:list`
- `work-orders:get`
- `work-orders:queue-list`

## 6. Verification Checklist

- [x] Director request can create and queue a work order.
- [x] Queue reserves one slot for critical work orders.
- [x] Starting next work order respects priority ordering.
- [x] Complete/fail transitions update work order and queue state.
- [x] Type-safe preload bridge is available to renderer.
- [x] Unit tests pass and node typecheck passes.

## 7. Implementation Status

Completed:
- Added runtime services:
	- `src/main/services/workOrderService.ts`
	- `src/main/services/queueService.ts`
	- `src/main/services/commandRouterService.ts`
- Added IPC handlers in `src/main/services/ipcService.ts` for work-order flow.
- Added preload bridge endpoints in `src/preload/index.ts` and typings in `src/preload/index.d.ts`.
- Added tests:
	- `src/main/services/queueService.test.ts`
	- `src/main/services/commandRouterService.test.ts`

Verification evidence:
- `npm run test -- src/main/services/queueService.test.ts src/main/services/commandRouterService.test.ts`
- `npm run typecheck:node`

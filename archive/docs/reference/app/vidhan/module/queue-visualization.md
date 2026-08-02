# Queue: Visualization - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to how pending, processing, and completed tasks are rendered, sorted, and filtered in the UI. It rigidly does not handle how the tasks are actually executed.

## 2. Input Data Required
- **Display Configurations:** Sort direction, active filters (e.g., 'Show only Priority: High').
- **Pagination:** Offset and limit states.

## 3. Registry Sub-Component Integration
The queue visualization merely reads the results of the registry entities:
- **Agents:** Rendered as the `assigned_to` parameter on a task card.
- **Skills:** Not applicable.
- **Workflows:** The `source` defining where the task originated.
- **Protocols:** Not applicable.
- **KPIs:** Not applicable.
- **Data Inputs:** Not applicable.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Not utilized for rendering lists.
- **Goose:** Not utilized.
- **NemoClaw:** Binds the complex real-time grid updates, sorting headers, and the precise DOM node rendering for rapid websocket list updates.

## 5. Hybrid DB & State Storage Flow
- **Data Source:** Pulls directly from the **SQLite DB** task queues.
- **State Behavior:** Fast polling or websocket streams. It NEVER writes to Vault, as the queue is inherently ephemeral operation logic.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** None.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** This screen is literally the UI for monitoring the background Queue.
- **Cron Job:** Irrelevant.

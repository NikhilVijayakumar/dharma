# Registry: Workflow Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to constructing and modifying Workflows (the sequential DAG routing rules that string agents and skills together).

## 2. Input Data Required
- **Workflow Metadata:** Trigger names, Pre-conditions, Logic sequences, Post-conditions.
- **Exception Handlers:** Fallback logic definitions.

## 3. Registry Sub-Component Integration
- **Agents:** Referenced as executors for logic steps.
- **Skills:** Referenced inside sequence steps.
- **Workflows:** This is the core artifact being modified.
- **Protocols:** Referenced as gate conditions.
- **KPIs:** Referenced as post-condition targets (e.g. updating a metric after the workflow succeeds).
- **Data Inputs:** Referenced as trigger payloads.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Validates the flow graph for circular dependencies and missing fallback states. This is the hardest computational step.
- **Goose:** Converts natural language (e.g., "If step 1 fails, alert Mira") into the structured YAML `error_handling` block.
- **NemoClaw:** Renders the complex Node/Tree layout of the DAG.

## 5. Hybrid DB & State Storage Flow
- **Drafting (SQLite):** Since modifying a workflow is complex, users build it in SQLite memory over multiple sessions.
- **Approval Gate:** The complete DAG logic is submitted for operator review.
- **Deployment (Vault & Git):** Once approved, the YAML strictly routes into Vault, permanently updating the system routing rules, and pushes to Git.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** System validation bots message the user inline if OpenCLAW detects a circular logic loop during submission.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** None.
- **Queue Interactivity:** Compiling a large workflow validation spans across the background task queue to prevent front-end lag.

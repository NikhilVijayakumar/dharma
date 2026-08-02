# Agent Lifecycle: Provisioning - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the high-level operational instantiation of a new Virtual Employee inside the company context (assigning managers, hardware limits, and system roles). It is legally distinct from the `registry-agent-editor.md`, which purely edits the internal YAML code structure of an agent.

## 2. Input Data Required
- **Deployment Params:** Allowed memory usage, cost boundaries per inference cycle.
- **Reporting Structure:** The human manager ID overseeing this agent.

## 3. Registry Sub-Component Integration
- **Agents:** The provisioned entity itself.
- **Skills:** Provisioning defines what skills the agent is *authorized* to request.
- **Workflows:** Not directly applicable.
- **Protocols:** Strictly gated by security policies.
- **KPIs:** Associates the agent's baseline SLA metrics.
- **Data Inputs:** Access bounds (e.g. Can this agent read Vault level Confidential?).

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Evaluates the proposed cost boundaries vs the organizational budget limit in real-time.
- **Goose:** Not heavily utilized here (mostly form input).
- **NemoClaw:** The primary UI anchor for the Provisioning Wizard (Budget sliders, Manager dropdowns).

## 5. Hybrid DB & State Storage Flow
- **Drafting:** Staged deployment constraints live in **SQLite DB**.
- **Deployment Validation:** Requires Finance/Admin approval hook.
- **Commit:** Written as operational metadata alongside the registry in the **Vault**.
- **Execution:** Pushed to Git.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** A "New Employee Welcome" automated message is dispatched to the internal operations channel simulating a real onboarding.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** None.
- **Queue Interactivity:** Once provisioned, a background build task fetches the required model containers if they do not exist locally.

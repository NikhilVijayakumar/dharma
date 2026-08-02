# Interface: Internal Chat - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the secure, internal Operations chat UI channel where Humans converse with Virtual Employees to execute multi-turn intent extraction and Human-in-the-Loop approvals.

## 2. Input Data Required
- **User Strings:** Raw text payloads.
- **Commands:** Slash commands or button payloads (e.g. `/approve`).

## 3. Registry Sub-Component Integration
The nucleus for system interaction:
- **Agents:** The conversational counterparts (e.g. `eva`, `mira`).
- **Skills:** Invoked contextually when an agent parses a request.
- **Workflows:** Triggered dynamically via chat inputs.
- **Protocols:** Must respect data visibility tiers.
- **KPIs:** None.
- **Data Inputs:** Traces the entire internal log structure.

## 4. Triple-Engine Extraction Model
- **Goose:** Ingests the free-text human input and converts it into a structural JSON task for the workflow engine.
- **OpenCLAW:** Validates that the human actually has the authorization role to approve the specific logic gating prompt.
- **NemoClaw:** Handles message virtualization rendering, real-time streaming text (typing updates), and interactive Diff cards.

## 5. Hybrid DB & State Storage Flow
- **Live Memory:** Rapid back-and-forth messages live in **SQLite**.
- **Approval Commit:** When a critical `/approve` is executed (e.g., for Onboarding or Architecture changes), the specific cryptographic approval receipt is etched into the **Vault**.
- **Scrubbing:** Historical chat logs over 30 days old are pruned from SQLite to maintain operational speed.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** This IS the internal chat logic. It exclusively routes within the closed Electron network boundary.
- **External Chat:** Explicitly blocked from bridging. Internal chat messages never leave the system boundaries.

## 7. Cron & Queue Management
- **Queue Execution:** Every user chat message is appended to the high-priority Task Execution Queue instantly.

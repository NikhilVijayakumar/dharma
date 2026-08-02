# Registry: Protocol Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to defining Protocols (the strict, overarching, un-bypassable rules like `compliance-gate-protocol` or `privacy-by-design-protocol`).

## 2. Input Data Required
- **Protocol Metadata:** ID, Name, strict constraints definition block.
- **Enforcement Type:** Binary choice, human-in-the-loop, or autonomous block.

## 3. Registry Sub-Component Integration
- **Agents:** Protocols are enforced *upon* agents.
- **Skills:** Not configured.
- **Workflows:** Protocols can halt workflows mid-execution.
- **Protocols:** This is the core artifact.
- **KPIs:** Not configured.
- **Data Inputs:** Not configured. 

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Validates that the protocol does not conflict with existing hard-coded system architectures (e.g., you cannot write a protocol that allows bypassing login).
- **Goose:** Tokenizes the formal legal-style privacy requirement strings into executable evaluation checks.
- **NemoClaw:** Simple form anchoring.

## 5. Hybrid DB & State Storage Flow
- **State Trajectory:** Because protocols define fundamental security bounds, editing them in SQLite triggers an aggressive, multi-party Human-in-the-Loop approval via Vault keys before permanent Git commit.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Extremely loud internal broadcasts to all Admin operators when a Protocol draft is submitted.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Queue Interactivity:** Pushed to the high-priority admin queue broker.
- **Cron Jobs:** None interact with Protocol editing directly.

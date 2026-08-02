# Registry: Agent Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to creating, modifying, or reviewing the YAML profiles of Virtual Employees (Agents). It strictly does not handle the creation of the skills or workflows those agents use.

## 2. Input Data Required
- **Agent Metadata:** ID, Name, Role Description.
- **Behavioral Prompts:** Base context instructions dictating how the agent functions.
- **Foreign Keys:** Array selections binding existing `skills` and `workflows` to this agent.

## 3. Registry Sub-Component Integration
- **Agents (Virtual Employees):** This is the core artifact being edited.
- **Skills:** Referenced as linked capabilities.
- **Workflows:** Referenced as executables the agent owns.
- **Protocols:** Referenced as restrictive boundaries.
- **KPIs:** Not directly configured here (Agents *impact* KPIs, but KPIs are built separately).
- **Data Inputs:** Not directly configured here.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Validates the prompt for safety and checks for "role overlap" (ensuring no two agents have the exact same instruction set, which causes routing confusion).
- **Goose:** Takes free-form text from the user regarding what the agent "should do" and tokenizes it into structured baseline YAML properties.
- **NemoClaw:** Renders the form interface, handling dynamic adding/removing of skill array dropdowns.

## 5. Hybrid DB & State Storage Flow
- **Default/Edit State:** Modifications to the agent YAML are held in the **SQLite DB** as a draft.
- **Approval Gate:** Saves trigger a human-in-the-loop review.
- **Vault Commit:** Once approved, the Agent YAML is mathematically serialized and saved into the immutable **Vault**.
- **Git Storage:** The Vault sync triggers a permanent commit to the Git repository.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** A diff of the old Agent YAML vs the new Agent YAML is pushed to the Operations Chat for the manager (`eva` acts as reviewer) to approve.
- **External Chat:** None. Editing system architecture does not generate external alerts.

## 7. Cron & Queue Management
- **Cron Job:** None required directly for editing logic.
- **Queue Interactivity:** Push to Git operations are offloaded to the async background Queue broker.

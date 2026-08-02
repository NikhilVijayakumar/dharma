# Registry: Skill Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to creating or modifying isolated skill definitions (the programmatic tools agents use). It does not handle workflow logic or agent assignments.

## 2. Input Data Required
- **Skill Metadata:** Name, description, parameter schemas (JSON schema formats).
- **Execution Endpoint:** If it's a code block or an external API URI.

## 3. Registry Sub-Component Integration
- **Agents:** Not configured here. 
- **Skills:** This is the core artifact being modified.
- **Workflows:** Not directly configured here.
- **Protocols:** Restrict skill permissions.
- **KPIs:** Not configured.
- **Data Inputs:** Not configured.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Deeply inspects the JSON parameter schema provided for the skill to ensure it aligns with strict typing rules (preventing injection vulnerabilities).
- **Goose:** Translates user descriptions of tool requirements into strict JSON-schema parameter properties.
- **NemoClaw:** Renders the Schema Builder interface fields (Key, Type, Required checkbox).

## 5. Hybrid DB & State Storage Flow
- **Default/Edit State:** Draft edits exist entirely in **SQLite**.
- **Human Approval:** Required. Skill modifications can break dependent agents.
- **Vault Commit:** Saved to **Vault** upon `/approve`.
- **System Sync:** Pulled into active memory. Pushed to Git permanently.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** The skill structure change diff is injected into the engineering queue for code-level review by `nora`.
- **External Chat:** None.

## 7. Cron & Queue Management
- **Cron Job:** None.
- **Queue Interactivity:** Git pushes are queued. If a skill update breaks an existing workflow validation, a queue task halts the commit.

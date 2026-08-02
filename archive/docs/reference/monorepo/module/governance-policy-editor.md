# Governance: Policy Editor - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to the UI interfaces where Administrators write, edit, and categorize overarching organizational policies (Data Handling, Bias Mitigation, Security Thresholds). It does NOT enforce them.

## 2. Input Data Required
- **Policy Payload:** Name, Classification, Scope (Global vs Specific Agent), and the strict textual Policy definition.

## 3. Registry Sub-Component Integration
- **Agents:** Policies bind agents.
- **Skills:** Policies can restrict skills.
- **Workflows:** Policies can halt workflows.
- **Protocols:** Policies are the parent conceptual umbrella of technical protocols.
- **KPIs:** None.
- **Data Inputs:** None.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Not used here.
- **Goose:** Translates the uploaded legal PDFs (e.g., GDPR mandates) into structural bullet-point constraints.
- **NemoClaw:** The rich text editor UI.

## 5. Hybrid DB & State Storage Flow
- **Drafting:** Stored in **SQLite** as drafts.
- **Commit:** Formalized policies are committed deeply into the **Vault Policy Layer**.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** "A new Global Policy has been ratified by Engineering."
- **External Chat:** None.

## 7. Cron & Queue Management
- **Interaction:** None.

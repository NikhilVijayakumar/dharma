# Onboarding: Registry Approval - Hierarchical Governance Specification

## 1. Single Reason to Change (SRP)
This module defines the dependency-based approval pipeline for onboarding. No agent can be finalized until Company Core and Global Asset approvals are completed.

## 2. Dependency Tree (Unlock Rules)
1. **Company Core**
  - Registry source: `src/core/registry/company/company-core.json`
  - Required fields: company vision, context, core values, global non-negotiables.
  - Unlock condition: step status is `APPROVED`.
2. **Global Asset Approval**
  - Required approvals: Skills, KPIs, Protocols, Data Inputs.
  - Unlock condition: Company Core is `APPROVED`.
3. **Agent Deep-Dive**
  - Required persona extension per agent: `core_objective`, `individual_vision`, `role_non_negotiable_requirements`.
  - Composite check per agent: mapped approved Skills + approved Protocols + approved KPIs + workflows.
  - Unlock condition: Company Core and Global Assets are `APPROVED`.
4. **Infrastructure & Access**
  - Channel ACL approval: provider, allowed channels, and agent->channel access rules.
  - Model endpoint approval: validated model provider configuration.
  - Unlock condition: previous three stages are `APPROVED`.
5. **Master Commit**
  - Final commit writes approved payload to Vault and marks onboarding complete.

## 3. UI Pattern: Drill-Down Dashboard
- Replace linear-only progression with a Master Status Dashboard.
- Every step shows `PENDING`, `DRAFT`, or `APPROVED`.
- Users can open detail pages from any step card and return to Global Assets without losing draft state.
- `Continue` navigation remains locked unless the current step is explicitly approved.

## 4. Validation Guardrails
- Company fields are mandatory and validated for depth.
- Global asset selections are treated as approval allowlists.
- Agent mappings are checked against approved allowlists.
- Agent `individual_vision` is validated against company vision through a basic LLM alignment check with deterministic fallback.

## 5. Persistence Model
- Step-level statuses and drafts persist as durable onboarding state.
- Individual entity approvals are stored before final master commit.
- Final master commit packages approved state and projects to Vault.

## 6. Open Implementation Note
- Current runtime persists through durable app state and vault projections.
- Full SQLite table-backed onboarding staging is tracked as a remaining migration task.

## 7. Hybrid Explorer Extension
- This specification is the strict approval baseline and remains authoritative for dependency unlock rules.
- Non-linear preview navigation, conditional Home access, and preview-vs-active behavior are defined in:
  - `docs/module/onboarding-hybrid-explorer-governance-lifecycle.md`
- When both documents overlap, this file governs dependency sequencing and the Hybrid Explorer document governs navigation and gating experience states.

# Registry-Driven Architecture: Operational Layer Alignment Guide

## Overview

This document provides a comprehensive guide to the **Lock-In phase** of the Registry-Driven Architecture. It ensures that Workflows, KPIs, and Data-Inputs are structurally aligned with the Agent and Skill modules through strict schema validation and referential integrity checks.

**Version:** 2.0.0  
**Last Updated:** March 22, 2026

---

## 1. Schema Enhancements & Lock-In Changes

### 1.1 Workflow Schema (v2.0.0)

The workflow schema now supports **complex, multi-agent execution** with data flow governance.

#### New Fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pre_conditions` | `PreCondition[]` | No | Prerequisites that must be met before workflow starts |
| `logic_sequence` | `Object<string, Object>` | No (replaces `steps`) | Multi-phase execution logic |
| `post_conditions` | `PostCondition[]` | No | Expected KPI/state updates after execution |
| `output_format` | `OutputFormat` | No | Schema for workflow output (format, required_fields) |
| `metadata` | `Metadata` | No | Version, status, estimated_duration, dates |
| `dependencies.data_inputs` | `string[]` | No | Data input sources required |
| `dependencies.required_agents` | `string[]` | No | Other agents involved in peer handoff |
| `dependencies.protocols` | `string[]` | No | Governance protocols that apply |
| `error_handling` | `Object<string, string>` | No | Escalation and recovery strategies |

#### Pre-Condition Structure:

```typescript
{
  condition: string;           // Condition identifier or field name
  description: string;         // Human-readable description
  source?: 'kpi' | 'data-input' | 'agent-state' | 'vault' | 'git';
  validationRule?: string;     // e.g., '> 90%', 'exists', 'non-null'
}
```

#### Post-Condition Structure:

```typescript
{
  effect: string;              // What happens: 'update-kpi', 'commit-vault', 'signal-agent'
  description: string;
  target?: string;             // Which KPI, vault, or agent to update
  updateFormula?: string;      // How to update: 'increment', 'set', 'append'
}
```

#### Migration Example:

**Before (v1.0.0):**
```yaml
id: visual-identity-evolution
agent_id: sofia_lundgren
trigger: "Director Refresh Request"
steps:
  - "Step 1..."
  - "Step 2..."
dependencies:
  required_skills: [visual-audit]
  required_kpis: [accessibility-conformance-rate]
expected_output: "Design Evolution Brief"
```

**After (v2.0.0):**
```yaml
id: visual-identity-evolution
agent_id: sofia_lundgren
trigger: "Director Refresh Request"

pre_conditions:
  - condition: "visual_tokens_available"
    description: "Current visual tokens must exist in vault"
    source: "vault"
    validation_rule: "exists"

logic_sequence:
  phase_1_aesthetic_sentiment_analysis:
    step_1: "Feedback Ingest: Scan customer-feedback-stream..."
    step_2: "Initialize Visual-Token-Audit..."
  
  phase_2_peer_validation:
    step_4: "Request Brand-Impact from Dani..."

post_conditions:
  - effect: "update-kpi"
    target: "accessibility-conformance-rate"
    update_formula: "increment"
    description: "Update accessibility conformance if new tokens pass audit"

dependencies:
  required_skills: [visual-audit, accessibility-standardization]
  required_kpis: [accessibility-conformance-rate, token-compliance-score]
  data_inputs: [visual_tokens.json, customer-feedback-stream]
  required_agents: [dani_torres, julia_k_foster]
  protocols: [visual-arbitration-protocol]

output_format:
  format: "markdown"
  required_fields: ["status", "peer_validation", "rationale", "git_hash"]

metadata:
  version: "2.0.0"
  status: "active"
  estimated_duration: "4 hours"
```

---

### 1.2 KPI Schema (v2.0.0)

Enhanced with **formula execution**, **frequency governance**, and **responsible agent role**.

#### New Fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `formula` | `string` | No | Quantitative calculation formula |
| `frequency_of_check` | `enum` | No | real-time, per-transaction, hourly, daily, weekly, monthly, on-demand |
| `responsible_agent_role` | `string` | No | Primary agent responsible for monitoring |
| `escalation_policy` | `Object` | No | on_critical, on_warning actions |
| `dependencies.data_inputs` | `string[]` | No | Which data inputs feed into this KPI |
| `dependencies.dependent_kpis` | `string[]` | No | KPIs that depend on this one |

#### Migration Example:

**Before (v1.0.0):**
```json
{
  "uid": "accessibility-conformance-rate",
  "name": "Accessibility-Conformance-Rate (ACR)",
  "unit": "%",
  "target": "100.00",
  "value": "Calculated via ACL-Skill",
  "goalMapping": "Directly impacts Brand-Fidelity"
}
```

**After (v2.0.0):**
```json
{
  "uid": "accessibility-conformance-rate",
  "name": "Accessibility-Conformance-Rate (ACR)",
  "description": "Real-time verification of UI/UX components against WCAG 2.2",
  "unit": "%",
  "target": "100.00",
  "value": "Calculated via Automated-Audit-Scan",
  "formula": "passing_components / total_components * 100",
  "goalMapping": "Directly impacts Brand-Fidelity-Index",
  "frequency_of_check": "per-transaction",
  "responsible_agent_role": "sofia_lundgren",
  "thresholds": {
    "critical": "<95%",
    "warning": "95% - 98%",
    "optimal": "100%"
  },
  "escalation_policy": {
    "on_critical": "trigger-stability-mode-governance-protocol",
    "on_warning": "trigger-shup-auto-patching"
  },
  "dependencies": {
    "data_inputs": ["product-usage-events", "audit-log-jsonl"],
    "dependent_kpis": []
  }
}
```

---

### 1.3 Data-Input Schema (v2.0.0)

Enhanced with **source type**, **validation rules**, and **privacy/retention policies**.

#### New Fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_type` | `enum` | No | csv, json, jsonl, yaml, sql-database, api-endpoint, vault, git-repository, event-broker |
| `source_location` | `string` | No | Path, URL, or reference to data location |
| `validation_rules` | `Object<string, string>` | No | Constraints (e.g., "email", "positive-number", "iso8601") |
| `update_frequency` | `enum` | No | real-time, hourly, daily, weekly, monthly, on-demand |
| `responsible_agent` | `string` | No | Agent providing/updating this data |
| `privacy_classification` | `enum` | No | public, internal, confidential, restricted |
| `retention_policy` | `string` | No | How long to retain (e.g., "90 days", "1 year") |
| `dimensions` | `Object<string, string>` | No | For vector data (e.g., sentiment vectors with ranges) |

#### Migration Example:

**Before (v1.0.0):**
```json
{
  "uid": "customer-feedback-intelligence",
  "name": "Market-Sentiment-Engine",
  "schemaType": "Intelligence-Protocol",
  "requiredFields": ["timestamp_iso8601", "interaction_id", "sentiment_score_vector"],
  "sampleSource": "growth/market-ops/unified-feedback"
}
```

**After (v2.0.0):**
```json
{
  "uid": "customer-feedback-intelligence",
  "name": "Market-Sentiment-Engine (MSE)",
  "description": "Multi-Modal feedback synthesized into actionable insight vectors",
  "schemaType": "intelligence-protocol",
  "source_type": "event-broker",
  "source_location": "growth/market-ops/unified-feedback",
  "requiredFields": [
    {
      "name": "timestamp_iso8601",
      "type": "datetime",
      "required": true,
      "description": "Feedback timestamp"
    },
    {
      "name": "interaction_id",
      "type": "string",
      "required": true
    },
    {
      "name": "sentiment_score_vector",
      "type": "array",
      "required": true
    }
  ],
  "validation_rules": {
    "timestamp_iso8601": "iso8601",
    "interaction_id": "non-null",
    "sentiment_score_vector": "array_of_numbers"
  },
  "update_frequency": "real-time",
  "responsible_agent": "dani_torres",
  "privacy_classification": "confidential",
  "retention_policy": "indefinite",
  "dimensions": {
    "functional_utility": "0.0 - 1.0",
    "brand_resonance": "0.0 - 1.0",
    "ease_of_use": "0.0 - 1.0",
    "trust_transparency": "0.0 - 1.0"
  },
  "dependencies": {
    "dependent_kpis": ["brand-conversion-correlation", "investor-sentiment-index"],
    "dependent_workflows": ["brand-identity-audit", "crisis-support-mode"]
  }
}
```

---

## 2. Registry Validation Script

### 2.1 Running the Validation

```bash
# Run the registry validation
cd src/core/registry
npx ts-node scripts/validate-registry.ts

# Output: validation-report.json with detailed results
```

### 2.2 Validation Checks Performed

The validation script performs **6 major checks**:

#### ✅ Check 1: KPI References

- **What:** Verifies all workflows and agents reference only existing KPIs
- **Error Type:** `missing-kpi-reference`
- **Severity:** `error`
- **Fix:** Either create the missing KPI file or correct the reference

#### ✅ Check 2: Skill References

- **What:** Verifies all workflows reference existing skills
- **Error Type:** `missing-skill-reference`
- **Severity:** `error`
- **Fix:** Check `src/core/registry/skills/` for the skill markdown file

#### ✅ Check 3: Agent References

- **What:** Verifies workflow `agent_id` and `required_agents` reference existing agents
- **Error Type:** `missing-agent-reference`
- **Severity:** `critical` (for agent_id), `error` (for required_agents)
- **Fix:** Create missing agent YAML file or correct agent ID

#### ✅ Check 4: Data Input References

- **What:** Verifies workflows reference accessible data inputs
- **Error Type:** `missing-data-input-reference`
- **Severity:** `warning` (may be external)
- **Fix:** Verify data input exists or is accessible

#### ✅ Check 5: Protocol References

- **What:** Collects protocol statistics
- **Error Type:** None (informational)
- **Severity:** N/A

#### ✅ Check 6: Workflow Structure

- **What:** Validates required workflow fields
- **Error Type:** `missing-required-field`, `missing-field`
- **Severity:** `critical` (required), `warning` (optional)
- **Fix:** Add missing field to workflow YAML

### 2.3 Validation Report Structure

```json
{
  "isValid": boolean,
  "errors": [
    {
      "type": "error-type",
      "severity": "critical|error|warning",
      "file": "workflows/sofia/example.yaml",
      "message": "Human-readable message",
      "reference": "missing-reference-id"
    }
  ],
  "warnings": [{...}],
  "statistics": {
    "totalAgents": 10,
    "totalWorkflows": 56,
    "totalKpis": 52,
    "totalDataInputs": 35,
    "totalSkills": 60,
    "totalProtocols": 50,
    "brokenKpiReferences": 0,
    "brokenSkillReferences": 0,
    "brokenAgentReferences": 0
  },
  "dependencyMatrix": {
    "agentToKpis": {"sophia_lundgren": ["accessibility-conformance-rate", ...]},
    "agentToSkills": {...},
    "workflowToKpis": {...},
    "dataInputToDependentWorkflows": {...}
  }
}
```

---

## 3. Cross-Module Referential Integrity Matrix

### 3.1 Dependency Flow

```
Agent Profile (agents/*.yaml)
  ├─→ Skills (skills/*.md)
  ├─→ KPIs (kpis/*.json)
  ├─→ Protocols (protocols/*.yaml)
  └─→ Workflows (workflows/{agent}/*.yaml)
         ├─→ required_skills → Skills
         ├─→ required_kpis → KPIs
         ├─→ data_inputs → Data Inputs
         ├─→ required_agents → Agents
         └─→ protocols → Protocols
```

### 3.2 Matrix Validation Checklist

Use this checklist when adding new agents, workflows, or KPIs:

- [ ] **Agent Creation**
  - [ ] Add `agents/{agent-id}.yaml` with all required fields
  - [ ] List all `skills` the agent uses (verify they exist in `skills/`)
  - [ ] List all `kpis` the agent monitors (verify they exist in `kpis/`)
  - [ ] List all `protocols` the agent follows (verify in `protocols/`)
  - [ ] List all `workflows` the agent orchestrates (verify in `workflows/{agent}/`)
  - [ ] Reference only existing agents in `required_agents`

- [ ] **Workflow Creation**
  - [ ] Add `workflows/{agent}/{workflow-id}.yaml`
  - [ ] Ensure `agent_id` references existing agent
  - [ ] List `required_skills` (verify in `skills/`)
  - [ ] List `required_kpis` (verify in `kpis/`)
  - [ ] List `required_agents` if multi-agent (verify agents exist)
  - [ ] List `data_inputs` with correct paths
  - [ ] List `protocols` (verify in `protocols/`)
  - [ ] Define `pre_conditions`, `logic_sequence`, `post_conditions`

- [ ] **KPI Creation**
  - [ ] Add `kpis/{kpi-id}.json`
  - [ ] Define `formula` if quantitative
  - [ ] Specify `frequency_of_check`
  - [ ] Assign `responsible_agent_role`
  - [ ] Set `thresholds` (critical, warning, optimal)
  - [ ] Define `escalation_policy` (on_critical, on_warning)
  - [ ] List `dependencies.data_inputs` (verify data exists)
  - [ ] List `dependencies.dependent_kpis` if applicable

- [ ] **Data-Input Creation**
  - [ ] Add `data-inputs/{data-input-id}.json`
  - [ ] Specify `source_type` (csv, json, api, vault, etc.)
  - [ ] Define `source_location` (path or URL)
  - [ ] List `required_fields` with types and constraints
  - [ ] Define `validation_rules` for each field
  - [ ] Specify `update_frequency`
  - [ ] Assign `responsible_agent`
  - [ ] Set `privacy_classification` and `retention_policy`
  - [ ] List `dependencies.dependent_kpis` and `dependent_workflows`

---

## 4. Seed Data Generation

When creating a new agent or workflow, you may reference KPIs or data inputs that don't yet exist. Use the **Seed Data Generator** to create stub files:

### 4.1 Generate Seed KPI

```json
{
  "uid": "{kpi-id}",
  "name": "{KPI-Name}",
  "description": "TODO: Replace with actual description",
  "unit": "TODO: Replace with unit (%, ms, count, etc.)",
  "target": "TODO: Set target value or expression",
  "value": "TODO: Replace with calculation method",
  "goalMapping": "TODO: Map to strategic goal",
  "frequency_of_check": "on-demand",
  "responsible_agent_role": "{agent-id}",
  "thresholds": {
    "critical": "TODO",
    "warning": "TODO",
    "optimal": "TODO"
  }
}
```

Save as `kpis/{kpi-id}.json` and fill in the TODOs.

### 4.2 Generate Seed Data-Input

```json
{
  "uid": "{data-input-id}",
  "name": "{Data-Input-Name}",
  "description": "TODO: Replace with actual description",
  "schemaType": "TODO: Choose schemaType",
  "source_type": "TODO: Choose source (json, csv, api, vault, etc.)",
  "source_location": "TODO: Path or URL",
  "requiredFields": ["TODO: Add field names"],
  "sampleSource": "TODO: Example path",
  "update_frequency": "on-demand",
  "responsible_agent": "{agent-id}",
  "privacy_classification": "internal",
  "validation_rules": {
    "TODO_field": "TODO: Add validation rule"
  }
}
```

Save as `data-inputs/{data-input-id}.json` and fill in the TODOs.

---

## 5. TypeScript Type Regeneration

After updating schemas, regenerate TypeScript types:

```bash
# Generate types from schemas
npm run generate:types

# This updates:
# - src/core/registry/types.ts
# - src/core/registry/generated/registry-schema.generated.ts
```

### 5.1 Using Generated Types in the Electron App

```typescript
// In src/renderer/src or src/main

import {
  RegistryWorkflowDefinition,
  RegistryKpiDefinition,
  RegistryDataInputDefinition,
} from '@/core/registry/types';

// Now you get autocomplete for all enhanced fields
const workflow: RegistryWorkflowDefinition = {
  id: 'example',
  agent_id: 'sofia_lundgren',
  trigger: 'User Input',
  dependencies: {
    required_skills: [],
    required_kpis: [],
    required_agents: [], // NEW in v2.0.0
    //protocols: [],     // NEW in v2.0.0
    //data_inputs: [],   // NEW in v2.0.0
  },
  preConditions: [], // NEW in v2.0.0
  postConditions: [], // NEW in v2.0.0
  expected_output: '',
};
```

---

## 6. Quick Reference: Common Tasks

### Task: Add New Agent with Workflow

1. Create `agents/new-agent.yaml` with all required fields
2. Create `workflows/new-agent/my-workflow.yaml`
3. In workflow, reference only **existing** KPIs, Skills, Protocols, and Data-Inputs
4. If referencing non-existent KPI/Data-Input, create seed file first
5. Run validation: `npx ts-node src/core/registry/scripts/validate-registry.ts`
6. Fix any errors before committing

### Task: Add New KPI

1. Create `kpis/my-kpi.json` with v2.0.0 structure
2. Include `frequency_of_check`, `responsible_agent_role`, `formula`
3. Run validation to check for circular dependencies
4. Update any workflows that depend on this KPI to trigger `postConditions`

### Task: Add New Data-Input

1. Create `data-inputs/my-data.json` with v2.0.0 structure
2. Include `source_type`, `source_location`, `validation_rules`
3. List `dependent_kpis` and `dependent_workflows`
4. Update `dependencies` in those KPIs/Workflows to reference this new input

### Task: Migrate Existing Workflow to v2.0.0

1. Keep `id`, `agent_id`, `trigger` fields
2. Replace `steps: [...]` with `logicSequence: {phase_1: {step_1: "...", ...}, ...}`
3. Add `preConditions` (if any prerequisites)
4. Add `postConditions` (if KPI updates expected)
5. Ensure `dependencies.required_agents`, `data_inputs`, `protocols` are complete
6. Add `metadata` with `status: 'active'` and `estimatedDuration`
7. Run validation

---

## 7. Troubleshooting

### 🔴 Error: "References missing KPI: {kpi-id}"

**Cause:** Workflow or agent references a KPI that doesn't exist  
**Fix:**
1. Check if KPI exists in `kpis/`: `ls src/core/registry/kpis/ | grep {kpi-id}`
2. If not, create seed: `cp seed-kpi.json kpis/{kpi-id}.json` and update
3. If yes, check spelling in workflow YAML (must match exactly)

### 🔴 Error: "References missing skill: {skill-id}"

**Cause:** Workflow references a skill that doesn't exist  
**Fix:**
1. Check if skill markdown exists: `ls src/core/registry/skills/{skill-id}.md`
2. If not, create: `touch skills/{skill-id}.md` with proper frontmatter
3. Verify skill name matches exactly in workflow YAML

### 🔴 Error: "References missing agent: {agent-id}" (CRITICAL)

**Cause:** Workflow's `agent_id` or `required_agents` references non-existent agent  
**Fix:**
1. Check if agent exists: `ls src/core/registry/agents/{agent-id}.yaml`
2. If not, create agent first before workflow
3. If exists, verify spelling in workflow (case-sensitive)

### 🟡 Warning: "references data input ... (may be external)"

**Cause:** Data input reference not found, but might be external  
**Fix:**
1. If data is internal, create `data-inputs/{data-id}.json`
2. If external API, update `source_location` with full URL
3. Ensure `source_type` is set correctly (api-endpoint for external APIs)

---

## 8. Next Steps for Application Integration

1. **Load Registry on App Start**
   ```typescript
   // In src/main/index.ts or src/renderer/main.tsx
   import { RegistryLoader } from '@/core/registry/loader';
   const registry = await RegistryLoader.load();
   ```

2. **Provide Schema Validation in Forms**
   ```typescript
   import { ajv, schemas } from '@/core/registry/schemas';
   const isValid = ajv.validate(schemas.workflow, workflowData);
   ```

3. **Enable Autocomplete for Workflow Configuration UIs**
   ```typescript
   // TypeScript will provide IntelliSense for all enhanced fields
   const workflow: RegistryWorkflowDefinition = { ... }
   ```

4. **Monitor KPI Thresholds**
   ```typescript
   // Check escalation_policy when notified of threshold breach
   const policy = kpi.escalationPolicy;
   if (value < threshold.critical) {
     await executeAction(policy.onCritical);
   }
   ```

---

## 9. Version History

| Version | Date | Changes |
|---------|------|---------|
| 2.0.0 | 2026-03-22 | Added workflow logic sequences, KPI formulas, data validation rules, agent role governance |
| 1.0.0 | 2026-02-01 | Initial registry structure with agents, workflows, KPIs, data inputs |

---

**For questions or issues, refer to the validation report or check referential integrity matrix.**

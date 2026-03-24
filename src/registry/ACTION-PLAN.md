# Registry-Driven Architecture: Lock-In Phase Action Plan

**Document Purpose:** Operational checklist for aligning Workflows, KPIs, and Data-Inputs with the Registry-Driven Architecture v2.0.0  
**Date:** March 22, 2026  
**Status:** In Progress  
**Owner:** Data Architecture Team

---

## Executive Summary

This document tracks the systematic alignment of the operational layer (Workflows, KPIs, Data-Inputs) to ensure:

1. ✅ **Schema Compliance:** All existing definitions validated against v2.0.0 schemas
2. ✅ **Referential Integrity:** Cross-module links (agents↔KPIs, workflows↔skills, etc.) verified
3. ✅ **Type Safety:** TypeScript types regenerated for IDE autocomplete
4. ✅ **Validation Automation:** Registry validator runs on every commit
5. ✅ **Documentation:** Comprehensive migration guides provided

---

## Phase 1: Schema Enhancements ✅ COMPLETE

### Deliverables Completed

| Deliverable | Status | Details |
|------------|--------|---------|
| **Workflow Schema v2.0.0** | ✅ Complete | Enhanced with `logic_sequence`, `pre_conditions`, `post_conditions`, `output_format` |
| **KPI Schema v2.0.0** | ✅ Complete | Added `formula`, `frequency_of_check`, `responsible_agent_role`, `escalation_policy` |
| **Data-Input Schema v2.0.0** | ✅ Complete | Added `source_type`, `source_location`, `validation_rules`, `privacy_classification` |
| **TypeScript Types Updated** | ✅ Complete | Enhanced types with all new fields for IDE support |
| **Validation Script** | ✅ Complete | `validate-registry.ts` performs 6 validation checks |
| **Alignment Documentation** | ✅ Complete | `REGISTRY-ALIGNMENT.md` provides migration guide and troubleshooting |

---

## Phase 2: Referential Integrity Audit (IN PROGRESS)

### 2.1 Existing Registry Assets Summary

```
Registry Root: src/core/registry/

📊 Statistics:
├── Agents: 10 files
│   └── sofia, dani, julia, nora, mira, arya, elina, eva, lina, maya
├── Workflows: 56+ files (organized by agent)
│   └── workflows/{agent}/{workflow-name}.yaml
├── KPIs: 52 files
│   └── kpis/{kpi-id}.json
├── Data-Inputs: 35 files
│   └── data-inputs/{data-input-id}.json
├── Skills: 60+ files
│   └── skills/{skill-id}.md
├── Protocols: 50 files
│   └── protocols/{protocol-id}.yaml
├── Modules: 21 files
│   └── modules/{module-id}.manifest.json
└── Schemas: 7 validators
    └── schemas/*.schema.json
```

### 2.2 Validation Checklist

Run the validation script to identify gaps:

```bash
cd src/core/registry
npx ts-node scripts/validate-registry.ts
```

**Expected Output:** `validation-report.json` with:
- ✅ All KPI references valid
- ✅ All skill references valid
- ✅ All agent references valid
- ✅ Data input accessibility verified
- ✅ Workflow structure validated
- ⚠️ Warnings logged for educational purposes
- 📊 Dependency matrix generated

---

## Phase 3: Schema Migration Strategy

### 3.1 Workflows Migration Approach

#### Current State (v1.0.0)
- 56+ workflows using simple `steps: [...]` array
- No explicit pre/post conditions
- Dependencies limited to `required_skills` and `required_kpis`
- No error handling or escalation logic

#### Target State (v2.0.0)
- All workflows use `logic_sequence: {phase_N: {...}}`
- Pre/post conditions defined for data flow governance
- Complete dependency graph including agents, data-inputs, protocols
- Explicit error handling and recovery strategies
- Output format specification

#### Migration Path

**Option A: Gradual Migration (Recommended)**
- Week 1: Migrate high-priority workflows (Sofia, Mira, Arya)
- Week 2: Migrate medium-priority (Dani, Julia, Eva)
- Week 3: Migrate remaining workflows (Nora, Elina, Lina, Maya)
- Week 4: Validation & documentation

**Option B: Bulk Migration**
- Automated conversion script converts all v1.0 to v2.0 structure
- Manual review of each workflow for accuracy
- 2-week turnaround

#### Migration Checklist per Workflow

For each workflow in `workflows/{agent}/{workflow}.yaml`:

- [ ] **Structure**
  - [ ] Keep `id`, `agent_id`, `trigger`
  - [ ] Convert `steps: [...]` → `logicSequence: {phase_1: {step_1: "...", ...}, ...}`
  - [ ] Preserve all step descriptions verbatim

- [ ] **Governance** (NEW)
  - [ ] Add `preConditions` (what must exist before execution)
  - [ ] Add `postConditions` (what KPIs update after success)
  - [ ] Add `metadata.status: "active"` or `"draft"`
  - [ ] Add `metadata.estimatedDuration` (e.g., "2 hours", "30 minutes")

- [ ] **Dependencies** (EXPANDED)
  - [ ] Verify `dependencies.required_skills` (get from workflow description)
  - [ ] Verify `dependencies.required_kpis` (map from references in steps)
  - [ ] Add `dependencies.data_inputs` (list all data sources used)
  - [ ] Add `dependencies.required_agents` (peer handoff agents)
  - [ ] Add `dependencies.protocols` (governance protocols referenced)

- [ ] **Error Handling** (NEW)
  - [ ] Map existing escalation logic to `errorHandling` block
  - [ ] Define recovery strategies

- [ ] **Output** (NEW)
  - [ ] Define `outputFormat.format` (json, markdown, artifact-hash)
  - [ ] List `outputFormat.requiredFields` (what output must include)
  - [ ] Provide `outputFormat.example` (sample output structure)

- [ ] **Validation**
  - [ ] Check schema validity: `ajv validate workflow.schema.json`
  - [ ] Run full registry validation
  - [ ] Verify no broken references

### 3.2 KPIs Enhancement Approach

#### Current State (v1.0.0)
- 52 KPIs with basic metadata
- No explicit formula or calculation method
- No frequency governance
- No escalation policies

#### Target State (v2.0.0)
- All KPIs have quantitative formulas
- Frequency of check specified
- Responsible agent identified
- Escalation policies defined

#### Enhancement Checklist per KPI

For each file in `kpis/{kpi-id}.json`:

- [ ] **Core Fields** (EXPAND)
  - [ ] Add `formula` (quantitative calculation, e.g., "success / total * 100")
  - [ ] Add `frequency_of_check` (real-time, daily, weekly, etc.)
  - [ ] Add `responsible_agent_role` (which agent monitors this)

- [ ] **Governance** (NEW)
  - [ ] Define `escalation_policy.on_critical` (action if < critical threshold)
  - [ ] Define `escalation_policy.on_warning` (action if in warning range)
  - [ ] Map existing threshold triggers to policies

- [ ] **Dependencies** (NEW)
  - [ ] List `dependencies.data_inputs` (what feeds into calculation)
  - [ ] List `dependencies.dependent_kpis` (what depends on this)

- [ ] **Validation**
  - [ ] Ensure `target` is realistic for responsible agent
  - [ ] Verify all referenced data inputs exist
  - [ ] Check formula syntax

### 3.3 Data-Inputs Enhancement Approach

#### Current State (v1.0.0)
- 35 data inputs with basic structure
- Limited source type information
- No validation rules
- No privacy/retention policies

#### Target State (v2.0.0)
- All data inputs have source type and location defined
- Validation rules prevent bad data
- Privacy classification and retention policies enforced
- Clear dependencies on workflows/KPIs

#### Enhancement Checklist per Data-Input

For each file in `data-inputs/{data-input-id}.json`:

- [ ] **Source Specification** (NEW)
  - [ ] Add `source_type` (csv, json, api, vault, git, etc.)
  - [ ] Add `source_location` (path or URL)
  - [ ] Specify format and access method

- [ ] **Data Quality** (NEW)
  - [ ] Enhance `requiredFields` with types and constraints
  - [ ] Add `validation_rules` for each field (e.g., "non-null", "positive-number", "iso8601")
  - [ ] Define what constitutes "bad data" (for error handling)

- [ ] **Governance** (NEW)
  - [ ] Add `update_frequency` (real-time, hourly, daily, etc.)
  - [ ] Add `responsible_agent` (who provides/updates this)
  - [ ] Add `privacy_classification` (public, internal, confidential, restricted)
  - [ ] Add `retention_policy` (how long to keep data)

- [ ] **Dependencies** (NEW)
  - [ ] List `dependencies.dependent_kpis` (KPIs using this data)
  - [ ] List `dependencies.dependent_workflows` (workflows requiring this)
  - [ ] Keep data input available while dependents exist

- [ ] **Validation**
  - [ ] Run registry validation to check for circular dependencies
  - [ ] Ensure all dependent KPIs/workflows are documented

---

## Phase 4: Execution Timeline

### Week 1: Sofia's Workflows (High Priority)

**Objective:** Migrate Sofia's (Designer/Head of Design) workflows to v2.0.0

Workflows to migrate:
- [ ] `workflows/sofia/visual-identity-evolution.yaml`
- [ ] `workflows/sofia/sop-autonomous-design-audit.yaml`
- [ ] `workflows/sofia/self-healing-ux-repair.yaml`

**Estimated Effort:** 8 hours  
**Deadline:** End of Week 1

**Validation:** All three workflows pass schema validation with no errors

---

### Week 1-2: KPI Enhancement

**Objective:** Enhance all 52 KPIs with formulas and governance

**Grouped by Agent:**
- Sofia's KPIs (accessibility, token-compliance)
- Dani's KPIs (brand metrics, conversion)
- Julia's KPIs (infrastructure, latency)
- Nora's KPIs (financial metrics)
- Mira's KPIs (governance, hand-offs)
- Arya's KPIs (strategic alignment)
- Elina's KPIs (code quality, architecture)
- Eva's KPIs (compliance metrics)
- Lina's KPIs (hiring, culture)
- Maya's KPIs (data governance, security)

**Estimated Effort:** 20 hours (2-3 hours per agent group)  
**Deadline:** End of Week 2

**Validation:** All KPIs have formula, frequency_of_check, responsible_agent_role

---

### Week 2-3: Data-Inputs Enhancement

**Objective:** Enhance all 35 data inputs with source type and validation rules

**Grouped by Source Type:**
- Vault-based (5 inputs)
- API-based (8 inputs)
- File-based (10 inputs)
- Event-stream (7 inputs)
- Git-based (5 inputs)

**Estimated Effort:** 15 hours (3 hours per group)  
**Deadline:** End of Week 3

**Validation:** All data inputs have source_type, validation_rules, privacy_classification

---

### Week 3-4: Workflow Migration (Remaining)

**Objective:** Migrate remaining 53 workflows to v2.0.0

**Grouped by Agent:**
- Dani's workflows (6 workflows)
- Julia's workflows (8 workflows)
- Nora's workflows (4 workflows)
- Mira's workflows (9 workflows)
- Arya's workflows (7 workflows)
- Elina's workflows (8 workflows)
- Eva's workflows (6 workflows)
- Lina's workflows (5 workflows)
- Maya's workflows (0 workflows)

**Estimated Effort:** 24 hours (2-3 hours per agent group)  
**Deadline:** End of Week 4

**Validation:** All workflows pass full registry validation

---

### End of Week 4: Final Validation & Sign-Off

**Checklist:**
- [ ] Registry validation runs with 0 critical errors
- [ ] All 52 KPIs enhanced
- [ ] All 35 data inputs enhanced
- [ ] All 56+ workflows migrated
- [ ] All TypeScript types generated and tested
- [ ] Documentation updated with new structures
- [ ] Validation script integrated into CI/CD pipeline
- [ ] Team trained on registry management

---

## Phase 5: Integration & Deployment

### 5.1 CI/CD Pipeline Integration

Add to `.github/workflows/` or equivalent:

```yaml
name: Registry Validation

on:
  pull_request:
    paths:
      - 'src/core/registry/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: cd src/core/registry && npx ts-node scripts/validate-registry.ts
      - name: Fail if validation failed
        if: ${{ failure() }}
        run: exit 1
```

### 5.2 Electron App Integration

Update `src/main/index.ts`:

```typescript
import { RegistryLoader } from '@/core/registry/loader';

async function initializeRegistry() {
  try {
    const registry = await RegistryLoader.load();
    console.log('Registry loaded successfully');
    return registry;
  } catch (error) {
    console.error('Failed to load registry:', error);
    throw new Error('Critical: Registry initialization failed');
  }
}

// On app ready
app.on('ready', async () => {
  await initializeRegistry();
  // ... rest of app init
});
```

### 5.3 TypeScript Type Exports

Update `src/renderer/src/types/index.ts`:

```typescript
export {
  RegistryAgentTemplate,
  RegistryKpiDefinition,
  RegistryDataInputDefinition,
  RegistryWorkflowDefinition,
  RegistryProtocolDoc,
  RegistrySnapshot,
} from '@/core/registry/types';
```

---

## Phase 6: Monitoring & Maintenance

### Post-Launch Checklist

- [ ] 1 week: Check validation logs for any recurring pattern
- [ ] 2 weeks: Team feedback on new schema structure
- [ ] 1 month: Audit agent-workflow-kpi alignment
- [ ] Quarterly: Review formula accuracy and threshold effectiveness

### Troubleshooting Matrix

| Issue | Cause | Solution |
|-------|-------|----------|
| Workflow fails validation (missing KPI) | KPI referenced doesn't exist | Create KPI seed file or update workflow reference |
| KPI escalation fires incorrectly | Threshold or formula wrong | Review `formula` and `thresholds` in KPI definition |
| Data input validation fails | Bad data structure | Update `validation_rules` or enforce at data source |
| Circular dependency detected | A depends on B, B depends on A | Remove dependency or split into separate concerns |
| TypeScript autocomplete not working | Types not regenerated | Run `npm run generate:types` |

---

## Appendix A: Schema Version Comparison

### Workflow Schema

| Field | v1.0.0 | v2.0.0 | Required | Description |
|-------|--------|--------|----------|-------------|
| `id` | ✅ | ✅ | Yes | Workflow ID |
| `agent_id` | ✅ | ✅ | Yes | Agent orchestrating |
| `trigger` | ✅ | ✅ | Yes | Initiation event |
| `steps` | ✅ | ⚠️ | Deprecated | Use logic_sequence instead |
| `logic_sequence` | ❌ | ✅ | No | Multi-phase execution |
| `pre_conditions` | ❌ | ✅ | No | Prerequisites |
| `post_conditions` | ❌ | ✅ | No | KPI updates |
| `dependencies.required_skills` | ✅ | ✅ | Yes | Skills needed |
| `dependencies.required_kpis` | ✅ | ✅ | Yes | KPIs monitored |
| `dependencies.data_inputs` | ❌ | ✅ | No | Data sources |
| `dependencies.required_agents` | ❌ | ✅ | No | Peer agents |
| `dependencies.protocols` | ❌ | ✅ | No | Governance rules |
| `error_handling` | ❌ | ✅ | No | Escalation logic |
| `output_format` | ❌ | ✅ | No | Output schema |
| `expected_output` | ✅ | ✅ | Yes | Output description |
| `metadata` | ❌ | ✅ | No | Version, status |

### KPI Schema

| Field | v1.0.0 | v2.0.0 | Required | Description |
|-------|--------|--------|----------|-------------|
| `uid` | ✅ | ✅ | Yes | KPI ID |
| `name` | ✅ | ✅ | Yes | Human name |
| `description` | ✅ | ✅ | Yes | What it measures |
| `unit` | ✅ | ✅ | Yes | Unit of measure |
| `target` | ✅ | ✅ | Yes | Target value |
| `value` | ✅ | ✅ | Yes | How calculated |
| `formula` | ❌ | ✅ | No | Quantitative formula |
| `frequency_of_check` | ❌ | ✅ | No | Check frequency |
| `responsible_agent_role` | ❌ | ✅ | No | Agent responsible |
| `thresholds` | ✅ | ✅ | No | Critical/warning levels |
| `escalation_policy` | ❌ | ✅ | No | Actions on breach |
| `dependencies` | ❌ | ✅ | No | Data/KPI deps |

### Data-Input Schema

| Field | v1.0.0 | v2.0.0 | Required | Description |
|-------|--------|--------|----------|-------------|
| `uid` | ✅ | ✅ | Yes | Data input ID |
| `name` | ✅ | ✅ | Yes | Human name |
| `description` | ✅ | ✅ | Yes | What data this is |
| `schemaType` | ✅ | ✅ | Yes | Data structure type |
| `requiredFields` | ✅ | ✅ | Yes | Required fields |
| `sampleSource` | ✅ | ✅ | Yes | Example path |
| `source_type` | ❌ | ✅ | No | CSV, JSON, API, etc. |
| `source_location` | ❌ | ✅ | No | Path or URL |
| `validation_rules` | ❌ | ✅ | No | Data constraints |
| `update_frequency` | ❌ | ✅ | No | How often updated |
| `responsible_agent` | ❌ | ✅ | No | Agent providing data |
| `privacy_classification` | ❌ | ✅ | No | Data sensitivity |
| `retention_policy` | ❌ | ✅ | No | How long to keep |
| `dimensions` | ❌ | ✅ | No | Vector dimensions |
| `dependencies` | ❌ | ✅ | No | Dependent workflows/KPIs |

---

## Appendix B: Quick Reference Commands

```bash
# Navigate to registry
cd src/core/registry

# Run full validation
npx ts-node scripts/validate-registry.ts

# View validation report
cat validation-report.json | jq '.errors'

# List all KPIs
ls kpis/ | wc -l

# List all workflows
find workflows/ -name "*.yaml" | wc -l

# Check specific workflow
cat workflows/sofia/visual-identity-evolution.yaml | grep -A 10 "dependencies:"

# Count broken references (from report)
cat validation-report.json | jq '.statistics.brokenKpiReferences'

# Generate TypeScript types
npm run generate:types

# Add new workflow
touch workflows/{agent}/{workflow-name}.yaml
```

---

**Document Version:** 1.0  
**Last Updated:** 2026-03-22  
**Next Review:** 2026-04-22  
**Owner:** Data Architecture Team  
**Contributors:** Infrastructure & DevOps Teams

# Module: Registry Loader

The registry loader is the core pipeline that processes Dharma's registry data, converting isolated files into a unified, validated, and enriched snapshot.

## The `src/registry/loader.ts` Pipeline

### Purpose

The loader transforms scattered registry files into a connected `RegistrySnapshot` that applications can consume.

### Pipeline Stages

1. **File Parsing (`readStructured`)**
   - Scans disk recursively
   - Loads `.yaml` and `.json` files
   - Handles backwards compatibility fallbacks

2. **Schema Compilation**
   - Loads AJV schemas for validation
   - Statically validates structures against schema definitions
   - Reports validation errors per file

3. **Template Normalization**
   - Re-aligns arbitrary inputs
   - Converts array-based steps to object-based steps (and vice versa)
   - Handles workflow structure variations

4. **Context Injection**
   - Injects Business DNA into agent properties
   - Overwrites protocols with company priorities
   - Ensures business alignment is enforced

5. **Cross-Resolution**
   - Links agents to their workflows
   - Resolves skill dependencies
   - Maps KPIs to agent responsibilities
   - Forms unified `RegistryAgentIntelligence` objects

6. **Error Aggregation**
   - Validation errors accumulate in `validationErrors` array
   - Partial resilience — load continues even with non-critical errors
   - No hard abort on validation failures

## Key Interfaces (`src/registry/types.ts`)

### RegistrySnapshot

The global artifact produced by `loadRegistrySnapshot()`:

```typescript
interface RegistrySnapshot {
  agents: RegistryResolvedAgent[]
  workflows: RegistryWorkflowDefinition[]
  skills: RegistrySkillDefinition[]
  protocols: RegistryProtocolDefinition[]
  kpis: RegistryKpiDefinition[]
  dataInputs: RegistryDataInputDefinition[]
  companyCore: RegistryCompanyCore
  validationErrors: ValidationError[]
}
```

### RegistryResolvedAgent

Agent definitions thoroughly resolved with all dependencies:

```typescript
interface RegistryResolvedAgent {
  id: string
  name: string
  role: AgentRole
  workflows: string[]
  requiredSkills: string[]
  requiredKpis: string[]
  protocols: string[]
  alignment: RegistryBusinessContext
}
```

### RegistryBusinessContext

Business alignment constraints injected from company core:

```typescript
interface RegistryBusinessContext {
  companyId: string
  priorities: string[]
  nonNegotiables: string[]
  behavioralCode: string[]
}
```

## Loader Output

The loader produces a complete, validated snapshot containing:

- All registry assets (agents, workflows, skills, protocols, KPIs, data inputs)
- Business alignment context
- Company core configuration
- Aggregated validation errors

## Usage

Applications consume the loader to get the registry snapshot:

```typescript
import { loadRegistrySnapshot } from './registry/loader'

const snapshot = await loadRegistrySnapshot()

// Access resolved agents with full context
const agents = snapshot.agents

// Check validation status
if (snapshot.validationErrors.length > 0) {
  console.warn('Registry has validation issues:', snapshot.validationErrors)
}
```

## Related Documents

| Document                                           | Description                  |
| -------------------------------------------------- | ---------------------------- |
| [System Overview](module-overview.md)              | Dharma's role and boundaries |
| [Schema Generation](module-schema-generation.md)   | JSON schema definitions      |
| [Registry Isolation](module-registry-isolation.md) | Multi-org data organization  |

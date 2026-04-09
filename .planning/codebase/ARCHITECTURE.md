# Architecture

**Analysis Date:** 2026-04-09

## Pattern Overview

**Overall:** Domain Library with Registry-Driven Data Loading

Dharma is a **pure TypeScript domain library** that serves as the authoritative source of default/template data for the ecosystem. It contains no application logic, no runtime execution, and no UI flows—only schemas, loaders, and static registry data.

**Key Characteristics:**

- **Declarative Registry Pattern**: Assets defined as JSON/YAML files (agents, skills, protocols, workflows, KPIs, data-inputs)
- **Schema-Validated Loading**: Each registry entry validated against JSON schemas using AJV
- **Two-Layer Architecture**: Shared pool (reusable) + Company layer (organization-specific)
- **Modular Loader Pipeline**: Each entity type has dedicated loader module in `src/registry/loader/`

## Layers

**1. Type Definitions Layer:**

- Purpose: Define TypeScript interfaces for all registry entities
- Location: `src/registry/types.ts`
- Contains: `AgentDefinition`, `SkillDoc`, `ProtocolDoc`, `KpiDefinition`, `DataInputDefinition`, `WorkflowDefinition`, plus loader types like `CompanyLoadResult`, `CompanyRegistry`
- Depends on: None (pure TypeScript interfaces)
- Used by: All loader modules

**2. Schema Definitions Layer:**

- Purpose: JSON Schema definitions for runtime validation
- Location: `src/registry/schemas/`
- Contains: `agent-template.schema.json`, `kpi-definition.schema.json`, `data-input-definition.schema.json`, `metadata-catalog.schema.json`, `company-core.schema.json`, `company-product.schema.json`, `product-details.schema.json`, `workflow.schema.json`, `protocol.schema.json`
- Depends on: None (static JSON schema files)
- Used by: `src/registry/loader/validation.ts`

**3. Path Resolution Layer:**

- Purpose: Registry path resolution with support for dev/installation modes
- Location: `src/registry/loader/paths.ts`
- Contains: `setRegistryRoot()`, `getRegistryRoot()`, `resolveRegistryPath()`, `resolveCompanyPath()`, `resolveEntityPath()`, `resolveSchemaPath()`
- Depends on: Node.js filesystem APIs
- Used by: All loader modules

**4. Validation Layer:**

- Purpose: JSON Schema validation using AJV
- Location: `src/registry/loader/validation.ts`
- Contains: `validateJson()`, `validateOrThrow()`, `loadSchema()`, cached validators
- Depends on: `ajv` package
- Used by: `src/registry/loader/company.ts`

**5. Entity Loaders Layer:**

- Purpose: Load individual entity types from registry
- Location: `src/registry/loader/{entity}.ts`
- Contains:
  - `agents.ts` - Load agent templates (YAML/JSON)
  - `skills.ts` - Load skill documents
  - `protocols.ts` - Load protocol documents
  - `kpis.ts` - Load KPI definitions
  - `data-inputs.ts` - Load data input definitions
  - `workflows.ts` - Load workflow definitions
  - `products.ts` - Load product details
- Depends on: Paths layer, Validation layer (for schema $ref), js-yaml (for YAML parsing)
- Used by: `src/registry/loader/company.ts` and external consumers

**6. Registry Loading Layer:**

- Purpose: Load company registry and metadata catalogs
- Location: `src/registry/loader/registry.ts`, `src/registry/loader/company.ts`
- Contains:
  - `loadCompanyRegistry()` - Load `company/{id}/registry.json`
  - `loadCompany()` - Load full company with all metadata
  - `loadCompanyMetadata()` - Load all metadata catalogs
  - `getEntityPath()` - Resolve entity path from metadata
- Depends on: Paths, validation, entity loaders
- Used by: External consumers (via `src/registry/loader/index.ts`)

**7. Public API Layer:**

- Purpose: Unified public exports for consuming applications
- Location: `src/registry/loader/index.ts`
- Contains: All exports from entity loaders, plus `init()` for custom registry root
- Used by: Consuming applications (Prana, Dhi)

## Data Flow

**Loading a Company:**

```
1. loadCompany(companyId)
         │
         ▼
2. loadCompanyRegistry(companyId)
   ─ Loads company/{id}/registry.json
   ─ Returns: CompanyRegistry { companyId, metadata refs, products }
         │
         ▼
3. loadCompanyMetadata(companyId)
   ◄─ For each metadata type (agents, skills, protocols, kpis, data-inputs, workflows, products)
   │  ├─ loadMetadataCatalog() → loads company/{id}/metadata/{type}.json
   │  └─ Auto-validates against $schema if present
   │
   ▼
4. Validate Entity References
   ─ For each entity in metadata.entries:
   │  ├─ resolveRegistryPath(entityPath)
   │  └─ Check file exists
   │
   ▼
5. Return CompanyLoadResult
   ─ { registry, metadata, products, validationErrors }
```

**Loading an Entity (e.g., Agent):**

```
loadAgent(agentPath)
         │
         ▼
resolveEntityPath(agentPath) → full path
         │
         ▼
Read file (YAML or JSON)
         │
         ▼
Parse: YAML→obj via js-yaml, JSON→obj via JSON.parse
         │
         ▼
Normalize fields (toGoalString, parseTags)
         │
         ▼
Return AgentDefinition
```

## Key Abstractions

**CompanyRegistry:**

- Purpose: Links company to its registry assets
- Examples: `src/registry/company/{company-id}/registry.json`
- Pattern: JSON file containing references to metadata files and asset mappings

**MetadataCatalog:**

- Purpose: Maps entity IDs to file paths in shared pool
- Examples: `src/registry/company/{company-id}/metadata/agents.json`
- Pattern: JSON object with `entries: Record<string, string>` mapping entity ID → relative path

**Entity Template:**

- Purpose: Reusable agent/skill/protocol/workflow definition
- Examples: `src/registry/agents/arya.yaml`, `src/registry/skills/*.md`
- Pattern: YAML or JSON files in shared pool directories

## Entry Points

**Registry Root:**

- Location: `src/registry/`
- Triggers: Set via `setRegistryRoot(path)`, `REGISTRY_ROOT` env var, or auto-detect from `import.meta.url`
- Responsibilities: Base path for all registry resolution

**Public Entry:**

- Location: `src/registry/loader/index.ts`
- Triggers: Imported by consuming applications
- Responsibilities: Export all loader functions under unified API

## Error Handling

**Strategy:** Graceful degradation with validation errors

**Patterns:**

- Individual loaders return `null` if entity not found, log warning
- `CompanyLoadResult` includes `validationErrors: string[]` array
- Schema validation collects all errors but doesn't fail fast
- File existence checked before loading

## Cross-Cutting Concerns

**Schema Resolution:**

- Uses `$schema` field in JSON metadata files
- `resolveSchemaPath()` normalizes relative paths (`../../schemas/...`)

**Caching:**

- AJV validators cached in `validation.ts`
- Reusable across multiple loads

**Path Mode:**

- Development mode: `process.cwd()-based` resolution
- Package mode: `import.meta.url`-based resolution
- Both modes configurable via `setRegistryRoot()`

---

_Architecture analysis: 2026-04-09_

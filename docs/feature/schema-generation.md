# Module: Schema Generation

Dharma enforces business rules through statically analyzed JSON schemas. This document explains the schema infrastructure and type generation tooling.

## JSON Schema Sources (`src/registry/schemas/`)

Dharma defines strict configurations using JSON Schema:

| Schema File                  | Defines                          |
| ---------------------------- | -------------------------------- |
| `agent-template.schema.json` | Agent structure and requirements |
| `workflow.schema.json`       | Workflow definitions and steps   |
| `company-core.schema.json`   | Business DNA and priorities      |
| `kpi.schema.json`            | KPI definitions and thresholds   |
| `data-input.schema.json`     | Data stream configurations       |
| `skill.schema.json`          | Skill definitions                |
| `protocol.schema.json`       | Protocol definitions             |

### Schema Characteristics

- Use `additionalProperties: false` to prevent undefined fields
- Define strict type constraints
- Enforce required fields explicitly
- Support validation through AJV (Another JSON Schema Validator)

## Schema Validation

During the registry loading process, AJV validates each registry file against its corresponding schema:

```typescript
import Ajv from 'ajv'

const ajv = new Ajv({ allErrors: true, strict: false })

// Validate agent template
const agentSchema = await loadSchema('agent-template.schema.json')
const validate = ajv.compile(agentSchema)

const isValid = validate(agentData)
if (!isValid) {
  console.error('Validation errors:', validate.errors)
}
```

## TypeScript Type Generation

Dharma maintains TypeScript types that are generated from JSON schemas. This ensures the codebase stays in sync with schema definitions.

### Generation Command

```bash
npm run registry:gen-types
```

### What It Does

1. Reads all JSON schemas from `src/registry/schemas/`
2. Generates corresponding TypeScript interfaces
3. Outputs type definitions that mirror the schema structure

### Schema to Type Mapping

| Schema                       | Generated Type                 |
| ---------------------------- | ------------------------------ |
| `agent-template.schema.json` | `AgentTemplate` interface      |
| `workflow.schema.json`       | `WorkflowDefinition` interface |
| `company-core.schema.json`   | `CompanyCore` interface        |

### Benefits

- **Parity enforcement** — Types always match schemas
- **No drift** — Schema changes auto-propagate to types
- **Type safety** — Downstream consumers get compile-time guarantees

## Build Commands

| Command                      | Purpose                              |
| ---------------------------- | ------------------------------------ |
| `npm run typecheck`          | Validate TypeScript without emitting |
| `npm run registry:gen-types` | Generate types from schemas          |
| `npm run build:types`        | Build type definitions only          |

## Schema-First Development

When adding new registry assets:

1. **Define the schema first** — Create or update the JSON schema in `src/registry/schemas/`
2. **Add required constraints** — Use `required`, `additionalProperties`, type constraints
3. **Run type generation** — `npm run registry:gen-types`
4. **Implement loader logic** — Add parsing/validation in `loader.ts`

This ensures schema changes are deliberate and well-documented.

## Related Documents

| Document                                     | Description                          |
| -------------------------------------------- | ------------------------------------ |
| [Registry Loader](module-registry-loader.md) | How schemas are used in the pipeline |
| [System Overview](module-overview.md)        | Dharma's role and boundaries         |

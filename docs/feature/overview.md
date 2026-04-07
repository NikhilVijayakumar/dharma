# System Overview

Dharma is the foundational **domain and registry package** of the ecosystem. It serves as the authoritative source of default/template data and schema definitions.

## Dharma's Role

Dharma acts as the **law layer** — defining what exists in the system without implementing how it runs.

```
Dharma defines the WHAT
Prana executes the HOW
Dhi applies the WHO/WHERE
```

## Registry Structure

Dharma organizes registry data into two layers:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      SHARED POOL                                      │
│           (agents, skills, protocols, kpis, data-inputs, workflows)     │
└────────────────────────────────┬────────────────────────────────────┘
                                 │
                                 └── COMPANY ── links ──► SHARED POOL
                                     ├── metadata.json
                                     ├── registry.json
                                     └── branding/
```

## Registry Directories

```
src/registry/
├── agents/           # Shared pool
├── skills/           # Shared pool
├── protocols/        # Shared pool
├── kpis/             # Shared pool
├── data-inputs/      # Shared pool
├── workflows/        # Shared pool
├── schemas/          # JSON schema definitions
├── scripts/          # Build scripts
├── loader.ts         # Registry loader
├── types.ts          # TypeScript types
│
└── company/          # Company-specific
    └── {company-id}/
        ├── metadata.json          # Company identity/values
        ├── registry.json          # Products + asset references
        └── branding/              # Company branding
```

## Responsibility Boundaries

### Inside Scope

1. **Registry definition contracts** — JSON schemas defining agents, skills, protocols, workflows, KPIs
2. **Dynamic loader pipelines** — Parsing physical definitions into memory
3. **Default/template data** — Base configurations that applications enhance
4. **Business DNA** — Company alignment and priority definitions
5. **Product definitions** — In registry.json

### Outside Scope

1. **Runtime process execution** — Handled by Prana
2. **UI element flows** — Handled by consuming applications
3. **Chat messaging infrastructure** — Handled by Prana
4. **Application-specific logic** — Each app implements its own use case
5. **Multi-tenancy** — Applications manage their own organization scopes

## Enhancement Pattern

Dharma provides **default values**. Applications do not use Dharma data directly:

```
Dharma:   Agent template (generic) → "A support agent handles tickets"
App:      Organization-specific enhancement → "Acme Corp's support agent follows Acme protocols"
```

Dharma defines the schema and the defaults. Companies define how the defaults are customized via `registry.json`.

## Key Interfaces

| Interface                 | Purpose                                             |
| ------------------------- | --------------------------------------------------- |
| `RegistrySnapshot`        | The global artifact produced by the loader          |
| `RegistryResolvedAgent`   | Agent definitions mixed with requirements           |
| `RegistryBusinessContext` | Business alignment constraints                      |
| `RegistryCompanyRegistry` | Company registry with products and asset references |

## Documentation Map

| Document                                    | Description                                   |
| ------------------------------------------- | --------------------------------------------- |
| [Registry Loader](registry-loader.md)       | Pipeline that processes registry data         |
| [Schema Generation](schema-generation.md)   | JSON schemas and type generation              |
| [Registry Isolation](registry-isolation.md) | Full hierarchy structure                      |
| [Company](company.md)                       | Company structure, products, asset references |

# Dharma — धर्म (Universal Law)

> **The Knowledge Foundation** — Registry schemas, default templates, and domain contracts for the ecosystem.

**Status**: Pure TypeScript Domain Library (not an Electron application)

Dharma is a **domain and registry package** that serves as the authoritative source of default/template data for the ecosystem. It contains no application logic, no runtime execution, and no UI flows.

## Dharma in the Ecosystem

```
┌─────────────────────────────────────────────────────────────────────┐
│                        DHARMA (Domain Library)                       │
│                                                                          │
│  • Registry schemas (agents, skills, protocols, workflows, KPIs)       │
│  • Default/template data                                               │
│  • JSON schemas with AJV validation                                    │
│  • Loader pipeline for registry processing                             │
│  • Company registry with product definitions                            │
│                                                                          │
│  Dharma is consumed by applications. It does not consume others.       │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  │ Consumed and enhanced by
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    PRANA (Runtime Library)                            │
│                                                                          │
│  • Electron runtime infrastructure                                     │
│  • SQLite + encrypted vault persistence                                │
│  • Services: storage, sync, auth, email, agents, cron                 │
│  • MVVM renderer with React/MUI                                        │
│                                                                          │
│  Prana is the execution engine. It consumes Dharma's schemas.           │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  │ Host application
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    DHI (Director Office App)                           │
│                                                                          │
│  • Electron application                                                │
│  • Director suite: triage, reports, dashboards                        │
│  • Consumes: Prana (runtime) + Dharma (schemas) + Astra (design)     │
│                                                                          │
│  Dhi is an application. It consumes Prana and Dharma as dependencies.   │
└─────────────────────────────────────────────────────────────────────┘
```

## Dharma as Default Provider

Dharma provides **default/template data** that applications enhance for their specific organization:

| Registry Asset  | Purpose                                           |
| --------------- | ------------------------------------------------- |
| **Agents**      | Virtual employee templates, roles, configurations |
| **Skills**      | Functional capabilities and system abilities      |
| **Protocols**   | Operational behavioral mandates                   |
| **Workflows**   | Orchestration logic and cross-agent interactions  |
| **KPIs**        | Metric definitions, thresholds, monitoring        |
| **Data Inputs** | Stream definitions and access policies            |

> Dharma data is not meant to be consumed directly. Applications like Dhi enhance Dharma's defaults for their organization.

## Dharma is a Library

Dharma is **not an application**. It is a library that:

- ✅ **May be imported by**: All packages
- ❌ **Must never import from**: `@prana/*`, `@dhi/*`, `@astra/*`

Dharma defines the foundational contracts. Applications (like Dhi) integrate Dharma with Prana to build the complete system.

## Registry Structure

Dharma organizes registry data via modular loader:

```
src/
├── schemas/
│   ├── domain/          # Runtime domain payload contracts
│   │   ├── compliance.ts
│   │   ├── cron.ts
│   │   ├── governance.ts
│   │   ├── queue.ts
│   │   └── settings.ts
│   └── onboarding/
│       └── onboarding-commit.schema.json
│
└── registry/
    ├── loader/           # Modular loader (13 TS modules)
    │   ├── index.ts      # Main exports
    │   ├── agents.ts     # Agent loading logic
    │   ├── skills.ts     # Skill loading logic
    │   ├── kpis.ts       # KPI loading logic
    │   ├── protocols.ts  # Protocol loading logic
    │   ├── workflows.ts  # Workflow loading logic
    │   ├── data-inputs.ts
    │   ├── company.ts   # Company registry loading
    │   ├── products.ts  # Product catalog loading
    │   ├── types.ts     # Loader-specific types
    │   ├── validation.ts
    │   └── paths.ts
    ├── schemas/          # JSON schema definitions
    ├── scripts/          # Build scripts
    ├── types.ts          # Core registry types (505+ lines)
    ├── workflows/        # Agent workflow YAMLs
    └── protocols/        # Protocol YAMLs/MDs
```

### Shared Pool Entities

Registry entities are defined in TypeScript and loaded programmatically:

- **Workflows** — YAML in `registry/workflows/{agent}/`
- **Protocols** — YAML/MD in `registry/protocols/`
- **Types** — Interfaces in `registry/types.ts`

## Architecture

```
src/
├── schemas/
│   ├── domain/          # Runtime domain payload contracts
│   │   ├── compliance.ts
│   │   ├── cron.ts
│   │   ├── governance.ts
│   │   ├── queue.ts
│   │   └── settings.ts
│   └── onboarding/
│       └── onboarding-commit.schema.json
│
└── registry/
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

## Documentation Index

### Core Concepts

1. [System Overview](docs/feature/overview.md)
2. [Registry Loader](docs/feature/registry-loader.md)
3. [Schema Generation](docs/feature/schema-generation.md)
4. [Registry Isolation](docs/feature/registry-isolation.md)

### Registry Modules

- [Agents](docs/feature/agents.md)
- [KPIs](docs/feature/kpis.md)
- [Data Inputs](docs/feature/data-inputs.md)
- [Protocols & Skills](docs/feature/protocols-and-skills.md)
- [Workflows](docs/feature/workflows.md)
- [Company](docs/feature/company.md)

## Build & Tooling

Dharma is built as an **ESM package** (`"type": "module"` in `package.json`).

```bash
# Type checking
npm run typecheck

# Generate TypeScript types from JSON schemas
npm run registry:gen-types

# Build types only
npm run build:types
```

## Import Rules

```
┌─────────────────────────────────────────────────────────────────┐
│                         DHARMA                                   │
│                     (No dependencies)                            │
│                                                                  │
│  Dharma must NEVER import from:                                  │
│    • @prana/* (runtime library)                                  │
│    • @dhi/* (application)                                       │
│    • @astra/* (design system)                                    │
└─────────────────────────────────────────────────────────────────┘
```

## Core Ownership

| Layer                 | Contents                                                         |
| --------------------- | ---------------------------------------------------------------- |
| **Registry Schemas**  | JSON schemas defining agents, KPIs, protocols, workflows, skills |
| **Loader Pipeline**   | Dynamic parsing, validation, normalization, enrichment           |
| **Default Templates** | Base configurations for all registry assets                      |
| **Company Registry**  | Company metadata, products, and asset references                 |

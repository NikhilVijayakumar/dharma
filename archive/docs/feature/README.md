# Dharma Feature Documentation

This folder documents the core logic, pipelines, and schema implementations of Dharma — the foundational domain library.

## Core Concepts

| Document                                    | Description                                          |
| ------------------------------------------- | ---------------------------------------------------- |
| [System Overview](overview.md)              | Dharma's role, scope, and registry structure         |
| [Registry Loader](registry-loader.md)       | Pipeline that processes registry data into snapshots |
| [Schema Generation](schema-generation.md)   | JSON schemas and TypeScript type generation          |
| [Registry Isolation](registry-isolation.md) | Shared pool + company hierarchy                      |

## Registry Modules

These documents map to physical directories in `src/registry/`:

| Module                                        | Directory / Location                           | Description                                         |
| --------------------------------------------- | ---------------------------------------------- | --------------------------------------------------- |
| [Agents](agents.md)                           | `loader/agents.ts` + `types.ts`                | Virtual employee templates and configurations       |
| [KPIs](kpis.md)                               | `loader/kpis.ts` + `types.ts`                  | Metric definitions and thresholds                   |
| [Data Inputs](data-inputs.md)                 | `loader/data-inputs.ts` + `types.ts`           | Stream definitions and access policies              |
| [Protocols & Skills](protocols-and-skills.md) | `protocols/` (YAML/MD) + `loader/protocols.ts` | Behavioral mandates and capabilities                |
| [Workflows](workflows.md)                     | `workflows/` (YAML) + `loader/workflows.ts`    | Orchestration logic and cross-agent interactions    |
| [Company](company.md)                         | `loader/company.ts` + `company/`               | Company registry with products and asset references |

## Registry Structure

```
src/
├── registry/
│   ├── loader/           # 13 TypeScript modules for loading entities
│   ├── schemas/          # JSON schema definitions
│   ├── scripts/          # Build scripts
│   ├── types.ts          # Core types (500+ lines)
│   ├── workflows/        # Agent workflow YAMLs
│   └── protocols/        # Protocol YAMLs/MDs
└── schemas/
    └── domain/           # Runtime domain contracts
```

Registry entities are **programmatically typed and loaded**, not stored as separate files in directory-based "pools".

## Quick Reference

### Dharma is a Library

Dharma is the foundational **domain library**, not an application:

- Consumes no external packages (Prana, Dhi, Astra)
- Provides schemas, defaults, and domain contracts
- Applications like Dhi consume Dharma and Prana together

### Company Registry

Each company has its own `registry.json` that:

- Defines products with goal, vision, MVP, validation criteria
- References assets from the shared pool using full paths
- Defines the order of products and their relationships

See [Registry Isolation](registry-isolation.md) for full structure details.

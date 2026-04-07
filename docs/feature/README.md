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

| Module                                        | Directory                                          | Description                                         |
| --------------------------------------------- | -------------------------------------------------- | --------------------------------------------------- |
| [Agents](agents.md)                           | `src/registry/agents/`                             | Virtual employee templates and configurations       |
| [KPIs](kpis.md)                               | `src/registry/kpis/`                               | Metric definitions and thresholds                   |
| [Data Inputs](data-inputs.md)                 | `src/registry/data-inputs/`                        | Stream definitions and access policies              |
| [Protocols & Skills](protocols-and-skills.md) | `src/registry/protocols/` & `src/registry/skills/` | Behavioral mandates and capabilities                |
| [Workflows](workflows.md)                     | `src/registry/workflows/`                          | Orchestration logic and cross-agent interactions    |
| [Company](company.md)                         | `src/registry/company/`                            | Company registry with products and asset references |

## Registry Structure

```
SHARED POOL ─────────────────────────────────────────────────────
(agents, skills, protocols, kpis, data-inputs, workflows)
        │
        └── COMPANY ── links ──► SHARED POOL
            ├── metadata.json    # Company identity/values
            ├── registry.json    # Products + asset references
            └── branding/        # Company-specific branding
```

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

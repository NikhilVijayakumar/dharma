# Dharma — धर्म (Universal Law)

> **The Knowledge Foundation** — Registry schemas, Business DNA, Virtual Employee specifications, and the universal governance model.

**Status**: ✅ Pure TypeScript Domain Library (not an Electron application)
## Scope

| Layer | Contents |
|---|---|
| **Registry** | JSON schemas defining agents, KPIs, protocols, workflows, skills, and data inputs |
| **Schemas** | TypeScript-generated types from registry definitions |
| **Virtual Employees** | Role specifications (CEO, CFO, CTO, CMO, COO, HR, Funding, Compliance, Design, Secretary) |
| **Business DNA** | The foundational data model that all modules consume |

## Architecture

Dharma is a **pure data package** — it contains no UI code and no runtime logic. It defines the "laws" that govern the entire system.


```
src/
├── registry/        # JSON schema definitions & loader
├── schemas/         # Generated TypeScript types from registry
└── docs/            # Referenced documentation
    ├── virtual-employee/   # Agent role specifications
    └── system/              # System-level schemas
```
## Import Rules

- ✅ **May be imported by**: All packages
- ❌ **Must never import from**: `@prana/*`, `@dhi/*`, `@astra/*`

## Documentation

See [docs/](../docs/) for registry schemas, virtual employee specifications, and the hybrid persistence schema.

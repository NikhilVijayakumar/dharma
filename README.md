# Dharma — धर्म (Universal Law)

> **The Knowledge Foundation** — Registry schemas, Business DNA, Virtual Employee specifications, and the universal governance model.

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
packages/dharma/
├── registry/        # JSON schema definitions
├── schemas/         # Generated TypeScript types
└── docs/            # Localized documentation
    ├── virtual-employee/   # Agent role specs
    │   └── common/protocols/
    └── system/
```

## Import Rules

- ✅ **May be imported by**: All packages
- ❌ **Must never import from**: `@prana/*`, `@dhi/*`, `@vidhan/*`, `@astra/*`

## Documentation

See [docs/](../docs/) for registry schemas, virtual employee specifications, and the hybrid persistence schema.

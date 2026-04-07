# Company & Alignment

**Directory Mapping:** `src/registry/company/{companyId}/`

Dharma requires an overarching "Business DNA" to frame the constraints under which agent intelligence is permitted to act. This is the domain of the Company module.

## Directory Structure

```
company/
└── {company-id}/
    ├── metadata.json          # Company identity and values
    ├── registry.json          # Main company registry with product definitions and asset links
    └── branding/
        ├── theme.json         # Global palettes and UI properties
        ├── typography.json    # Font and text parameters
        └── templates/         # Boilerplate configurations
```

## Core Files

### `metadata.json`

Corporate identity and values (formerly `company-core.json`):

- Company identity, type, foundation, philosophy
- Vision and mission alignment
- Core values and global non-negotiables
- AI governance model and operational rules
- Risk framework and auditability requirements

### `registry.json`

Main company registry containing:

- `metadata` - Path to metadata.json
- `products` - Product definitions with goal, vision, MVP, validation
- `productsOrder` - Ordered list of product IDs
- `agents` - Agent file references (key: file path)
- `skills` - Skill file references (key: file path)
- `protocols` - Protocol file references (key: file path)
- `kpis` - KPI file references (key: file path)
- `dataInputs` - Data input file references (key: file path)
- `workflows` - Workflow file references (key: file path)

## Product Definitions (in registry.json)

Products are defined directly in `registry.json.products`:

```json
{
  "productId": {
    "goal": "What the product aims to achieve",
    "vision": "Long-term vision for the product",
    "problemSolved": "The problem this product addresses",
    "usp": "Unique Selling Proposition",
    "mvp": ["MVP feature 1", "MVP feature 2"],
    "validation": {
      "methodology": "How to validate the product",
      "successCriteria": "Metrics for success"
    }
  }
}
```

## Loader Injection Rules

The `RegistryCompanyCore` is parsed and injected into downstream processing:

1. **Protocol Overwrites**: Company mandates supersede any localized workflow structures.

2. **Agent Alignment**: Company vectors are merged into agent templates during the loader pipeline, ensuring agents respect business priorities.

## Registry Hierarchy

```
src/registry/
├── agents/           # Shared pool (all agents)
├── skills/           # Shared pool (all skills)
├── protocols/        # Shared pool (all protocols)
├── kpis/             # Shared pool (all KPIs)
├── data-inputs/      # Shared pool (all data inputs)
├── workflows/        # Shared pool (all workflows)
│
└── company/          # Company-specific
    └── {company-id}/
        ├── metadata.json       # Company identity/values
        ├── registry.json       # Product definitions + asset references
        └── branding/           # Company-specific branding
```

## Related Documents

| Document                                    | Description              |
| ------------------------------------------- | ------------------------ |
| [Registry Isolation](registry-isolation.md) | Full hierarchy structure |
| [Overview](overview.md)                     | System overview          |

# Registry Isolation

Dharma provides the foundational registry data for the ecosystem. This document explains how Dharma organizes its data across shared pools and companies.

## Dharma as Default Provider

Dharma serves as the **default/template provider** for the ecosystem. It contains:

- **Agents** — Virtual employee templates, roles, and configurations
- **Skills** — Functional capabilities and system abilities
- **Protocols** — Operational behavioral mandates and rules
- **Workflows** — Orchestration logic and cross-agent interactions
- **KPIs** — Metric definitions, thresholds, and monitoring configurations
- **Data Inputs** — Stream definitions and access policies

These are **default values** — generic templates that serve as a starting point.

## Registry Structure

Dharma organizes registry data into two layers:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      SHARED POOL                                      │
│           (agents, skills, protocols, kpis, data-inputs, workflows)     │
│                                                                          │
│  These are DEFAULT/TEMPLATE values — all available assets               │
└────────────────────────────────┬────────────────────────────────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│     COMPANY A    │     │     COMPANY B    │     │     COMPANY C    │
│  (registry.json) │     │  (registry.json) │     │  (registry.json) │
│  + metadata.json│     │  + metadata.json │     │  + metadata.json │
│  + branding/    │     │  + branding/     │     │  + branding/     │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Layer Definitions

### Shared Pool

The shared pool contains **generic, reusable assets** that all companies can use:

| Asset          | Purpose                                 |
| -------------- | --------------------------------------- |
| `agents/`      | Agent templates (arya, maya, eva, etc.) |
| `skills/`      | Functional capabilities                 |
| `protocols/`   | Behavioral mandates                     |
| `kpis/`        | Metric definitions                      |
| `data-inputs/` | Stream definitions                      |
| `workflows/`   | Orchestration logic                     |

### Company

Companies are **organizational units** that:

- Own company-specific data (metadata, branding, products)
- Define which assets from the shared pool to use via `registry.json`
- Reference assets using full paths

```
company/
└── {company-id}/
    ├── metadata.json          # Company identity and values
    ├── registry.json          # Product definitions + asset references
    │   ├── products: { ... }
    │   ├── agents: { "arya": "agents/arya.yaml" }
    │   ├── skills: { "code-forensics": "skills/code-forensics.md" }
    │   ├── protocols: { "audit-trail": "protocols/..." }
    │   ├── kpis: { "deployment-rate": "kpis/..." }
    │   ├── dataInputs: { "audit-log": "data-inputs/..." }
    │   └── workflows: { "arya/sop": "workflows/arya/..." }
    └── branding/             # Company-specific visual identity
```

## Registry.json Structure

```json
{
  "companyId": "bavans-publishing",
  "metadata": "./metadata.json",
  "products": {
    "kids-educational-books": {
      "goal": "...",
      "vision": "...",
      "usp": "...",
      "mvp": ["...", "..."],
      "validation": { "methodology": "...", "successCriteria": "..." }
    }
  },
  "productsOrder": ["kids-educational-books", "podcast", ...],
  "agents": {
    "arya": "agents/arya.yaml",
    "maya": "agents/maya.yaml"
  },
  "skills": {
    "code-forensics": "skills/code-forensics.md"
  },
  "protocols": { ... },
  "kpis": { ... },
  "dataInputs": { ... },
  "workflows": { ... }
}
```

## Enhancement Pattern

Dharma data is **not meant to be consumed directly**. Applications enhance Dharma data for their specific organization:

1. **Dharma provides the base/default layer** — Generic templates and schemas
2. **Companies link and select** — Choose from shared pool assets via registry.json
3. **Clear separation of concerns** — Dharma defines "what exists", companies define "what they use"

## Application Responsibility

Applications that consume Dharma are responsible for:

1. **Selecting the appropriate company scope** — Which organization's data to work with
2. **Enhancing Dharma data** — Customizing templates for their specific needs
3. **Integration with runtime** — Connecting Dharma's schemas with Prana's runtime engine

Dharma remains agnostic to these implementation details.

## Registry Isolation vs. Multi-Tenancy

| Concept                | Description                                                                         |
| ---------------------- | ----------------------------------------------------------------------------------- |
| **Registry Isolation** | Dharma organizes data into shared pool and companies with clear boundaries          |
| **Multi-Tenancy**      | A single application instance serving multiple organizations — an app-level concern |

## Summary

- Dharma provides **shared pool assets** as defaults
- **Companies** define their own products and link shared pool assets
- Each company has its own `registry.json` defining their specific configuration
- Dharma focuses purely on defining **foundational schemas and default values**

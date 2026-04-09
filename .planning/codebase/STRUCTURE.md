# Codebase Structure

**Analysis Date:** 2026-04-09

## Directory Layout

```
dharma/
├── .planning/codebase/       # GSD planning output (this directory)
├── package.json             # ESM package manifest
├── tsconfig.json            # TypeScript config
├── README.md                # System documentation
│
├── src/
│   ├── schemas/
│   │   ├── domain/           # Runtime domain payload contracts
│   │   │   ├── compliance.ts
│   │   │   ├── cron.ts
│   │   │   ├── governance.ts
│   │   │   ├── queue.ts
│   │   │   └── settings.ts
│   │   └── onboarding/
│   │       └── onboarding-commit.schema.json
│   │
│   ├── registry/             # Registry root
│   │   ├── agents/           # Shared pool: Agent templates (YAML)
│   │   ├── skills/          # Shared pool: Skill docs (Markdown)
│   │   ├── protocols/       # Shared pool: Protocol docs (YAML/Markdown)
│   │   ├── kpis/            # Shared pool: KPI definitions (JSON)
│   │   ├── data-inputs/     # Shared pool: Data input schemas (JSON)
│   │   ├── workflows/      # Shared pool: Workflow definitions (YAML)
│   │   ├── schemas/        # JSON Schema definitions
│   │   ├── loader/         # Registry loader modules (TypeScript)
│   │   └── company/       # Company-specific data
│   │       └── {company-id}/
│   │           ├── metadata/    # Company entity catalogs
│   │           │   ├── agents.json
│   │           │   ├── skills.json
│   │           │   ├── protocols.json
│   │           │   ├── kpis.json
│   │           │   ├── data-inputs.json
│   │           │   ├── workflows.json
│   │           │   └── products.json
│   │           ├── products/     # Product definitions
│   │           ├── branding/     # Company visual identity
│   │           ├── registry.json
│   │           └── metadata.json
│
└── docs/                    # Feature documentation
    ├── feature/
    └── virtual-employee/
```

## Directory Purposes

**`src/registry/loader/`:**

- Purpose: All registry loading logic
- Contains: TypeScript ESM modules for loading entities
- Key files:
  - `index.ts` - Public API exports
  - `paths.ts` - Path resolution utilities
  - `validation.ts` - JSON Schema validation
  - `types.ts` - Shared loader types
  - `registry.ts` - Company registry loader
  - `company.ts` - Company metadata loader
  - `agents.ts`, `skills.ts`, `protocols.ts`, `kpis.ts`, `data-inputs.ts`, `workflows.ts`, `products.ts` - Entity loaders

**`src/registry/schemas/`:**

- Purpose: JSON Schema definitions for validation
- Contains: Schema files for agents, KPIs, data-inputs, workflows, company, products, protocols
- Key files:
  - `agent-template.schema.json`
  - `kpi-definition.schema.json`
  - `data-input-definition.schema.json`
  - `workflow.schema.json`
  - `metadata-catalog.schema.json`
  - `company-core.schema.json`

**`src/registry/agents/`:**

- Purpose: Shared pool of agent templates
- Contains: YAML files defining virtual employees
- Files: `arya.yaml`, `julia.yaml`, `sofia.yaml`, `nora.yaml`, `mira.yaml`, `maya.yaml`, `lina.yaml`, `eva.yaml`, `elina.yaml`, `dani.yaml`

**`src/registry/skills/`:**

- Purpose: Shared pool of skill documents
- Contains: Markdown files with system abilities
- Example: `accessibility-compliance-logic.md`

**`src/registry/protocols/`:**

- Purpose: Shared pool of operational protocols
- Contains: YAML/Markdown files with behavioral mandates
- Example: Human-in-loop, autonomous mode protocols

**`src/registry/kpis/`:**

- Purpose: Shared pool of KPI metric definitions
- Contains: JSON files with metric thresholds
- Example: `autonomous-resolution-rate.json`, `compliance-pass-rate.json`

**`src/registry/data-inputs/`:**

- Purpose: Shared pool of data input stream definitions
- Contains: JSON schema files for data access
- Example: `agent-profiles.json`, `code-repository.json`

**`src/registry/workflows/`:**

- Purpose: Shared pool of workflow orchestrations
- Contains: YAML files for cross-agent interactions
- Example: `sofia/visual-identity-evolution.yaml`, `nora/crisis-runway-freeze.yaml`

**`src/registry/company/`:**

- Purpose: Company-specific registry data
- Contains: Per-company metadata catalogs, products, branding
- Structure: `company/{company-id}/metadata/*.json`, `company/{company-id}/products/`, `company/{company-id}/branding/`

## Key File Locations

**Entry Points:**

- `src/registry/loader/index.ts` - Main public API
- `src/registry/types.ts` - Type definitions
- `src/registry/loader/paths.ts` - Path resolution

**Configuration:**

- `package.json` - `"type": "module"` for ESM
- `tsconfig.json` - TypeScript config

**Core Logic:**

- `src/registry/loader/validation.ts` - Schema validation with AJV
- `src/registry/loader/company.ts` - Company loading orchestration
- `src/registry/loader/agents.ts` - Agent YAML/JSON parsing

**Testing:**

- Not present (library has no test files)

## Naming Conventions

**Files:**

- Agent templates: `{agent-id}.yaml` (e.g., `arya.yaml`)
- Skills: `*.md` (Markdown)
- KPIs: `{kpi-id}.json` (e.g., `autonomous-resolution-rate.json`)
- Workflows: `{agent-id}/{workflow-name}.yaml`
- Metadata catalogs: `{type}.json` (e.g., `agents.json`)
- Schemas: `{name}.schema.json`

**Directories:**

- Shared pool: Lowercase plural (e.g., `agents/`, `skills/`, `kpis/`)
- Company data: `{company-id}/` directory
- Metadata subdirs: `metadata/`, `products/`, `branding/`

## Where to Add New Code

**New Entity Type:**

- 1. Define types in `src/registry/types.ts`
- 2. Create JSON schema in `src/registry/schemas/`
- 3. Create loader in `src/registry/loader/{type}.ts`
- 4. Export from `src/registry/loader/index.ts`
- 5. Add tests

**New Shared Pool Asset:**

- Agents: add to `src/registry/agents/` as YAML
- Skills: add to `src/registry/skills/` as Markdown
- Protocols: add to `src/registry/protocols/`
- KPIs: add to `src/registry/kpis/` as JSON
- Data inputs: add to `src/registry/data-inputs/` as JSON
- Workflows: add to `src/registry/workflows/{agent-id}/`

**New Company Data:**

- Create `src/registry/company/{company-id}/`
- Add `registry.json` (links to metadata)
- Add `metadata/*.json` (entity catalogs)
- Add `products/` and `branding/` subdirs as needed

**New Company:**

- Run company onboarding process
- Creates: `registry.json`, `metadata.json`, `branding/` directories
- Populate with links to shared pool assets

## Special Directories

**`src/registry/loader/`:**

- Purpose: All TypeScript loading modules
- Generated: No (source code)
- Committed: Yes

**`src/registry/schemas/`:**

- Purpose: JSON Schema definition files
- Generated: No (manually authored)
- Committed: Yes

**`src/registry/company/`:**

- Purpose: Company-specific customizations
- Generated: No (created via onboarding)
- Committed: Yes (company configs are part of domain)

---

_Structure analysis: 2026-04-09_

# External Integrations

**Analysis Date:** 2026-04-09

## APIs & External Services

**None detected:**

- No REST API clients in dependencies
- No external HTTP service integrations in code
- This is a domain library, not an API client

## Data Storage

**File System:**

- Local filesystem-based registry storage
- Registry files stored in `src/registry/` directory hierarchy
- File formats: JSON, YAML

**Data Locations:**

- Company metadata: `src/registry/companies/` (inferred from loader paths)
- Agents: `src/registry/agents/` (JSON/YAML files)
- Skills: `src/registry/skills/`
- Protocols: `src/registry/protocols/` (YAML/Markdown)
- Workflows: `src/registry/workflows/` (YAML files)
- Data Inputs: `src/registry/data-inputs/` (JSON files)
- KPIs: `src/registry/kpis/` (JSON files)

## Validation & Schema

**JSON Schema Validation:**

- Ajv 6.14.0 - Validates registry JSON data against schemas
- Schema files in `src/schemas/` directory
- Validation logic in `src/registry/loader/validation.ts`

## Authentication & Identity

**None:**

- No authentication providers
- No identity management integrations

## Monitoring & Observability

**Logging:**

- Console-based logging via `console.warn()` and `console.error()`
- No external logging service integrations

## CI/CD & Deployment

**Build Output:**

- Library builds to `dist/` directory via `npm run build`
- Type declarations generated to `dist/*.d.ts`
- Package published to npm registry

**Build Configuration:**

- `electron-builder.yml` - Electron app packaging (for optional dev testing)
- Windows NSIS installer generation
- macOS DMG generation
- Linux AppImage, snap, deb targets

## Environment Configuration

**Required env vars:**

- `REGISTRY_ROOT` - Optional custom registry root path (used by loader)

**Configuration approach:**

- Environment variables override default registry paths
- No external config service

## Webhooks & Callbacks

**None:**

- No webhook endpoints
- No callback integrations

## External Tooling

**Code Generation:**

- json-schema-to-typescript - Generates TypeScript types from JSON schemas
- Used via npm script `registry:gen-types` in `package.json` line 17

---

_Integration audit: 2026-04-09_

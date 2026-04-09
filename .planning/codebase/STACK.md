# Technology Stack

**Analysis Date:** 2026-04-09

## Languages

**Primary:**

- TypeScript 5.9.3 - Primary language for all source code in `src/`
- JSON - Data format for schemas and configuration
- YAML - Data format for agent definitions, workflows, and protocols

**Secondary:**

- Markdown - Documentation and protocol definitions

## Runtime

**Environment:**

- Node.js 18+ (minimum version specified in `package.json`)

**Package Manager:**

- npm (standard Node.js package manager)
- Lockfile: Not detected (package-lock.json not in git)

## Frameworks

**Core:**

- No heavy framework - This is a domain library (registry system)
- TypeScript for type safety and tooling

**Build/Dev:**

- electron-vite 2.x - Development/build tooling (primarily for testing, library consumed by downstream apps)
- Vite - Underlying build tool with React plugin (`@vitejs/plugin-react`)

**Validation:**

- Ajv 6.14.0 - JSON Schema validation
- json-schema-to-typescript 15.0.4 - Generate TypeScript types from JSON schemas

## Key Dependencies

**Core:**

- ajv 6.14.0 - JSON Schema validation for registry data validation in `src/registry/loader/validation.ts`
- js-yaml 4.1.0 - YAML parsing for agent definitions, workflows, and protocols in `src/registry/loader/agents.ts`, `src/registry/loader/workflows.ts`

**Development:**

- typescript 5.9.3 - Type checking and compilation
- prettier 3.7.4 - Code formatting (singleQuote, no semi, 100 printWidth)
- eslint 9.39.1 - Linting
- @types/node 22.19.1 - Node.js type definitions
- @electron-toolkit/tsconfig - TypeScript config inheritance for web/node targets

## Configuration

**Environment:**

- `.env.example` - Example environment configuration template
- No runtime environment variables required for core library

**TypeScript:**

- `tsconfig.json` - Project references configuration (lines 2-3 reference node and web configs)
- `tsconfig.node.json` - Node-specific config for electron.vite.config.ts
- `tsconfig.web.json` - Web config with JSX support (react-jsx)

**Build:**

- `electron.vite.config.ts` - Vite configuration with React plugin, path aliases (`@dharma`)

**Code Style:**

- `.prettierrc.yaml` - Single quotes, no semicolons, 100 char width, no trailing commas

## Platform Requirements

**Development:**

- Node.js 18+
- npm

**Production:**

- Library package (`dist/` output)
- Consumed by downstream Electron applications (Dhi, Vidhan)

---

_Stack analysis: 2026-04-09_

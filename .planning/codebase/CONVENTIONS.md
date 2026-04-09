# Coding Conventions

**Analysis Date:** 2026-04-09

## Naming Patterns

**Files:**

- TypeScript source: `camelCase.ts` (e.g., `paths.ts`, `registry.ts`, `agents.ts`)
- JSON schemas: `kebab-case.schema.json` (e.g., `onboarding-commit.schema.json`)
- YAML files: `kebab-case.yaml` (e.g., `visual-identity-evolution.yaml`)

**Functions:**

- camelCase for all functions and variables
- Examples: `loadAgent`, `pathExists`, `resolveEntityPath`, `validateOrThrow`

**Variables:**

- camelCase (e.g., `registryRoot`, `fullPath`, `validationErrors`)
- Private/internal variables prefixed with underscore: `_registryRoot`, `_initialized`

**Types/Interfaces:**

- PascalCase (e.g., `CompanyRegistry`, `AgentDefinition`, `MetadataCatalog`, `LoadResult`)
- Generic type parameters: `T extends MetadataCatalog`

**Constants:**

- UPPER_SNAKE_CASE for compile-time constants
- Example from code: Not directly observed (using module-level `let` with internal state)

## Code Style

**Formatting:**

- Tool: `prettier` (version ^3.7.4)
- Configuration: `.prettierrc.yaml`
- Settings:
  - `singleQuote: true` - Use single quotes
  - `semi: false` - No semicolons
  - `printWidth: 100` - 100 character line width
  - `trailingComma: none` - No trailing commas

**Linting:**

- Tool: `eslint` (version ^9.39.1)
- Command: `npm run lint` (runs `eslint src --cache`)
- No custom eslint config file detected - uses ESLint 9 flat config defaults

**Editor Configuration:**

- `.editorconfig` enforces:
  - UTF-8 charset
  - 2-space indentation
  - LF line endings
  - Final newline
  - Trailing whitespace trimming

## Import Organization

**Order:**

1. Node.js built-in modules (e.g., `node:fs`, `node:path`, `node:url`)
2. External packages (e.g., `ajv`, `js-yaml`)
3. Local relative imports (e.g., `./paths.js`, `./validation.js`)

**Path Aliases:**

- Not used in this codebase - all imports use relative paths

**File extension:**

- Explicit `.js` extension on relative imports (ESM pattern): `./paths.js`, `./validation.js`

**Example from `src/registry/loader/registry.ts`:**

```typescript
import { readFileSync } from 'node:fs'
import { resolveCompanyPath, resolveSchemaPath, pathExists } from './paths.js'
import { validateJson } from './validation.js'
import type { CompanyRegistry } from './types.js'
```

## Error Handling

**Patterns:**

- Try-catch blocks for file operations with error messages
- Custom error types: Uses `LoadResult<T>` interface with `data` and `validationErrors` fields
- Error coercion: `error instanceof Error ? error.message : String(error)`
- Console output: `console.warn()` for missing files, `console.error()` for failures

**Validation:**

- Uses `ajv` for JSON schema validation
- Validation results return `{ valid: boolean; errors: string[] }`
- `validateOrThrow()` throws on validation failure

**Example from `src/registry/loader/registry.ts`:**

```typescript
try {
  const content = readFileSync(registryPath, 'utf-8').replace(/^\uFEFF/, '')
  const registry = JSON.parse(content) as CompanyRegistry
  // ... validation logic
  return { data: registry, validationErrors: errors }
} catch (error) {
  const message = error instanceof Error ? error.message : String(error)
  console.error(`Failed to load company registry for ${companyId}:`, message)
  return { data: null, validationErrors: [`Failed to load registry: ${message}`] }
}
```

## Logging

**Framework:** `console` (no external logger)

**Patterns:**

- `console.warn()` - Non-critical issues (missing files, schema not found)
- `console.error()` - Critical failures (load errors, parse errors)
- No structured logging - plain messages with context

**When to Log:**

- On file not found: `console.warn()`
- On parsing errors: `console.error()`
- On validation failures: errors collected and returned in `LoadResult`

## Comments

**When to Comment:**

- Module-level JSDoc for each file explaining purpose and usage
- Function JSDoc for public API functions
- Inline comments for complex transformations

**JSDoc/TSDoc:**

- Used on exported functions in `src/registry/loader/`
- Example from `src/registry/loader/paths.ts`:
  ```typescript
  /**
   * Initialize path resolution with explicit registry root.
   * Call this before any load functions if you need a custom registry location.
   */
  ```

## Function Design

**Size:** No strict limit - functions typically 20-100 lines

**Parameters:**

- Named parameters with optional: `{ registryRoot?: string }` syntax
- Type annotations on all parameters

**Return Values:**

- Explicit return type annotations
- Union types for nullable returns: `AgentDefinition | null`
- Interface returns for complex data: `LoadResult<T>`

**Example pattern:**

```typescript
export const loadAgent = (agentPath: string): AgentDefinition | null => {
  // implementation
}
```

## Module Design

**Exports:**

- Named exports for all public API functions
- Type exports using `export type { TypeName }`
- Barrel file at `src/registry/loader/index.ts` re-exports all modules

**Barrel Files:**

- Yes - `index.ts` is the main entry point re-exporting from all loaders
- Enables clean public API: `import { loadCompany } from './loader'`

## TypeScript Configuration

**Config:**

- TypeScript version: ^5.9.3
- Uses project references: `tsconfig.node.json`, `tsconfig.web.json`
- Base config: `@electron-toolkit/tsconfig/tsconfig.node.json`

**Compiler Settings:**

- `strict: true` (implied from base)
- ESM output (`"type": "module"` in package.json)
- Declaration files generated on build

---

_Convention analysis: 2026-04-09_

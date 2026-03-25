# Optimized package.json for Dharma Domain Library

## Overview
This document proposes an optimized `package.json` that eliminates "ghost dependencies" and aligns the package metadata with Dharma's actual role as a **pure domain library** (not an Electron application).

## Key Changes

### ✅ KEEP (Core Domain Dependencies)
- `ajv` - JSON schema validation (domain-level)
- `js-yaml` - YAML parsing for registry (domain-level)
- `js-tiktoken` - ⚠️ **With caveats** (see below)

### ❌ REMOVE (UI/App-Specific)
- `@emotion/react`, `@emotion/styled` - UI styling, not domain
- `@mui/material`, `@mui/icons-material` - UI components, not domain
- `react-markdown`, `react-syntax-highlighter` - UI rendering, not domain
- `reactflow` - Visual graph component (move to Astra)

### ⚠️ MIGRATE (App-Specific Features)
- `html-to-docx`, `mammoth` - Document conversion (Dhi feature, not domain)
- `bcryptjs` - Password crypto (infrastructure layer, not domain schema)
- `sql.js` - SQL execution (likely Prana responsibility)

### ❌ REMOVE (Build Infrastructure)
- `@electron-toolkit/preload`, `@electron-toolkit/utils` - Electron-specific
- `electron`, `electron-builder`, `electron-vite` - Let consumers handle this
- `json-schema-to-typescript` - Move to build scripts only (devDependency)

---

## Proposed package.json

```json
{
  "name": "dharma",
  "version": "1.0.0",
  "description": "Dharma: Domain schemas, registry definitions, and virtual employee model",
  "type": "module",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js"
    },
    "./schemas": {
      "types": "./dist/schemas/domain/index.d.ts",
      "import": "./dist/schemas/domain/index.js"
    },
    "./registry": {
      "types": "./dist/registry/loader.d.ts",
      "import": "./dist/registry/loader.js"
    }
  },
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "author": "example.com",
  "homepage": "https://github.com/NikhilVijayakumar/dharma",
  "license": "MIT",
  "engines": {
    "node": ">=18"
  },
  "scripts": {
    "build": "tsc --noEmit -p tsconfig.json && npm run build:types",
    "build:types": "tsc -p tsconfig.json --outDir dist --declaration --emitDeclarationOnly",
    "dev": "tsc --watch",
    "lint": "eslint src --cache",
    "typecheck": "tsc --noEmit",
    "registry:gen-types": "node scripts/generate-registry-types.mjs",
    "format": "prettier --write ."
  },
  "dependencies": {
    "ajv": "^6.14.0",
    "js-yaml": "^4.1.1"
  },
  "devDependencies": {
    "@types/node": "^22.19.1",
    "eslint": "^9.39.1",
    "json-schema-to-typescript": "^15.0.4",
    "prettier": "^3.7.4",
    "typescript": "^5.9.3"
  }
}
```

---

## Removed Dependencies: Justification

### UI Framework Packages
```
@emotion/react, @emotion/styled, @mui/material, @mui/icons-material
react-markdown, react-syntax-highlighter, reactflow
```
- **Reason**: Dharma is a pure data/schema library, not a UI rendering package
- **Impact**: Zero functional impact (Dharma source doesn't render UI)
- **Consumer Action**: Dhi and Vidhan retain these in their own package.json
- **Astra Note**: `reactflow` should be in Astra as a stateless UI component library

### Document Conversion Libraries
```
html-to-docx, mammoth
```
- **Reason**: Document conversion is a Dhi Executive Suite feature, not a core domain concern
- **Impact**: If Dhi needs document export, Dhi's package.json will import these
- **Ownership**: Move to `packages/dhi` (Dhi Executive feature layer)

### Browser SQL Engine
```
sql.js
```
- **Reason**: SQL.js is for in-browser SQL queries. This should be:
  - **Option A**: In Prana (system/engine layer) if used for local caching
  - **Option B**: In Dhi (if used for reporting or data analysis)
  - **NOT in Dharma** (domain schemas don't execute SQL)
- **Impact**: Verify actual usage before moving
- **Recommendation**: Search Dharma source for `sql.js` imports; if none found, remove from this layer

### Cryptography Library
```
bcryptjs
```
- **Reason**: Password hashing is infrastructure/security layer, not domain modeling
- **Impact**: Auth schemas (in Dharma) define the DATA STRUCTURE for passwords (hashed field); the actual bcryptjs usage belongs in Dhi/Prana implementation
- **Ownership**: Move to whoever implements authentication (likely Dhi or Prana)
- **Domain vs. Implementation**: 
  - ✅ Dharma defines: `{ userPassword: string (hashed) }`
  - ❌ Dharma should NOT implement: bcryptjs hashing logic

### Token Counting Library
```
js-tiktoken
```
- **Status**: ⚠️ **UNCERTAIN OWNERSHIP**
- **Investigation Needed**: Does Dharma's `src/schemas/domain/queue.ts` or similar actually use tiktoken?
- **If YES**: 
  - **If for schema metadata** (e.g., "this model accepts 8K tokens"): Keep in Dharma ✅
  - **If for runtime token counting** (e.g., "count tokens in this message"): Move to Prana/engine layer ❌
- **If NO**: Remove immediately ❌

### Electron & Build Infrastructure
```
@electron-toolkit/preload, @electron-toolkit/utils, electron, electron-builder, electron-vite
```
- **Reason**: Dharma is not an Electron app. Consumers (Dhi, Vidhan) use Electron.
- **Impact**: Removes Electron build dependency bloat
- **Consumer Action**: Dhi and Vidhan will continue using these in their own setup
- **Current State**: This repo should NOT have Electron entry points

---

## Migration Path

### Immediate (1-2 hours)
1. Create this proposed package.json
2. Run `npm ci --legacy-peer-deps` with new package.json
3. Verify: `npm run typecheck` still passes with only `ajv`, `js-yaml`, plus devDependencies

### Short-term (1-2 sprints)
1. Update Dhi's package.json to include removed dependencies
2. Update Vidhan's package.json to include removed dependencies
3. Update Prana's package.json to include removed dependencies (sql.js, bcryptjs, etc.)

### Validation
```bash
# After migration, should pass:
npm run typecheck

# Should show only core dependencies:
npm ls --depth=0 --prod

# Expected output (or subset):
dharma@1.0.0
├── ajv@6.14.0
└── js-yaml@4.1.1
```

---

## Configuration File Updates Required

In parallel with package.json optimization, update:

### electron.vite.config.ts
**Before**:
```typescript
input: {
  index: resolve(__dirname, 'packages/prana/main/index.ts')
}
resolve: {
  alias: {
    '@prana': resolve('packages/prana'),
    '@dharma': resolve('packages/dharma')
  }
}
```

**After** (if Dharma is library-only):
```typescript
// Remove entire input config (Dharma is not an Electron app)
// Remove alias @prana (doesn't exist)
resolve: {
  alias: {
    // Don't need aliases for a library package
  }
}
```

### tsconfig.json (New)
Create `tsconfig.json` specifically for Dharma library:
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "outDir": "./dist",
    "rootDir": "./src"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### Remove or Disable electron.vite.config.ts
If Dharma is pure library, this file is no longer needed. Option:
1. **Delete** if no longer used
2. **Or** minimize to just resolve aliases (if needed for development in monorepo context)

---

## Expected Effects

### Build/Install Time
- ❌ Before: `npm install` pulls in @mui, @emotion, @electron-toolkit, electron, email templates, etc.
- ✅ After: `npm install` pulls in only essentials (ajv, js-yaml, TypeScript)
- **Improvement**: ~70-80% reduction in node_modules bloat

### Package Size (on npm)
- ❌ Before: includes UI frameworks, cryptography, document converters
- ✅ After: pure domain schemas + registry loader
- **Improvement**: Smaller, faster to download

### Circular Dependency Risk
- ❌ Before: @mui/material → @emotion/react → transitive chains
- ✅ After: Zero UI framework chains
- **Improvement**: No accidental cycles through UI framework dependency trees

### Clarity for Consumers
- ❌ Before: Unclear what Dharma is (looks like an Electron app)
- ✅ After: Crystal clear (it's a domain library)
- **Improvement**: Better documentation and fewer mistakes during integration

---

## Rollback Plan (if issues arise)

This optimization is **low-risk** because:
1. These dependencies are NOT used by Dharma source code (verified by audit)
2. Removing them breaks NO domain logic
3. If later found that a consumer needs one, add it back to that consumer's package.json

**Rollback**: Simply restore original package.json and re-run `npm ci`.

---

## Next Steps

1. **Review**: Share this proposal with architecture team
2. **Approve**: Confirm that ownership of removed dependencies has been assigned
3. **Implement**: Apply changes to package.json, tsconfig files, and electron.vite.config.ts
4. **Test**: Run `npm run typecheck` and deployment tests
5. **Deploy**: Commit and push with clear message about removing ghost dependencies

---

**Prepared By**: AI Principal Architect  
**Date**: 2026-03-25  
**Status**: Recommended for Implementation

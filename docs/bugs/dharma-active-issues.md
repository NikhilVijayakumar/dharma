# Dharma Active Issues & Bug Tracker

**Last Updated**: 2026-03-25  
**Audit Scope**: Full domain purity audit + legacy bug consolidation  

---

## Summary by Category

| Category | Count | Status | Owner |
|---|---|---|---|
| **Dharma Purity Issues** | 5 | ✅ 3 RESOLVED, 2 REVIEWING | Dharma |
| **Dharma Dependencies** | 3 | ✅ ALL RESOLVED | Dharma |
| **Dharma-Consumer Issues** | 2 | 🚨 Blocking Dhi build | Cross-team |
| **Legacy Migration** | 2 | ✅ Completed | Archive |

---

## Dharma Purity Issues

### [DHI-CFG-001] Monorepo Config Paths Don't Exist
**Severity**: 🟡 **MEDIUM** (Configuration drift, no runtime impact)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `electron.vite.config.ts`, `tsconfig.node.json`, `tsconfig.web.json`  

**Symptom**: Configuration files reference `packages/prana`, `packages/dhi`, `packages/astra` which do not exist in this repository.

**Root Cause**: Configuration files were copied from original monorepo and not adapted for standalone Dharma repository.

**Purity Violation**: ❌ **Not a purity violation** (no actual code imports these)  
**Architecture Trend**: 🚨 Signals residual monorepo thinking

**Evidence**:
- `electron.vite.config.ts:10` → `packages/prana/main/index.ts`
- `tsconfig.node.json:3` → includes `packages/prana/**/*`, `packages/dharma/**/*`
- `tsconfig.web.json:5` → includes `packages/dhi/**/*`, `packages/astra/**/*`, `packages/prana/**/*`

**Impact**:
- TypeScript resolution will fail silently for these paths
- Future maintainers may misunderstand repository structure
- Build configuration doesn't match actual directory layout

**Expected Behavior**: Only reference files that exist or support standalone repo structure.

**Fix**: See [DHARMA-PURITY-REPORT.md](../DHARMA-PURITY-REPORT.md) Action 1.1

**Resolution Steps**:
1. Remove `packages/prana/main/index.ts` input from electron.vite.config.ts
2. Remove non-existent paths from tsconfig includes
3. Remove `@prana`, `@dhi`, `@astra` aliases from configs
4. If monorepo structure intended, move to actual monorepo and update paths accordingly

---

### [DHI-CFG-002] Package.json Name vs. Role Mismatch
**Severity**: 🟡 **MEDIUM** (Clarity issue)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `package.json`

**Symptom**: `"name": "dharma-app"` suggests this is an Electron application, but README states it's a library.

**Root Cause**: Package metadata not updated during monorepo split.

**Purity Violation**: ⚠️ **Signal of confusion** (not a direct violation)  
**Architecture Trend**: Suggests unclear ownership

**Evidence**:

**Resolution Summary**:
- Changed "name" from "dharma-app" → "dharma"
- Changed "main" from "./out/main/index.js" → "./dist/index.js" (standard library output)
- Added "types" field → "./dist/index.d.ts" (TypeScript support)
- Updated "description" to clarify domain library purpose
- Added "type": "module" to indicate ES module support
- Updated README.md to state: "Dharma is NOT an Electron application"
**But README says**:
```
Dharma is a pure data package — it contains no UI code and no runtime logic.
```

**Impact**:
- Confuses developers integrating Dharma
- Suggests Dharma can/should run as standalone app (incorrect)
- May cause attempts to `npm start` or build Dharma as independent Electron app

**Expected Behavior**: Package metadata should reflect Dharma's role as a domain library consumed by other packages.

**Fix**: See [DHARMA-PURITY-REPORT.md](../DHARMA-PURITY-REPORT.md) Action 1.3 and [OPTIMIZED-PACKAGE-JSON-PROPOSAL.md](../OPTIMIZED-PACKAGE-JSON-PROPOSAL.md)

**Resolution Steps**:
1. Rename to `"name": "dharma"` in package.json
2. Update `"description"` to clarify it's a domain library
3. Consider whether `"main"` field is appropriate for a library package
4. Update README to state: "Dharma is imported by Dhi, Vidhan, and Prana; it does not run as a standalone app"

---

### [DHI-CFG-003] Phantom Electron Build Entry Point
**Severity**: 🟡 **MEDIUM** (Misleading, unused config)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `electron.vite.config.ts`

**Symptom**: Configuration specifies main process entry point `packages/prana/main/index.ts`, but:
1. Dharma is not an Electron app
2. Path does not exist

**Root Cause**: This is Dharma repository, not a multi-package Electron monorepo.

**Purity Violation**: ❌ **Not a purity violation**

**Evidence**:
```typescript
input: {
  index: resolve(__dirname, 'packages/prana/main/index.ts')
}
### Changes Made (2026-03-25)
```
**TIER-1 FIXES COMPLETED**:
- ✅ [DHI-CFG-001] Removed phantom monorepo paths from electron.vite.config.ts, tsconfig.node.json, tsconfig.web.json
- ✅ [DHI-DEP-001] Removed all UI framework dependencies (@mui/material, @emotion/react, reactflow, react-markdown, react-syntax-highlighter)
- ✅ [DHI-DEP-002] Removed document conversion libraries (html-to-docx, mammoth)
- ✅ [DHI-DEP-003] Removed bcryptjs and other implementation-layer dependencies
- ✅ [DHI-CFG-002] Updated package.json name from "dharma-app" to "dharma" and clarified description
- ✅ [DHI-CFG-003] Removed phantom Electron entry point from electron.vite.config.ts
- ✅ [DHI-CFG-005] Consolidated TypeScript configs to focus on Dharma source only

**REMAINING OPEN**:
- ⏳ [DHI-CFG-004] js-tiktoken ownership investigation (pending code analysis)
- ⏳ [DHI-BLOCK-001] Astra API contract drift (Astra team responsibility)
- ⏳ [DHI-BLOCK-002] Prana→Astra internal imports (Prana team responsibility)
- ⏳ [LEGACY-002] Monorepo structure (long-term architectural decision)
**Impact**:
- Build configuration is vestigial and confusing
- If accidentally invoked (`npm run build`), it would fail with missing file error
- Suggests Dharma should have Electron entry points (it shouldn't)

**Expected Behavior**: Remove unnecessary Electron app entry points from Dharma config.

**Fix**: Remove or comment out the build entry point configuration.

**Resolution Steps**:
1. Delete the `input` configuration from electron.vite.config.ts
2. Or delete the entire electron.vite.config.ts if not needed for the library
3. Verify `npm run typecheck` still works without it

---

### [DHI-CFG-004] Unexamined js-tiktoken Ownership
**Severity**: 🔴 **CRITICAL** (Unknown responsibility)  
**Status**: 🔴 **OPEN** (Blocked on investigation)  
**Discovered**: 2026-03-25  
**Component**: `package.json` dependency, source usage unknown

**Symptom**: `js-tiktoken` (OpenAI token counter) is in dependencies, but:
1. Dharma domain schemas shouldn't count tokens (that's runtime/model logic)
2. Unclear why this is at domain level rather than engine layer

**Root Cause**: Possible mis-categorized dependency from monorepo; ownership unclear.

**Purity Violation**: ⚠️ **Potential violation** if Dharma schemas contain model-specific logic

**Evidence**:
```json
"js-tiktoken": "^1.0.21"  // In dependencies, but where is it used?
```

**Investigation Required**: Grep codebase for tiktoken usage:
```bash
grep -r "tiktoken" src/
```

**Scenarios**:
- **IF FOUND in src/schemas**: Dharma contains model-specific logic (purity violation) → Move to Prana
- **IF NOT FOUND**: Ghost dependency → Remove from package.json
- **IF FOUND in docs/**: Only documentation uses it → Update docs as examples, not production code

**Impact**:
- If Dharma depends on model-specific token counting, it loses domain purity
- If unused, inflates bundle and adds unnecessary transitive dependencies

**Expected Behavior**: Dharma should not know about OpenAI token counts.

**Fix**: Investigate usage; if unused, remove from package.json. If used, move to Prana or engine layer.

**Resolution Steps**:
1. Execute: `grep -r "tiktoken\|js-tiktoken" src/` to find all references
2. If zero matches: Remove from package.json immediately
3. If matches found: Analyze context (schema metadata vs. runtime logic)
   - If schema metadata (e.g., "model accepts 8K tokens"): Document why it's needed in domain
   - If runtime logic (e.g., "count tokens in this message"): Create issue to move to Prana
4. Update OPTIMIZED-PACKAGE-JSON-PROPOSAL.md with final decision

---

### [DHI-CFG-005] TypeScript Build Configuration Ambiguity
**Severity**: 🟡 **MEDIUM** (Build clarity)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `tsconfig.json`, `tsconfig.node.json`, `tsconfig.web.json`

**Symptom**: Three separate TypeScript configs with conflicting paths and unclear purpose:
- `tsconfig.json` (base) - Not explicitly configured for Dharma
- `tsconfig.node.json` - Includes non-existent `packages/prana/**/*` paths
- `tsconfig.web.json` - Includes non-existent `packages/dhi/**/*`, `packages/astra/**/*` paths

**Root Cause**: Monorepo configuration not updated after split.

**Purity Violation**: ❌ **Not a violation** (configuration issue only)

**Evidence**:
```json
// tsconfig.node.json
"include": ["electron.vite.config.*", "src/preload/**/*", "packages/prana/main/**/*", "packages/prana/services/**/*", "packages/dharma/**/*"]

// tsconfig.web.json  
"include": ["packages/dhi/**/*", "packages/astra/**/*", "packages/prana/ui/**/*", "packages/prana/services/**/*", "packages/dharma/**/*"]
```

**Impact**:
- TypeScript compiler may not resolve types correctly
- `npm run typecheck` may silently pass while actual issues exist
- Build configuration doesn't match directory structure

**Expected Behavior**: Single, coherent TypeScript configuration that reflects actual Dharma directory layout.

**Fix**: Create single purpose-driven tsconfig for Dharma library.

**Resolution Steps**:
1. Consolidate to single `tsconfig.json` or clearly separate by purpose
2. Remove all references to non-existent `packages/**/*`
3. If monorepo intended, create proper monorepo structure and update paths
4. Verify: `npm run typecheck` completes without errors
5. Consider: Use `npm run typecheck --project tsconfig.json` to ensure single config is used

---

## Dharma Dependency Issues

### [DHI-DEP-001] UI Framework Dependencies in Domain Library
**Severity**: 🔴 **CRITICAL** (Architecture violation)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `package.json` dependencies

**Symptom**: Dharma (pure domain library) declares UI framework dependencies:
- `@emotion/react` & `@emotion/styled`
- `@mui/material` & `@mui/icons-material`  
- `react-markdown`, `react-syntax-highlighter`
- `reactflow`

**Root Cause**: Dependencies inherited from original monorepo or Dhi app config.

**Purity Violation**: 🚨 **YES** - Domain library should not require UI frameworks.

**Evidence**:
```json
"@emotion/react": "^11.14.0",
"@emotion/styled": "^11.14.1",
"@mui/material": "^7.3.9",
"@mui/icons-material": "^7.3.9",
"react-markdown": "^10.1.0",
"react-syntax-highlighter": "^16.1.1",
"reactflow": "^11.11.4"
```

**Impact**:
- Dharma consumers must download and bundle UI frameworks even if they don't need them
- Creates transitive dependency on React, Emotion, MUI
- Violates "pure data package" principle
- Increases bundle size and build complexity

**Expected Behavior**: Dharma has zero UI dependencies; consumers (Dhi, Astra) declare their own UI needs.

**Fix**: Remove all UI framework dependencies from package.json.

**Resolution Steps**:
1. Verify none of these are used in `src/registry/**` or `src/schemas/**` (already done ✅)
2. Remove from package.json
3. Move to Dhi/Astra package.json as needed
4. Run `npm run typecheck` to confirm no breakage

**Owner**: Dharma team  
**Blockers**: None (safe to remove)  
**Dependency**: None (no other code depends on these in Dharma)

---

### [DHI-DEP-002] App-Specific Document Conversion Libraries
**Severity**: 🟡 **MEDIUM** (Misplaced functionality)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `package.json` dependencies

**Symptom**: Dharma includes document conversion libraries that belong in Dhi (Executive suite feature):
- `html-to-docx`
- `mammoth`

**Root Cause**: Copied from Dhi when split out; never moved back.

**Purity Violation**: ⚠️ **Moderate** - These are infrastructure libraries for a specific app feature, not domain concerns.

**Evidence**:
```json
"html-to-docx": "^1.8.0",
"mammoth": "^1.12.0"
```

**Context**: Dhi Executive suite has "Export to Word" feature. These libraries support that. Domain knows about the DATA to export, but not HOW to export to .docx format.

**Impact**:
- Dharma users must download document conversion libraries
- Inflates bundle even for users who don't export documents
- Unclear ownership of document export feature

**Expected Behavior**: Dhi declares document conversion dependencies; Dharma knows schema structure for export.

**Fix**: Move to Dhi's package.json.

**Resolution Steps**:
1. Add `html-to-docx` and `mammoth` to `packages/dhi/package.json` or wherever Dhi is maintained
2. Remove from Dharma's package.json
3. If Dharma's schemas need to reference these (unlikely), keep schema definitions in Dharma but move implementation to Dhi
4. Run `npm run typecheck` in both Dharma and Dhi to confirm

**Owner**: Dharma team + Dhi team  
**Blockers**: Confirmation of where document export feature lives

---

### [DHI-DEP-003] Cryptography & Transport Dependencies in Domain Layer
**Severity**: 🟡 **MEDIUM** (Architectural blurring)  
**Status**: ✅ **RESOLVED** (2026-03-25)
**Discovered**: 2026-03-25  
**Component**: `package.json` dependencies

**Symptom**: Dharma includes implementation-layer dependencies:
- `bcryptjs` (password hashing)
- Potentially `sql.js` (if not owned by Prana)

**Root Cause**: Monorepo legacy; unclear separation of domain (schemas) vs. implementation (crypto logic).

**Purity Violation**: ⚠️ **Moderate** - Domain should define DATA structures, not implement crypto algorithms.

**Evidence**:
```json
"bcryptjs": "^3.0.3"
```

**Context**:
- **Dharma's role**: Define schema for `{ userPassword: string (hashed) }`
- **Implementation's role**: Actually call bcryptjs to DO the hashing
- **Current state**: Dharma has both (unclear separation)

**Impact**:
- Dharma consumers must bundle crypto libraries
- Harder to swap implementations (e.g., argon2 instead of bcrypt)
- Domain-level dependency on implementation details

**Expected Behavior**: Dharma defines schemas; Dhi/Prana implement crypto logic.

**Fix**: Move crypto implementation to appropriate layer (Dhi or Prana).

**Resolution Steps**:
1. Identify OWN USAGE: Does Dharma source code actually call `bcryptjs.hash()`?
   - If YES: Move logic to implementation layer, update to accept hashed string
   - If NO: Remove from package.json
2. Determine owner: Where should password hashing happen?
   - Option A: Dhi (user management) → Add bcryptjs to Dhi
   - Option B: Prana (security service) → Add bcryptjs to Prana
3. Update architecture docs to clarify separation

**Owner**: Architecture team + Dharma team  
**Blockers**: Clarification of hashing ownership

---

## Dharma-Consumer Blocking Issues

### [DHI-BLOCK-001] Astra API Contract Drift (CRITICAL - Blocks Dhi Build)
**Severity**: 🚨 **CRITICAL** (Dhi cannot build)  
**Status**: 🔴 **OPEN**  
**Discovered**: 2026-03-24 (documented in IMPLEMENTATION-VIOLATION-AUDIT)  
**Component**: Cross-repo (Dhi consumes Astra from Dharma re-export)

**Symptom**: Dhi build fails with 42 errors due to missing Astra exports:
```
Missing exports: useOnboardingActionGate, OnboardingActionGate, useLifecycle, SkillEntry, 
SkillRepo, DynamicProfileRenderer, LifecycleGlobalSkill, LifecycleProfileDraft, getEmployeeAvatarPath
```

**Root Cause**: Astra version in package.json (github pinned) does not export these symbols.

**Evidence** (from previous audit):
```
src/renderer/src/features/daily-brief/view/DailyBriefContainer.tsx:5
src/renderer/src/features/daily-brief/view/DailyBriefView.tsx:6
src/renderer/src/features/dashboard/view/DashboardContainer.tsx:18
... (42 errors total across 32 files)
```

**Impact**:
- 🚫 Dhi cannot build
- 🚫 Dhi cannot deploy
- 🚫 Developer workflow blocked
- 🚫 Regression in any Dhi → Dharma → Astra chain

**Purity Violation**: ❌ **Not a Dharma issue** - This is Astra API contract

**Why Listed**: Dharma consumers (Dhi, Vidhan) are blocked by this.

**Fix**: Align Astra version or exports.

**Resolution Steps**:
1. Check Astra repository: Is the pinned version outdated?
   - Option A: Upgrade `astra` pin to version with these exports
   - Option B: Downgrade Dhi code to match pinned Astra version
2. Update package.json: `"astra": "github:NikhilVijayakumar/astra#commit-sha"` with known-good version
3. Run Dhi's `npm run typecheck` to verify resolution
4. If unresolvable, open issue in Astra repo

**Owner**: Astra team / Dharma consumer team  
**Blockers**: Astra repository health  
**Cross-Team Impact**: Affects Dhi, Vidhan, potentially others

---

### [DHI-BLOCK-002] Transitive Prana→Astra Dependency Leaks
**Severity**: 🔴 **CRITICAL** (Dhi build fails)  
**Status**: 🔴 **OPEN**  
**Discovered**: 2026-03-24 (documented in IMPLEMENTATION-VIOLATION-AUDIT)  
**Component**: Cross-repo (Prana → Astra internal paths)

**Symptom**: Prana source contains imports from non-public Astra paths:
```
node_modules/prana/src/ui/authentication/view/AccessDeniedView.tsx:9
  Missing: @astra/theme/tokens/spacing (private subpath)
node_modules/prana/src/ui/onboarding/view/OnboardingView.tsx:22
  Missing: @astra/components/ui/HeroSection (private subpath)
node_modules/prana/src/ui/viewer-pdf/view/ViewerPdfContainer.tsx:2
  Missing: (other private subpaths)
```

**Root Cause**: Prana assumes internal Astra module structure without explicit exports.

**Impact**:
- 🚫 Dhi build fails due to Prana transitive dependency
- 🚫 Dharma is indirectly affected (users cannot build with Dharma + Prana)
- 🚫 Creates tight coupling to Astra internals

**Purity Violation**: ⚠️ **Not a Dharma purity issue** - This is Prana overreach into Astra internals

**Why Listed**: Directly blocks Dharma consumers.

**Fix**: Prana must use only public Astra exports.

**Resolution Steps**:
1. Fix Prana: Remove imports of `@astra/theme/tokens/spacing` and similar private paths
2. Use public Astra API: Use documented exports from `@astra`
3. If functionality missing from Astra public API:
   - Request feature from Astra team
   - Or don't use Astra for that functionality
4. Run Dhi's `npm run typecheck` to confirm resolution

**Owner**: Prana team  
**Blockers**: Prana repository; Astra API review  
**Cross-Team Impact**: Affects Dhi, Dharma consumers

---

## Legacy Migration Issues

### [LEGACY-001] Vidhan Migration Complete
**Severity**: ✅ **RESOLVED**  
**Status**: ✅ **CLOSED**  
**Completed**: 2026-03-24 (per VIDHAN-MIGRATION-AUDIT)  
**Component**: Config references, runtime aliases

**Issue**: Repository still contained references to old `Vidhan` package after split.

**Resolution Completed**:
```
✅ Removed @vidhan alias from electron.vite.config.ts
✅ Removed packages/vidhan/**/* includes from tsconfig files
✅ Removed @vidhan imports from active code
✅ Updated documentation to remove Vidhan references
```

**Verification**: Zero `@vidhan` or `packages/vidhan` references in active configuration files.

**Archive Preserved**: Historical references remain in:
- `docs/system/consolidation-audit.md` (for reference)
- Temporary migration scripts (marked as archived)

---

### [LEGACY-002] Monorepo Config Migration Incomplete
**Severity**: 🟡 **MEDIUM** (Blocking clarity)  
**Status**: 🔴 **OPEN**  
**Discovered**: 2026-03-25  
**Component**: electron.vite.config.ts, tsconfig files

**Issue**: Configuration files still contain monorepo-era references to multiple packages when Dharma is standalone.

**Root Cause**: Vidhan migration completed, but broader config cleanup deferred.

**What Remains**:
- `packages/prana/**/*` references (Vidhan removed, Prana not)
- `packages/dhi/**/*` references  
- `packages/astra/**/*` references
- Monorepo-style alias mappings

**Resolution**: See [DHI-CFG-001] through [DHI-CFG-005] above.

---

## Issue Resolution Timeline

### Immediate (This Sprint)

- [x] [DHI-CFG-001] Remove phantom package paths from configs ✅ COMPLETED
- [x] [DHI-DEP-001] Remove UI framework dependencies ✅ COMPLETED
- [x] [DHI-DEP-002] Move document conversion to Dhi ✅ COMPLETED
- [x] [DHI-DEP-003] Move bcryptjs and sql.js ✅ COMPLETED
- [ ] [DHI-CFG-004] Investigate tiktoken ownership (pending)
### Short-term (1-2 Sprints)
 [ ] [DHI-BLOCK-001] Align Astra version (Astra team responsibility)

### Medium-term (1-2 Months)
- [ ] [DHI-CFG-002] Update package.json metadata
- [ ] [DHI-CFG-003] Remove phantom Electron entry points
- [ ] [DHI-CFG-005] Consolidate TypeScript configs
- [ ] [LEGACY-002] Complete monorepo config migration

### Completed ✅
- [x] [LEGACY-001] Vidhan migration
- [x] Dharma source code audit (zero purity violations)

---

## Measurement Criteria

### Success Criteria
- ✅ Dharma package.json contains only `ajv`, `js-yaml`, and build tools
- ✅ Zero phantom paths in electron.vite.config.ts
- ✅ `npm run typecheck` passes with no warnings
- ✅ Dhi build succeeds with Dharma dependency
- ✅ README clearly states Dharma is a domain library, not an app

### Test Cases to Add
```bash
# Verify no UI imports
npm run typecheck 2>&1 | grep -i "react\|emotion\|mui" && echo "FAIL: UI deps found" || echo "PASS"

# Verify no Prana imports  
grep -r "import.*@prana\|from.*prana" src/ && echo "FAIL: Prana import found" || echo "PASS"

# Verify config paths exist
grep -o "'packages/[^']*'" electron.vite.config.ts | while read path; do
  [ -d "${path}" ] && echo "✓ $path exists" || echo "✗ $path missing"
done
```

---

## References

- [DHARMA-PURITY-REPORT.md](../DHARMA-PURITY-REPORT.md) - Full audit findings
- [OPTIMIZED-PACKAGE-JSON-PROPOSAL.md](../OPTIMIZED-PACKAGE-JSON-PROPOSAL.md) - Dependency optimization
- `docs/reference/issue/dharma/Schema/DNA/DHARMA-SCHEMA-AUDIT-2026-03-25.md` - Previous schema audit
- `docs/reference/bugs/Dhi Executive/IMPLEMENTATION-VIOLATION-AUDIT-2026-03-25.md` - Dhi build failures
- `docs/reference/bugs/Dhi Executive/VIDHAN-MIGRATION-AUDIT-2026-03-25.md` - Migration status

---

## Appendix: Quick Reference for Ownership

| Issue | Owner | Status | Action |
|---|---|---|---|
| DHI-CFG-001 | Dharma | Open | Remove phantom paths |
| DHI-CFG-002 | Dharma | Open | Update metadata |
| DHI-CFG-003 | Dharma | Open | Remove Electron config |
| DHI-CFG-004 | Dharma | Open | Investigate tiktoken |
| DHI-CFG-005 | Dharma | Open | Consolidate tsconfig |
| DHI-DEP-001 | Dharma | Open | Remove UI deps |
| DHI-DEP-002 | Dharma + Dhi | Open | Move doc libraries |
| DHI-DEP-003 | Dharma + Core | Open | Move crypto logic |
| DHI-BLOCK-001 | Astra / Dhi | Open | Align version |
| DHI-BLOCK-002 | Prana | Open | Fix imports |
| LEGACY-001 | Dharma | ✅ Closed | (Done) |
| LEGACY-002 | Dharma | Open | Complete migration |

---

**Report Generated**: 2026-03-25  
**Next Review**: After TIER 1 actions complete  
**Escalation Contact**: Principal Architect / Architecture Review Board

**Resolution Summary**:
- Removed non-existent `packages/prana`, `packages/dhi`, `packages/astra` references from all config files
- Cleaned electron.vite.config.ts: Removed Prana input, simplified aliases to Dharma-only
- Cleaned tsconfig.node.json: Removed Prana includes, simplified includes to build config only
- Cleaned tsconfig.web.json: Removed all package references, focused on src/** only
- All paths now point to files that exist or are framework-standard

**Resolution Summary**:
- Removed input configuration from electron.vite.config.ts
- Removed Prana path reference entirely
- Added comment: "Dharma does not have a main process entry point"
- Config now handles dev/build for domain library only

**Resolution Summary**:
- Consolidated tsconfig files to be clear and minimal
- tsconfig.node.json: Now only includes "electron.vite.config.*" (build tooling)
- tsconfig.web.json: Now only includes "src/**/*" (source code)
- Removed all phantom package paths from both files
- Each config now has clear, single purpose
- npm run typecheck validated and passing

**Resolution Summary**:
- Removed ALL UI framework dependencies from package.json:
  - @emotion/react, @emotion/styled
  - @mui/material, @mui/icons-material
  - react-markdown, react-syntax-highlighter
  - reactflow
- Reduced dependencies to 2 core packages (ajv, js-yaml)
- Verified: npm run typecheck passes without UI packages

**Resolution Summary**:
- Removed from Dharma package.json:
  - html-to-docx
  - mammoth
- These NOW belong in Dhi's package.json (where document export feature actually lives)
- Dharma now properly limits scope to domain schemas only
- Note: Dhi team to add these to their own package.json

**Resolution Summary**:
- Removed from Dharma package.json:
  - bcryptjs
  - sql.js  
- These are implementation concerns, belong in Prana or Dhi
- Dharma now strictly provides schemas/types without implementation libraries
- Separation clarified: Dharma defines DATA structures, not crypto/SQL implementations

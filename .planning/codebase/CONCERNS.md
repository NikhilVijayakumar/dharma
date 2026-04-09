# Codebase Concerns

**Analysis Date:** 2026-04-09

## Technical Debt

**Complex Module Design in `src/registry/loader/company.ts`**

- Issue: The `loadCompanyMetadata` function loads 7 separate catalog files sequentially with no parallelization
- Files: `src/registry/loader/company.ts` (lines 84-112)
- Impact: Loading company metadata is slow - each `loadMetadataCatalog` call blocks on file I/O. With 7 files loaded sequentially, this adds latency
- Fix approach: Use `Promise.all` to load catalogs in parallel since they're independent operations

**YAML Parsing Without Strict Validation**

- Issue: Workflows and protocols use both JSON and YAML formats with loose parsing via `js-yaml`
- Files: `src/registry/loader/workflows.ts`, `src/registry/loader/protocols.ts`
- Impact: YAML parsing errors may not surface until runtime, potentially causing silent failures
- Fix approach: Add try-catch with schema validation for parsed YAML content

**Implicit Fallback IDs in Workflow Loading**

- Issue: Workflow loader creates fallback agent IDs from path parts without validation
- Files: `src/registry/loader/workflows.ts` (lines 70-76)
- Impact: Workflows may load with incorrect or missing agent_id if the field is absent
- Fix approach: Require `agent_id` field or fail explicitly

**Mixed Use of CommonJS and ESM**

- Issue: Package uses ESM (`"type": "module"`) but `validation.ts` uses `require()` in init function
- Files: `src/registry/loader/index.ts` (lines 95-97)
- Impact: Works in current setup but may break with different bundlers or Node versions
- Fix approach: Use dynamic `import()` instead of `require()`

## Registry Complexity Concerns

**High Protocol Count**

- Issue: Registry contains 63 protocol YAML files with potential overlap in behavioral mandates
- Files: `src/registry/protocols/*.yaml`
- Impact: Managing protocol updates becomes complex; conflicts may go unnoticed
- Fix approach: Implement protocol versioning and dependency tracking

**Large Workflow Count**

- Issue: Registry contains 57+ workflow YAML files across multiple agent directories
- Files: `src/registry/workflows/**/*.yaml`
- Impact: Significant storage footprint; workflow dependencies are not enforced
- Fix approach: Add workflow dependency validation to `src/registry/scripts/validate-registry.ts`

**Schema Validation is Optional**

- Issue: JSON schemas exist but validation only triggers if `$schema` field is present in data files
- Files: `src/registry/loader/validation.ts`, `src/registry/loader/registry.ts` (lines 31-42)
- Impact: Malformed registry files may load without errors; broken schemas silently accepted
- Fix approach: Make schema validation mandatory or at least warn when $schema is missing

## Security Considerations

**Path Resolution Uses process.cwd()**

- Issue: Registry root falls back to `process.cwd()` when other methods fail
- Files: `src/registry/loader/paths.ts` (lines 52-57)
- Current mitigation: Generally safe since this is a library consumed by applications
- Recommendations: Document that applications should set `REGISTRY_ROOT` explicitly in production

**No Input Sanitization on File Paths**

- Issue: Path resolution doesn't validate against path traversal attacks
- Files: `src/registry/loader/paths.ts`
- Risk: If registry paths come from untrusted sources, could access unintended files
- Recommendations: Add validation that resolved paths stay within registry root

## Known Issues

**BOM Character Handling**

- Issue: Code strips BOM (`^\uFEFF`) from files manually
- Files: Multiple loader files use `.replace(/^\uFEFF/, '')`
- Impact: Inconsistent handling; some files may have BOM issues that aren't caught
- Fix approach: Handle BOM at a single point in the loader pipeline

**Cache Never Expires**

- Issue: Schema and validator caches in `validation.ts` are never cleared
- Files: `src/registry/loader/validation.ts` (lines 8-10)
- Impact: Long-running processes may use stale schema versions after file updates
- Fix approach: Add cache invalidation on file modification detection or time-based expiry

## Fragile Areas

**Validation Script Uses Hardcoded Paths**

- Issue: `validate-registry.ts` uses relative `../` path instead of configurable root
- Files: `src/registry/scripts/validate-registry.ts` (line 19)
- Why fragile: Script won't work when run from different directories
- Safe modification: Accept registry root as CLI argument

**TypeScript Types Generated Manually**

- Issue: Types in `src/registry/types.ts` are hand-written, not auto-generated from JSON schemas
- Files: `src/registry/types.ts`
- Safe modification: Types may drift from schema definitions; maintain sync manually
- Test coverage: No runtime validation ensures types match schemas

## Missing Critical Features

**No Registry Versioning**

- Problem: There's no mechanism to version or migrate registry data between schema versions
- Blocks: Upgrading schemas requires manual migration

**No Circular Dependency Detection**

- Problem: Validations check references but don't detect circular dependencies between workflows
- Blocks: Workflows could create infinite loops in execution

## Test Coverage Gaps

**No Test Files in Repository**

- What's not tested: All loader functionality - path resolution, validation, file loading
- Files: Entire `src/registry/loader/` directory
- Risk: Silent failures in edge cases; broken references when adding new companies
- Priority: High

**No E2E Validation Script Execution**

- What's not tested: Running `validate-registry.ts` produces expected output
- Files: `src/registry/scripts/validate-registry.ts`
- Risk: Script may be broken; no CI integration
- Priority: Medium

---

_Concerns audit: 2026-04-09_

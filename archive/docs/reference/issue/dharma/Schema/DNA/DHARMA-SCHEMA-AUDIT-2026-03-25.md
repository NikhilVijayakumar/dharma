# DHARMA Schema Audit (2026-03-25)

## Scope
- Source reviewed from dependency tree: `e:/Python/dhi/node_modules/dharma/src`
- Objective: verify Dharma shared-library compliance with split model and dependency isolation

## Findings Summary
- No direct import-level dependency violations were detected in this audit scope.
- No references to `@dhi` or `@vidhan` imports were found in Dharma source scan.

## Status
- Current status: Pass (for the specific checks executed)
- Risk level: Low

## Residual Risk / Follow-up
- This audit focused on dependency-coupling signals and obvious app-specific coupling patterns.
- Recommended next step for deeper assurance:
  - run Dharma-native typecheck/lint inside `e:/Python/dharma` with its own lockfile/environment;
  - validate schema/runtime docs alignment against current `registry` and `schemas` exports.

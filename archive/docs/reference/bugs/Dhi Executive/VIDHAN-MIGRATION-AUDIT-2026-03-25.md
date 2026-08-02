# BUG-VIDHAN-MIGRATION: Repository Split Drift Audit

Date: 2026-03-25
Scope: `e:/Python/dhi` with local sibling repositories `e:/Python/astra`, `e:/Python/dharma`, `e:/Python/prana`

## Summary
Legacy `Vidhan` references remained after the package split. The highest-risk references were in active runtime/build configuration (`@vidhan` aliases and imports). These were removed from Dhi, Dharma, and Prana active configs.

## Confirmed Fixes

### Dhi (runtime/build)
- Removed renderer alias `@vidhan` from `electron.vite.config.ts`.
- Removed `packages/vidhan/**/*` include and `@vidhan/*` path mapping from `tsconfig.web.json`.
- Removed all `@vidhan` imports and routes from `src/renderer/src/main.tsx`.
- Updated `README.md` to reflect standalone Dhi + git dependencies (`astra`, `dharma`, `prana`) and bug/issue routing rules.
- Updated `scripts/distribute-docs.ps1` to remove Vidhan package distribution stage.
- Updated `scripts/categorize-bugs.ps1` to classify legacy Vidhan-tagged items under `[BUG-VIDHAN-MIGRATION]`.

### Dharma (local repo)
- Removed `@vidhan` alias from `electron.vite.config.ts`.
- Removed `packages/vidhan/**/*` include and `@vidhan/*` path mapping from `tsconfig.web.json`.
- Removed explicit Vidhan import rule reference from `README.md`.

### Prana (local repo)
- Removed `@vidhan` alias from `electron.vite.config.ts`.
- Removed `packages/vidhan/**/*` include and `@vidhan/*` path mapping from `tsconfig.web.json`.
- Removed explicit Vidhan import rule reference from `README.md`.
- Updated legacy documentation and UI strings/comments that still referenced Vidhan:
  - `docs/system/modular-architecture.md`
  - `src/ui/layout/MainLayout.tsx`
  - `src/ui/splash/view/SplashView.tsx`
  - `src/services/ipcResponseFactory.ts`
  - `src/ui/shared-components/PlaceholderPage.tsx`

## Remaining Legacy Mentions (Non-runtime)
The following are still expected to contain historical Vidhan references and should be treated as archived migration context rather than active wiring:
- `docs/system/consolidation-audit.md` (historical monorepo analysis)
- Temporary migration scripts in root (`tmp-*.mjs`) that mention `packages/vidhan`
- Vendored/legacy package snapshots under `packages/` (if retained for archival reasons)

## Risk Note
Removing `@vidhan` routes from Dhi removes those screens from this app runtime. This is intentional for split compliance, but product teams should confirm if equivalent features now live in a separate Vidhan app.

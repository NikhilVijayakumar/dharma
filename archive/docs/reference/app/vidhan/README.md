# VIDHAN Documentation Split

- Type: Application
- Scope: Administration app. Contains modules not available in Dhi and consumes prana+dharma.
- Source baseline: docs/monorepo/* (commit c8f0a2617aab71add9f093044d0dda1c2854602e)

## Folder Map
- module/: feature/module documentation mapped to vidhan
- core/: coding/runtime conventions relevant to vidhan
- system/: architecture/system docs relevant to vidhan
- bugs/: historical monorepo bug docs mapped to vidhan

## Ownership Rule
- Dhi and Vidhan are apps.
- Prana and Dharma are shared libraries.
- Any app-facing module not available in Dhi is mapped to Vidhan.
- Astra is intentionally excluded from this split.


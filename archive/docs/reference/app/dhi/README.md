# DHI Documentation Split

- Type: Application
- Scope: Director Office app. Contains executive features and consumes prana+dharma.
- Source baseline: docs/monorepo/* (commit c8f0a2617aab71add9f093044d0dda1c2854602e)

## Folder Map
- module/: feature/module documentation mapped to dhi
- core/: coding/runtime conventions relevant to dhi
- system/: architecture/system docs relevant to dhi
- bugs/: historical monorepo bug docs mapped to dhi

## Ownership Rule
- Dhi and Vidhan are apps.
- Prana and Dharma are shared libraries.
- Any app-facing module not available in Dhi is mapped to Vidhan.
- Astra is intentionally excluded from this split.


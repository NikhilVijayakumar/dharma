# PRANA Documentation Split

- Type: Library
- Scope: Core engine/shared runtime library consumed by both Dhi and Vidhan.
- Source baseline: docs/monorepo/* (commit c8f0a2617aab71add9f093044d0dda1c2854602e)

## Folder Map
- module/: feature/module documentation mapped to prana
- core/: coding/runtime conventions relevant to prana
- system/: architecture/system docs relevant to prana
- bugs/: historical monorepo bug docs mapped to prana

## Ownership Rule
- Dhi and Vidhan are apps.
- Prana and Dharma are shared libraries.
- Any app-facing module not available in Dhi is mapped to Vidhan.
- Astra is intentionally excluded from this split.


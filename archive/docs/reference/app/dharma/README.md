# DHARMA Documentation Split

- Type: Library
- Scope: Virtual employee and schema/governance library consumed by both Dhi and Vidhan.
- Source baseline: docs/monorepo/* (commit c8f0a2617aab71add9f093044d0dda1c2854602e)

## Folder Map
- module/: feature/module documentation mapped to dharma
- core/: coding/runtime conventions relevant to dharma
- system/: architecture/system docs relevant to dharma
- bugs/: historical monorepo bug docs mapped to dharma

## Ownership Rule
- Dhi and Vidhan are apps.
- Prana and Dharma are shared libraries.
- Any app-facing module not available in Dhi is mapped to Vidhan.
- Astra is intentionally excluded from this split.


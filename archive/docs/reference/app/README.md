# App/Library Documentation Split (Monorepo Baseline)

This folder splits the original monorepo documentation into app/library ownership buckets.

Targets:
- docs/app/dhi
- docs/app/vidhan
- docs/app/prana
- docs/app/dharma

Rules used:
- Dhi and Vidhan are application-level docs.
- Prana and Dharma are shared libraries.
- Features absent in Dhi are allocated to Vidhan.
- Astra is intentionally excluded from this split.

Out-of-scope historical docs (Astra-owned in monorepo):
- GAP-009-registry-ui-contract-mismatch.md
- GAP-011-localization-and-hardcoded-ui-strings.md
- GAP-022-typography-api-limitations.md

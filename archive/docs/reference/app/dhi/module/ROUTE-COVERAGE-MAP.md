# Dhi Route to Monorepo Module Coverage Map

Purpose:
- Reconcile current Dhi route names with original monorepo module document names.
- Prevent false negatives during split-completeness audits.

## Mapping

- `/triage` -> `triage-incident-routing.md` + `triage-incident-resolution.md`
- `/suites` -> `executive-suites.md`
- `/daily-brief` -> `report-daily-brief.md`
- `/funding-digest` -> `report-funding-digest.md`
- `/weekly-review` -> `report-weekly-review.md`
- `/notifications` + `/notification-centre` -> `notification-centre.md`
- `/profile/:id` -> `virtual-employee-dashboard.md`

## Shared-library-backed routes in Dhi

These are rendered in Dhi runtime but owned by shared libraries and therefore documented under Prana split docs:

- `/vault` -> `docs/app/prana/module/vault-folder-structure.md` and `docs/app/prana/module/vault-knowledge-repository.md`
- `/vault-knowledge` -> `docs/app/prana/module/vault-knowledge-repository.md`
- `/infrastructure` + `/infrastructure-layers` -> `docs/app/prana/module/infrastructure-layers.md`
- `/onboarding*` routes -> onboarding module docs under `docs/app/prana/module/`
- `/viewer-markdown` + `/viewer-pdf` -> viewer module docs under `docs/app/prana/module/`

## Conclusion

No uncovered Dhi route-level feature was found relative to original monorepo module documentation after applying this route-name reconciliation.

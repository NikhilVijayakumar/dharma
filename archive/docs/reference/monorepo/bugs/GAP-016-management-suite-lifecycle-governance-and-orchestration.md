# GAP-016: Management Suite Lifecycle Governance and Orchestration

## Scope
Post-onboarding lifecycle governance, draft-vs-vault controls, and cron/queue resiliency.

## Current Gaps
1. Lifecycle write path bypasses approval loop.
- Current behavior: profile/skill/KPI/data-input updates write directly through lifecycle save handlers.
- Risk: no explicit Director approval gate before durable overwrite.

2. No persistent draft-vs-vault queue for lifecycle entities.
- Current behavior: UI shows current/proposed text but draft state is not managed as durable governance records for approval/rejection.
- Risk: no formal pending/rejected/approved lifecycle trail.

3. Cron creation path lacks proposal-approval state machine.
- Current behavior: cron jobs are upserted directly into active schedule store.
- Risk: repetitive automation can be activated without explicit governance approval.

4. Queue resiliency is in-memory for execution queue.
- Current behavior: queue service is process memory only.
- Risk: interrupted runs on shutdown are not recovered from durable queue state.

5. Missing unified task audit log for scheduler/queue governance actions.
- Current behavior: no dedicated management-suite audit feed for cron proposal approvals, failures, overrides, and recovery actions.
- Risk: weak operator visibility and incident forensics.

6. Documentation mismatch.
- Missing docs/module/management-suite.md.
- Missing docs/system/queue-orchestration.md.
- Existing docs claim SQLite queue semantics not matched by implementation.

## Fix Plan
1. Add SQLite-backed lifecycle governance service for staged drafts and approval actions.
2. Redirect lifecycle save operations to stage-only (PENDING) records.
3. Add explicit Director approval and rejection API for lifecycle staged drafts.
4. Add SQLite-backed orchestration queue store for cron proposal queue, pending tasks, interrupted-task recovery, and audit logs.
5. Update cron scheduler to enqueue and recover missed/interrupted tasks on restart.
6. Expose task audit and lifecycle draft queue in management suite UI.
7. Add docs/module/management-suite.md and docs/system/queue-orchestration.md aligned to implemented behavior.

# Management Suite

## Purpose

Management Suite is the governance-first control plane for post-onboarding changes.
Execution screens consume approved state only. Any edits to lifecycle entities are staged as drafts and require Director review before they are committed to the vault template.

## Scope

- Lifecycle profile changes for virtual employees.
- Global skill markdown updates.
- KPI target/value overrides.
- Data input content override updates.
- Creation of new data input definitions.
- Cron schedule proposals.
- Queue audit and orchestration visibility.

## Governance Model

### Draft Versus Vault

- `PENDING`: Proposed change exists in SQLite draft/proposal store.
- `APPROVED`: Director accepted change; system applies it to vault/scheduler.
- `REJECTED`: Director rejected change.
- `OVERRIDDEN`: Director superseded change.

### Clean Execution Principle

- Execution modules must not mutate lifecycle templates directly.
- Mutation APIs stage proposals in governance storage.
- Approval APIs perform final commit to system state.

## API Surface

### Lifecycle

- `operations:update-lifecycle-profile` -> stage profile draft.
- `operations:update-lifecycle-skill` -> stage skill draft.
- `operations:update-lifecycle-kpi` -> stage KPI draft.
- `operations:update-lifecycle-data-input` -> stage data input content draft.
- `operations:create-lifecycle-data-input` -> stage new data input definition draft.
- `operations:list-lifecycle-drafts` -> view draft queue.
- `operations:review-lifecycle-draft` -> approve/reject/override.

### Scheduling and Queue Governance

- `operations:create-cron-proposal` -> submit new cron schedule proposal.
- `operations:list-cron-proposals` -> view cron proposal queue.
- `operations:review-cron-proposal` -> approve/reject/override cron proposal.
- `operations:get-task-audit-log` -> list queue execution audit events.

## Persistence

Management Suite governance data is persisted in SQLite via `governance-lifecycle-queue.sqlite`:

- `lifecycle_drafts`
- `cron_proposals`
- `task_queue`
- `task_audit_log`

This allows restart-safe governance review state and queue recovery support.

## UI Review Interface

### ReviewActionModal Component
Centralized modal for all governance review actions (approve/reject/override).

**Features:**
- Shows entity summary (type, ID, name, proposed JSON).
- **Approve Button**: Optional reviewer note allowed. Always enabled.
- **Reject Button**: Required reviewer note (mandatory minimum 4 characters). Disabled until note provided.
- **Override Button**: Optional note for audit trail.

**Usage Pattern:**
1. User clicks approve/reject on a draft or proposal card.
2. Modal opens showing entity summary.
3. User provides optional feedback (approve) or mandatory feedback (reject).
4. Modal validates and calls review action with `reviewNote`.
5. On success: queue refreshes, modal closes, card removed or status updated in historical view.

**Purpose:**
- Enforce feedback collection on rejections so AI/operators understand improvement opportunities.
- Optional feedback on approvals for audit trail enhancement.
- Single UX pattern across all review flows (lifecycle, schedules, future queues).

### Status Filtering for Draft/Proposal Queues
Both lifecycle draft queue (lifecycle manager page) and cron proposal queue (settings view) support status filtering via MUI Tabs:

**Tab Options:**
- `PENDING`: Awaiting Director review.
- `APPROVED`: Director approved; change applied to system.
- `REJECTED`: Director rejected; change not applied.
- `OVERRIDDEN`: Director replaced/superseded existing change.

**Behavior:**
- Default tab: PENDING (most common operational need).
- Tab change: Re-fetches queue for selected status (no full-page reload).
- Filters respect same underlying API (supports `listLifecycleDrafts(status)`, `listCronProposals(status)`).

### Queue Orchestration Page (Future)
Planned consolidated view combining:
- Lifecycle drafts (all statuses, filterable).
- Cron proposals (all statuses, filterable).
- Task audit log (read-only, paginated).

**Advanced Features (deferred):**
- Unified search across all queues.
- Multi-select bulk actions (approve/reject multiple items).
- Export audit trail (CSV/JSON).

## Integration Points

### Lifecycle Manager Page
- Path: `src/renderer/src/features/settings/pages/AgentLifecycleManagerPage.tsx`
- Displays: Lifecycle draft queue (default PENDING status).
- Actions: Approve/Reject drafts via ReviewActionModal.
- Status filtering: Tabs to view APPROVED/REJECTED/OVERRIDDEN history.

### Settings View (Governance Tab)
- Path: `src/renderer/src/features/settings/view/SettingsView.tsx`
- Displays: Cron proposal queue + Task audit log.
- Actions: Approve/Reject proposals via ReviewActionModal.
- Status filtering: Tabs to view proposal history.

### Reviewer Feedback Logging
- All approve/reject/override actions recorded in task audit log with:
  - Action type (APPROVED_LIFECYCLE_DRAFT, REJECTED_CRON_PROPOSAL, etc.).
  - Reviewer name (currently hardcoded to "DIRECTOR").
  - Review timestamp.
  - reviewNote (reason for decision).
  - Affected entity details.

### Validation & Error Handling
- Client-side: Modal prevents reject submission if note empty.
- Server-side: IPC handler validates status transition rules.
- Recovery: On rejection, entity remains in SQLite for future re-proposal.
- Audit trail: All actions (including validation failures) logged.

## Accessibility & Localization

All UI text is localized via `src/renderer/src/localization/i18n.ts`:
- `settings.review.modal.title`
- `settings.review.modal.approveLabel`
- `settings.review.modal.rejectLabel`
- `settings.review.modal.noteLabel`
- `settings.review.modal.noteRequired`
- `settings.review.modal.noteMinLength`
- Plus queue-specific labels (schedule proposal queue, lifecycle draft queue, task audit log).

Modal uses MUI accessible components (Dialog, TextField, Button) for keyboard navigation and screen reader support.

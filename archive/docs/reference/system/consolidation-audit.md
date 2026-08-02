# Cross-Package Consolidation & UI Purity Audit

This report identifies every component, domain model, and utility that should be extracted from Dhi and Vidhan into the three shared packages: Dharma (data), Prana (engine), and Astra (UI).

## Dependency Integrity — Current State

| Rule | Status | Evidence |
|---|---|---|
| Dhi & Vidhan never import from each other | ✅ PASS | `grep '@dhi' packages/vidhan/` and `grep '@vidhan' packages/dhi/` — zero results |
| Prana never imports from Dhi or Vidhan | ✅ PASS | `grep '@dhi\|@vidhan' packages/prana/` — zero results |
| Dharma imports zero UI packages | ✅ PASS | `grep '@prana\|@dhi\|@vidhan\|@astra' packages/dharma/` — zero results |
| Astra imports zero business packages | ✅ PASS | `grep '@prana\|@dhi\|@vidhan' packages/astra/` — zero results (fixed in prior audit) |

---

## 🏢 Dharma Audit — Shared Domain Models

**Problem**: Domain interfaces are defined inline in each feature's repo file. When both Dhi and Vidhan need the same data shape, each defines its own copy.

### Extraction Candidates

| Interface | Current Location | Consumers | Priority |
|---|---|---|---|
| `AuditLogEntry` | `vidhan/governance/repo/GovernanceRepo.ts` | Governance, Compliance, QueueMonitor | HIGH |
| `GovernanceDecision` | `vidhan/governance/repo/GovernanceRepo.ts` | Governance UI, potential Dhi oversight views | HIGH |
| `QueueTask` | `vidhan/queue-monitor/repo/QueueMonitorRepo.ts` | QueueMonitor, potential Dhi dashboard | HIGH |
| `QueueSubagentTreeNode` | `vidhan/queue-monitor/repo/QueueMonitorRepo.ts` | QueueMonitor subagent tree view | MEDIUM |
| `TriageItem` | `dhi/triage/repo/TriageRepo.ts` | Triage UI only (Dhi-specific) | LOW |
| `TriageMemoryHit` | `dhi/triage/repo/TriageRepo.ts` | Triage memory search (Dhi-specific) | LOW |
| `CompliancePayload` | `vidhan/compliance/repo/ComplianceRepo.ts` | Compliance UI, potential Dhi dashboard | MEDIUM |
| `DesignAuditPayload` / `AuditMetric` | `vidhan/design-audit/repo/DesignAuditRepo.ts` | Design audit, potential Dhi oversight | MEDIUM |
| `Candidate` / `HiringSimPayload` | `vidhan/hiring-sim/repo/HiringSimRepo.ts` | HiringSim only (Vidhan-specific) | LOW |
| `SettingsPayload` | `vidhan/settings/repo/SettingsRepo.ts` | Settings (shared concept) | MEDIUM |
| `SyncStatusSnapshot` | `vidhan/settings/repo/SettingsRepo.ts` | Sync health widget | MEDIUM |
| `ScheduleEntry` / `CronProposalEntry` | `vidhan/settings/repo/SettingsRepo.ts` | Cron management | HIGH |
| `RuntimeChannelConfigurationPayload` | `vidhan/settings/repo/SettingsRepo.ts` | Channel config | MEDIUM |

### Recommended Action

Create `packages/dharma/schemas/domain/` with these type modules:

```
packages/dharma/schemas/domain/
├── governance.ts      # AuditLogEntry, GovernanceDecision
├── queue.ts           # QueueTask, QueueSubagentTreeNode
├── compliance.ts      # CompliancePayload
├── cron.ts            # ScheduleEntry, CronProposalEntry, TaskAuditLogEntry
├── settings.ts        # SettingsPayload, SyncStatusSnapshot
├── channel.ts         # RuntimeChannelConfigurationPayload
└── index.ts           # Re-exports
```

Dhi-specific interfaces (`TriageItem`, `TriageMemoryHit`) and Vidhan-specific ones (`Candidate`, `HiringSimPayload`, `DesignAuditPayload`) should **remain** in their feature repos since they have single consumers.

---

## ⚡ Prana Audit — Shared Engine Utilities

### 1. Repo Boilerplate Factory

**Problem**: Every repo in both Dhi (7 repos) and Vidhan (6 repos) wraps `window.api.*` calls with identical `ServerResponse` boilerplate:

```typescript
return {
  isSuccess: true,
  isError: false,
  status: HttpStatusCode.SUCCESS,
  statusMessage: 'Loaded',
  data: payload,
} as ServerResponse<T>;
```

**Action**: Create `@prana/services/ipcResponseFactory.ts`:

```typescript
import { HttpStatusCode, ServerResponse } from 'astra';

export const successResponse = <T>(data: T, message = 'Loaded'): ServerResponse<T> => ({
  isSuccess: true,
  isError: false,
  status: HttpStatusCode.SUCCESS,
  statusMessage: message,
  data,
});

export const failureResponse = <T>(message: string): ServerResponse<T> => ({
  isSuccess: false,
  isError: true,
  status: HttpStatusCode.SERVER_ERROR,
  statusMessage: message,
  data: null as unknown as T,
});
```

**Impact**: Eliminates ~130 lines of boilerplate across 13 repo files.

### 2. Global UI State Hooks

Currently correctly placed. The `volatileSessionStore` was moved to Astra in the prior audit. The `useLifecycle` hook is properly in `@astra/common/state/LifecycleProvider` and consumed by both packages.

**No action needed** — Prana's auth hooks (`useAuth` equivalent via `useVolatileSessionStore`) and Astra's lifecycle hooks are already centralized.

---

## 🛠️ Astra Audit — Shared UI Components

### 1. ReviewActionModal → Astra (HIGH)

**Current**: `vidhan/features/settings/components/ReviewActionModal.tsx` (235 lines)

**Problem**: This is a generic governance review modal (Approve/Reject with notes). It's used in Vidhan's Agent Lifecycle Manager but the **identical pattern** is needed in Dhi for Director-level approval flows (cron proposals, governance decisions).

**Action**: Move to `@astra/components/ui/ReviewActionModal.tsx`. Both packages import from Astra.

### 2. SyncHealthWidget → Astra (MEDIUM)

**Current**: `vidhan/features/settings/components/SyncHealthWidget.tsx`

**Problem**: Sync health is a system-wide concern. The Director may want to view sync status on their dashboard.

**Action**: Move to `@astra/components/ui/SyncHealthWidget.tsx`.

### 3. Toast/Snackbar Pattern → Astra Hook (HIGH)

**Problem**: Both Dhi and Vidhan re-implement the same toast notification pattern:

```typescript
const [toast, setToast] = useState<{ severity: 'success' | 'error'; message: string } | null>(null);
// ... later in JSX:
<Snackbar open={Boolean(toast)} autoHideDuration={2600} onClose={() => setToast(null)}>
  <Alert severity={toast?.severity} onClose={() => setToast(null)}>{toast?.message}</Alert>
</Snackbar>
```

Found in: `AgentLifecycleManagerPage.tsx`, `EmployeeProfileView.tsx` (and likely in future features).

**Action**: Create `@astra/common/hooks/useToast.ts` + `@astra/components/ui/ToastProvider.tsx`:

```typescript
// useToast.ts
export const useToast = () => {
  const [toast, setToast] = useState<ToastState | null>(null);
  const showSuccess = (message: string) => setToast({ severity: 'success', message });
  const showError = (message: string) => setToast({ severity: 'error', message });
  const dismiss = () => setToast(null);
  return { toast, showSuccess, showError, dismiss };
};
```

### 4. Inline Data-Table vs Astra DataTable (MEDIUM)

**Problem**: Astra already has `DataTable.tsx` (a proper MUI Table component), but Dhi's `TriageView.tsx` builds its own inline data-table with `Box`-based row layouts instead of using the existing `DataTable`.

**Action**: Refactor `TriageView` to use `@astra/components/ui/DataTable` with action-column renderers instead of the custom inline layout (lines 99–161).

### 5. Search/Filter Panel → Astra (MEDIUM)

**Problem**: Both Triage (Dhi) and QueueMonitor (Vidhan) have search/filter panels with `TextField` + `Button` patterns for querying data. They also share a "memory hit" / "context event" list pattern.

**Action**: Create `@astra/components/ui/SearchFilterPanel.tsx` — a reusable panel with:
- Search input with Enter-key submission
- Action button
- Results list with configurable row renderer

### 6. Placeholder Container Pattern (LOW)

**Problem**: 26 containers across Dhi and Vidhan are identical placeholder stubs:

```tsx
<Box sx={{ p: 4, width: '100%' }}>
  <HeroSection headline="..." description="Module awaiting data and logic bindings. (...)" />
</Box>
```

**Action**: Create `@astra/components/ui/PlaceholderPage.tsx`:

```typescript
export const PlaceholderPage: FC<{ headline: string; code: string }> = ({ headline, code }) => (
  <Box sx={{ p: 4, width: '100%' }}>
    <HeroSection headline={headline} description={`Module awaiting data and logic bindings. (${code})`} />
  </Box>
);
```

---

## Deep-Dive Extraction Analysis

### Context Compaction UI

**Finding**: The "Context Compaction" UI currently lives only in Vidhan's QueueMonitor (`useQueueMonitorViewModel.ts` lines 25–53). It calls `window.api.contextEngine.prepareNewContext()` and `startNewWithContext()`.

**Recommendation**: Keep the **logic** in the QueueMonitor viewmodel (it's an administrative action). If the Director needs to see token reduction progress, create a **read-only** variant in Dhi's dashboard that displays `context.totalTokens`, `context.totalCompactions`, and `context.recentEvents` from the same IPC endpoint — no logic duplication needed.

### Entity Detail Pages (Agent/Skill/KPI)

**Finding**: Vidhan's `AgentLifecycleManagerPage` (760 lines) handles View/Edit for Agents, Skills, KPIs, and Data Inputs. Dhi's `EmployeeProfileView` (128 lines) handles read/edit for a single Agent profile.

**Key insight**: Both use `useLifecycle()` from `@astra/common/state/LifecycleProvider` — the state management is already shared. The UI difference is:
- **Dhi**: Single-profile focus with `DynamicProfileRenderer` (already in Astra)
- **Vidhan**: Multi-section manager with selectors for all entity types

**Recommendation**: `DynamicProfileRenderer` is already correctly in Astra. Extract the **KPI Editor Card** and **Skill Editor Card** patterns from `AgentLifecycleManagerPage` into Astra as standalone components (`KpiEditorCard`, `SkillEditorCard`) so both packages can compose them differently.

### Google Bridge / Doc Previewer

**Finding**: No Google-doc previewer component currently exists in the codebase. The `GoogleBridgeService` exists as a backend service in `packages/prana/main/services/` but has no dedicated UI component.

**Recommendation**: When this UI is built, it should go directly into `@astra/components/google/` since both the Director (viewing generated docs) and Admin (generating docs) need it.

---

## Summary Matrix

| Component | From | To | Impact | Priority |
|---|---|---|---|---|
| Domain interfaces (governance, queue, cron) | Feature repos | `@dharma/schemas/domain/` | Type safety across packages | HIGH |
| `ipcResponseFactory` | Inline in 13 repos | `@prana/services/` | ~130 lines eliminated | HIGH |
| `ReviewActionModal` | `@vidhan/features/settings/` | `@astra/components/ui/` | Shared governance UI | HIGH |
| `useToast` hook | Inline in 4+ views | `@astra/common/hooks/` | DRY principle | HIGH |
| `SyncHealthWidget` | `@vidhan/features/settings/` | `@astra/components/ui/` | Shared system status | MEDIUM |
| `SearchFilterPanel` | Inline in Triage/QueueMonitor | `@astra/components/ui/` | Reusable search UX | MEDIUM |
| `PlaceholderPage` | 26 duplicate containers | `@astra/components/ui/` | Code reduction | LOW |
| Triage inline data-table | `@dhi/features/triage/` | Use `@astra/DataTable` | Consistency | LOW |
| `KpiEditorCard` / `SkillEditorCard` | `@vidhan/AgentLifecycleManager` | `@astra/components/ui/` | Shared entity editing | MEDIUM |

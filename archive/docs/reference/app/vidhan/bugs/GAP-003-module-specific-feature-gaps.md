# GAP-003: Module-Specific Feature Gaps ΓÇö Documented Functionality vs. Implementation

**Severity**: HIGH  
**Category**: Feature Completeness  
**Date**: 2026-03-19  
**Status**: BLOCKED ON GAP-002 (operationsService refactoring)  
**Affects**: governance, vault, triage, executive-suites, infrastructure, compliance, employee-profiles, vault-knowledge, settings, onboarding, notification-centre  

---

## Summary

Each module document specifies detailed components, behaviors, and integrations. The actual implementations are minimal scaffolds that call IPC endpoints and render basic data displays. This document catalogs the specific missing functionality per module.

---

## 1. Governance Lab (`governance-lab.md` vs. `features/governance/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `NotionSidePeek` | Fixed 320px right panel for report preview (push-layout, internal scroll) | Γ¥î Not implemented |
| `AuditLogStream` | Virtual-scrolled monospace log from `audit_log.jsonl` | Γ¥î Not implemented |
| `CommitActionFooter` | Tactile snap animation + gold border on commit success | Γ¥î Not implemented |
| `DiffPreview` | Technical diff view (additions/deletions) before commit | Γ¥î Not implemented |

### Missing Functionality
- **No real git operations** ΓÇö No `git commit`, `git push`, `git add`, or `git diff` from governance UI
- **No `audit_log.jsonl` parsing** ΓÇö Audit logs are seeded from `operationsService.ts`, not read from the append-only JSONL file
- **No Eva AUDIT_PASS validation** ΓÇö `ComplianceScanService.ts` does not exist; no compliance gate before commits
- **No Julia GitGovernorService.ts** ΓÇö No `git push --force` blocking, no commit format enforcement (`DECISION: [Agent Name] - [Title]`)
- **No Mira context-synthesis** ΓÇö No aggregation of completed tasks into synthesis reports
- **No tactile commit feedback** ΓÇö No linear scale pulse (1.0 ΓåÆ 1.01 ΓåÆ 1.0 in Γëñ150ms)
- **Commit message format not enforced** ΓÇö Should be `DECISION: [Agent Name] - [Title]`

### Missing Visual Enforcement
- Gold border only on commit success ΓÇö not enforced
- Monospace audit log (IBM Plex Mono 11px) ΓÇö not implemented
- Virtual scroll for audit logs > 200 rows ΓÇö not implemented

---

## 2. Triage Module (`triage-module.md` vs. `features/triage/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `PriorityStack` | Exactly 3 decision cards with opacity dimming for non-focused items | Γ¥î Not implemented |
| `StatusTicker` | Bloomberg-style mono-density footer (32px, IBM Plex Mono) | Γ¥î Not implemented |
| `ActionShortcuts` | Global keyboard listener: 1=Commit, 2=Defer, 3=Archive | Γ¥î Not implemented |
| `DailyBriefView` | 8AM summary renderer within triage | Γ¥î Not implemented |

### Missing Functionality
- **No Eisenhower Matrix classification** ΓÇö Items have simple statuses (PENDING/ANALYSIS/CLEARED), not the documented Urgency/Importance scoring
- **No 3-Decision Rule enforcement** ΓÇö Max 3 visually dominant elements above fold is not enforced
- **No opacity dimming** ΓÇö Non-focused cards should dim to 40-60% opacity
- **No keyboard-first navigation** ΓÇö Superhuman-style 1/2/3 shortcuts not implemented
- **No real Queue System Protocol** ΓÇö No 10-slot queue buffer, no crisis slot reservation
- **No Lina CONTEXT_LEVEL integration** ΓÇö No `DEEP_FOCUS` / `QUICK_REVIEW` / `FATIGUE_HIGH` adjusting card verbosity
- **No SSH/engine health in StatusTicker**

### Missing Visual Enforcement
- Background `#0B0E14` ΓÇö may be correct but not verified against triage spec
- Gold accent Γëñ 8% pixel area ΓÇö no enforcement mechanism
- Monospace accent `#79C0FF` for Status Ticker ΓÇö not implemented

---

## 3. Executive Suites (`executive-suites.md` vs. `features/suites/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `RunwayGauge` | Tabular-numeric runway with Gold Alert when < 4 months | Γ¥î Not implemented |
| `ZeroFillChart` | Minimalist line chart ΓÇö no gradients, no area fills | Γ¥î Not implemented |
| `RiskSimulatorChart` | Projection graph with 200ms linear transitions | Γ¥î Not implemented |
| `AgentWorkspaceTab` | Per-agent tab rendering KPIs and processed output | Γ¥î Not implemented |
| `KPIBadge` | Status badge renderer (HEALTHY/WARNING/CRITICAL/STALE) | Γ¥î Not implemented |

### Missing Functionality
- **No agent workspace tabs** ΓÇö Should have 9 tabs (Finance, Operations, Technology, Marketing, Compliance, Design, Strategy, Funding, HR) mapping to the 10 virtual employees
- **No KPI registry integration** ΓÇö Should read from `/governance/kpi/registry.json`; entire KPI system is absent
- **No Risk Register** ΓÇö `risk_register.json` integration not implemented
- **No `/processed/{agent}/` output reading** ΓÇö Each agent's processed output directory doesn't exist in practice
- **Agent profiles are generic** ΓÇö Uses "The Overseer", "Security Architect", "Operations Coordinator" instead of Arya, Nora, Julia, Elina, Eva, Dani, Sofia, Lina, Maya, Mira

---

## 4. Infrastructure Layers (`infrastructure-layers.md` vs. `features/infrastructure/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `StabilityModeManager` | Crisis protocol activation (2-of-3 confirmation from Nora/Eva/Elina) | Γ¥î Not implemented |
| `SystemThemeContainer` | Root theme shell with crisis override (gold ΓåÆ safety red) | Γ¥î Not implemented |
| `SystemHealthMonitor` | RAM/CPU tracking with queue pause at 80%/90% thresholds | Γ¥î Not implemented |
| `SSHStatusIndicator` | Persistent footer SSH status indicator (green/red, no pulsing) | Γ¥î Not implemented |
| `IPCDataBridge` | IPC adapter with latency monitoring (< 16ms threshold) | Γ¥î Not implemented |
| `MemoryWarningBanner` | RAM > 80% notification | Γ¥î Not implemented |
| `CrisisModeOverlay` | Strips UI to diagnostics only during crisis | Γ¥î Not implemented |

### Missing Functionality
- **No Crisis Protocol** ΓÇö Full Stability Mode (accent shift, module restriction, motion reduction) not implemented
- **No memory protection** ΓÇö RAM/CPU thresholds for queue pausing not monitored
- **No IPC latency monitoring** ΓÇö No real measurement, just fabricated values
- **No custom titlebar** ΓÇö 32px titlebar with drag regions not verified
- **No pinch-to-zoom lock**

---

## 5. Vault Module (`vault-module.md` vs. `features/vault/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `ClinicalDropzone` | Stripe-inspired drag-and-drop with dashed borders | Γ¥î Not implemented (uses native dialog) |
| `ForensicParserTracker` | ASCII character rotator (`/ - \ |`) for row-level validation | Γ¥î Not implemented |
| `SchemaGuardianCheck` | GitHub-style per-column pass/fail check list | Γ¥î Not implemented |
| `ClassificationBadge` | T1-T4 visual tag on file cards | Γ¥î Not implemented |

### Missing Functionality
- **No drag-and-drop** ΓÇö Uses `dialog.showOpenDialog()` instead of clinical dropzone UI
- **No row-level validation progress** ΓÇö No ASCII rotator visible during parsing
- **No per-column validation display** ΓÇö Schema validation happens in main process but results are not displayed column-by-column in UI
- **No data classification display** ΓÇö T1-T4 tags not shown visually on file cards
- **No PII detection display** ΓÇö PII columns should be tagged immediately upon detection
- **Overlay width** ΓÇö Should be Γëñ 420px Raycast-style; actual implementation is full-page

---

## 6. Vault Knowledge Repository (`vault-knowledge-repository.md` vs. `features/vault-knowledge/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `VaultLockStatus` | Lock/unlock badge (green/red) | Γ¥î Not implemented |
| `VaultCommitDialog` | Diff view before git commit | Γ¥î Not implemented |
| `AgentTempPanel` | Pending agent outputs with approve/reject | Partially Γ£ô (approve/reject exists in IPC) |

### Missing Functionality
- **No vault password entry** ΓÇö Documentation says Director must enter vault password to unlock
- **No explicit unlock flow** ΓÇö Vault auto-hydrates from encrypted archive
- **No crash guard** ΓÇö Should clean up unencrypted temp on unexpected shutdown
- **No Platform Adapter abstraction** ΓÇö Documentation specifies `WindowsVaultAdapter` ΓåÆ `PlatformAdapter` pattern; code uses direct file ops

---

## 7. Settings (`settings.md` vs. `features/settings/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `EngineSelector` | LLM/model provider configuration UI | Γ¥î Not implemented |
| `SystemPreferences` | Accessibility, animations, theme overrides, localization | Γ¥î Not implemented |

### Missing Functionality
- **Only stores language** ΓÇö Settings save only `{ language: string }` (line 59 in ipcService)
- **No model gateway configuration** ΓÇö Cannot change provider order, API keys, or models from UI
- **No theme toggles** ΓÇö No dark/light mode switching
- **No accessibility controls** ΓÇö No reduced motion toggle
- **No API key management** ΓÇö Keys only configurable via `.env` file, not UI
- **No encryption of sensitive settings** ΓÇö Documented to encrypt API keys immediately upon entry

---

## 8. Onboarding (`onboarding.md` vs. `features/onboarding/`)

### Missing Functionality
- **No 5-step wizard** ΓÇö Company Profile, Product & Tech, Financial, Operations, Goals & Strategy questionnaire not implemented
- **No KPI Generation** ΓÇö No per-agent AI-generated KPIs via model gateway
- **No KPI Review** ΓÇö No editable KPI cards with approve/edit/remove actions
- **No `registry.json` creation** ΓÇö KPI registry is never created from onboarding data
- **No re-access from Settings** ΓÇö Cannot re-run onboarding from settings

---

## 9. Notification Centre (`notification-centre.md` vs. `features/notification-centre/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `notification-config` | Severity thresholds, quiet hours, per-agent filter toggles | Γ¥î Not implemented |

### Missing Functionality
- **No severity-based behavior** ΓÇö CRITICAL should be modal-level prominence; WARNING should be banner + feed; INFO should be feed only; LOW should be grouped in daily digest
- **No configurable thresholds** ΓÇö No quiet hours, no per-agent filtering
- **No CRITICAL modal push** ΓÇö Critical notifications are just list items, not modal-level prompts

---

## 10. Employee Profiles (`virtual-employee-profiles.md` vs. `features/employee-profiles/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `ProfileHero` | Avatar + triggers + name + role header | Γ¥î Not implemented |
| `BackstoryPanel` | Editable multi-paragraph character narrative | Γ¥î Not implemented |
| `WorkflowTable` | 4-stage protocol mapping display | Γ¥î Not implemented |
| `ToolsInventory` | 3-column Skills (blue) / Rules (amber) / Scripts (green) | Γ¥î Not implemented |
| `HandshakePanel` | Inbound/outbound agent connections | Γ¥î Not implemented |
| `KPITable` | Metric/Target/Source table | Γ¥î Not implemented |
| `CrisisProtocol` | Active/Inactive badge | Γ¥î Not implemented |
| `MentionAutocomplete` | @mention popup with fuzzy search | Γ¥î Not implemented |
| `ChatMessageBubble` | Avatar + message + attachment | Γ¥î Not implemented |
| `ChatUploadAttachment` | File attach + status display | Γ¥î Not implemented |
| `EmployeeAvatarUpload` | Avatar management with drag-drop | Γ¥î Not implemented |

### Missing Functionality
- **No editable persona fields** ΓÇö Name, Goal, Backstory, Workflow should be Director-editable
- **No agentic asset display** ΓÇö Skills, Rules, Services from `docs/virtual-employee/*.md` not rendered
- **No @mention system** ΓÇö No autocomplete, no fuzzy search, no trigger registry
- **No in-chat upload flow** ΓÇö No file attachment routing through Mira ΓåÆ Schema Guardian
- **No handshake visualization** ΓÇö Agent communication protocol connections not displayed

---

## 11. Compliance (`compliance.md` vs. `features/compliance/`)

### Missing Components
| Documented Component | Purpose | Status |
|:---------------------|:--------|:-------|
| `SLATracker` | Latency and resolution time monitoring per agent | Γ¥î Not implemented |

### Missing Functionality
- **No `ComplianceScanService.ts`** ΓÇö Documented as main process service; does not exist
- **No `AuditLogService.ts`** ΓÇö Documented as main process service; does not exist
- **No assertion registry** ΓÇö No real compliance assertion tracking
- **No SLA metrics** ΓÇö No per-agent/department bottleneck tracking
- **No real-time compliance feed** ΓÇö Data is a heuristic derivation from gateway + skill signals

---

## 12. Daily Brief (`daily-brief.md` vs. `features/daily-brief/`)

### Missing Functionality
- **No 8AM auto-generation** ΓÇö Cron scheduler exists but doesn't trigger actual Mira compilation
- **No Mira AI compilation** ΓÇö Brief is assembled from system telemetry, not from Mira processing through model gateway
- **No interactive approval queue** ΓÇö Documented YES/NO/DEFER buttons with 24h auto-expire not implemented
- **No agent KPI reading from `registry.json`** ΓÇö Function status summary should read real KPI data

---

## 13. Weekly Review (`weekly-review.md` vs. `features/weekly-review/`)

### Missing Functionality
- **No per-agent structured questionnaire** ΓÇö 3 standard questions + domain-specific questions per agent not implemented
- **No AI-generated agent reports** ΓÇö Reports are static templates with telemetry-derived text
- **No Mira compilation** ΓÇö No secretary aggregation into unified review
- **Agents don't answer questions** ΓÇö The 10 agents don't actually generate responses through the model gateway

---

## Root Cause Analysis

The 60+ missing components across modules are blocked by three upstream gaps:

| Blocker | Gap | Impact |
|:--------|:----|:-------|
| No real data sources | **GAP-002** | Can't build components that visualize non-existent data (e.g., `RunwayGauge` needs real funding data, `PriorityStack` needs real Eisenhower scores) |
| No virtual employees | **GAP-005** | Can't build agent-specific components (e.g., `AgentWorkspaceTab`, KPI badges specific to agent type) |
| No Clean Architecture | **GAP-001** | Components are inline in monolithic views; no ability to compose/reuse specialized widgets |

## Resolution

**Implementation Order (Dependency-aware):**

1. **Phase 1: Fix GAP-002** (operationsService) ΓÇö Extract real data sources
   - Implement `ComplianceScanService.ts`, `VisualAuditService.ts`, `SystemHealthService.ts`, etc.
   - Result: Real data starts flowing to modules
   - Enables: Component building for modules with real data

2. **Phase 2: Fix GAP-005** (Virtual Employees) ΓÇö Implement agent runtime
   - Create 10 agent TypeScript services
   - Implement Work Order system
   - Result: Agents can generate content for daily/weekly briefs
   - Enables: Agent-specific components and per-agent workspace tabs

3. **Phase 3: Build Specialized Components** (per GAP-003)
   - For each module, implement documented components in `presentation/components/`
   - Priority order: Governance ΓåÆ Triage ΓåÆ Suites ΓåÆ Compliance ΓåÆ Design Audit ΓåÆ Infrastructure ΓåÆ Others
   - Each component reads from real service (not hardcoded) and render specialized UX

4. **Phase 4: Fix GAP-001** (Architecture) ΓÇö Introduce Clean Architecture
   - Migrate components to domain/entities + use cases + data models
   - Full Clean Architecture adoption after Phase 3 stabilizes

**Current Workaround:**
- Modules display flat MVVM scaffolds with simulated data
- All 60+ specialized UI components are **not implemented**
- Modules are feature-complete for IPC wiring, but incomplete for user-facing UX

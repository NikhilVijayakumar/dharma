# GAP-001: Renderer Architecture Mismatch ΓÇö Flat MVVM vs. Documented Clean Architecture

**Severity**: HIGH  
**Category**: Architecture / Structural  
**Date**: 2026-03-19  
**Status**: DOCUMENTED AS MVP TRADE-OFF  
**Affects**: All 21 renderer feature modules  

---

## Summary

Every module document (`docs/module/*.md`) specifies a **Clean Architecture** folder structure with three layers ΓÇö `data/`, `domain/`, and `presentation/` ΓÇö containing entities, use cases, abstract repositories, and concrete repository implementations. The actual renderer implementation uses a **flat MVVM scaffold** with only 3-4 files per feature: `repo/`, `viewmodel/`, `view/`, and sometimes `view/*Container.tsx`.

## Expected (per docs)

Each module doc prescribes a structure like:

```
src/renderer/src/features/{module}/
Γö£ΓöÇΓöÇ data/
Γöé   Γö£ΓöÇΓöÇ repositories/
Γöé   Γöé   ΓööΓöÇΓöÇ {Module}RepositoryImpl.ts     # Concrete IPC-backed repo
Γöé   Γö£ΓöÇΓöÇ models/
Γöé   Γöé   ΓööΓöÇΓöÇ {Entity}DTO.ts               # Data transfer objects
Γöé   ΓööΓöÇΓöÇ validators/ or parsing/
Γö£ΓöÇΓöÇ domain/
Γöé   Γö£ΓöÇΓöÇ entities/
Γöé   Γöé   ΓööΓöÇΓöÇ {DomainEntity}.ts            # Core domain models
Γöé   Γö£ΓöÇΓöÇ usecases/
Γöé   Γöé   ΓööΓöÇΓöÇ {Action}UseCase.ts           # Business logic orchestration
Γöé   ΓööΓöÇΓöÇ repositories/
Γöé       ΓööΓöÇΓöÇ {Module}Repository.ts        # Abstract interface
ΓööΓöÇΓöÇ presentation/
    Γö£ΓöÇΓöÇ viewmodels/
    Γöé   ΓööΓöÇΓöÇ {Module}ViewModel.ts         # State management + domain binding
    Γö£ΓöÇΓöÇ components/
    Γöé   ΓööΓöÇΓöÇ {SpecializedWidget}.tsx       # Module-specific UI components
    ΓööΓöÇΓöÇ {Module}View.tsx                  # Main container
```

## Actual

Every renderer feature has a flat structure:

```
src/renderer/src/features/{module}/
Γö£ΓöÇΓöÇ repo/
Γöé   ΓööΓöÇΓöÇ {Module}Repo.ts         # Direct IPC caller, no abstraction
Γö£ΓöÇΓöÇ viewmodel/
Γöé   ΓööΓöÇΓöÇ use{Module}ViewModel.ts # React hook, no domain layer
Γö£ΓöÇΓöÇ view/
Γöé   Γö£ΓöÇΓöÇ {Module}Container.tsx   # Thin container
Γöé   ΓööΓöÇΓöÇ {Module}View.tsx        # Single monolithic view
```

**No feature has:**
- `domain/entities/` ΓÇö zero domain entity files
- `domain/usecases/` ΓÇö zero use case files  
- `domain/repositories/` ΓÇö zero abstract repository interfaces
- `data/models/` ΓÇö zero DTO files
- `data/validators/` ΓÇö zero validator files
- `presentation/components/` ΓÇö zero specialized UI components (everything in one View file)

## Impact

1. **No separation of concerns** ΓÇö Business logic is mixed into viewmodels and repositories.
2. **No testability** ΓÇö Without abstract interfaces and use cases, unit testing domain logic is impossible.
3. **No reusability** ΓÇö Domain entities like `Signal`, `DraftReport`, `ValidationResult`, `DataClassification` etc. are inline types, not reusable entities.
4. **Documentation is misleading** ΓÇö The folder structures in module docs do not match reality, making onboarding difficult.

## Affected Modules (all 21)

| Module | Documented Components | Actual Files |
|:-------|:---------------------|:-------------|
| governance | 8 components (DiffPreview, AuditLogStream, NotionSidePeek, etc.) | 4 files |
| vault | 6 components (ClinicalDropzone, ForensicParserTracker, SchemaGuardianCheck, etc.) | 4 files |
| triage | 6 components (PriorityStack, StatusTicker, ActionShortcuts, etc.) | 4 files |
| executive-suites (suites) | 7 components (RunwayGauge, ZeroFillChart, RiskSimulatorChart, etc.) | 4 files |
| infrastructure | 5 components (SSHStatusIndicator, MemoryWarningBanner, CrisisModeOverlay, etc.) | 3 files |
| compliance | 4 components (ComplianceDashboard, SLATracker, etc.) | 4 files |
| employee-profiles | 11 components (ProfileHero, ToolsInventory, MentionAutocomplete, etc.) | 3 files |
| vault-knowledge | 6 components (VaultExplorerView, AgentTempPanel, VaultCommitDialog, etc.) | 4 files |
| settings | 4 components (EngineSelector, SystemPreferences, etc.) | 4 files |
| onboarding | 3 components (wizard, generator, review) | 3 files |
| queue-monitor | 4 components (QueueSlotGrid, PipelineTracker, etc.) | 4 files |
| daily-brief | 2 components (brief-deck, approval-queue) | 4 files |
| weekly-review | 2 components (review-questionnaire, review-summary) | 4 files |
| funding-digest | 3 components (FundingDigest, etc.) | 4 files |
| hiring-sim | 3 components (HiringWizard, etc.) | 4 files |
| design-audit | 3 components (DesignAuditReport, etc.) | 4 files |
| notification-centre | 2 components (notification-feed, notification-config) | 4 files |
| splash | 4 components (InitSequence, BootStatusBar, etc.) | 4 files |
| dashboard | 3 components | 3 files |
| file-viewers | 4 viewer components | 4 files |
| authentication | 5 view files | 13 files Γ£ô |

## Resolution

### MVP Trade-Off (Current State)

The flat MVVM structure was chosen as a **deliberate MVP trade-off** to accelerate feature delivery:
- Single repo/viewmodel/view per feature enables rapid IPC wiring
- Minimal abstraction layers -> faster prototyping
- Acceptable for features that call fixed IPC endpoints with minimal business logic

### Design Debt Recognition

However, this trade-off creates technical debt:
1. **No separation of concerns** ΓÇö Business logic mixed into viewmodels
2. **No testability** ΓÇö Domain logic cannot be unit-tested in isolation
3. **No reusability** ΓÇö Domain entities like `Signal`, `DraftReport`, `ValidationResult` are inline types
4. **Misleading documentation** ΓÇö Module docs specify Clean Architecture but code doesn't match
5. **Harder maintenance** ΓÇö Future developers onboarding see mismatch between docs and code

### Remediation Path (Phase-based)

**Phase 1: Stabilization (Short-term)**
- Add a comment in each module's README explaining the MVP trade-off
- Document the "Future Architecture" section in each module spec
- No code changes required

**Phase 2: Foundation Exports (Medium-term)**
- For modules with real domain complexity (governance, triage, vault), extract:
  - Abstract repository interface ΓåÆ `domain/repositories/`
  - Domain entities ΓåÆ `domain/entities/`
  - Use case logic ΓåÆ `domain/usecases/`
- Keep existing repo/viewmodel/view but have them use the new abstractions

**Phase 3: Specialization (Long-term)**
- Implement `presentation/components/` for each module with documented specialized widgets
- This requires fixing GAP-002 (real data sources) first

**Phase 4: Full Migration (Production)**
- Once all modules have real data, migrate all to full Clean Architecture
- Target: 90+ specialized UI components deployed

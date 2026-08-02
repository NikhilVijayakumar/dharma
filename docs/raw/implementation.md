# Implementation Plan

## Table of Contents

> *Deterministic rules for this domain: `common/tier5/audit/deterministic/document/implementation.yaml`*

- [Purpose](#purpose)
- [Plan Scenarios](#plan-scenarios)
- [Generation Plan](#generation-plan)
- [Refactor Plan](#refactor-plan)
- [Change Request Plan](#change-request-plan)
- [Enhancement Plan](#enhancement-plan)
- [Security Fix Plan](#security-fix-plan)
- [Required Sections](#required-sections)
- [Goals](#goals)
- [Non-Goals](#non-goals)
- [Success Criteria](#success-criteria)
- [Responsibilities](#responsibilities)
- [Scope](#scope)
- [Out of Scope](#out-of-scope)
- [Inputs](#inputs)
- [Outputs](#outputs)
- [Traceability](#traceability)
- [Relationships](#relationships)
- [Required Characteristics](#required-characteristics)
- [Generation Rules](#generation-rules)
- [Enhancement Rules](#enhancement-rules)
- [Audit Rules](#audit-rules)
- [Validation Rules](#validation-rules)
- [Summary](#summary)
- [Common Mistakes](#common-mistakes)
- [Documentation Folder](#documentation-folder)
- [Usage](#usage)
- [Related](#related)
- [Revision History](#revision-history)
- [Crate Folder Structure](#crate-folder-structure)


---

## Purpose

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/03-purpose.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Purpose

This Implementation Plan documents the as-built record for [feature/system name]. It defines [scope], establishes boundaries via [out of scope], and traces implementation decisions back to [upstream documents].
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Architecture(05), Engineering(07), Security(03)

### Examples

**Correct:**
> This Implementation Plan documents the as-built record for the payment processing feature. It covers the payment flow from checkout to confirmation, excludes currency conversion and subscription billing, and traces every implementation decision back to Feature(04) payment requirements and Security(03) PCI-DSS constraints.

**Incorrect:**
> This section describes the purpose of the project and how we plan to build it.
> *Why wrong: Describes project intent rather than recording what was actually built; confuses Implementation Purpose with Vision or Feature documentation.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** State the feature name and scope explicitly; link every relationship to its upstream document number; define boundaries by listing what is excluded
- **Don't:** Describe project vision or strategy; use vague phrases like "various aspects"; conflate purpose with goals or success criteria

This document defines the standard for Implementation Plans — per-feature documents that record how code was generated, what deviations were made, and why.

Implementation is the generation point where all upstream documentation converges into working code. Unlike other standards that define what to build, Implementation records what was actually built and how it satisfies (or intentionally deviates from) every upstream decision.

---

## Plan Scenarios

Not every implementation plan covers the entire project. The plan type depends on what is being implemented and why.

### Full Generation

> **scenario:** New repo or major feature set — implement all features from scratch using Feature Technical(10)
> **scope:** Entire project or feature set
> **inputs:** Feature(04), Feature Technical(10), Architecture(05), Engineering(07), External Context(08), Security(03)
> **outputs:** Complete implementation plan covering all features, with per-feature generation plans

Use Full Generation when:
- Starting a new repository
- Implementing a major feature set from scratch
- The team needs a complete implementation roadmap

Full Generation produces per-feature implementation plans. Each feature gets its own generation plan using Feature(04) and Feature Technical(10).

### Per Feature

> **scenario:** New feature — implement a specific feature using its upstream documentation
> **scope:** Per feature
> **inputs:** Feature(04), Feature Technical(10), relevant upstream docs
> **outputs:** Implementation plan for the specific feature

Use Per Feature when:
- Adding a new feature to an existing project
- The feature has unique implementation requirements
- Feature-specific deviation tracking is needed

Per Feature produces a focused implementation plan. Upstream verification is limited to the feature's scope.

### Enhancement

> **scenario:** Existing feature improvement — enhance a specific feature or section
> **scope:** Per feature or per section (UI, backend, data layer)
> **inputs:** Feature(04) updated requirements, specific upstream docs
> **outputs:** Enhancement plan for the specific feature/section

Use Enhancement when:
- Improving performance of an existing feature
- Enhancing UX for a specific section
- Optimizing a specific component

Enhancement produces a targeted implementation plan. Scope is limited to what changed.

### Refactor

> **scenario:** Structural improvement — restructure code without changing behavior
> **scope:** Per module or per section
> **inputs:** Architecture(05) target structure, Engineering(07) code standards
> **outputs:** Refactor plan with behavior preservation verification

Use Refactor when:
- Improving code structure without changing behavior
- Aligning code with updated Architecture(05)
- Reducing technical debt in a specific module

Refactor produces a structural improvement plan. Behavior preservation is verified through existing tests.

### Change Request

> **scenario:** Behavior modification — change existing functionality to meet new requirements
> **scope:** Per feature or per behavior
> **inputs:** Feature(04) updated requirements, Feature Technical(10) updated runtime behavior
> **outputs:** Change request plan with impact analysis and rollback strategy

Use Change Request when:
- Modifying existing behavior per stakeholder request
- Changing API contracts
- Updating business logic

Change Request produces a modification plan. Impact analysis and rollback strategy are mandatory.

### Scope Options

| Scope | When to Use | Required Inputs |
|-------|-------------|-----------------|
| Entire project | New repo or major version | All upstream docs |
| Per feature | New or changed feature | Feature(04) + relevant upstream |
| Per module | Module-level refactor | Architecture(05) + Engineering(07) |
| Crate section | Crate/module changes | Feature Technical(10) + Architecture(05) |
| Backend section | API/data changes | Architecture(05) + Engineering(07) |
| Security section | Security fix | Security(03) + QA(12) |

---

## Generation Plan

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/01-generation_plan.yaml`*

### Template

> **minimum_content:** 4 subsections (inputs, generation sequence, verification, deviations)
> **length_guidance:** extensive
> **diagram_requirements:** flowchart

```markdown
## Generation Plan

### Inputs

List the upstream documents consumed.

### Generation Sequence

Step-by-step code generation process in tier order.

### Verification Checkpoints

Points where generated code is verified against upstream docs.

### Deviation Recording

How deviations from upstream docs are documented and justified.
```

**Required subsections:** Inputs, Generation Sequence, Verification Checkpoints, Deviation Recording
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Feature Technical(10), Architecture(05), Engineering(07), External Context(08), Security(03)

### Examples

**Correct:**
> **Inputs:** Feature(04) notification requirements, Feature Technical(10) message producer runtime behavior, Architecture(05) message queue topology, Engineering(07) async processing standards, Security(03) data-at-rest encryption.
> **Generation Sequence:** 1) Verify notification requirements against Feature(04). 2) Validate runtime behavior against Feature Technical(10). 3) Implement message producer per Architecture(05) queue topology. 4) Apply encryption per Security(03). 5) Verify coding standards per Engineering(07).
> **Verification Checkpoints:** After step 2 — UX matches mockups. After step 5 — all unit tests pass, encryption verified.
> **Deviation Recording:** Deviated from Architecture(05) by using a persistent queue instead of in-memory; rationale: notification delivery must survive process restarts.

**Incorrect:**
> **Inputs:** None listed.
> **Generation Sequence:** Write code, test it, deploy it.
> **Verification Checkpoints:** None.
> **Deviation Recording:** None needed.
> *Why wrong: No upstream documents referenced, generation sequence lacks tier ordering, no verification checkpoints defined, and deviation recording is dismissed rather than established as a process.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** List every upstream document consumed in the Inputs subsection; number the generation sequence steps in tier order; specify exact verification criteria at each checkpoint
- **Don't:** Omit upstream document references; skip verification checkpoints; write generation steps without referencing which standard each satisfies

Every new feature implementation starts with a Generation Plan. The plan verifies alignment with all upstream documentation before code is written.

---

## Refactor Plan

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/04-refactor_plan.yaml`*

### Template

> **minimum_content:** 3 subsections (target architecture, behavior preservation, verification)
> **length_guidance:** moderate
> **diagram_requirements:** component

```markdown
## Refactor Plan

### Target Architecture

The desired structure after refactoring.

### Behavior to Preserve

Existing functionality that must remain unchanged.

### Verification Strategy

How behavior preservation is verified (test suite must pass before and after).
```

**Required subsections:** Target Architecture, Behavior to Preserve, Verification Strategy
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Architecture(05), Engineering(07), Feature(04)

### Examples

**Correct:**
> **Target Architecture:** Consolidate the three notification modules (email, sms, push) into a single notification service with a strategy pattern dispatcher, per Architecture(05) service consolidation directive.
> **Behavior to Preserve:** All existing notification delivery contracts — email must arrive within 5 seconds, SMS within 10 seconds, push within 2 seconds. API request/response shapes remain identical.
> **Verification Strategy:** Run full integration test suite (42 tests) before and after refactor. Both runs must produce identical pass/fail results. Benchmark delivery latency for each channel.

**Incorrect:**
> **Target Architecture:** Rewrite the notification system in a new framework.
> **Behavior to Preserve:** None — this is a full rewrite.
> **Verification Strategy:** Manual testing after deployment.
> *Why wrong: Refactor must preserve existing behavior explicitly, not discard it; target architecture lacks specificity; verification strategy relies on manual testing rather than automated regression.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** Specify the target architecture by referencing Architecture(05); enumerate every behavior contract that must be preserved with measurable criteria; require automated test suite pass/fail comparison before and after
- **Don't:** Introduce new features or behavior changes in a refactor; rely on manual testing for verification; omit the before/after test comparison

Refactoring is not feature generation. It is structural improvement with behavior preservation. Every refactor must verify that existing tests still pass.

---

## Change Request Plan

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/05-change_request_plan.yaml`*

### Template

> **minimum_content:** 3 subsections (change description, impact analysis, rollback strategy)
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Change Request Plan

### Change Description

What behavior is changing and why.

### Impact Analysis

Which upstream docs, modules, and tests are affected.

### Rollback Strategy

How to revert the change if verification fails.
```

**Required subsections:** Change Description, Impact Analysis, Rollback Strategy
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Feature Technical(10), QA(12)

### Examples

**Correct:**
> **Change Description:** The checkout API response must now include a loyalty points balance field. Stakeholder request per updated Feature(04) requirements.
> **Impact Analysis:** Affects checkout API response schema, three downstream crates consuming the response, QA(12) integration tests (8 tests need new assertions), and Feature Technical(10) API documentation.
> **Rollback Strategy:** Deploy with feature flag disabled. If verification fails, toggle flag off — old response schema is restored with zero downtime. Database migration is additive only (new column), safe to leave in place.

**Incorrect:**
> **Change Description:** Add loyalty points to checkout.
> **Impact Analysis:** Should be straightforward.
> **Rollback Strategy:** Revert the commit.
> *Why wrong: Change description lacks specificity about what and why; impact analysis is vague with no affected modules or tests identified; rollback strategy does not account for database migrations or frontend deployments.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** Describe the exact behavior change with before/after state; list every affected module, API endpoint, and test case in impact analysis; define a rollback strategy that accounts for database migrations and feature flags
- **Don't:** Write vague change descriptions like "improve X"; skip the impact analysis or list no affected components; assume rollback is just "revert the commit"

Change requests modify existing behavior. Every change request must include impact analysis and a rollback strategy before implementation begins.

---

## Enhancement Plan

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/06-enhancement_plan.yaml`*

### Template

> **minimum_content:** 3 subsections (targets, approach, verification)
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Enhancement Plan

### Improvement Targets

Measurable goals (e.g., latency reduction, UX score improvement).

### Enhancement Approach

How the improvement is achieved without changing core behavior.

### Regression Verification

Tests that confirm no existing behavior is broken.
```

**Required subsections:** Improvement Targets, Enhancement Approach, Regression Verification
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Architecture(05), Engineering(07)

### Examples

**Correct:**
> **Improvement Targets:** Reduce search query latency from 800ms to under 200ms at P95. Current baseline measured via Engineering(07) performance benchmarks.
> **Enhancement Approach:** Add a read-through cache layer between the search controller and database, per Architecture(05) caching patterns. No changes to the search algorithm or response format.
> **Regression Verification:** Run the full search integration test suite (23 tests) to confirm identical results. Verify cache invalidation works correctly on data updates. Confirm no change in API response schema.

**Incorrect:**
> **Improvement Targets:** Make search faster.
> **Enhancement Approach:** Rewrite the search engine from scratch.
> **Regression Verification:** None — performance improvement is the only goal.
> *Why wrong: Improvement targets are not measurable; enhancement approach changes core behavior rather than improving existing functionality; regression verification is absent, risking broken existing features.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** Define improvement targets as measurable metrics with baseline and target values; describe the enhancement approach as additive layers that do not alter core behavior; specify the regression test suite that must pass unchanged
- **Don't:** Set improvement targets without measurable criteria; rewrite core components instead of layering improvements; skip regression verification or assume no breakage

Enhancements improve existing functionality. Every enhancement must define measurable improvement targets and verify no regression.

---

## Security Fix Plan

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/02-security_fix_plan.yaml`*

### Template

> **minimum_content:** 4 subsections (vulnerability, fix, verification, re-test)
> **length_guidance:** extensive
> **diagram_requirements:** sequence

```markdown
## Security Fix Plan

### Vulnerability Description

What was found and where.

### Fix Approach

How the vulnerability is addressed.

### Verification

Confirmation the fix resolves the vulnerability.

### Re-test Requirements

Security tests that must pass post-fix.
```

**Required subsections:** Vulnerability Description, Fix Approach, Verification, Re-test Requirements
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Security(03), QA(12), Architecture(05), Engineering(07)

### Examples

**Correct:**
> **Vulnerability Description:** SQL injection vulnerability in the user search endpoint (GET /api/users?name=). User-supplied `name` parameter is interpolated directly into a SQL query string without parameterization. Identified in Security(03) threat model as high-severity for data exfiltration.
> **Fix Approach:** Replace string interpolation with parameterized queries using the database driver's prepared statement API, per Engineering(07) secure coding standards. Apply fix to all three search endpoints (users, products, orders).
> **Verification:** Confirm parameterized queries reject injected payloads — test with `'; DROP TABLE users; --` and `1' OR '1'='1`. Verify legitimate search results are unchanged.
> **Re-test Requirements:** Run QA(12) full security test suite (15 tests). specifically the SQL injection test category (5 tests). Verify no new injection vectors introduced in affected endpoints.

**Incorrect:**
> **Vulnerability Description:** Security bug in search.
> **Fix Approach:** Add input validation.
> **Verification:** Tested manually.
> **Re-test Requirements:** None specified.
> *Why wrong: Vulnerability description lacks location, severity, and upstream reference; fix approach is vague without specifying the exact remediation technique; verification is manual rather than reproducible; re-test requirements are missing entirely.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** Specify the exact vulnerability location (endpoint, parameter, file path) and its severity; reference the Security(03) threat model entry; define reproducible verification steps with specific attack payloads; list the QA(12) security test suite and category counts
- **Don't:** Write vague descriptions like "security issue in search"; use manual testing as the sole verification method; omit re-test requirements or skip checking for new vulnerability introduction

Security fixes are the highest-priority implementation type. They must follow a strict process: identify, fix, verify, re-test. No shortcuts.

---

## Required Sections

Every Implementation Plan must contain the following sections.
Sections are identified by heading text; the compiler maps each to a semantic type.

| Section | semantic_type | Required | Aliases | Content Requirements |
|---------|--------------|----------|---------|---------------------|
| Generation Plan | `generation_plan` | ✓ | New Feature Plan, Feature Generation | Step-by-step generation sequence, verification checkpoints, deviation recording process |
| Refactor Plan | `refactor_plan` | | Restructure Plan, Code Refactoring | Target architecture, behavior preservation strategy, before/after test verification |
| Change Request Plan | `change_request_plan` | | CR Plan, Modification Plan | Impact analysis, rollback strategy, test update identification |
| Enhancement Plan | `enhancement_plan` | | Improvement Plan, Optimization Plan | Measurable improvement targets, regression verification, no core behavior changes |
| Security Fix Plan | `security_fix_plan` | ✓ | Vulnerability Fix, Security Patch | Fix approach, vulnerability verification, security re-test requirements |
| Purpose | `purpose` | | Overview, Summary | Scope definition, out-of-scope boundaries, document relationships and responsibilities |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

An Implementation Plan aims to:

* Record what was actually built, not what was planned
* Document deviations from upstream documentation with rationale
* Verify alignment with all applicable upstream standards
* Provide a traceable record of implementation decisions
* Enable consistent implementation across features

---

## Non-Goals

Implementation Plans do not define:

* Build pipeline configuration
* Deployment procedures
* QA testing strategy
* Release management
* Product vision or strategy

These belong to Build(14), QA(12), Engineering(07), and Vision(01).

---

## Success Criteria

An Implementation Plan is successful when:

* Every implementation can be traced back to upstream documentation
* Deviations are documented with rationale
* The as-built record accurately reflects what was built
* QA(12) can verify the implementation against the plan
* Build(14) can package the implementation with confidence

---

## Responsibilities

Implementation Plans are responsible for:

* Recording what was actually built
* Documenting deviations from upstream documentation
* Verifying alignment with Feature(04), Feature Technical(10), Architecture(05), Engineering(07), External Context(08), Security(03)
* Providing the as-built record for QA(12) verification
* Enabling Build(14) artifact generation

---

## Scope

Implementation Plans may describe:

* Generation plans for new features
* Refactor plans for structural improvements
* Change request plans for behavior modifications
* Enhancement plans for performance/usability improvements
* Security fix plans for vulnerability remediation
* Deviations from upstream documentation with rationale
* Module boundaries and known technical debt

---

## Out of Scope

Implementation Plans must not describe:

* Product vision or strategy
* Architecture or system design
* QA testing strategy or test implementation
* Build pipeline configuration or deployment
* Release management or versioning policy

These belong to their own documentation standards.

---

## Inputs

Implementation Plans derive from all applicable upstream documentation:

* Feature(04) — what to build
* Feature Technical(10) — crate/module structure and runtime behavior
* Architecture(05) — structural constraints
* Engineering(07) — code standards and practices
* External Context(08) — external constraints
* Security(03) — security requirements

Not all inputs apply to every implementation. The plan must identify which inputs are relevant.

---

## Outputs

Implementation Plans provide direction for:

* QA(12) — what to test and how to verify
* Build(14) — what to package and how to validate
* Future implementation — the as-built record for the next developer

---

## Traceability

```text
Feature(04) ────────────── what to build
Feature Technical(10) ──── crate/module structure and runtime behavior
Architecture(05) ───────── structural constraints
Engineering(07) ─────────── code standards
External Context(08) ────── external constraints
Security(03) ────────────── security requirements
         ↓
    Implementation(13) ─── as-built record
         ↓
    QA(12) ──────────────── verifies against plan
    Build(14) ───────────── packages verified code
```

---

## Relationships

| Document | Relationship |
|---|---|
| Feature(04) | Defines what Implementation builds |
| Feature Technical(10) | Defines crate/module structure and runtime behavior for Implementation |
| Architecture(05) | Constrains Implementation structure |
| Engineering(07) | Provides Implementation code standards |
| External Context(08) | Provides external Implementation constraints |
| Security(03) | Provides Implementation security requirements |
| QA(12) | Verifies Implementation against plan |
| Build(14) | Packages verified Implementation |

---

## Required Characteristics

Implementation Plans should be:

* As-built — record what was actually built, not what was planned
* Deviation-aware — document and justify every departure from upstream docs
* Verification-ready — provide enough detail for QA(12) to verify
* Traceable — every implementation decision links back to upstream documentation
* Honest — record technical debt and known limitations

---

## Audit Rules

| ID | Check | Severity |
|----|-------|----------|
| `impl-001` | Has generation plan | error |
| `impl-002` | Has security fix plan | error |
| `impl-003` | References all applicable upstream standards | error |
| `impl-004` | Deviations are documented with rationale | error |
| `impl-005` | As-built record matches actual implementation | warning |
| `impl-006` | QA verification chain is complete | error |

---

## Validation Rules

An Implementation Plan is considered valid when:

* Generation plan exists and references applicable upstream standards
* Security fix plan exists
* All deviations are documented with rationale
* The as-built record accurately reflects what was built
* QA(12) can verify the implementation against the plan
* No undocumented behavior exists

---

## Generation Rules

When generating an Implementation Plan:

* Start from Feature(04) requirements
* Identify all applicable upstream standards
* Verify alignment with each before writing code
* Record deviations with rationale
* Update the as-built record as implementation progresses
* Ensure QA(12) can verify the result

---

## Enhancement Rules

When enhancing an Implementation Plan:

* Update the as-built record when implementation changes
* Preserve deviation rationale even when deviations are resolved
* Add new upstream standard references when applicable
* Remove references to deprecated features
* Keep the plan accurate to the current implementation state

---

## Summary

Implementation Plans are the generation point of the documentation ecosystem. They record what was actually built, how it satisfies (or intentionally deviates from) every upstream decision, and provide the verification contract for QA(12). Unlike other standards that define what to build, Implementation records what was built and why. Every implementation decision traces back to upstream documentation — Feature requirements, Architecture structure, Engineering standards, External Context constraints, and Security requirements.

---

## Common Mistakes

Examples of incorrect Implementation content include:

* Recording the plan instead of the as-built reality
* Not documenting deviations from upstream documentation
* Skipping verification against upstream standards
* Treating Implementation as a generic standard instead of a per-feature plan
* Forgetting to update the as-built record when implementation changes

---

## Documentation Folder

Implementation Plans live under:

```text
docs/raw/implementation/
```

---

## Usage

Written per feature when implementation begins or changes; read by anyone implementing the next feature, running QA verification, or configuring Build pipelines. Use `samgraha compile --domain implementation` to validate structure, and `samgraha audit --domain implementation` to verify the plan covers all applicable upstream standards.

## Related

- [Feature Standard](feature.md) — defines what Implementation builds
- [Architecture Standard](architecture.md) — constrains Implementation structure
- [Security Standard](security.md) — provides security requirements
- [QA Standard](qa.md) — verifies Implementation against plan
- [Build Plan](build.md) — packages verified Implementation


---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| Draft | — | — | Initial proposal. Per-feature generation plan replacing generic standard. |


## Crate Implementation Patterns

Implementation plans for systems features must map to the defined architecture:

* Specify the exact Traits to be added or modified.
* Detail the module and file structure within the crate.
* Define the concrete types and their lifetime parameters.
* Detail the exact error type hierarchy.

## Crate Folder Structure

> *Structural rules: `common/tier5/audit/deterministic/section/implementation/08-crate_folder_structure.yaml`*

### Template

> **minimum_content:** 1 file tree
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
This feature is implemented using the standard Cargo crate structure:

```text
crate_name/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── models.rs
│   ├── error.rs
│   └── service.rs
└── tests/
    └── integration_test.rs
```

[Explain any deviations or additions to this standard structure for this specific feature.]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> The `auth` crate follows the standard layout and adds a `benches/` directory for crypto benchmarks.

**Incorrect:**
> All code is in `lib.rs`.
> *Why wrong: Fails to structure the crate modularly.*

### Writing Guidance

- **Tone:** structural
- **Voice:** third person
- **Structure:** file tree blocks
- **Audience:** backend developer
- **Do:** Use the canonical Cargo layout.

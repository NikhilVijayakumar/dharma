# Build Plan

## Table of Contents

> *Deterministic rules for this domain: `common/tier7/audit/deterministic/document/build.yaml`*

- [Purpose](#purpose)
- [Crate Publishing](#crate-publishing)

  - [Template](#template)
- [Plan Scenarios](#plan-scenarios)
- [Documentation Quality](#documentation-quality)
  - [Template](#template)
- [Security Checks](#security-checks)
  - [Template](#template)
- [Size Checks](#size-checks)
  - [Template](#template)
- [ML Artifact Management](#ml-artifact-management)
  - [Template](#template)
- [CI/CD Validation](#cicd-validation)
  - [Template](#template)
- [Obfuscation & Optimization](#obfuscation--optimization)
  - [Template](#template)
- [Versioning & Naming](#versioning--naming)
  - [Template](#template)
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

---

## Purpose

> *Structural rules: `common/tier7/audit/deterministic/section/build/04-purpose.yaml`*

### Template

> **minimum_content:** 3 paragraphs
> **length_guidance:** moderate
> **diagram_requirements:** flowchart

```markdown
# Build Plan

## Purpose

> **semantic_type:** `purpose`
> (metadata block)

(1-2 sentence statement of what Build defines)
(comparison to other standards — what Build does and does not cover)
(relationship to other standards in the documentation ecosystem)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Implementation(13), QA(12), Engineering(07)
```

### Examples

**Correct:**
> The Build Plan defines how verified code is packaged, validated, and delivered as shippable artifacts. It covers artifact generation, quality gates, security validation, and versioning — but does not define what to build (Implementation(13)), how to test it (QA(12)), or how to deploy it.

**Incorrect:**
> The Build Plan defines the CI/CD pipeline configuration, deployment procedures, and release management workflows.
> *Why wrong: Deployment procedures and release management are out of scope for Build(14) — they belong to their own documentation standards.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** architect
- **Do:** State scope boundaries as definitive exclusions; explicitly name which standards handle what Build does not cover; define Build's role in the documentation ecosystem
- **Don't:** Conflate Build with CI/CD implementation details; describe deployment or release procedures; leave scope boundaries ambiguous

This document defines the standard for Build Plans — artifact generation and validation pipelines that take verified code and produce shippable packages.

Unlike other standards that define what to build or how to verify it, Build defines how to package, validate, and deliver what was built. Not every build stage applies to every project — the plan defines applicability conditions.

---

## Plan Scenarios

Not every build plan covers the entire pipeline. The plan type depends on what is changing and why.

### Full Generation

> **scenario:** New project or major version — define the complete build pipeline from scratch
> **scope:** Entire project
> **inputs:** Engineering(07) CI/CD standards, Security(03) requirements, Implementation(13) artifact types
> **outputs:** Complete build plan covering all applicable stages with quality gates

Use Full Generation when:
- Starting a new project
- Rebuilding the pipeline from scratch
- The team needs a complete build strategy

Full Generation produces a comprehensive build plan. All applicable stages are defined with quality gates.

### New Build Rule

> **scenario:** Adding a new build stage or quality gate to the existing pipeline
> **scope:** Specific build stage
> **inputs:** The specific upstream docs for the new stage
> **outputs:** Build plan addition for the specific stage

Use New Build Rule when:
- Adding ML artifact management (DVC versioning)
- Adding a new security check category
- Adding obfuscation for release builds
- Adding size constraints for mobile/embedded

New Build Rule produces a targeted build plan addition. Scope is limited to the new stage.

### New Integration

> **scenario:** Integrating a new tool or service into the build pipeline
> **scope:** Specific integration
> **inputs:** External Context(08) for the new tool, Engineering(07) for pipeline config
> **outputs:** Build plan for the specific integration

Use New Integration when:
- Adding a new CI/CD tool
- Integrating a new security scanner
- Adding a new deployment target
- Connecting a new artifact repository

New Integration produces a focused integration plan. Scope is limited to the new tool/service.

### Environment Change

> **scenario:** Adapting the build pipeline for a new environment or deployment target
> **scope:** Specific environment
> **inputs:** External Context(08) environment constraints, Security(03) environment requirements
> **outputs:** Build plan adaptation for the specific environment

Use Environment Change when:
- Adding a new deployment environment (staging, production, edge)
- Adapting for a new platform (mobile, desktop, cloud)
- Meeting new compliance requirements for a target environment

Environment Change produces an adaptation plan. Scope is limited to the new environment.

### Security Threat

> **scenario:** Responding to a new security threat or vulnerability category
> **scope:** Specific security check
> **inputs:** Security(03) updated threat model, QA(12) security test requirements
> **outputs:** Build plan for the specific security check

Use Security Threat when:
- A new vulnerability category is discovered
- Security(03) threat model is updated
- A new security compliance requirement is added

Security Threat produces a targeted security check plan. Scope is limited to the new threat category.

### Scope Options

| Scope | When to Use | Required Inputs |
|-------|-------------|-----------------|
| Entire pipeline | New project or major rebuild | All upstream docs |
| Specific stage | Adding/modifying one stage | Stage-specific upstream docs |
| Specific tool | New integration | External Context(08) for tool |
| Specific environment | New deployment target | External Context(08) environment |
| Security check | New threat category | Security(03) |

---

## Documentation Quality

> *Structural rules: `common/tier7/audit/deterministic/section/build/01-documentation_quality.yaml`*

### Template

> **minimum_content:** 1 paragraph
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Documentation Quality

> **semantic_type:** `documentation_quality`
> (metadata block)

(1-2 sentence description of what documentation quality checks cover)
(state that this stage is mandatory and gates all downstream stages)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** samgraha audit pipeline
```

### Examples

**Correct:**
> Documentation quality checks validate that all documentation compiles without errors and passes the samgraha audit pipeline. This stage is mandatory and gates all downstream build stages.

**Incorrect:**
> Documentation quality checks verify that README files are written in clear English and follow the project's style guide.
> *Why wrong: Content style and writing quality are out of scope — documentation quality validates structural completeness and audit compliance, not prose style.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Reference the samgraha audit pipeline by name; state that this stage is mandatory and gates all downstream stages; specify what "valid" means in concrete terms
- **Don't:** Describe writing style or prose quality checks; conflate documentation quality with content decisions; omit the mandatory/gating nature of this stage

Documentation quality is the first build stage. If the documentation is invalid, nothing downstream should proceed.

---

## Security Checks

> *Structural rules: `common/tier7/audit/deterministic/section/build/02-security_checks.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Security Checks

> **semantic_type:** `security_checks`
> (metadata block)

(1-2 sentence description of what security checks cover)
(statement that checks are mandatory and map to Security(03) threat categories)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Security(03)
```

### Examples

**Correct:**
> Security checks run dependency vulnerability scanning, SAST on source code, and secrets detection. All critical and high severity findings block the build. Checks map to the threat categories defined in Security(03).

**Incorrect:**
> Security checks run optional vulnerability scans and log warnings for critical findings without blocking the build.
> *Why wrong: Security checks must be mandatory with blocking thresholds for critical/high findings — logging warnings defeats the purpose of security validation.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Define concrete severity thresholds (critical/high block, low/medium log); map each check to a Security(03) threat category; specify what happens when a check fails
- **Don't:** Allow security checks to be optional; use vague terms like "thoroughly" instead of measurable thresholds; omit the blocking behavior for critical findings

Security checks are mandatory for all projects. Map checks to Security(03) threat categories.

---

## Size Checks

> *Structural rules: `common/tier7/audit/deterministic/section/build/05-size_checks.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Size Checks

> **semantic_type:** `size_checks`
> (metadata block)

(1-2 sentence description of what size checks cover)
(statement that this stage is conditional and applies to size-sensitive projects)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Engineering(07)
```

### Examples

**Correct:**
> Size checks enforce a 5 MB limit on the distributable package. Measurement uses uncompressed artifact size. Exceeding the limit blocks the build with an actionable error message.

**Incorrect:**
> Size checks monitor documentation line counts and report the total without any thresholds or enforcement.
> *Why wrong: Without defined limits and enforcement actions, size checks provide no value — they must specify measurable thresholds and what happens when they are exceeded.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Define numeric limits per artifact type (e.g., "5 MB for distributable package"); specify the measurement method (compressed vs. uncompressed); state enforcement action (block vs. warn)
- **Don't:** Leave size limits undefined or use relative terms like "reasonable"; omit the measurement method; skip enforcement action definition

Size checks are conditional — required for projects with size constraints (mobile apps, embedded systems, CLI tools).

---

## ML Artifact Management

> *Structural rules: `common/tier7/audit/deterministic/section/build/06-ml_artifact_management.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## ML Artifact Management

> **semantic_type:** `ml_artifact_management`
> (metadata block)

(1-2 sentence description of what ML artifact management covers)
(statement that this stage is conditional and applies to ML projects)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04)
```

### Examples

**Correct:**
> ML artifacts are versioned using semantic versioning (model-1.2.3). Data versions are tracked with DVC. Experiments are logged in MLflow with parameters, metrics, and model hashes. Each build reproduces the same model from the same data version.

**Incorrect:**
> ML models are saved as model-latest.pkl and overwritten on each training run. Training data is stored locally without versioning.
> *Why wrong: Without versioning, model lineage is untraceable and reproducibility is impossible — this is the core problem ML artifact management solves.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Name specific tools (DVC, MLflow) and their roles; define the versioning scheme format explicitly; state reproducibility requirements (same data version → same model)
- **Don't:** Use generic terms like "version your models" without specifying the scheme; omit the tooling stack; conflate artifact management with model training

ML artifact management is conditional — required for projects with ML models. Use DVC for data versioning, MLflow for experiment tracking.

---

## CI/CD Validation

> *Structural rules: `common/tier7/audit/deterministic/section/build/07-cicd_validation.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** flowchart

```markdown
## CI/CD Validation

> **semantic_type:** `cicd_validation`
> (metadata block)

(1-2 sentence description of what CI/CD validation covers)
(statement that this stage is conditional and applies to projects with automated pipelines)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** flowchart (gate sequence)
**Required cross-references:** Engineering(07), QA(12)
```

### Examples

**Correct:**
> Gate sequence: lint → unit tests → integration tests → security scan → build → package. A failure at any gate blocks subsequent gates. Deployment is blocked until all gates pass. Failures notify the team via the configured alert channel.

**Incorrect:**
> All CI/CD checks run in parallel. If a security check fails, the build continues and the artifact is deployed anyway.
> *Why wrong: CI/CD validation must enforce that failures block downstream stages — deploying artifacts that failed security checks defeats the purpose of the pipeline.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Define the gate sequence as an ordered list (lint → test → security → build); specify failure handling per gate; identify deployment blockers explicitly
- **Don't:** Allow parallel execution of dependent gates; omit failure handling policies; deploy artifacts that failed any gate

CI/CD validation is conditional — required for projects with automated pipelines. Define gates in the right sequence.

---

## Obfuscation & Optimization

> *Structural rules: `common/tier7/audit/deterministic/section/build/08-obfuscation_optimization.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Obfuscation & Optimization

> **semantic_type:** `obfuscation_optimization`
> (metadata block)

(1-2 sentence description of what obfuscation and optimization covers)
(statement that this stage is conditional and applies to release builds)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Engineering(07)
```

### Examples

**Correct:**
> Release builds apply minification and tree-shaking, reducing bundle size by approximately 40%. Development builds skip obfuscation and preserve source maps for debugging. The size reduction is measured and reported in the build log.

**Incorrect:**
> All builds apply full obfuscation, including development builds. Source maps are never generated.
> *Why wrong: Obfuscating development builds breaks debugging capability — this stage must differentiate between build types and preserve debug info where needed.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Differentiate transformation rules per build type (release vs. development); quantify impact (e.g., "40% size reduction"); state what debug info is preserved and where
- **Don't:** Apply transformations uniformly across build types; omit measurable impact metrics; skip the trade-off between security/size and debuggability

Obfuscation and optimization are conditional — apply to release builds, not development builds. Document the trade-offs.

---

## Versioning & Naming

> *Structural rules: `common/tier7/audit/deterministic/section/build/03-versioning_naming.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Versioning & Naming

> **semantic_type:** `versioning_naming`
> (metadata block)

(1-2 sentence description of what versioning and naming covers)
(statement that this stage is mandatory and applies to all projects)

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Engineering(07)
```

### Examples

**Correct:**
> Artifacts use semantic versioning (MAJOR.MINOR.PATCH). Library artifacts are named `{name}-{version}.{ext}`. Breaking changes increment MAJOR. Compatibility rules: MAJOR bumps require migration guide; MINOR bumps are backward-compatible.

**Incorrect:**
> Artifacts are versioned sequentially (v1, v2, v3) with no naming convention. There are no documented compatibility rules between versions.
> *Why wrong: Sequential versioning without a defined scheme or compatibility rules makes it impossible to determine the impact of an upgrade or maintain multiple versions.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** paragraphs
- **Audience:** engineer
- **Do:** Name the versioning scheme (semver, calver) and when to use each; define the naming template (e.g., `{name}-{version}.{ext}`); document compatibility rules for each version component
- **Don't:** Use ad-hoc or sequential versioning without justification; omit compatibility rules between versions; leave naming conventions implicit

Versioning and naming are mandatory for all projects. Define the scheme once, apply it consistently.

---

## Required Sections

Every Build Plan must contain the following sections.
Sections are identified by heading text; the compiler maps each to a semantic type.

| Section | semantic_type | Required | Aliases | Content Requirements |
|---------|--------------|----------|---------|---------------------|
| Documentation Quality | `documentation_quality` | ✓ | Doc Quality, Documentation Checks | Must define quality gates for documentation, specify which domains are validated, and reference samgraha audit pipeline |
| Security Checks | `security_checks` | ✓ | Security Validation, Vulnerability Scanning | Must specify vulnerability scanning methods, severity thresholds, and reference Security(03) threat categories |
| Size Checks | `size_checks` | | Size Validation, Bloat Checks | Must define measurable size limits per artifact type, measurement methods, and enforcement actions |
| ML Artifact Management | `ml_artifact_management` | | ML Versioning, Model Management | Must define versioning scheme for models and data, experiment tracking approach, and reproducibility requirements |
| CI/CD Validation | `cicd_validation` | | Pipeline Validation, Gate Enforcement | Must define gate sequence, failure handling policies, and deployment blockers |
| Obfuscation & Optimization | `obfuscation_optimization` | | Optimization, Minification | Must specify which transformations apply per build type, configuration, and impact on debuggability |
| Versioning & Naming | `versioning_naming` | ✓ | Versioning, Naming Conventions | Must define version scheme (semver, calver), naming convention for all artifacts, and compatibility rules |
| Purpose | `purpose` | | Overview, Summary | Must state what Build defines, its scope boundaries, and relationship to other documentation standards |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

A Build Plan aims to:

* Define which build stages apply to the project
* Specify quality gates that block deployment
* Ensure artifacts are validated before packaging
* Provide versioning and naming consistency
* Enable automated enforcement of quality standards

---

## Non-Goals

Build Plans do not define:

* Product vision or strategy
* Architecture or system design
* Implementation details or code structure
* QA testing strategy
* Deployment automation or release management

These belong to their own documentation standards.

---

## Success Criteria

A Build Plan is successful when:

* All applicable build stages are defined
* Quality gates enforce standards automatically
* Artifacts are validated before deployment
* Versioning and naming are consistent
* Security checks catch vulnerabilities before release

---

## Responsibilities

Build Plans are responsible for:

* Defining artifact generation and validation pipeline
* Enforcing quality gates automatically
* Ensuring artifacts meet security, size, and quality standards
* Providing versioning and naming consistency
* Recording build results for traceability

---

## Scope

Build Plans may describe:

* Documentation quality validation
* Security scanning and vulnerability detection
* Size constraint enforcement
* ML artifact versioning and management
* CI/CD pipeline gate configuration
* Obfuscation and optimization settings
* Versioning schemes and naming conventions

Not all build stages apply to every project. The plan defines applicability conditions.

---

## Out of Scope

Build Plans must not describe:

* Product vision or strategy
* Architecture or system design
* Code implementation details
* QA testing strategy (what to test)
* Deployment procedures or release management

These belong to their own documentation standards.

---

## Inputs

Build Plans derive from:

* Implementation(13) — what to package
* QA(12) — test results that gate the build
* Engineering(07) — code standards and CI/CD configuration
* Security(03) — security check requirements
* External Context(08) — external build constraints

---

## Outputs

Build Plans provide:

* Validated, packaged artifacts ready for deployment
* Build reports with quality gate results
* Versioned artifacts with naming consistency
* Security scan results for compliance records

---

## Traceability

```text
Implementation(13) ── as-built code
QA(12) ────────────── test results
Engineering(07) ───── code standards
Security(03) ──────── security requirements
External Context(08) ─ external constraints
         ↓
    Build(14) ──────── artifact generation + validation
         ↓
    Deployment ─────── validated artifact
```

---

## Relationships

| Document | Relationship |
|---|---|
| Implementation(13) | Provides code to package |
| QA(12) | Provides test results that gate the build |
| Engineering(07) | Provides code standards and CI/CD config |
| Security(03) | Provides security check requirements |
| External Context(08) | Provides external build constraints |

---

## Required Characteristics

Build Plans should be:

* Applicable — not every stage applies to every project
* Automated — quality gates run without manual intervention
* Measurable — pass/fail criteria are quantifiable
* Traceable — build results link back to source code and tests
* Secure — security checks are mandatory, not optional

---

## Audit Rules

| ID | Check | Severity |
|----|-------|----------|
| `build-001` | Has documentation quality checks | error |
| `build-002` | Has security checks | error |
| `build-003` | Has versioning and naming scheme | error |
| `build-004` | Conditional stages have applicability conditions | warning |
| `build-005` | Quality gates define pass/fail criteria | error |
| `build-006` | Security checks reference Security(03) | error |

---

## Validation Rules

A Build Plan is considered valid when:

* All mandatory stages exist (documentation quality, security, versioning)
* Conditional stages have clear applicability conditions
* Quality gates define measurable pass/fail criteria
* Security checks reference Security(03) requirements
* Build results are recorded for traceability

---

## Generation Rules

When generating a Build Plan:

* Start from the project profile (web app, CLI, ML pipeline, library)
* Identify which build stages apply
* Define quality gates with measurable pass/fail criteria
* Map security checks to Security(03) requirements
* Define versioning scheme before the first release
* Ensure CI/CD pipeline enforces all applicable gates

---

## Enhancement Rules

When enhancing a Build Plan:

* Add build stages when new artifact types emerge
* Improve quality gates as tooling matures
* Update security checks as threats evolve
* Refine versioning conventions as patterns solidify
* Keep applicability conditions current with project needs

---

## Summary

Build Plans are the artifact generation and validation layer of the documentation ecosystem. They take verified Implementation(13) code and produce shippable packages through a series of quality gates. Not every build stage applies to every project — the plan defines applicability conditions based on project type. Mandatory stages (documentation quality, security, versioning) apply everywhere; conditional stages (size checks, ML artifact management, obfuscation) apply only when relevant. Every build stage has measurable pass/fail criteria enforced automatically through CI/CD.

---

## Common Mistakes

Examples of incorrect Build content include:

* Making all stages mandatory when some are conditional
* No pass/fail criteria — gates exist but don't block anything
* Security checks as optional instead of mandatory
* No versioning scheme defined before the first release
* Manual quality gates instead of automated enforcement

---

## Documentation Folder

Build Plans live under:

```text
docs/raw/build/
```

---

## Usage

Written when the project's build pipeline is defined or changes; read by anyone configuring CI/CD, setting up quality gates, or packaging artifacts. Use `samgraha compile --domain build` to validate structure, and `samgraha audit --domain build` to verify the build plan covers all applicable stages.

## Related

- [Implementation Plan](implementation.md) — provides code to package
- [QA Standard](qa.md) — provides test results that gate the build
- [Engineering Standard](engineering.md) — provides code standards and CI/CD config
- [Security Standard](security.md) — provides security check requirements
- [README Standard](readme.md) — references build artifacts


---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| Draft | — | — | Initial proposal. Artifact generation plan replacing generic standard. |


## Cargo Build and Workspace

Systems build standards must address:

* **Cargo Workspaces:** Managing multi-crate projects and shared dependencies.
* **Feature Flags:** Defining conditional compilation features and ensuring mutually exclusive features are handled.
* **CI/CD Checks:** Enforcing `cargo fmt`, `cargo clippy`, and `cargo audit` in pipelines.
* **Publishing:** Criteria and versioning rules for publishing to crates.io or private registries.

## Crate Publishing

> *Structural rules: `common/tier7/audit/deterministic/section/build/10-crate_publishing.yaml`*

### Template

> **minimum_content:** 1 paragraph
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
Internal crates are managed and published according to the system's versioning strategy.

[Detail the CI/CD steps that run `cargo publish --dry-run`, manage workspace dependencies, and handle private registry publishing.]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> CI runs `cargo publish --dry-run` on every PR. On tag, crates are published to our private AWS CodeArtifact registry.

**Incorrect:**
> A developer publishes it from their laptop.
> *Why wrong: Publishing must be automated in the build pipeline.*

### Writing Guidance

- **Tone:** authoritative
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** DevOps engineer
- **Do:** Automate publishing.

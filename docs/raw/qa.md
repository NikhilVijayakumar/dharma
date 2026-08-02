# QA Standard

## Table of Contents

> *Deterministic rules for this domain: `common/tier6/audit/deterministic/document/qa.yaml`*

- [Purpose](#purpose)
- [Plan Scenarios](#plan-scenarios)
- [Test Strategy](#test-strategy)
- [Unit Testing](#unit-testing)
- [Integration Testing](#integration-testing)
- [End-to-End Testing](#end-to-end-testing)
- [Smoke Testing](#smoke-testing)
- [Load Testing](#load-testing)
- [Scalability Testing](#scalability-testing)
- [Security Testing](#security-testing)
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
- [Audit Rules](#audit-rules)
- [Validation Rules](#validation-rules)
- [Generation Rules](#generation-rules)
- [Enhancement Rules](#enhancement-rules)
- [Summary](#summary)
- [Common Mistakes](#common-mistakes)
- [Documentation Folder](#documentation-folder)
- [Usage](#usage)
- [Related](#related)
- [Property Testing](#property-testing)
- [Benchmark Testing](#benchmark-testing)


---

## Purpose

> *Structural rules: `common/tier6/audit/deterministic/section/qa/05-purpose.yaml`*

### Template

> **minimum_content:** 2 paragraphs
> **length_guidance:** moderate
> **diagram_requirements:** flowchart

```markdown
## Purpose

> **semantic_type:** `purpose`
> **scope:** [How features are verified against specifications]
> **out_of_scope:** [Implementation details, build pipelines, deployment procedures]
> **contributes:** [Provides verification layer for upstream standards]
> **relationships:** [Feature(04) defines what to test; Architecture(05) defines boundaries; ...]
> **responsibilities:** [Define test types, applicability, and verification chain]
> **generation_rules:** [Start from Feature requirements; identify applicable test types]
> **enhancement_rules:** [Add test types for new risk areas; remove obsolete tests]
> **validation_rules:** [Test types defined; applicability explicit; verification chain complete]
> **audit_rules:** [Must exist; must define test types; must reference upstream; must not define implementation]

[Opening paragraph stating the verification scope of QA documentation]

[Paragraph explaining relationship to upstream documents (Feature, Architecture, Design, Security) and downstream documents (Implementation, Build)]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Architecture(05), Feature-Technical(10), Security(03), Implementation(13)

### Examples

**Correct:**
> This document defines how Project Nova verifies that features meet their specifications. It derives verification targets from Feature requirements, Architecture boundaries, and Security constraints. The testing strategy covers all applicable risk areas with measurable pass/fail criteria, and every test type traces back to the upstream documentation it validates.

**Incorrect:**
> This document defines the QA process for Project Nova. The team uses Jest for unit tests and Cypress for E2E tests.
> *Why wrong: This section defines verification scope and philosophy, not tool choices or implementation details. Framework selection belongs in Engineering(07), not here.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** architect
- **Do:** State the verification scope before any implementation details; reference every upstream document by its standard number and role; use measurable language for what the section governs
- **Don't:** Mention specific test frameworks, tools, or CI/CD systems; describe what the team "does" — describe what the document "defines"; conflate purpose with a roadmap or project status

This document defines the standard for Quality Assurance documentation within the engineering documentation ecosystem.

QA Documentation defines how features are verified against their specifications — the testing strategy, test types, and verification chain from feature requirements to implementation.

Unlike other standards that define what to build, QA defines how to verify that what was built matches what was specified.

---

## Plan Scenarios

Not every QA plan covers the entire project. The plan type depends on what is being tested and why.

### Full Generation

> **scenario:** New project or major version — define the complete test strategy for all features
> **scope:** Entire project
> **inputs:** Feature(04) requirements, Architecture(05) system boundaries, Feature-Technical(10) runtime behavior, Security(03) requirements
> **outputs:** Complete QA plan covering all test types for all features, with applicability conditions

Use Full Generation when:
- Starting a new project
- Releasing a major version with significant changes
- The team needs a complete test strategy from scratch

Full Generation produces a comprehensive test strategy. All applicable test types are defined with coverage targets.

### Per Feature

> **scenario:** New feature — define QA plan for that specific feature
> **scope:** Per feature
> **inputs:** Feature(04) for the specific feature, Feature-Technical(10), relevant upstream docs
> **outputs:** QA plan covering applicable test types for the specific feature

Use Per Feature when:
- Adding a new feature to an existing project
- The feature has unique testing requirements
- Feature-specific test coverage needs to be defined

Per Feature produces a focused QA plan. Test types are selected based on the feature's risk profile.

### Enhancement

> **scenario:** Existing feature change — add or update tests for specific section
> **scope:** Per feature or per section (UI tests, API tests, etc.)
> **inputs:** Feature(04) updated requirements, specific upstream docs
> **outputs:** Updated test coverage for the changed feature/section

Use Enhancement when:
- Modifying an existing feature
- Adding tests for a specific section (UI, API, data layer)
- Improving test coverage for an existing area

Enhancement produces targeted test additions. Scope is limited to what changed.

### Scope Options

| Scope | When to Use | Required Inputs |
|-------|-------------|-----------------|
| Entire project | New project or major version | All upstream docs |
| Per feature | New feature | Feature(04) + relevant upstream |
| Runtime workflow section | Entry-point/runtime behavior changes | Feature-Technical(10) + Feature(04) |
| API section | Backend changes | Architecture(05) + Engineering(07) |
| Security section | Security-related changes | Security(03) |
| Data section | Data layer changes | Architecture(05) data flow |

---

## Test Strategy

> *Structural rules: `common/tier6/audit/deterministic/section/qa/01-test_strategy.yaml`*

### Template

> **minimum_content:** 1 table, 1 paragraph
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Test Strategy

> **semantic_type:** `test_strategy`
> **scope:** [Overall testing approach — which test types apply, priority, order]
> **out_of_scope:** [Specific test implementations, framework choices, CI/CD configuration]
> **contributes:** [Provides testing roadmap for all test-related decisions]
> **relationships:** [Derived from Feature(04) and Architecture(05); referenced by Implementation(13)]
> **responsibilities:** [Define testing pyramid; specify mandatory vs. conditional test types]
> **generation_rules:** [Start from project profile; map test types to risk areas]
> **enhancement_rules:** [Adjust test mix as project evolves; add for new risk areas]
> **validation_rules:** [Covers all risk areas; priorities justified; no gaps in verification chain]
> **audit_rules:** [Must exist; must define applicability; must reference project profile; must be justified]

| Test Type | Applicability | Priority |
|-----------|--------------|----------|
| Unit Testing | [condition] | Mandatory |
| Integration Testing | [condition] | Mandatory/Conditional |
| End-to-End Testing | [condition] | Conditional |
| Smoke Testing | [condition] | Conditional |
| Load Testing | [condition] | Conditional |
| Scalability Testing | [condition] | Conditional |
| Security Testing | [condition] | Mandatory |

[Paragraph explaining the testing pyramid approach and how test types were selected based on project profile]
```

**Required subsections:** Test Type table with Applicability and Priority columns
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Feature(04), Architecture(05), Engineering(07)

### Examples

**Correct:**
> | Test Type | Applicability | Priority |
> |-----------|--------------|----------|
> | Unit Testing | All projects | Mandatory |
> | Integration Testing | Multi-component system with 4 services | Mandatory |
> | End-to-End Testing | Application has web UI | Conditional |
> | Smoke Testing | Application is deployed to production | Conditional |
> | Load Testing | Expected 500+ concurrent users | Conditional |
> | Security Testing | All projects | Mandatory |
>
> Project Profile: Web application with REST API, background workers, and PostgreSQL database. Risk areas prioritized by user-facing surface area and data sensitivity.

**Incorrect:**
> | Test Type | Applicability | Priority |
> |-----------|--------------|----------|
> | Unit Testing | Yes | High |
> | Integration Testing | Yes | High |
> | E2E Testing | Yes | High |
>
> We will test everything because all tests are important.
> *Why wrong: Applicability must state WHEN a test type applies (a condition), not just "Yes." Priority must use the standard scale (Mandatory/Conditional), not adjectives. The strategy must be justified by project profile and risk analysis.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** third person
- **Structure:** tables
- **Audience:** architect
- **Do:** Use the Mandatory/Conditional priority scale consistently; state applicability as a condition, not a boolean; justify each test type's inclusion by referencing project profile and risk areas
- **Don't:** Use adjectives like "High/Medium/Low" for priority; list test types without applicability conditions; assume every test type applies to every project

Every QA document must define the overall test strategy before individual test types. The strategy maps test types to project needs.

| Test Type | Applicability | Priority |
|-----------|--------------|----------|
| Unit Testing | All projects | Mandatory |
| Integration Testing | Projects with multiple components | Mandatory |
| End-to-End Testing | Applications with user-facing interfaces | Conditional |
| Smoke Testing | Deployed applications | Conditional |
| Load Testing | Applications expecting concurrent users | Conditional |
| Scalability Testing | Applications expecting growth | Conditional |
| Security Testing | All projects | Mandatory |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Unit Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/02-unit_testing.yaml`*

### Template

> **minimum_content:** 1 subsection
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Unit Testing

> **semantic_type:** `unit_testing`
> **scope:** [Testing individual components or functions in isolation]
> **out_of_scope:** [Integration, user-facing behavior, performance]
> **contributes:** [Verifies individual building blocks work correctly]
> **relationships:** [Validates Feature(04) at component level; references Engineering(07)]
> **responsibilities:** [Define coverage targets, naming conventions, assertion standards]
> **generation_rules:** [Derive from Feature(04); one behavior per test; Arrange-Act-Assert]
> **enhancement_rules:** [Add tests for new behaviors; update on change; remove deleted]
> **validation_rules:** [Coverage targets defined; naming conventions explicit; assertions clear]
> **audit_rules:** [Must exist; must define coverage; must not test implementation details]

### Coverage Targets

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Line coverage | [X%] | [Tool] |
| Branch coverage | [X%] | [Tool] |
| Function coverage | [X%] | [Tool] |

### Conventions

- **Naming:** [pattern, e.g., `test_<unit>_<scenario>_<expected>`]
- **Pattern:** Arrange-Act-Assert
- **One assertion per behavior**
```

**Required subsections:** Coverage Targets table
**Optional subsections:** Conventions
**Required diagrams:** none
**Required cross-references:** Feature(04), Engineering(07)

### Examples

**Correct:**
> ### Coverage Targets
>
> | Metric | Target | Measurement Method |
> |--------|--------|-------------------|
> | Line coverage | 80% | ProjectNova test runner |
> | Branch coverage | 75% | ProjectNova test runner |
> | Function coverage | 90% | ProjectNova test runner |
>
> ### Conventions
>
> - **Naming:** `test_<unit>_<scenario>_<expected>`
> - **Pattern:** Arrange-Act-Assert
> - **One assertion per behavior**

**Incorrect:**
> Unit tests should cover most of the code. The team writes unit tests for all new features and fixes.
> *Why wrong: Coverage targets must be measurable with explicit percentages and a defined measurement method. Vague statements like "most of the code" or "all new features" cannot be audited or verified.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** tables
- **Audience:** engineer
- **Do:** Specify numeric coverage thresholds per metric (line, branch, function); name the measurement tool; define one behavior per test in conventions
- **Don't:** Use qualitative targets like "good coverage"; list frameworks instead of behavioral conventions; define implementation-level details like file paths or class structures

Unit testing is applicable to all projects. Define coverage targets appropriate to the project's risk profile.

---

## Integration Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/03-integration_testing.yaml`*

### Template

> **minimum_content:** 1 table
> **length_guidance:** moderate
> **diagram_requirements:** component

```markdown
## Integration Testing

> **semantic_type:** `integration_testing`
> **scope:** [Testing how components interact]
> **out_of_scope:** [Individual component behavior, user workflows, performance]
> **contributes:** [Verifies Architecture(05) component model works]
> **relationships:** [Validates Architecture(05) component interactions; references Feature(04)]
> **responsibilities:** [Define boundaries, contract verification, data flow validation]
> **generation_rules:** [Derive from Architecture(05) component model; test each path]
> **enhancement_rules:** [Add for new connections; update on interface change]
> **validation_rules:** [Boundaries defined; contracts explicit; data flow complete]
> **audit_rules:** [Must exist for multi-component; must reference Architecture(05); must test paths]

### Integration Boundaries

| Boundary | Components | Contract | Verification Method |
|----------|-----------|----------|-------------------|
| [Boundary 1] | [Component A ↔ Component B] | [API/Protocol] | [Test approach] |
| [Boundary 2] | [Component B ↔ Component C] | [API/Protocol] | [Test approach] |

[Diagram showing component boundaries and communication paths]
```

**Required subsections:** Integration Boundaries table
**Optional subsections:** none
**Required diagrams:** Component diagram showing integration boundaries
**Required cross-references:** Architecture(05), Feature(04)

### Examples

**Correct:**
> ### Integration Boundaries
>
> | Boundary | Components | Contract | Verification Method |
> |----------|-----------|----------|-------------------|
> | API Gateway ↔ Auth Service | Gateway ↔ Auth | OAuth2 token endpoint | Contract test with mock IdP |
> | Auth Service ↔ User Database | Auth ↔ PostgreSQL | SQL schema + query contracts | Integration test with test database |
> | Worker Queue ↔ Processing Service | Queue ↔ Worker | Message schema v2 | Contract test with fixture messages |
>
> [Diagram showing the three component boundaries and their communication paths]

**Incorrect:**
> We test that the API works with the database and the cache. All services communicate over HTTP.
> *Why wrong: Integration boundaries must be explicitly listed as a table mapping specific component pairs, their contracts, and verification methods. Vague descriptions of "services communicating" don't define testable boundaries.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** tables
- **Audience:** engineer
- **Do:** Map each boundary to specific component pairs from Architecture(05); state the contract type (API schema, message format, protocol); define a concrete verification method per boundary
- **Don't:** Describe integration testing in prose without a boundary table; use vague terms like "services talk to each other"; omit the contract type or verification method for any listed boundary

Integration testing is mandatory for projects with multiple components. Map test coverage to Architecture(05) component boundaries.

---

## End-to-End Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/06-e2e_testing.yaml`*

### Template

> **minimum_content:** 1 table
> **length_guidance:** moderate
> **diagram_requirements:** flowchart

```markdown
## End-to-End Testing

> **semantic_type:** `e2e_testing`
> **scope:** [Testing complete runtime workflows from entry point to final output]
> **out_of_scope:** [Component behavior, API contracts, implementation details]
> **contributes:** [Verifies Feature-Technical(10) runtime behavior works in the running system]
> **relationships:** [Validates Feature-Technical(10) runtime_behavior; references Feature(04); tests Implementation(13)]
> **responsibilities:** [Define critical workflows, expected outcomes, acceptance criteria]
> **generation_rules:** [Derive from Feature-Technical(10) runtime_behavior and Feature(04) criteria]
> **enhancement_rules:** [Add for new workflows; update on runtime behavior change]
> **validation_rules:** [Critical workflows covered; outcomes explicit; criteria testable]
> **audit_rules:** [Must exist for externally-facing systems; must reference Feature-Technical(10); must have pass/fail]

### Critical Runtime Workflows

| Workflow | Runtime Behavior Reference | Expected Outcome | Pass/Fail Criteria |
|---------|-----------------|------------------|-------------------|
| [Workflow 1] | Feature-Technical(10) §[section] | [Expected result] | [Measurable criteria] |
| [Workflow 2] | Feature-Technical(10) §[section] | [Expected result] | [Measurable criteria] |

[Flowchart showing happy path and critical edge cases]
```

**Required subsections:** Critical Runtime Workflows table
**Optional subsections:** none
**Required diagrams:** Flowchart of runtime workflow paths
**Required cross-references:** Feature-Technical(10), Feature(04), Implementation(13)

### Examples

**Correct:**
> ### Critical Runtime Workflows
>
> | Workflow | Runtime Behavior Reference | Expected Outcome | Pass/Fail Criteria |
> |---------|-----------------|------------------|-------------------|
> | CLI ingest command | Feature-Technical(10) §Runtime Behavior — Ingest Pipeline | Input file parsed, validated, and written to output store | Process exits 0; output file contains expected row count; no panics logged |
> | Client request round-trip | Feature-Technical(10) §Runtime Behavior — Request Handling | Request routed through service layer and response returned | Response status matches contract; latency under budget; no unhandled `Result::Err` |
>
> [Flowchart showing happy path and critical edge cases for the ingest pipeline and request handling]

**Incorrect:**
> Test that the binary runs and doesn't crash.
> *Why wrong: Critical runtime workflows must be mapped to specific Feature-Technical(10) runtime behavior references with explicit expected outcomes and measurable pass/fail criteria. Generic descriptions without traceability to the runtime behavior spec cannot be verified or audited.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** tables
- **Audience:** engineer
- **Do:** Link each workflow to a specific Feature-Technical(10) runtime_behavior section; define expected outcomes as observable system states; write pass/fail criteria as automated assertions
- **Don't:** List workflows without a Feature-Technical(10) reference; describe expected outcomes in subjective terms; leave pass/fail criteria implicit or unmeasurable

End-to-end testing is conditional — required for systems with externally-facing entry points (CLI, API, network protocol). Map test coverage to Feature-Technical(10) runtime workflows.

---

## Smoke Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/07-smoke_testing.yaml`*

### Template

> **minimum_content:** 1 list
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
## Smoke Testing

> **semantic_type:** `smoke_testing`
> **scope:** [Quick sanity checks after deployment]
> **out_of_scope:** [Deep functional testing, performance, edge cases]
> **contributes:** [First line of defense after deployment]
> **relationships:** [References Implementation(13) artifacts; validates Build(14) output]
> **responsibilities:** [Define scope, pass/fail criteria, execution timing]
> **generation_rules:** [Start from critical journey; verify app starts; check data flows]
> **enhancement_rules:** [Add for new critical paths; keep suite fast]
> **validation_rules:** [Fast (< 5 min); covers critical paths; clear pass/fail]
> **audit_rules:** [Must exist for deployed; must be fast; must cover critical paths]

### Core Functions

- [ ] Application starts successfully
- [ ] [Core function 1] responds correctly
- [ ] [Core function 2] responds correctly
- [ ] Database connectivity verified
- [ ] Authentication endpoint responds

**Maximum execution time:** [X minutes]
**Pass criteria:** All checks pass
**Fail criteria:** Any check fails
```

**Required subsections:** Core Functions checklist
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Implementation(13), Build(14)

### Examples

**Correct:**
> ### Core Functions
>
> - [ ] Application starts and responds on port 8080
> - [ ] Health check endpoint returns 200 OK
> - [ ] Database connection pool initializes (max 10 connections)
> - [ ] Authentication endpoint accepts valid credentials
> - [ ] Primary API endpoint returns expected response schema
>
> **Maximum execution time:** 3 minutes
> **Pass criteria:** All checks pass
> **Fail criteria:** Any check fails — block deployment rollback

**Incorrect:**
> Smoke test: make sure the app works. Check that users can log in and the dashboard loads.
> *Why wrong: Smoke tests require a structured checklist of core functions with pass/fail criteria and a maximum execution time. Unstructured descriptions without timing constraints cannot function as deployment gates.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** bullet lists
- **Audience:** engineer
- **Do:** Use a checkbox list format for core functions; state a maximum execution time in minutes; define pass as "all checks pass" and fail as "any check fails"
- **Don't:** Write smoke tests as prose narratives; include edge cases or deep functional checks; omit the maximum execution time threshold

Smoke testing is conditional — required for deployed applications. Must be fast enough to run after every deployment.

---

## Load Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/08-load_testing.yaml`*

### Template

> **minimum_content:** 1 table
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Load Testing

> **semantic_type:** `load_testing`
> **scope:** [Testing behavior under expected and peak load]
> **out_of_scope:** [Functional correctness, security, deployment]
> **contributes:** [Verifies performance meets Feature(04) and Architecture(05) requirements]
> **relationships:** [References Architecture(05) scalability; validates Engineering(07) standards]
> **responsibilities:** [Define load profiles, performance targets, acceptable degradation]
> **generation_rules:** [Derive from expected user load; define baseline/target/stress]
> **enhancement_rules:** [Update profiles as user base grows; adjust targets]
> **validation_rules:** [Profiles realistic; targets measurable; degradation defined]
> **audit_rules:** [Must exist for concurrent apps; must define profiles; must have targets]

### Load Profiles

| Profile | Concurrent Users | Duration | Expected Response Time | Error Rate Threshold |
|---------|-----------------|----------|----------------------|---------------------|
| Baseline | [X] | [X min] | [X ms] | [X%] |
| Target | [X] | [X min] | [X ms] | [X%] |
| Stress | [X] | [X min] | [X ms] | [X%] |

### Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Response time (p95) | [X ms] | [Tool] |
| Throughput | [X req/s] | [Tool] |
| Error rate | [X%] | [Tool] |
```

**Required subsections:** Load Profiles table, Performance Targets table
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Architecture(05), Engineering(07), Feature(04)

### Examples

**Correct:**
> ### Load Profiles
>
> | Profile | Concurrent Users | Duration | Expected Response Time | Error Rate Threshold |
> |---------|-----------------|----------|----------------------|---------------------|
> | Baseline | 50 | 10 min | 200 ms | 0.1% |
> | Target | 200 | 30 min | 500 ms | 0.5% |
> | Stress | 500 | 15 min | 1000 ms | 2.0% |
>
> ### Performance Targets
>
> | Metric | Target | Measurement |
> |--------|--------|-------------|
> | Response time (p95) | 500 ms | Load testing tool metrics |
> | Throughput | 100 req/s | Load testing tool metrics |
> | Error rate | < 0.5% | Application logs |

**Incorrect:**
> The app should be fast. We expect it to handle many users without slowing down.
> *Why wrong: Load testing requires specific numerical profiles (concurrent user counts, durations, response times) and measurable performance targets. Qualitative descriptions like "fast" and "many users" cannot be tested or verified.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** tables
- **Audience:** engineer
- **Do:** Define load profiles with numeric concurrent-user counts and durations; set performance targets with specific thresholds (p95, throughput, error rate); name the measurement tool for each target
- **Don't:** Use qualitative load descriptions ("heavy traffic"); define targets without units (ms, req/s, %); skip the baseline profile — every load plan needs a known-good reference point

Load testing is conditional — required for applications expecting concurrent users. Define realistic load profiles based on expected usage.

---

## Scalability Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/09-scalability_testing.yaml`*

### Template

> **minimum_content:** 1 table
> **length_guidance:** moderate
> **diagram_requirements:** none

```markdown
## Scalability Testing

> **semantic_type:** `scalability_testing`
> **scope:** [Testing behavior as load increases beyond normal]
> **out_of_scope:** [Current performance, functional testing, deployment]
> **contributes:** [Verifies Architecture(05) scalability decisions work under growth]
> **relationships:** [References Architecture(05) scalability constraints]
> **responsibilities:** [Define growth scenarios, breaking points, scaling expectations]
> **generation_rules:** [Start from Architecture(05) scalability model; test at 2x, 5x, 10x]
> **enhancement_rules:** [Update scenarios as architecture evolves; adjust breaking points]
> **validation_rules:** [Growth scenarios defined; breaking points documented; behavior characterized]
> **audit_rules:** [Must exist for growth apps; must reference Architecture(05); must document breaking points]

### Growth Scenarios

| Scenario | Load Multiplier | Expected Behavior | Breaking Point | Scaling Strategy |
|----------|----------------|-------------------|----------------|-----------------|
| Moderate growth | 2x baseline | [Expected behavior] | [Where it breaks] | [How it scales] |
| Significant growth | 5x baseline | [Expected behavior] | [Where it breaks] | [How it scales] |
| Extreme growth | 10x baseline | [Expected behavior] | [Where it breaks] | [How it scales] |

### Breaking Points

| Component | Breaking Point | Failure Mode | Recovery Strategy |
|-----------|---------------|--------------|-------------------|
| [Component] | [Threshold] | [How it fails] | [How to recover] |
```

**Required subsections:** Growth Scenarios table, Breaking Points table
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Architecture(05)

### Examples

**Correct:**
> ### Growth Scenarios
>
> | Scenario | Load Multiplier | Expected Behavior | Breaking Point | Scaling Strategy |
> |----------|----------------|-------------------|----------------|-----------------|
> | Moderate growth | 2x baseline | Response time increases < 20% | N/A | Horizontal pod autoscaling |
> | Significant growth | 5x baseline | Response time increases < 50% | 8x — connection pool exhaustion | Add read replicas + connection pooling |
> | Extreme growth | 10x baseline | Graceful degradation with queuing | 15x — message queue overflow | Rate limiting + queue partitioning |
>
> ### Breaking Points
>
> | Component | Breaking Point | Failure Mode | Recovery Strategy |
> |-----------|---------------|--------------|-------------------|
> | Database connection pool | 800 connections | New connections rejected | Drain idle connections; scale read replicas |
> | Message queue | 100k pending messages | Messages dropped after 1h TTL | Increase consumer count; archive old messages |

**Incorrect:**
> The system should scale to handle more users as we grow. We will add servers when needed.
> *Why wrong: Scalability testing requires defined growth scenarios with specific load multipliers, expected behaviors, and documented breaking points. Statements about future intent don't characterize how the system actually behaves under growth pressure.*

### Writing Guidance

- **Tone:** technical
- **Voice:** imperative
- **Structure:** tables
- **Audience:** architect
- **Do:** Define growth scenarios with explicit load multipliers (2x, 5x, 10x); document the breaking point and failure mode for each critical component; specify a recovery strategy per component
- **Don't:** Describe scalability as future intent ("we plan to scale"); omit breaking points — untested limits are unknown limits; list components without stating how each fails under load

Scalability testing is conditional — required for applications expecting significant growth. Document where the system breaks and how it scales.

---

## Security Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/04-security_testing.yaml`*

### Template

> **minimum_content:** 1 table
> **length_guidance:** extensive
> **diagram_requirements:** none

```markdown
## Security Testing

> **semantic_type:** `security_testing`
> **scope:** [Verifying Security(03) requirements are enforced]
> **out_of_scope:** [Security architecture design, threat modeling, policy definition]
> **contributes:** [Verifies Security(03) and Engineering(07) standards are implemented]
> **relationships:** [Validates Security(03) threat model; references Engineering(07); tests Implementation(13)]
> **responsibilities:** [Define test types, coverage targets, severity thresholds]
> **generation_rules:** [Derive from Security(03) threat model; map types to categories]
> **enhancement_rules:** [Add for new threats; adjust thresholds; remove mitigated]
> **validation_rules:** [All threat categories covered; thresholds defined; targets measurable]
> **audit_rules:** [Must exist; must reference Security(03); must cover mandatory categories; must have thresholds]

### Security Test Types

| Test Type | Threat Category Coverage | Tool | Frequency | Severity Threshold |
|-----------|------------------------|------|-----------|-------------------|
| SAST | [categories] | [Tool] | [frequency] | [Critical/High/Medium] |
| DAST | [categories] | [Tool] | [frequency] | [Critical/High/Medium] |
| Dependency scanning | [categories] | [Tool] | [frequency] | [Critical/High/Medium] |
| Secrets detection | [categories] | [Tool] | [frequency] | [Critical/High/Medium] |

### Severity Thresholds

| Severity | Fail Build? | Required Response Time | Examples |
|----------|------------|----------------------|----------|
| Critical | Yes | [X hours] | [Examples] |
| High | Yes | [X days] | [Examples] |
| Medium | No | [X days] | [Examples] |
| Low | No | [X sprints] | [Examples] |
```

**Required subsections:** Security Test Types table, Severity Thresholds table
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** Security(03), Engineering(07), Implementation(13)

### Examples

**Correct:**
> ### Security Test Types
>
> | Test Type | Threat Category Coverage | Tool | Frequency | Severity Threshold |
> |-----------|------------------------|------|-----------|-------------------|
> | SAST | Injection, XSS, insecure deserialization | Static analyzer | Every commit | Critical |
> | DAST | Authentication bypass, privilege escalation | DAST scanner | Nightly build | Critical |
> | Dependency scanning | Known CVEs in third-party packages | Dependency checker | Daily | High |
> | Secrets detection | Hardcoded credentials, API keys | Secret scanner | Every commit | Critical |
>
> ### Severity Thresholds
>
> | Severity | Fail Build? | Required Response Time | Examples |
> |----------|------------|----------------------|----------|
> | Critical | Yes | 4 hours | Remote code execution, SQL injection |
> | High | Yes | 24 hours | Authentication bypass, privilege escalation |
> | Medium | No | 7 days | Information disclosure, missing security headers |
> | Low | No | Next sprint | Verbose error messages, outdated TLS version |

**Incorrect:**
> Run security scans and fix any issues found. Use whatever security tools the team prefers.
> *Why wrong: Security testing requires explicit test types mapped to threat categories, a defined tool, scan frequency, and severity thresholds that determine build pass/fail. Generic directives without measurable thresholds cannot be audited.*

### Writing Guidance

- **Tone:** prescriptive
- **Voice:** imperative
- **Structure:** tables
- **Audience:** security engineer
- **Do:** Map each test type to Security(03) threat categories; define severity thresholds with explicit fail-build rules and response times; specify scan frequency for every test type
- **Don't:** List security tools without stating what threat categories they cover; use severity labels without defining their build impact; omit scan frequency — untimed scans are unverifiable scans

Security testing is mandatory for all projects. Map test coverage to Security(03) threat categories. Severity thresholds determine pass/fail.

---

## Required Sections

Every QA document must contain the following sections.
Sections are identified by heading text; the compiler maps each to a semantic type.

| Section | semantic_type | Required | Aliases | Content Requirements |
|---------|--------------|----------|---------|----------------------|
| Test Strategy | `test_strategy` | ✓ | Testing Strategy, QA Strategy | Test type applicability table with priority levels; project profile mapping; risk-based justification |
| Unit Testing | `unit_testing` | ✓ | Unit Tests, Component Tests | Coverage targets; naming conventions; assertion standards; Arrange-Act-Assert pattern |
| Integration Testing | `integration_testing` | ✓ | Integration Tests, Contract Tests | Integration boundaries; contract verification approach; data flow validation; Architecture(05) component mapping |
| End-to-End Testing | `e2e_testing` | | E2E Tests, Acceptance Tests | Critical runtime workflows; expected outcomes; acceptance criteria; Feature-Technical(10) runtime behavior mapping |
| Smoke Testing | `smoke_testing` | | Sanity Tests, Post-Deploy Checks | Core function scope; pass/fail criteria; execution timing; maximum duration threshold |
| Load Testing | `load_testing` | | Performance Tests, Stress Tests | Load profiles (expected, peak, stress); performance targets; acceptable degradation thresholds |
| Scalability Testing | `scalability_testing` | | Scale Tests, Growth Tests | Growth scenarios (2x, 5x, 10x); breaking points; scaling behavior characterization |
| Security Testing | `security_testing` | ✓ | Security Tests, Vulnerability Tests | Security test types (SAST, DAST, dependency scanning, secrets detection); severity thresholds; coverage targets per threat category |
| Purpose | `purpose` | | Overview, Summary | Verification scope; testing philosophy; relationship to upstream and downstream documents |

Section headings are case-insensitive. Sections not listed here are stored as `generic` type — preserved but not queryable by type.

---

## Goals

A QA document aims to:

* Define which test types are needed for the project
* Specify applicability conditions so teams know what applies
* Create a verification chain from Feature(04) requirements to Implementation(13)
* Define measurable pass/fail criteria for each test type
* Guide test infrastructure investment based on risk

---

## Non-Goals

QA Documentation does not define:

* Test implementation details or framework choices
* CI/CD pipeline configuration
* Deployment procedures
* Release management processes
* Bug tracking or issue management

These responsibilities belong to Engineering(07) and Build(14).

---

## Success Criteria

QA Documentation is successful when:

* Every applicable test type has clear coverage targets
* The verification chain from requirements to implementation is complete
* Pass/fail criteria are measurable and automated where possible
* Test strategy evolves with the project without becoming outdated
* Teams know exactly which tests to write and why

---

## Responsibilities

QA Documentation is responsible for defining:

* Which test types apply to the project
* What each test type verifies against upstream documentation
* Coverage targets and pass/fail criteria
* The verification chain from requirements to implementation
* How test results map to documentation compliance

QA defines the verification contract. Implementation fulfills it. Build validates it.

---

## Scope

QA Documentation may describe:

* Test strategy and test type applicability
* Unit test coverage targets and conventions
* Integration test boundaries and contract verification
* End-to-end test scenarios and acceptance criteria
* Smoke test scope and execution timing
* Load test profiles and performance targets
* Scalability test growth scenarios and breaking points
* Security test coverage and severity thresholds

---

## Out of Scope

QA Documentation must not describe:

* Product vision or strategy
* Architecture or system design
* Implementation details or code structure
* Build pipeline configuration
* Deployment procedures
* Release management

These belong to their own documentation standards.

---

## Inputs

QA Documentation derives from:

* Feature(04) — what to verify
* Architecture(05) — system boundaries and component model
* Feature-Technical(10) — runtime behavior to validate
* Engineering(07) — test infrastructure and code standards
* External Context(08) — external testing constraints
* Security(03) — security test requirements
* Implementation(13) — what was actually built (as-built verification)

---

## Outputs

QA Documentation provides direction for:

* Test implementation and execution
* Build(14) test pipeline configuration
* Implementation(13) verification records
* Security(03) compliance verification

---

## Traceability

```text
Feature(04) ──── defines what to test
Architecture(05) ── defines system boundaries for testing
Feature-Technical(10) ─ defines runtime behavior to validate
Engineering(07) ─── defines test infrastructure
External Context(08) ─ defines external test constraints
Security(03) ────── defines security test requirements
       ↓
  QA(12) ──────── defines testing strategy and verification chain
       ↓
  Implementation(13) ── gets tested against QA(12) contract
       ↓
  Build(14) ────── validates test results in pipeline
```

---

## Relationships

| Document | Relationship |
|---|---|
| Feature(04) | Defines what QA verifies |
| Architecture(05) | Defines system boundaries for test scope |
| Feature-Technical(10) | Defines runtime behavior for E2E testing |
| Engineering(07) | Provides test infrastructure standards |
| External Context(08) | Provides external testing constraints |
| Security(03) | Defines security test requirements |
| Implementation(13) | Is what QA tests against |
| Build(14) | Validates QA results in pipeline |

---

## Required Characteristics

QA Documentation should be:

* Risk-driven — test types prioritized by impact
* Measurable — coverage targets and pass/fail criteria are quantifiable
* Applicable — test types have clear applicability conditions
* Traceable — every test type maps to upstream documentation
* Evolving — test strategy grows with the project

---

## Audit Rules

| ID | Check | Severity |
|----|-------|----------|
| `qa-001` | Has test strategy | error |
| `qa-002` | Test strategy defines applicability conditions | error |
| `qa-003` | Unit testing section exists | error |
| `qa-004` | Security testing section exists | error |
| `qa-005` | Integration testing section exists (multi-component projects) | warning |
| `qa-006` | Each test type references upstream documentation | error |
| `qa-007` | Coverage targets are measurable | error |
| `qa-008` | Pass/fail criteria are defined | error |

---

## Validation Rules

QA Documentation is considered valid when:

* Test strategy defines which test types apply
* Each applicable test type has measurable coverage targets
* Every test type references the upstream documentation it verifies
* Pass/fail criteria are explicit and automatable
* The verification chain is complete from requirements to implementation
* Security testing covers all Security(03) threat categories

---

## Generation Rules

When generating QA Documentation:

* Start from Feature(04) requirements — what needs to be verified
* Map test types to project profile (web app, CLI, ML pipeline, library)
* Define applicability conditions for each test type
* Create verification chain from test types to upstream documentation
* Set measurable coverage targets based on risk profile
* Define pass/fail criteria that can be automated

---

## Enhancement Rules

When enhancing QA Documentation:

* Add test types when new risk areas emerge
* Update coverage targets as project maturity increases
* Remove tests for deprecated features
* Improve test strategy alignment with current project profile
* Preserve the verification chain when upstream documentation changes

---

## Summary

QA Documentation is the verification layer of the documentation ecosystem. It defines which tests to write, what they verify, and how to know when the implementation is correct. Every test type maps to upstream documentation — Feature requirements, Architecture boundaries, Design workflows, Engineering standards, and Security requirements. The test strategy is risk-driven and project-specific: not every test type applies to every project, but every project needs a clear verification contract.

---

## Common Mistakes

Examples of incorrect QA content include:

* Defining test implementations instead of test contracts
* Skipping applicability conditions — assuming all test types apply everywhere
* No verification chain — tests exist but don't trace to requirements
* Coverage targets without measurement method
* Security testing as an afterthought instead of mandatory
* Defining CI/CD pipeline details instead of test strategy

---

## Documentation Folder

QA documents live under:

```text
docs/raw/qa/
```

---

## Usage

Written when the project's testing strategy is defined or changes; read by anyone writing tests or configuring test pipelines. Use `samgraha compile --domain qa` to validate structure, and `samgraha audit --domain qa` to verify the test strategy covers all applicable risk areas.

## Related

- [Feature Standard](feature.md) — defines what QA verifies
- [Architecture Standard](architecture.md) — defines system boundaries for test scope
- [Security Standard](security.md) — defines security test requirements
- [Implementation Plan](implementation.md) — what QA tests against
- [Build Plan](build.md) — validates QA results in pipeline



## Systems Testing Strategy

Systems QA standards must address:

* **Cargo Test:** Standard unit and integration testing via the `cargo test` framework.
* **Property Testing:** Using tools like `proptest` or `quickcheck` to validate invariants over large input domains.
* **Benchmarking:** Verifying performance regressions using tools like `criterion`.
* **Fuzzing:** Long-running fuzz tests for security-critical parsing or networking logic.

## Property Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/10-property_testing.yaml`*

### Template

> **minimum_content:** 1 paragraph
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
Core algorithmic logic and state machines are validated using property-based testing.

[Describe the use of tools like `proptest` or `quickcheck` to fuzz inputs and assert invariants.]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> The parsing logic is tested using `proptest` to ensure that for any arbitrary UTF-8 string input, the parser either succeeds or returns a formatted `ParseError` without panicking.

**Incorrect:**
> We will write a few unit tests.
> *Why wrong: Fails to address property testing invariants.*

### Writing Guidance

- **Tone:** technical
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** QA engineer
- **Do:** Define invariants that hold true across all generated inputs.

---

## Benchmark Testing

> *Structural rules: `common/tier6/audit/deterministic/section/qa/11-benchmark_testing.yaml`*

### Template

> **minimum_content:** 1 paragraph
> **length_guidance:** concise
> **diagram_requirements:** none

```markdown
Performance-critical paths are continuously benchmarked to prevent regressions.

[Describe the use of tools like `criterion` to measure latency, throughput, and memory allocations.]
```

**Required subsections:** none
**Optional subsections:** none
**Required diagrams:** none
**Required cross-references:** none

### Examples

**Correct:**
> The core routing algorithm is benchmarked using `criterion`. Regressions greater than 5% will fail the CI pipeline.

**Incorrect:**
> We will test if it feels fast.
> *Why wrong: Benchmarking must be statistical and automated.*

### Writing Guidance

- **Tone:** authoritative
- **Voice:** third person
- **Structure:** paragraphs
- **Audience:** systems engineer
- **Do:** Specify statistical measurement tools.

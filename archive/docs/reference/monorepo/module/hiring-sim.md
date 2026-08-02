# Hiring Simulation - Atomic Universal Specification

## A. Operational Intent
Model role definitions, candidate generation, and onboarding readiness decisions for virtual or hybrid team expansion.

## B. Registry Dependency
- Agent Profiles: lina, arya, mira
- Skills: competency-mapping-logic, talent-evaluation, onboarding-sequencing
- Protocols: competency-calibration-protocol, onboarding-sequencing-protocol, privacy-by-design-protocol
- Workflows: lina/seed-autonomous-alignment, lina/seed-human-in-loop-escalation

## C. The Triple-Engine Extraction Logic
### OpenCLAW
Evaluate role-fit trade-offs and recommend candidate shortlists with explainable scoring.

### Goose
Extract wizard inputs into stage tasks: role scope -> candidate synth -> interview path -> decision.

### NemoClaw
Navigate multi-step wizard, candidate cards, interview planners, and approval screens.

## D. Hybrid Data Lifecycle
### SQLite (High-Performance)
Persist wizard progress, scoring matrices, and decision drafts for iterative edits.

### Vault (Secure Commit State)
Commit approved hiring plan, final shortlist rationale, and policy-required PII evidence.

## E. Channel and Execution
- Cronjobs: Weekly talent-pipeline recalibration.
- Internal Chat: HR-ops discussion thread for approval and calibration.
- External Channels: No default external channel; optional Telegram for hiring freeze/escalation notices.
- Dynamic UI Contract: Render form fields, thresholds, and stage controls directly from registry YAML/JSON schemas; do not hardcode input contracts.
- No-Dead-End Navigation: Every detail page must expose Back to previous context and Home to workspace root to satisfy Electron no-URL flow constraints.
- Manual Override: Every registry-backed view must show Current State and Proposed Improvement side-by-side before commit.

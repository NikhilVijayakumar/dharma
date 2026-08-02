# Notification Centre - Atomic Universal Specification

## A. Operational Intent
Centralize severity-aware alert delivery while preventing notification fatigue and preserving escalation integrity.

## B. Registry Dependency
- Agent Profiles: mira, eva, julia
- Skills: incident-forensics, automated-policy-validation, queue-health-management
- Protocols: incident-escalation-protocol, conflict-escalation-protocol, queue-prioritization-protocol
- Workflows: mira/crisis-command-prioritization, eva/seed-human-in-loop-escalation

## C. The Triple-Engine Extraction Logic
### OpenCLAW
Classify alert severity and recommend suppression, grouping, or immediate escalation.

### Goose
Extract raw events into normalized alert objects with owner and action requirements.

### NemoClaw
Navigate alert feed tabs, severity filters, quiet-hour controls, and acknowledge actions.

## D. Hybrid Data Lifecycle
### SQLite (High-Performance)
Maintain live alert queue, dedupe windows, and user delivery preferences.

### Vault (Secure Commit State)
Commit critical alert history, acknowledged escalations, and policy overrides.

## E. Channel and Execution
- Cronjobs: Minute-level alert consolidation and digest generation jobs.
- Internal Chat: Real-time internal alert broadcasting with ack tracking.
- External Channels: Telegram delivery for critical and warning tiers.
- Dynamic UI Contract: Render form fields, thresholds, and stage controls directly from registry YAML/JSON schemas; do not hardcode input contracts.
- No-Dead-End Navigation: Every detail page must expose Back to previous context and Home to workspace root to satisfy Electron no-URL flow constraints.
- Manual Override: Every registry-backed view must show Current State and Proposed Improvement side-by-side before commit.

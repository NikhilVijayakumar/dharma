# Administration Suite

## Purpose

Administration Suite is the Director Office execution layer for organization-wide administration. It governs policy lifecycle, staff operations, employee wellbeing evaluation, and cross-channel intelligence inputs used for policy refinement.

## Scope

- Policy authoring topology and policy index governance.
- Staff registry and operational records (attendance, holiday, evaluations).
- Employee feedback and happiness evaluation contracts.
- External integration coordination (Google Workspace, MCP, webhook automations).
- Administrative communication channels (email and social-trend ingestion).

## Vault Contract

Administration artifacts live under:
- /org/administration/policies/
- /org/administration/staff/
- /org/administration/attendance/
- /org/administration/calendar/
- /org/administration/evaluations/
- /org/administration/feedback/
- /org/administration/meetings/
- /org/administration/integrations/
- /org/administration/channels/

Policy files must follow atomic-unit constraints:
- one policy concern per markdown file,
- small enough for safe context-window usage,
- listed in policy-index.json for discoverability.

## Data Model

### Staff Registry

Primary seed format: CSV
- employee_id
- full_name
- department
- role
- email
- status
- manager
- join_date
- employment_type
- location
- kpi_profile
- sheet_row_ref

### Attendance and Calendar

- attendance-template.csv
- holiday-calendar.csv

### Feedback and Meeting Notes

- employee-happiness-form-template.md
- weekly-admin-meeting-notes.template.md

## Integration Topology

### Planned Integrations

- Google Sheets
  - staff registry, attendance, holiday calendar, KPI tracking
- Google Docs/Drive
  - policy publishing and controlled sharing
- Google Forms
  - employee feedback collection
- MCP connectors
  - extensible integration layer and external data tooling
- Webhook automation (Zapier-compatible)
  - fallback orchestration for notifications and status relays

### Channel Model

- Mail channel
  - daily and weekly status, approvals, meeting summaries
- Social trend channel
  - read-only trend intelligence (Twitter/X via approved adapter)

## Governance Rules

1. No external publish action runs autonomously without human validation.
2. Policy changes must include owner, rationale, and review cadence.
3. Feedback and wellbeing data is internal-only and access-controlled.
4. Integration credentials must use least-privilege scopes.
5. Conversion outputs must preserve document lineage metadata.

## Runtime Discoverability

Routing map keys include:
- administration_policy_index
- administration_staff_registry
- administration_integration_config
- administration_google_sheets_mapping
- administration_feedback_template
- administration_meeting_template
- administration_channel_intelligence

## Planned KPIs

- policy-review-sla-compliance
- staff-kpi-on-time-evaluation-rate
- employee-happiness-response-rate
- integration-sync-health
- policy-update-latency

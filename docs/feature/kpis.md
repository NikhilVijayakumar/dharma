# Module: KPIs System

**Directory Mapping:** `src/registry/kpis/`

The KPI definition layer is responsible for measuring systemic intelligence efficiency and maintaining numeric validation threshold alerts.

## KPI Definitions

Configurations in `src/registry/kpis/` generate `RegistryKpiDefinition` payloads storing specific operational targets.

### Core Properties

| Property         | Description                                                                          |
| ---------------- | ------------------------------------------------------------------------------------ |
| **Thresholds**   | Bands for _Optimal_, _Warning_, and _Critical_ states                                |
| **Escalation**   | Logic determining automated alert routing when metrics flag outside limits           |
| **Goal Mapping** | Links each metric to overarching `Agent` logic blocks dictating the responsible role |

### Available KPIs

The `src/registry/kpis/` directory contains JSON definitions for various operational metrics including:

- Accessibility metrics (accessibility-conformance-rate)
- Code quality metrics (codebase-compliance-score, architectural-debt-ratio)
- Deployment metrics (deployment-success-rate, change-lead-time)
- Compliance metrics (compliance-pass-rate, audit-trail-completeness-score)
- Agent performance metrics (autonomous-resolution-rate, autonomous-bug-resolution-rate)
- Business metrics (funding-cost-efficiency, runway-coverage-months)
- And 40+ more metric definitions

Each KPI definition includes:

- Metric identity and description
- Target values and thresholds
- Escalation rules
- Agent role associations

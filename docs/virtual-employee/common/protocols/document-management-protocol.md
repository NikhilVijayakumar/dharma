# Document Management Protocol

## Overview
The Dhi virtual employee system processes two categories of documents: **CSV** (structured numerical data) and **Markdown** (qualitative analysis and reports). This protocol defines how documents enter the system, how they are stored, how agents discover and access them, and how concurrent multi-agent access is handled safely — all within the strict "Manual-In / Manual-Out" and "Air-Gapped" data flow philosophy.

---

## 1. Document Types

| Type | Format | Purpose | Examples |
|:-----|:-------|:--------|:---------|
| **Structured Data** | `.csv` | Numerical/tabular data for agent ingestion | Jira exports, ledger data, analytics metrics |
| **Unstructured Data** | `.md` | Qualitative analysis, reports, decisions | Executive reports, strategy memos, campaign plans |

**Hard Rule**: Numerical data must always be CSV. Qualitative analysis must always be Markdown. One topic per file. No mixing.

---

## 2. Document Ingestion (How Data Enters the System)

### 2.1 Manual Upload Only
The system follows a strict **no auto-ingestion, no scraping** policy. Documents enter the system exclusively through the Director's manual action.

```
Director exports data (Jira, Quickbooks, Google Sheets, etc.)
         │
         ▼
Director drags files into Electron App "Vault" zone
         │
         ▼
Electron moves files to /governance/data/stage/
         │
         ▼
Schema Guardian validates structure
         │
    ┌────┴────┐
    ▼         ▼
 VALID     INVALID
    │         │
    ▼         ▼
 Moved to    Rejected with
 /data/raw/  error details
```

### 2.2 Schema Validation
Before any document reaches an agent, it passes through the **Schema Guardian** (defined in `/governance/.dcm/schema_validation.json`).

```json
{
  "schemas": {
    "ledger": {
      "required_columns": ["date", "description", "amount", "category"],
      "column_types": { "date": "ISO-8601", "amount": "number" },
      "max_rows": 10000
    },
    "jira_export": {
      "required_columns": ["issue_key", "status", "assignee", "story_points", "sprint"],
      "column_types": { "story_points": "number" }
    },
    "analytics": {
      "required_columns": ["page", "views", "bounce_rate", "conversion"],
      "column_types": { "views": "number", "bounce_rate": "number", "conversion": "number" }
    }
  }
}
```

**Validation Rules**:
- Missing required columns → `SCHEMA_VIOLATION`. File rejected. Director notified.
- Wrong column type → `TYPE_MISMATCH`. File rejected.
- File exceeds `max_rows` → `SIZE_VIOLATION`. Director must split the file.
- Validation uses a monospace rotator UI (`/ - \ |`) for row-level progress (from Phase 1 Vault feature spec).

### 2.3 Staging vs. Raw vs. Archive
| Directory | Purpose | Who Writes | Who Reads |
|:----------|:--------|:-----------|:----------|
| `/governance/data/stage/` | Temporary hold for unvalidated uploads | Director (via Electron) | Schema Guardian |
| `/governance/data/raw/` | Current valid snapshot of all CSVs | Schema Guardian (after validation) | All agents (read-only) |
| `/governance/data/processed/` | Agent-generated intermediate outputs | Agents (write) | Other agents (read-only) |
| `/governance/data/archive/` | Historical snapshots (previous versions) | System (on new upload) | Agents for trend analysis |

---

## 3. Document Routing (How Agents Discover What They Need)

### 3.1 Mira's Document Router
When a new validated document lands in `/data/raw/`, Mira's `command-router.md` skill identifies which agent(s) need it based on the schema type.

| Schema Type | Primary Agent | Secondary Agents |
|:------------|:--------------|:-----------------|
| `ledger` | Nora (CFO) | Maya (Funding), Arya (CEO) |
| `jira_export` | Elina (COO) | Julia (CTO) |
| `analytics` | Dani (CMO) | Maya (Funding) |
| `health_logs` | Julia (CTO) | Elina (COO) |
| `newsletter_metrics` | Dani (CMO) | Maya (Funding) |
| `audit_trail` | Eva (Compliance) | None |
| `design_tokens` | Sofia (Design) | Julia (CTO) |

### 3.2 Notification Flow
```
New file in /data/raw/
       │
       ▼
Mira identifies schema type
       │
       ▼
Mira emits DOCUMENT_AVAILABLE event with:
  { 
    "file": "ledger_2026-02.csv", 
    "schema": "ledger",
    "primary_agent": "NORA_CFO",
    "secondary_agents": ["MAYA_FUNDING", "ARYA_CEO"]
  }
       │
       ▼
Primary agent receives Work Order with document path
Secondary agents receive read-only access notification
```

---

## 4. Concurrent Multi-Agent Access

### 4.1 The Problem
Multiple agents may need the same CSV simultaneously (e.g., Nora and Maya both need ledger data). Since we operate on a sequential queue, agents don't literally run at the same time, but they may need consistent snapshots.

### 4.2 Read-Only Access Protocol
**Rule**: All agents have **read-only** access to `/data/raw/`. No agent may modify source documents.

| Operation | Allowed | Who |
|:----------|:--------|:----|
| Read from `/data/raw/` | ✅ | All agents |
| Write to `/data/raw/` | ❌ | Nobody (only Schema Guardian writes here) |
| Read from `/data/processed/` | ✅ | All agents |
| Write to `/data/processed/` | ✅ | Only the agent that created the processed output |
| Read from `/data/archive/` | ✅ | All agents |
| Write to `/data/archive/` | ❌ | System only (auto-archive on new upload) |

### 4.3 Snapshot Isolation
When an agent begins processing a document:
1. The agent reads the file from `/data/raw/` and loads it into its context.
2. If the Director uploads a new version of the same file while the agent is processing, the new version goes to `/data/stage/` → validated → replaces `/data/raw/`.
3. The currently processing agent continues with its in-memory copy (stale but consistent).
4. On the next queue cycle, the agent will pick up the updated version.

**Consistency Rule**: Agents always process the version that existed when their task began. No mid-task document swapping.

### 4.4 Processed Output Namespace
Each agent writes intermediate results to a namespaced subdirectory to prevent conflicts:
```
/governance/data/processed/
├── nora/
│   ├── forecast_result.json
│   └── budget_status.json
├── julia/
│   ├── architecture_report.json
│   └── health_report.json
├── dani/
│   └── audience_report.json
├── sofia/
│   └── visual_audit_report.json
├── elina/
│   └── ops_metrics.json
└── eva/
    └── compliance_result.json
```

**Cross-Agent Reading**: Agent B can read Agent A's processed output (e.g., Maya reads `/processed/nora/forecast_result.json` for runway data). This is how handshake data flows between agents without direct communication.

---

## 5. Document Lifecycle

```
UPLOAD → STAGE → VALIDATE → RAW → PROCESS → REPORT → APPROVE → COMMIT
  │        │        │         │       │         │         │         │
Director  Electron Schema   Available Agents   Draft    Director  Git
          Vault    Guardian  to agents invoke   report   reviews   save
                                      services  in .md
```

### 5.1 Step-by-Step
1. **Upload**: Director exports data from external tool → drags into Electron Vault.
2. **Stage**: File lands in `/data/stage/`.
3. **Validate**: Schema Guardian checks against `/governance/.dcm/schema_validation.json`.
   - Pass → moves to `/data/raw/`. Previous version archived.
   - Fail → rejected. Error details shown in Vault UI.
4. **Raw**: File available to all agents. Mira routes to primary agent.
5. **Process**: Agent invokes domain TypeScript services via IPC. Outputs go to `/data/processed/{agent}/`.
6. **Report**: Agent synthesizes a `.md` report in **Draft** state. Saved to `/governance/decisions/drafts/`.
7. **Approve**: Director reviews the draft report in Electron UI.
8. **Commit**: Director clicks "Approve & Commit". Electron executes:
   ```bash
   git add .
   git commit -m "DECISION: [Agent Name] - [Title]"
   ```
   Only the Director can trigger `git push origin main`.

### 5.2 PII Handling in Documents
As governed by Eva's `local-first-enforcement.md`:
- CSVs containing `PII`-tagged columns are processed exclusively via LM Studio (local) through `ModelGatewayService.ts`.
- Agents must hash or strip PII fields before writing to `/data/processed/`.
- The `/data/raw/` directory is **never** synced to any cloud service.
- Eva's `ComplianceScanService.ts` checks all processed outputs for PII leakage before `AUDIT_PASS`.

---

## 6. Document Freshness & Staleness

### 6.1 Freshness Tracking
Each document in `/data/raw/` has a freshness metadata entry:
```json
{
  "file": "ledger_2026-02.csv",
  "uploaded_at": "2026-02-28T08:00:00Z",
  "schema": "ledger",
  "row_count": 450,
  "freshness": "CURRENT",
  "expiry": "2026-03-07T08:00:00Z"
}
```

### 6.2 Staleness Rules
| Data Type | Freshness Window | Action When Stale |
|:----------|:-----------------|:------------------|
| Financial ledger | 7 days | Mira requests Director to upload new export |
| Jira export | 3 days | Mira requests Director to upload new sprint data |
| Analytics | 7 days | KPI marked `STALE` in registry |
| Health logs | 1 day | Julia's `SystemHealthService.ts` auto-generates fresh data |

When a document expires:
1. Its freshness status changes to `STALE`.
2. KPIs depending on this source are marked `STALE` in `registry.json`.
3. Mira includes a data request in the Daily Brief: `"📁 Data request: Ledger CSV expired. Please upload fresh export."`.
4. No agent may produce a report using stale data without a `STALE_DATA_WARNING` disclaimer in the output.

---

## 7. Relevant Agent Assets

| Agent | Document Interaction |
|:------|:---------------------|
| **Mira** | Routes documents to agents. Requests fresh uploads. `command-router.md` identifies schema type. |
| **Nora** | Reads ledger CSVs. Writes forecast JSON. `IngestVaultService.ts` parses + sanitizes. |
| **Julia** | Reads health logs. Writes architecture reports. Generates health data via `SystemHealthService.ts`. |
| **Elina** | Reads Jira exports. Writes ops metrics. `WorkflowMonitorService.ts` tracks queue health. |
| **Dani** | Reads analytics/newsletter CSVs. Writes audience reports. `AudienceAnalyzerService.ts` processes. |
| **Sofia** | Reads design token files. Writes visual audit reports. `VisualAuditService.ts` processes. |
| **Eva** | Reads audit trails. Writes compliance results. `ComplianceScanService.ts` validates all outputs. |
| **Maya** | Reads Nora's processed forecasts + Dani's audience reports. Cross-agent reading. |
| **Arya** | Reads Nora's forecasts + Julia's reports. Cross-agent reading for strategic alignment. |
| **Lina** | Reads Director interaction history (internal log, not a CSV upload). |

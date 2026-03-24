# Security Protocol

## Overview
The Dhi virtual employee system processes sensitive data — financial ledgers, strategic plans, codebase artifacts, and personal interaction logs. This protocol defines the **common security baseline** that every agent must meet before executing any task. Each agent then layers domain-specific security measures via their own skills, rules, and scripts on top of this foundation.

This protocol-based approach (not inheritance) ensures all agents meet a universal minimum security standard while allowing specialized agents like Eva (Compliance) and Julia (CTO) to enforce deeper checks within their domains.

---

## 1. Core Security Principles

Derived from the DCM-1 Operating Playbook:
1. **Zero-Trust Local-First**: No data leaves the local machine unless explicitly permitted by the Director's engine configuration.
2. **No Autonomous Execution**: AI recommends. Director decides. No auto-commits, no auto-deploys.
3. **SSH-as-Identity**: The Director's SSH key is the sole authentication anchor. No cloud-stored credentials.
4. **Append-Only Audit Trail**: Every action by every agent is logged immutably.
5. **Least Privilege**: Each agent accesses only what its role requires.

---

## 2. Data Classification

All data processed by agents is classified into one of four sensitivity tiers:

| Tier | Label | Examples | Allowed Model Provider | Storage |
|:-----|:------|:---------|:-----------------------|:--------|
| **T1** | `PUBLIC` | Marketing copy, blog posts | Any provider via `ModelGatewayService` (LM Studio, OpenRouter, or Gemini) | Any directory |
| **T2** | `INTERNAL` | Architecture reports, KPI summaries | Any provider via `ModelGatewayService` | `/governance/` only |
| **T3** | `CONFIDENTIAL` | Financial forecasts, hiring plans | LM Studio (local) preferred; cloud providers only with aggregated/anonymized data | `/governance/data/` encrypted at rest |
| **T4** | `RESTRICTED` | Raw ledger CSVs, PII, SSH keys, API tokens | LM Studio (local) **only** — `ComplianceScanService.ts` enforces this | Never transmitted. Local-only processing. |

### 2.1 Classification Enforcement
- Every file in `/governance/data/raw/` must have a classification tag in its schema validation entry.
- Agents must read the classification before processing and select the appropriate CLI engine.
- Eva's `ComplianceScanService.ts` validates that no `T4` data appears in any cloud provider payload.

---

## 3. Common Security Baseline (All Agents Must Follow)

These are the **minimum security requirements** every agent must satisfy during every stage of the Stage Execution Protocol.

### 3.1 Stage 1 (Init) — Authentication & Authorization
| Requirement | Description |
|:------------|:------------|
| **SSH Verification** | System verifies valid SSH key for the Governance Repo on every launch via `git ls-remote`. If SSH fails, system locks. |
| **Role Verification** | Agent confirms its identity and loads only its authorized rules. No agent may impersonate another. |
| **Data Classification Check** | Before touching any file, agent reads the classification tier and selects the correct CLI engine. |

### 3.2 Stage 2 (Plan) — Input Validation
| Requirement | Description |
|:------------|:------------|
| **Schema Validation** | All CSV inputs must pass Schema Guardian checks (see Document Management Protocol). |
| **PII Scanning** | Agent scans all input fields for PII markers (names, emails, phone numbers, SSN patterns). If found, data is tagged `T4`. |
| **Dependency Whitelist** | Any external library or API call planned must be checked against an approved whitelist. |

### 3.3 Stage 3 (Execute) — Runtime Protection
| Requirement | Description |
|:------------|:------------|
| **Sandboxed Execution** | Services execute within `/governance/` directory boundaries only. No access to system directories, user home, or external volumes. |
| **No Network by Default** | Services must not make network calls unless explicitly using the External Source Protocol (which requires Eva's pre-approval). |
| **Output Sanitization** | All service outputs must strip `T4` data before writing to `/data/processed/`. PII fields are hashed. |
| **Memory Limits** | Services must respect the host's 80% RAM ceiling (see Queue System Protocol). |

### 3.4 Stage 4 (Synthesis) — Output Security
| Requirement | Description |
|:------------|:------------|
| **Eva's Compliance Audit** | No output reaches `COMPLETE` state without passing `ComplianceScanService.ts`. |
| **Audit Log Entry** | Every action is appended to `/governance/audit/audit_log.jsonl` via `AuditLogService.ts`. |
| **Redaction** | If output is destined for the Notification Centre (via `NotificationService.ts`), all `T3+` data must be redacted or aggregated. |
| **No Auto-Commit** | All outputs enter `DRAFT` state. Only Director can approve and commit to Git. |

---

## 4. Threat Model

| Threat | Vector | Mitigation | Responsible Agent |
|:-------|:-------|:-----------|:------------------|
| **PII Leakage** | Agent sends raw CSV to cloud provider | Data classification + `local-first-enforcement.md` | Eva |
| **Unauthorized Git Push** | Agent commits without Director approval | `git-lock-safety.md` + `GitGovernorService.ts` blocks `git push` | Eva, Julia |
| **Credential Exposure** | API keys appear in logs or outputs | `.env` file excluded from Git. Keys encrypted locally. `ComplianceScanService.ts` scans for key patterns. | Eva |
| **Dependency Supply Chain** | Malicious npm package introduced | `no-bloat-coding.md` whitelist + Eva's audit | Julia, Eva |
| **Agent Impersonation** | Agent A submits Work Order as Agent B | Each Work Order includes agent identity hash verified by Queue Controller | Mira, Eva |
| **Hallucination Exploit** | LLM generates harmful instructions | AI never auto-executes. All output is `DRAFT` until Director approves. | All agents |
| **Data Staleness Attack** | Agent operates on expired data to produce misleading reports | Document freshness protocol. Stale data forces `STALE_DATA_WARNING` disclaimer. | Mira, All agents |
| **Queue Overflow DoS** | Rogue agent floods queue with Work Orders | Max 10 slots + 20 backlog + hard block at 30. Elina monitors. | Mira, Elina |
| **Deadlock Starvation** | Circular handshake dependency starves queue | Deadlock detection in `bottleneck-resolution.md`. Force-timeout after 10 minutes. | Elina |

---

## 5. Per-Agent Security Layer (Domain-Specific Extensions)

Each agent applies the common security baseline **plus** their own specialized security measures.

### Eva (Compliance) — Security Lead
| Extension | Asset | Description |
|:----------|:------|:------------|
| Full audit enforcement | Service: `ComplianceScanService.ts` | Scans every artifact from every agent before `COMPLETE`. Checks for PII, unauthorized network calls, unapproved dependencies, RBAC violations. |
| Immutable logging | Service: `AuditLogService.ts` | Append-only `audit_log.jsonl`. Cannot be overwritten or truncated by any agent. |
| Filing & IP monitoring | Skill: `governance-enforcement.md` | Tracks regulatory deadlines, contract expirations, and IP exposure. |
| DCM-1 enforcement | Rule: `dcm-1-strict.md` | Validates every agent action against the 5 core DCM-1 principles. |
| Data egress blocking | Rule: `local-first-enforcement.md` | Blocks `T4` data from ever reaching cloud providers. |
| Git access control | Rule: `git-lock-safety.md` | Blocks force-pushes, concurrent branch writes, and non-standard commit formats. |

### Julia (CTO) — Technical Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Dependency audit | Service: `ArchitectureAuditService.ts` | Scans for known CVEs in all project dependencies. |
| Test gate | Rule: `test-first-mandate.md` | No code ships without passing tests. Prevents regression security bugs. |
| Architecture integrity | Skill: `clean-architecture-enforcement.md` | Ensures infrastructure layer (where external calls live) is isolated from domain layer. |
| Git commit safety | Service: `GitGovernorService.ts` | Validates Eva's `AUDIT_PASS` before allowing any commit. Director must manually push. |

### Nora (CFO) — Financial Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| PII sanitization | Service: `IngestVaultService.ts` | Hashes all PII fields (names, account numbers) during CSV ingestion. Raw data never leaves `/data/raw/`. |
| Local-only processing | Rule: `local-first-finance.md` | Financial model execution is restricted to LM Studio (local) only via `ModelGatewayService`. Aggregated summaries may use cloud providers. |
| Budget guard | Service: `BudgetAuditService.ts` | Prevents any agent from consuming API resources beyond allocated budget. |

### Dani (CMO) — Communication Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Outbound content review | Rule: `brand-tone-enforcement.md` | All external communications are reviewed for accidental data disclosure. |
| External source sandboxing | Protocol: `external-source-protocol.md` | Market data ingestion is sandboxed and Eva-audited before processing. |

### Sofia (Design) — Asset Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Token integrity | Service: `VisualAuditService.ts` | Ensures no hardcoded values bypass the token system (which could expose internal design decisions). |
| Component isolation | Skill: `ui-ux-analysis.md` | Verifies presentation layer cannot access domain-layer data directly. |

### Mira (Secretary) — Queue Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Agent identity verification | Service: `QueueService.ts` | Every Work Order includes an agent identity hash. Rejects unrecognized agents. |
| Overflow protection | Rule: `dispatch-logic.md` | Hard caps prevent queue flooding. |

### Elina (COO) — Operational Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Deadlock detection | Skill: `bottleneck-resolution.md` | Prevents denial-of-service via circular handshake dependencies. |
| Resource monitoring | Service: `WorkflowMonitorService.ts` | Tracks RAM/CPU. Blocks new tasks if host is under resource pressure. |

### Lina (HR) — Interaction Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Director privacy | Service: `InteractionAnalyzerService.ts` | Director's interaction logs are `T3` classified. Never shared with external APIs. |
| Notification gating | Rule: `notification-fatigue.md` | Prevents information overload which could cause Director to miss critical security alerts. |

### Arya (CEO) — Strategic Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Competitive intelligence containment | Skill: `strategic-alignment.md` | Strategic assessments are `T3` classified. Never included in raw form in Notification Centre pushes. |
| Approval gatekeeping | Rule: `approval-matrix.md` | Prevents strategic decisions from auto-executing without Director review. |

### Maya (Funding) — Pitch Security
| Extension | Asset | Description |
|:----------|:------|:------------|
| Investor data handling | Rule: `brand-equity.md` | External investor information is `T3` classified. Pitch drafts are Director-reviewed before any sharing. |
| External source sandboxing | Protocol: `external-source-protocol.md` | Grant/credit program data ingested via sandbox only. |

---

## 6. Security Incident Response

If a security violation is detected (PII leak, unauthorized access, credential exposure):

```
Eva detects violation via ComplianceScanService.ts
       │
       ▼
Immediate COMPLIANCE_LOCKDOWN event emitted
       │
       ├── All active tasks PAUSED
       ├── Queue frozen (no new Work Orders accepted)
       ├── Audit log snapshot taken
       │
       ▼
Eva generates incident report:
{
  "incident_id": "SEC-2026-0228-001",
  "type": "PII_LEAK",
  "severity": "CRITICAL",
  "agent": "NORA_CFO",
  "details": "Raw account numbers detected in cloud provider payload",
  "containment": "Payload blocked before transmission",
  "required_action": "Director must review and clear lockdown"
}
       │
       ▼
Mira pushes CRISIS notification to Director via NotificationService
       │
       ▼
Director reviews → approves containment → clears lockdown
       │
       ▼
System resumes. Incident logged permanently in audit trail.
```

---

## 7. Security Audit Schedule

| Audit Type | Frequency | Responsible | Output |
|:-----------|:----------|:------------|:-------|
| Compliance scan of all agent outputs | Every task completion | Eva | `compliance_result.json` |
| Dependency vulnerability check | Weekly | Julia | `architecture_report.json` |
| Data classification review | Monthly | Eva + Director | Updated `schema_validation.json` |
| Audit log integrity check | Weekly | Eva | Append-only verification |
| Credential rotation reminder | Quarterly | Eva | Director notification |
| Full security posture review | Quarterly | Eva + Julia + Elina | `/governance/decisions/security_review.md` |

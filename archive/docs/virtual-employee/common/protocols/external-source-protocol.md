# External Source Protocol

## Overview
The Dhi architecture explicitly favors a isolated, local-first ("Bunker") mentality. However, certain agents (Dani, Maya, Julia) require external market metrics, dependency updates, or API interactions. To do this safely, they must implement the **External Source Protocol**.

## Authorized Access Methods

To prevent unauthorized data leaks, Virtual Employees do not simply `curl` websites wildly. All external requests must be handled through deterministic, pre-approved ingestion pathways.

### 1. Data Ingestion (Read-Only)
If an agent needs information from the outside (e.g., Dani reading Twitter sentiment, Maya reading SEC filings):
- The agent must use `ModelGatewayService` (LM Studio local) to construct a highly specific `fetch_request.json`.
- The request must pass **Eva's Compliance Check** to ensure it contains zero Internal IP or PII within the query parameters.
- If passed, `ExternalFetcherService.ts` in the Electron main process retrieves the raw text.
- The raw text is saved to a local, isolated `_ingest/` directory for the agent to parse safely.

### 2. Data Egress (Write)
If an agent needs to publish data outside the system (e.g., Dani deploying a marketing site):
- The payload must be completely static and generated entirely offline.
- The payload must undergo a final `COMPLIANCE_PASS` from Eva.
- Egress strictly uses secure, authenticated tokens via `EgressProxyService.ts` in the Electron main process, not direct agent terminal commands.

## Protocol Structure
Agents accessing external sources must define the request in this format:
```json
{
  "protocol": "EXTERNAL_SOURCE_READ",
  "target_url": "https://api.github.com/repos/...",
  "justification": "Required by Julia to check dependency vulnerabilities. Rule: clean-architecture-enforcement.",
  "contains_pii": false
}
```

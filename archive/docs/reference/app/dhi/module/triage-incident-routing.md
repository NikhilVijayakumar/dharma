# Triage: Incident Routing - Atomic Feature Specification

## 1. Single Reason to Change (SRP)
This document handles updates **exclusively** related to evaluating inbound errors, exceptions, or user complaints and classifying them to the correct workflow/agent queue. It does NOT resolve the incident.

## 2. Input Data Required
- **Incident Payload:** Exception stack traces, user description, or SLA breach IDs.

## 3. Registry Sub-Component Integration
- **Agents:** The routing destination (e.g., `eva` for security, `mira` for ops).
- **Skills:** `incident-forensics`.
- **Workflows:** `mira/sop-autonomous-routing`.
- **Protocols:** Checks privacy levels before routing data.

## 4. Triple-Engine Extraction Model
- **OpenCLAW:** Computes the severity metric of the inbound problem.
- **Goose:** Analyzes an unstructured error trace and extracts a JSON `{ "category": "database", "urgency": "high" }`.
- **NemoClaw:** The drag-and-drop routing board UI (if manual override is invoked).

## 5. Hybrid DB & State Storage Flow
- **SQLite:** Stores unclassified incidents.
- **Action:** Moves the incident to a designated queue. Never touches the Vault, as routing is ephemeral.

## 6. Chat Scenarios (Internal vs External)
- **Internal Chat:** Submits incidents directly to Agent DMs ("You have a new High Priority Ticket").

## 7. Cron & Queue Management
- **Cron Configuration:** Polling intervals hook into error logs.
- **Failover / Catch-up Mechanic:** Upon startup, the engine queries SQLite for `unrouted_incidents = true` and pushes them to the router immediately.

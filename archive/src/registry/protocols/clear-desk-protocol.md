---
id: clear-desk-protocol
title: Clear Desk Protocol
tags: [email, confirmation, director, governance]
---

# Clear Desk Protocol

Governs the Director's confirmation gate for batch email operations — specifically the "mark processed emails as read" action after triage completion.

## Principle
Derived from Goose's `PermissionConfirmation` pattern. No batch email state mutation occurs without explicit Director consent.

## Trigger
Activated after Mira completes triage for an email heartbeat batch.

## Flow
1. System presents batch summary to Director: total emails, breakdown by action (drafted, FYI, archived, spam-filtered).
2. Director reviews and selects one of:
   - **Approve** (`AllowOnce`): All processed emails in the batch are marked as read via IMAP STORE.
   - **Reject** (`DenyOnce`): All emails remain unread for manual review in the Director's email client.
3. Confirmation timeout: 4 hours. After timeout, system escalates with an in-app notification reminder. Never auto-marks.

## Constraints
- Only one pending Clear Desk confirmation per account at a time.
- If Director has not responded and a new heartbeat fires, the new batch is held until the previous confirmation is resolved.
- All confirmation actions are logged in `task_audit_log`.

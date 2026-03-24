---
id: email-triage-classifier
title: Email Triage Classifier
tags: [email, triage, classification, priority]
---

# Email Triage Classifier

Classifies incoming emails by department and priority for the Secretary's Email Desk.

## Capabilities

- Domain classification: Finance, Compliance, HR, Operations, Strategy, Design, General.
- Priority scoring: CRITICAL, URGENT, IMPORTANT, ROUTINE based on sender reputation, subject keywords, and body analysis.
- Sender reputation lookup from historical interaction patterns in `email_intake_log`.
- Keyword extraction for department routing signals.

## Required Inputs

- Raw email metadata: sender, subject, date, body preview.
- Agent Directory Registry for department mapping.
- Historical sender interaction data.

## Outputs

- `department` tag for agent routing.
- `priority` classification.
- `isSpam` boolean (deferred to spam-newsletter-filter for rule-based pre-check).
- Confidence score per classification.

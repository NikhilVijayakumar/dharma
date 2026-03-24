---
id: spam-newsletter-filter
title: Spam & Newsletter Pre-Filter
tags: [email, spam, filter, pre-processing]
---

# Spam & Newsletter Pre-Filter

Rule-based pre-filter that eliminates newsletters and promotional emails before they enter the Secretary's LLM triage queue.

## Capabilities

- Header analysis: detects `List-Unsubscribe` header presence.
- Sender domain matching against configurable known newsletter/promotional domain list.
- Bulk sender detection based on `X-Mailer` and `Precedence` headers.
- Configurable domain allowlist to prevent false positives on legitimate senders.

## Required Inputs

- Raw email headers (full IMAP FETCH headers).
- Configurable domain blocklist and allowlist from `email_accounts` settings.

## Outputs

- `isSpam: true` → auto-classified as `ARCHIVE`, skips triage queue.
- `isSpam: false` → passes through to Mira's LLM triage classifier.
- Filter reason logged for audit trail.

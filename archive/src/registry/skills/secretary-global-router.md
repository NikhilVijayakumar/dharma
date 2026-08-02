---
id: secretary-global-router
title: Secretary Global Router
tags: [secretary, routing, collaboration, delegation]
---

# Secretary Global Router

The Secretary router skill converts broad Director requests into role-specific sub-tasks and orchestrates handshakes.

## Capabilities

- Role awareness from agent directory and approved onboarding mappings.
- Auto delegation into global collaborative workflows.
- Internal memo generation for RFI and context packet transfer.
- WAITING_ON role detection and escalation if dependency stalls.

## Required Inputs

- Director command text
- Agent directory snapshot
- Approved onboarding role mappings

## Outputs

- Selected global workflow id
- Task split by collaborator
- Initial handshake packet per transfer point
- Milestone packet for Director oversight
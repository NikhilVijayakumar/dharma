# Module: Workflows

**Directory Mapping:** `src/registry/workflows/`

Workflows form the connective logic execution tissue controlling interactions between independent agent nodes. This module is the most complex configuration logic space in Dharma.

## Logic Orchestration Vectors

`RegistryWorkflowDefinition` configurations establish exactly how systemic graphs connect:
1. **Agent Logic Isolation**: Workflows map implicitly to distinct Agent instances, dictating their triggering events and localized phase executions.
2. **Global Integration Checking**: Dependencies resolve aggressively. Missing skills or KPIs immediately append errors to the loader validation streams.

## Multi-Actor Handoffs
The `handoffRules` schemas support asynchronous flow logic:
- Defines explicit source and target agent relationships (`transfer_points`).
- Restricts payload execution rules (guaranteeing exact data envelopes prior to transitions).
- Regulates dynamic human-in-loop structures allowing Request For Intelligence (RFI) blocking logic execution flags.

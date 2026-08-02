# Module: Agents

**Directory Mapping:** `src/registry/agents/`

The Agents module defines the generative identities, properties, constraints, and requirements for actors within the intelligence grid.

## Templates and Intelligence Composition

Agents are defined primarily via standard schemas forming `RegistryAgentTemplate` and `RegistryAgentDefinition`. These static templates specify arrays of ID references defining necessary KPI links, Skill requirements, and Workflow logic paths.

When passed through `src/registry/loader.ts`, the template is synthesized into deeply resolved objects:

- **`RegistryResolvedAgent`**: The agent mapping resolved deeply against available KPI thresholds, skills, and data requirement objects.
- **`RegistryAgentIntelligence`**: A holistic wrapper combining the resolved agent profile alongside natively injected workflows and inherited company/system protocols.

## Verification & Alignment Checking

A loader utility method runs during initiation to execute alignment checks (`evaluateAgentAlignment()`). It inspects the `goal`, `backstory`, and `non-negotiable_requirements` properties to calculate deterministic alignment adherence constraints against the broader `company-core.json` business framework.

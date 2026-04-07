# Module: Data Inputs

**Directory Mapping:** `src/registry/data-inputs/`

This system provides strongly typed interface validations restricting how unstructured and structured real-world data streams penetrate the agent processing boundaries.

### Contract Parameters

`RegistryDataInputDefinition` governs data feeds via strict constraints:

1. **Schema Typed Modes**: Tabular streams, timeseries events, or internal identity protocols.
2. **Access Policies**: Definitions require mapping to explicit `privacyClassification` tags (Internal, Highly Restricted, Public) to block agent dissemination.
3. **Cross-Resolution Dependencies**: Data inputs operate hand-in-hand with KPIs and Workflows; during system load mapping, any workflow triggering against a non-existent or un-mapped input feed produces a blocking logic diagnostic violation payload.

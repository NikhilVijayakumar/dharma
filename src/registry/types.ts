export interface RegistryKpiThresholds {
  critical?: string;
  warning?: string;
  optimal?: string;
}

export interface RegistryKpiMetadata {
  priority?: 'Critical (Tier-1)' | 'High (Tier-2)' | 'Medium (Tier-3)' | 'Low (Tier-4)';
  samplingFrequency?: string;
  shorthand?: string;
  createdDate?: string;
  lastModified?: string;
}

export interface RegistryKpiEscalationPolicy {
  onCritical?: string;
  onWarning?: string;
}

export interface RegistryKpiDependencies {
  dataInputs?: string[];
  dependentKpis?: string[];
}

export interface RegistryKpiDefinition {
  uid: string;
  name: string;
  description: string;
  unit: string;
  target: string;
  value: string;
  formula?: string;
  goalMapping: string;
  frequencyOfCheck?: 'real-time' | 'per-transaction' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'on-demand';
  responsibleAgentRole?: string;
  thresholds?: RegistryKpiThresholds;
  metadata?: RegistryKpiMetadata;
  escalationPolicy?: RegistryKpiEscalationPolicy;
  dependencies?: RegistryKpiDependencies;
}

export interface RegistryDataInputField {
  name: string;
  type?: 'string' | 'number' | 'boolean' | 'array' | 'object' | 'datetime';
  required?: boolean;
  description?: string;
}

export interface RegistryDataInputDefinition {
  uid: string;
  name: string;
  description: string;
  schemaType: 'tabular' | 'event-stream' | 'document' | 'timeseries' | 'identity-protocol' | 'intelligence-protocol' | 'manifest-schema' | 'audit-trail';
  sourceType?: 'csv' | 'json' | 'jsonl' | 'yaml' | 'sql-database' | 'api-endpoint' | 'vault' | 'git-repository' | 'event-broker' | 'vault-context' | 'file-system';
  sourceLocation?: string;
  requiredFields: Array<string | RegistryDataInputField>;
  sampleSource: string;
  validationRules?: Record<string, string>;
  updateFrequency?: 'real-time' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'on-demand';
  responsibleAgent?: string;
  privacyClassification?: 'public' | 'internal' | 'confidential' | 'restricted';
  retentionPolicy?: string;
  dimensions?: Record<string, string>;
  dependencies?: {
    dependentKpis?: string[];
    dependentWorkflows?: string[];
  };
  metadata?: {
    createdDate?: string;
    lastModified?: string;
    version?: string;
  };
}

export interface RegistryInputDefinition {
  id: string;
  description: string;
  required: boolean;
}

export interface RegistryAgentRequirement {
  agentId: string;
  role: string;
  required: boolean;
}

export interface RegistryModuleManifest {
  id: string;
  title: string;
  route: string | null;
  enabled: boolean;
  ownership: string;
  kpis: RegistryKpiDefinition[];
  inputs: RegistryInputDefinition[];
  agentRequirements: RegistryAgentRequirement[];
}

export interface RegistryAgentTemplate {
  name: string;
  role: string;
  backstory: string;
  goal: string;
  core_objective: string;
  individual_vision: string;
  role_non_negotiable_requirements: string[];
  objectives_long_term: string[];
  personality_traits: string[];
  interaction_style: string;
  constraints: string[];
  skills: string[];
  kpis: string[];
  data?: string[];
  data_requirements: string[];
  protocols: string[];
  workflows?: string[];
}

export interface RegistryAgentDefinition extends RegistryAgentTemplate {
  uid: string;
}

export interface RegistryResolvedAgent {
  uid: string;
  name: string;
  role: string;
  backstory: string;
  goal: string;
  coreObjective: string;
  individualVision: string;
  roleNonNegotiableRequirements: string[];
  objectivesLongTerm: string[];
  personalityTraits: string[];
  interactionStyle: string;
  constraints: string[];
  skills: RegistrySkillDoc[];
  kpis: RegistryKpiDefinition[];
  dataRequirements: RegistryDataInputDefinition[];
}

export interface RegistryProtocolDoc {
  id: string;
  title: string;
  tags: string[];
  format: 'yaml' | 'markdown';
  content: string;
  sourceFile: string;
  rules: string[];
}

export interface RegistryWorkflowPreCondition {
  condition: string;
  description: string;
  source?: 'kpi' | 'data-input' | 'agent-state' | 'vault' | 'git';
  validationRule?: string;
}

export interface RegistryWorkflowPostCondition {
  effect: string;
  description: string;
  target?: string;
  updateFormula?: string;
}

export interface RegistryWorkflowLogicPhase {
  [key: string]: string | Record<string, string>;
}

export interface RegistryWorkflowDependencies {
  required_skills: string[];
  required_kpis: string[];
  data_inputs?: string[];
  required_agents?: string[];
  protocols?: string[];
}

export interface RegistryWorkflowCollaborator {
  agent_id: string;
  role: string;
  responsibility: string;
  required?: boolean;
}

export interface RegistryWorkflowTransferPoint {
  id: string;
  from_agent: string;
  to_agent: string;
  condition: string;
  context_packet_required_fields: string[];
  rfi_allowed?: boolean;
  director_approval_required?: boolean;
}

export interface RegistryWorkflowHandoffRules {
  transfer_points: RegistryWorkflowTransferPoint[];
  milestone_approval_roles?: string[];
}

export interface RegistryWorkflowOutputFormat {
  format?: 'json' | 'markdown' | 'yaml' | 'artifact-hash';
  requiredFields?: string[];
  example?: string;
}

export interface RegistryWorkflowMetadata {
  version?: string;
  createdDate?: string;
  lastModified?: string;
  status?: 'draft' | 'active' | 'deprecated';
  estimatedDuration?: string;
}

export interface RegistryWorkflowDefinition {
  id: string;
  agent_id: string;
  trigger: string;
  workflow_mode?: 'single-agent' | 'global-collaborative';
  preConditions?: RegistryWorkflowPreCondition[];
  logicSequence?: Record<string, Record<string, string>>;
  steps?: string[]; // Deprecated: for backward compatibility
  postConditions?: RegistryWorkflowPostCondition[];
  dependencies: RegistryWorkflowDependencies;
  collaborators?: RegistryWorkflowCollaborator[];
  handoffRules?: RegistryWorkflowHandoffRules;
  errorHandling?: Record<string, string>;
  outputFormat?: RegistryWorkflowOutputFormat;
  expected_output: string;
  metadata?: RegistryWorkflowMetadata;
  sourceFile?: string;
}

export interface RegistryAgentIntelligence {
  agentId: string;
  profile: RegistryResolvedAgent;
  protocols: RegistryProtocolDoc[];
  workflows: RegistryWorkflowDefinition[];
}

export interface RegistryKpiFieldRequirement {
  key: string;
  minWords: number;
  quality: 'required' | 'recommended';
}

export interface RegistryKpiMinimumRequirementEntry {
  stepId: string;
  requiredFields: RegistryKpiFieldRequirement[];
}

export interface RegistryKpiMinimumRequirements {
  version: string;
  minimumRequirements: RegistryKpiMinimumRequirementEntry[];
}

export interface RegistrySkillDoc {
  id: string;
  title: string;
  tags: string[];
  content: string;
  sourceFile: string;
}

export interface RegistryCompanyCore {
  identity?: {
    name?: string;
    type?: string;
    foundation?: string;
    philosophy?: string[];
  };
  vision: string;
  context: string | Record<string, unknown>;
  core_values: string[];
  global_non_negotiables: string[];
  operational_rules?: Record<string, string[]>;
  mission_alignment?: Record<string, unknown>;
  updatedAt?: string;
}

export interface RegistryProductDetails {
  product_and_ecosystem: {
    philosophy?: string[];
    core_content_units?: string[];
    educational_track?: Record<string, unknown>;
    fiction_track?: Record<string, unknown>;
    distribution_system?: Record<string, unknown>;
    content_flow_engine?: Record<string, unknown>;
    ai_influencer_system?: Record<string, unknown>;
    monetization_model?: Record<string, unknown>;
    current_focus?: Record<string, unknown>;
  };
  updatedAt?: string;
}

export interface RegistryBusinessContext {
  company: RegistryCompanyCore;
  product: RegistryProductDetails;
  companyProtocolRules: string[];
  alignmentKeywords: string[];
}

export interface RegistryAlignmentFieldStatus {
  field: 'goal' | 'backstory' | 'core_objective' | 'individual_vision' | 'role_non_negotiable_requirements';
  aligned: boolean;
  reason: string;
}

export interface RegistryAgentAlignmentCheck {
  agentId: string;
  aligned: boolean;
  issues: RegistryAlignmentFieldStatus[];
}

export interface RegistrySnapshot {
  version: number;
  loadedAt: string;
  sourceRoot: string;
  manifests: RegistryModuleManifest[];
  onboarding: {
    company: RegistryCompanyCore;
    product: RegistryProductDetails;
    businessContext: RegistryBusinessContext;
    alignmentChecks: RegistryAgentAlignmentCheck[];
    directorApprovalWarnings?: string[];
    agentSetup: RegistryResolvedAgent[];
    initialDrafts: Record<string, Array<{ key: string; value: string }>>;
    fieldSchema: Record<string, unknown[]>;
  };
  agents: RegistryAgentDefinition[];
  kpis: RegistryKpiDefinition[];
  dataInputs: RegistryDataInputDefinition[];
  kpiRequirements: RegistryKpiMinimumRequirements[];
  skills: RegistrySkillDoc[];
  protocols: RegistryProtocolDoc[];
  workflows: RegistryWorkflowDefinition[];
  agentIntelligence: RegistryAgentIntelligence[];
  validationErrors: string[];
}

export interface RegistryVersionInfo {
  version: number;
  loadedAt: string;
  fingerprint: string;
  hasExternalChanges: boolean;
}

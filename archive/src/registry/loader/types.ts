/**
 * Type definitions for the registry loader.
 */

// Company Registry (registry.json)
export interface CompanyRegistry {
  companyId: string
  companyCore: string
  branding?: {
    theme?: string
    typography?: string
    templates?: string
  }
  metadata: {
    agents: string
    skills: string
    protocols: string
    kpis: string
    'data-inputs': string
    workflows: string
    products: string
  }
  products: string
  updatedAt?: string
  migrationNote?: string
}

// Metadata Catalog (metadata/*.json)
export interface MetadataCatalog {
  companyId: string
  description?: string
  entries: Record<string, string>
  updatedAt?: string
}

export interface MetadataAgents extends MetadataCatalog {
  agentAssignments?: Record<
    string,
    {
      role?: string
      productAssignment?: string
      priority?: string
    }
  >
}

// Products Metadata (metadata/products.json)
export interface ProductAssignment {
  details: string
  documentation?: string
  owner?: string
  stage?: 'planning' | 'mvp' | 'growth' | 'mature' | 'deprecated'
  priorities?: string[]
  assignedKpis?: string[]
  assignedProtocols?: string[]
  assignedBranding?: string
  metrics?: Record<string, unknown>
  developmentPlan?: {
    currentPhase?: string
    targetLaunchDate?: string | null
    estimatedCost?: number | null
    roiTarget?: number | null
  }
}

export interface ProductsMetadata {
  companyId: string
  description?: string
  products: Record<string, ProductAssignment>
  productsOrder: string[]
  updatedAt?: string
}

// Company Metadata (all metadata/*.json combined)
export interface CompanyMetadata {
  agents: MetadataAgents | null
  skills: MetadataCatalog | null
  protocols: MetadataCatalog | null
  kpis: MetadataCatalog | null
  dataInputs: MetadataCatalog | null
  workflows: MetadataCatalog | null
  products: ProductsMetadata | null
}

// Full Company Load Result
export interface CompanyLoadResult {
  registry: CompanyRegistry
  metadata: CompanyMetadata
  validationErrors: string[]
}

// Product Details
export interface ProductDetails {
  productId: string
  companyId: string
  goal: string
  vision: string
  problemSolved: string
  usp: string
  mvp: string[]
  validation: {
    methodology: string
    successCriteria: string
  }
  contentFormats?: string[]
  targetAudience?: string[]
  contentTrack?: 'education' | 'fiction' | 'both'
  ageGroups?: string[]
  genres?: string[]
  updatedAt?: string
}

// Product Documentation
export interface ProductDocumentation {
  content: string
  metadata?: {
    productId?: string
    companyId?: string
    type?: string
    generatedAt?: string
  }
}

// Product Catalog (products/index.json)
export interface ProductCatalog {
  companyId: string
  description?: string
  products: Record<
    string,
    {
      details: string
      documentation?: string
    }
  >
  productsOrder: string[]
  updatedAt?: string
}

// Agent types
export interface AgentTemplate {
  name: string
  role: string
  backstory: string
  goal: string
  core_objective: string
  individual_vision: string
  role_non_negotiable_requirements: string[]
  objectives_long_term: string[]
  personality_traits: string[]
  interaction_style: string
  constraints: string[]
  skills: string[]
  kpis: string[]
  data?: string[]
  data_requirements: string[]
  protocols: string[]
  workflows?: string[]
}

export interface AgentDefinition extends AgentTemplate {
  uid: string
}

// Skill types
export interface SkillDoc {
  id: string
  title: string
  tags: string[]
  content: string
  sourceFile: string
}

// Protocol types
export interface ProtocolDoc {
  id: string
  title: string
  tags: string[]
  format: 'yaml' | 'markdown'
  content: string
  sourceFile: string
  rules: string[]
}

// KPI types
export interface KpiDefinition {
  uid: string
  name: string
  description: string
  unit: string
  target: string
  value: string
  formula?: string
  goalMapping: string
  frequencyOfCheck?:
    | 'real-time'
    | 'per-transaction'
    | 'hourly'
    | 'daily'
    | 'weekly'
    | 'monthly'
    | 'on-demand'
  responsibleAgentRole?: string
  thresholds?: {
    critical?: string
    warning?: string
    optimal?: string
  }
  metadata?: {
    priority?: string
    samplingFrequency?: string
    shorthand?: string
    createdDate?: string
    lastModified?: string
  }
  escalationPolicy?: {
    onCritical?: string
    onWarning?: string
  }
  dependencies?: {
    dataInputs?: string[]
    dependentKpis?: string[]
  }
}

// Data Input types
export interface DataInputDefinition {
  uid: string
  name: string
  description: string
  schemaType:
    | 'tabular'
    | 'event-stream'
    | 'document'
    | 'timeseries'
    | 'identity-protocol'
    | 'intelligence-protocol'
    | 'manifest-schema'
    | 'audit-trail'
  sourceType?:
    | 'csv'
    | 'json'
    | 'jsonl'
    | 'yaml'
    | 'sql-database'
    | 'api-endpoint'
    | 'vault'
    | 'git-repository'
    | 'event-broker'
    | 'vault-context'
    | 'file-system'
  sourceLocation?: string
  requiredFields: Array<
    | string
    | {
        name: string
        type?: string
        required?: boolean
        description?: string
      }
  >
  sampleSource: string
  validationRules?: Record<string, string>
  updateFrequency?: 'real-time' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'on-demand'
  responsibleAgent?: string
  privacyClassification?: 'public' | 'internal' | 'confidential' | 'restricted'
  retentionPolicy?: string
  dimensions?: Record<string, string>
  dependencies?: {
    dependentKpis?: string[]
    dependentWorkflows?: string[]
  }
  metadata?: {
    createdDate?: string
    lastModified?: string
    version?: string
  }
}

// Workflow types
export interface WorkflowDefinition {
  id: string
  agent_id: string
  trigger: string
  workflow_mode?: 'single-agent' | 'global-collaborative'
  preConditions?: Array<{
    condition: string
    description: string
    source?: 'kpi' | 'data-input' | 'agent-state' | 'vault' | 'git'
    validationRule?: string
  }>
  logicSequence?: Record<string, Record<string, string>>
  steps?: string[]
  postConditions?: Array<{
    effect: string
    description: string
    target?: string
    updateFormula?: string
  }>
  dependencies: {
    required_skills: string[]
    required_kpis: string[]
    data_inputs?: string[]
    required_agents?: string[]
    protocols?: string[]
  }
  collaborators?: Array<{
    agent_id: string
    role: string
    responsibility: string
    required?: boolean
  }>
  handoffRules?: {
    transfer_points: Array<{
      id: string
      from_agent: string
      to_agent: string
      condition: string
      context_packet_required_fields: string[]
      rfi_allowed?: boolean
      director_approval_required?: boolean
    }>
    milestone_approval_roles?: string[]
  }
  errorHandling?: Record<string, string>
  outputFormat?: {
    format?: 'json' | 'markdown' | 'yaml' | 'artifact-hash'
    requiredFields?: string[]
    example?: string
  }
  expected_output: string
  metadata?: {
    version?: string
    createdDate?: string
    lastModified?: string
    status?: 'draft' | 'active' | 'deprecated'
    estimatedDuration?: string
  }
  sourceFile?: string
}

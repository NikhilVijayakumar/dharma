/**
 * Load company metadata (metadata/*.json) with automatic schema validation.
 */

import { readFileSync } from 'node:fs'
import { resolveCompanyPath, resolveRegistryPath, resolveSchemaPath, pathExists } from './paths.js'
import { loadCompanyRegistry } from './registry.js'
import type { CompanyRegistry, LoadResult } from './registry.js'
import { validateJson } from './validation.js'
import type {
  CompanyMetadata,
  CompanyLoadResult,
  MetadataCatalog,
  MetadataAgents,
  ProductsMetadata
} from './types.js'

/**
 * Load a single metadata catalog file with auto-validation against $schema.
 */
export const loadMetadataCatalog = <T extends MetadataCatalog>(
  companyId: string,
  metadataPath: string
): LoadResult<T> => {
  const fullPath = resolveCompanyPath(companyId, metadataPath)
  const errors: string[] = []

  if (!pathExists(fullPath)) {
    console.warn(`Metadata catalog not found: ${fullPath}`)
    return { data: null, validationErrors: [`Metadata not found: ${metadataPath}`] }
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    const data = JSON.parse(content) as T

    // Auto-detect and validate against schema from $schema field
    if ('$schema' in data && typeof data.$schema === 'string') {
      const schemaPath = resolveSchemaPath(data.$schema)
      if (pathExists(schemaPath)) {
        const validation = validateJson(data, schemaPath)
        if (!validation.valid) {
          errors.push(...validation.errors.map((e) => `${metadataPath}: ${e}`))
        }
      } else {
        console.warn(`Schema file not found: ${schemaPath}`)
      }
    }

    return { data, validationErrors: errors }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error)
    console.error(`Failed to load metadata catalog ${metadataPath}:`, message)
    return { data: null, validationErrors: [`Failed to load ${metadataPath}: ${message}`] }
  }
}

/**
 * Load all metadata catalogs for a company.
 */
export const loadCompanyMetadata = (
  companyId: string
): { metadata: CompanyMetadata; validationErrors: string[] } => {
  const registryResult = loadCompanyRegistry(companyId)
  const errors: string[] = [...registryResult.validationErrors]

  if (!registryResult.data) {
    return {
      metadata: {
        agents: null,
        skills: null,
        protocols: null,
        kpis: null,
        dataInputs: null,
        workflows: null,
        products: null
      },
      validationErrors: errors
    }
  }

  const registry = registryResult.data

  const agentsResult = loadMetadataCatalog<MetadataAgents>(companyId, registry.metadata.agents)
  const skillsResult = loadMetadataCatalog<MetadataCatalog>(companyId, registry.metadata.skills)
  const protocolsResult = loadMetadataCatalog<MetadataCatalog>(
    companyId,
    registry.metadata.protocols
  )
  const kpisResult = loadMetadataCatalog<MetadataCatalog>(companyId, registry.metadata.kpis)
  const dataInputsResult = loadMetadataCatalog<MetadataCatalog>(
    companyId,
    registry.metadata['data-inputs']
  )
  const workflowsResult = loadMetadataCatalog<MetadataCatalog>(
    companyId,
    registry.metadata.workflows
  )
  const productsResult = loadMetadataCatalog<ProductsMetadata>(
    companyId,
    registry.metadata.products
  )

  errors.push(
    ...agentsResult.validationErrors,
    ...skillsResult.validationErrors,
    ...protocolsResult.validationErrors,
    ...kpisResult.validationErrors,
    ...dataInputsResult.validationErrors,
    ...workflowsResult.validationErrors,
    ...productsResult.validationErrors
  )

  return {
    metadata: {
      agents: agentsResult.data,
      skills: skillsResult.data,
      protocols: protocolsResult.data,
      kpis: kpisResult.data,
      dataInputs: dataInputsResult.data,
      workflows: workflowsResult.data,
      products: productsResult.data
    },
    validationErrors: errors
  }
}

/**
 * Load a company with all its metadata and validate entity references.
 */
export const loadCompany = (companyId: string): CompanyLoadResult | null => {
  const registryResult = loadCompanyRegistry(companyId)

  if (!registryResult.data) {
    console.error(`Failed to load company registry: ${companyId}`)
    return null
  }

  const registry = registryResult.data
  const metadataResult = loadCompanyMetadata(companyId)
  const metadata = metadataResult.metadata
  const validationErrors: string[] = [
    ...registryResult.validationErrors,
    ...metadataResult.validationErrors
  ]

  // Validate entity references exist
  if (metadata.agents?.entries) {
    for (const [agentId, agentPath] of Object.entries(metadata.agents.entries)) {
      const fullPath = resolveRegistryPath(agentPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Agent not found: ${agentId} -> ${agentPath}`)
      }
    }
  }

  if (metadata.skills?.entries) {
    for (const [skillId, skillPath] of Object.entries(metadata.skills.entries)) {
      const fullPath = resolveRegistryPath(skillPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Skill not found: ${skillId} -> ${skillPath}`)
      }
    }
  }

  if (metadata.protocols?.entries) {
    for (const [protocolId, protocolPath] of Object.entries(metadata.protocols.entries)) {
      const fullPath = resolveRegistryPath(protocolPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Protocol not found: ${protocolId} -> ${protocolPath}`)
      }
    }
  }

  if (metadata.kpis?.entries) {
    for (const [kpiId, kpiPath] of Object.entries(metadata.kpis.entries)) {
      const fullPath = resolveRegistryPath(kpiPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`KPI not found: ${kpiId} -> ${kpiPath}`)
      }
    }
  }

  if (metadata.dataInputs?.entries) {
    for (const [dataInputId, dataInputPath] of Object.entries(metadata.dataInputs.entries)) {
      const fullPath = resolveRegistryPath(dataInputPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Data input not found: ${dataInputId} -> ${dataInputPath}`)
      }
    }
  }

  if (metadata.workflows?.entries) {
    for (const [workflowId, workflowPath] of Object.entries(metadata.workflows.entries)) {
      const fullPath = resolveRegistryPath(workflowPath)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Workflow not found: ${workflowId} -> ${workflowPath}`)
      }
    }
  }

  if (metadata.products?.products) {
    for (const [productId, product] of Object.entries(metadata.products.products)) {
      const fullPath = resolveCompanyPath(companyId, product.details)
      if (!pathExists(fullPath)) {
        validationErrors.push(`Product details not found: ${productId} -> ${product.details}`)
      }
    }
  }

  return {
    registry,
    metadata,
    validationErrors
  }
}

/**
 * Get an entity path from company metadata.
 */
export const getEntityPath = (
  company: CompanyLoadResult,
  category: keyof CompanyMetadata,
  entityId: string
): string | null => {
  const catalog = company.metadata[category]
  if (!catalog || !('entries' in catalog)) {
    return null
  }

  const entries = (catalog as MetadataCatalog).entries
  return entries[entityId] ?? null
}

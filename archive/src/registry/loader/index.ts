/**
 * Dharma Registry Loader
 *
 * A modular loader for the Dharma registry system.
 * Supports both development and installed package modes.
 *
 * Usage:
 *
 * // Initialize with custom registry root (optional)
 * import { setRegistryRoot } from './loader'
 * setRegistryRoot('/custom/path')  // or use REGISTRY_ROOT env var
 *
 * // Load company with all metadata
 * import { loadCompany } from './loader'
 * const company = await loadCompany('bavans-publishing')
 *
 * // Load entities using paths from metadata
 * import { loadAgent, loadSkill, loadKpi } from './loader'
 * const arya = loadAgent(company.metadata.agents.entries.arya)
 * const skill = loadSkill(company.metadata.skills.entries['bavans-value-boundary-enforcement'])
 *
 * // Load products
 * import { loadProductCatalog, loadProduct, loadProductDocs } from './loader'
 * const catalog = loadProductCatalog('bavans-publishing')
 * const podcast = loadProduct('bavans-publishing', catalog.products.podcast.details)
 * const docs = loadProductDocs('bavans-publishing', catalog.products.podcast.documentation)
 */

// Paths
export { setRegistryRoot, getRegistryRoot, getCompanyRoot } from './paths.js'
export {
  resolveRegistryPath,
  resolveCompanyPath,
  resolveEntityPath,
  resolveSchemaPath,
  pathExists
} from './paths.js'

// Validation
export { validateJson, validateOrThrow, loadSchema, clearValidationCache } from './validation.js'

// Registry
export { loadCompanyRegistry, companyRegistryExists } from './registry.js'
export type { LoadResult } from './registry.js'

// Company & Metadata
export { loadCompany, loadCompanyMetadata, loadMetadataCatalog, getEntityPath } from './company.js'

// Products
export {
  loadProductCatalog,
  loadProduct,
  loadProductDocs,
  loadProductById,
  loadProductDocsById,
  loadProductWithDocs,
  loadProductWithDocsById
} from './products.js'

// Entities
export { loadAgent, agentExists } from './agents.js'
export { loadSkill, skillExists } from './skills.js'
export { loadProtocol, protocolExists } from './protocols.js'
export { loadKpi, kpiExists } from './kpis.js'
export { loadDataInput, dataInputExists } from './data-inputs.js'
export { loadWorkflow, workflowExists } from './workflows.js'

// Types
export type {
  CompanyRegistry,
  MetadataCatalog,
  MetadataAgents,
  ProductAssignment,
  ProductsMetadata,
  CompanyMetadata,
  CompanyLoadResult,
  ProductDetails,
  ProductDocumentation,
  ProductCatalog,
  AgentTemplate,
  AgentDefinition,
  SkillDoc,
  ProtocolDoc,
  KpiDefinition,
  DataInputDefinition,
  WorkflowDefinition
} from './types.js'

/**
 * Initialize the registry loader.
 * Call this before any load functions if you need a custom registry location.
 */
export const init = (options?: { registryRoot?: string }): void => {
  if (options?.registryRoot) {
    const { setRegistryRoot } = require('./paths.js')
    setRegistryRoot(options.registryRoot)
  }
}

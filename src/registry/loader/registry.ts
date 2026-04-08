/**
 * Load company registry.json with automatic schema validation.
 */

import { readFileSync } from 'node:fs'
import { resolveCompanyPath, resolveSchemaPath, pathExists } from './paths.js'
import { validateJson } from './validation.js'
import type { CompanyRegistry } from './types.js'

export interface LoadResult<T> {
  data: T | null
  validationErrors: string[]
}

/**
 * Load a company registry file (registry.json) with auto-validation.
 */
export const loadCompanyRegistry = (companyId: string): LoadResult<CompanyRegistry> => {
  const registryPath = resolveCompanyPath(companyId, 'registry.json')
  const errors: string[] = []

  if (!pathExists(registryPath)) {
    console.warn(`Company registry not found: ${registryPath}`)
    return { data: null, validationErrors: [`Registry not found: ${companyId}`] }
  }

  try {
    const content = readFileSync(registryPath, 'utf-8').replace(/^\uFEFF/, '')
    const registry = JSON.parse(content) as CompanyRegistry

    // Auto-detect and validate against schema from $schema field
    if (registry.$schema) {
      const schemaPath = resolveSchemaPath(registry.$schema)
      if (pathExists(schemaPath)) {
        const validation = validateJson(registry, schemaPath)
        if (!validation.valid) {
          errors.push(...validation.errors.map((e) => `registry.json: ${e}`))
        }
      } else {
        console.warn(`Schema file not found: ${schemaPath}`)
      }
    }

    return { data: registry, validationErrors: errors }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error)
    console.error(`Failed to load company registry for ${companyId}:`, message)
    return { data: null, validationErrors: [`Failed to load registry: ${message}`] }
  }
}

/**
 * Check if a company registry exists.
 */
export const companyRegistryExists = (companyId: string): boolean => {
  const registryPath = resolveCompanyPath(companyId, 'registry.json')
  return pathExists(registryPath)
}

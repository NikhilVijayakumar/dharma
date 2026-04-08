/**
 * Path resolution utilities for the registry loader.
 * Supports both development (process.cwd) and installed package (import.meta.url) modes.
 */

import { existsSync } from 'node:fs'
import { dirname, isAbsolute, join, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

let _registryRoot: string | null = null
let _initialized = false

/**
 * Initialize path resolution with explicit registry root.
 * Call this before any load functions if you need a custom registry location.
 */
export const setRegistryRoot = (path: string): void => {
  _registryRoot = path
  _initialized = true
}

/**
 * Get the current registry root.
 * Resolution order:
 * 1. Explicitly set via setRegistryRoot()
 * 2. REGISTRY_ROOT environment variable
 * 3. Package installation location (import.meta.url)
 */
export const getRegistryRoot = (): string => {
  if (_registryRoot) {
    return _registryRoot
  }

  if (process.env.REGISTRY_ROOT) {
    _registryRoot = process.env.REGISTRY_ROOT
    _initialized = true
    return _registryRoot
  }

  // Try to detect package location using import.meta.url
  try {
    const currentFile = fileURLToPath(import.meta.url)
    const currentDir = dirname(currentFile)
    // loader/paths.ts -> loader -> registry -> src -> package root
    const packageRoot = join(currentDir, '..', '..', '..', '..')

    // Check if we're in node_modules (package installed as dependency)
    if (packageRoot.includes('node_modules')) {
      // Navigate from node_modules/dharma/src/registry/loader to node_modules/dharma
      _registryRoot = join(packageRoot, 'src', 'registry')
    } else {
      // Development mode - use process.cwd()
      _registryRoot = join(process.cwd(), 'src', 'registry')
    }
  } catch {
    // Fallback for CommonJS or environments without import.meta
    _registryRoot = join(process.cwd(), 'src', 'registry')
  }

  _initialized = true
  return _registryRoot
}

/**
 * Get the company root directory.
 */
export const getCompanyRoot = (): string => {
  return join(getRegistryRoot(), 'company')
}

/**
 * Get a path relative to the registry root.
 */
export const resolveRegistryPath = (...segments: string[]): string => {
  return join(getRegistryRoot(), ...segments)
}

/**
 * Get a path relative to a company directory.
 */
export const resolveCompanyPath = (companyId: string, ...segments: string[]): string => {
  return join(getCompanyRoot(), companyId, ...segments)
}

/**
 * Check if a path exists.
 */
export const pathExists = (path: string): boolean => {
  try {
    return existsSync(path)
  } catch {
    return false
  }
}

/**
 * Resolve a path that may be relative to registry root or absolute.
 */
export const resolveEntityPath = (path: string): string => {
  if (isAbsolute(path)) {
    return path
  }
  return resolveRegistryPath(path)
}

/**
 * Resolve a company-relative path.
 */
export const resolveCompanyEntityPath = (companyId: string, path: string): string => {
  if (isAbsolute(path)) {
    return path
  }
  return resolveCompanyPath(companyId, path)
}

/**
 * Get the relative path from registry root to an absolute path.
 */
export const getRelativePath = (absolutePath: string): string => {
  const root = getRegistryRoot()
  return relative(root, absolutePath)
}

/**
 * Check if registry root is initialized and valid.
 */
export const isRegistryInitialized = (): boolean => {
  if (!_initialized) {
    getRegistryRoot() // Trigger initialization
  }
  return _registryRoot !== null && pathExists(getRegistryRoot())
}

/**
 * Resolve a schema path from a $schema reference in JSON files.
 * Schema references are relative to the file location, e.g.:
 *   "../../schemas/metadata-agents.schema.json" in company/<id>/metadata/agents.json
 *   "../schemas/company-core.schema.json" in company/<id>/registry.json
 *
 * This function normalizes the path to be relative to the registry root.
 */
export const resolveSchemaPath = (schemaRef: string): string => {
  // Strip leading "../" sequences to get path relative to registry root
  const normalized = schemaRef.replace(/^(\.\.\/)+/, '')
  return resolveRegistryPath(normalized)
}

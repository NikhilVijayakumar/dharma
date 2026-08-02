/**
 * Load data input definitions.
 */

import { readFileSync } from 'node:fs'
import { resolveEntityPath, pathExists } from './paths.js'
import type { DataInputDefinition } from './types.js'

/**
 * Load a data input from a path (relative to registry root).
 */
export const loadDataInput = (dataInputPath: string): DataInputDefinition | null => {
  const fullPath = resolveEntityPath(dataInputPath)

  if (!pathExists(fullPath)) {
    console.warn(`Data input not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    return JSON.parse(content) as DataInputDefinition
  } catch (error) {
    console.error(`Failed to load data input: ${dataInputPath}`, error)
    return null
  }
}

/**
 * Check if a data input exists.
 */
export const dataInputExists = (dataInputPath: string): boolean => {
  return pathExists(resolveEntityPath(dataInputPath))
}

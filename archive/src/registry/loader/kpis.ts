/**
 * Load KPI definitions.
 */

import { readFileSync } from 'node:fs'
import { resolveEntityPath, pathExists } from './paths.js'
import type { KpiDefinition } from './types.js'

/**
 * Load a KPI from a path (relative to registry root).
 */
export const loadKpi = (kpiPath: string): KpiDefinition | null => {
  const fullPath = resolveEntityPath(kpiPath)

  if (!pathExists(fullPath)) {
    console.warn(`KPI not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    return JSON.parse(content) as KpiDefinition
  } catch (error) {
    console.error(`Failed to load KPI: ${kpiPath}`, error)
    return null
  }
}

/**
 * Check if a KPI exists.
 */
export const kpiExists = (kpiPath: string): boolean => {
  return pathExists(resolveEntityPath(kpiPath))
}

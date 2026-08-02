/**
 * Load agent files.
 */

import { readFileSync } from 'node:fs'
import { basename, extname } from 'node:path'
import { load as parseYaml } from 'js-yaml'
import { resolveEntityPath, pathExists } from './paths.js'
import type { AgentDefinition } from './types.js'

/**
 * Parse tags from a string.
 */
const parseTags = (raw: string | undefined): string[] => {
  if (!raw) return []
  return raw
    .split(',')
    .map((item) => item.trim())
    .filter((item) => item.length > 0)
}

/**
 * Convert a goal field to string.
 */
const toGoalString = (goal: unknown): string => {
  if (typeof goal === 'string') return goal.trim()
  if (Array.isArray(goal)) {
    return goal
      .filter((e): e is string => typeof e === 'string')
      .map((e) => e.trim())
      .join(' | ')
  }
  if (goal && typeof goal === 'object') {
    return Object.entries(goal as Record<string, unknown>)
      .filter(([, v]) => typeof v === 'string')
      .map(([, v]) => (v as string).trim())
      .join(' | ')
  }
  return ''
}

/**
 * Load an agent from a path (relative to registry root).
 */
export const loadAgent = (agentPath: string): AgentDefinition | null => {
  const fullPath = resolveEntityPath(agentPath)

  if (!pathExists(fullPath)) {
    console.warn(`Agent not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    const extension = extname(fullPath).toLowerCase()
    let raw: Record<string, unknown>

    if (extension === '.json') {
      raw = JSON.parse(content)
    } else if (extension === '.yaml' || extension === '.yml') {
      raw = parseYaml(content) as Record<string, unknown>
    } else {
      console.error(`Unsupported agent file extension: ${extension}`)
      return null
    }

    const uid = (raw.uid as string) || basename(fullPath, extension)
    const goalText = toGoalString(raw.goal)
    const dataRefs = parseTags(raw.data_requirements as string | undefined)

    return {
      uid,
      name: typeof raw.name === 'string' ? raw.name : '',
      role: typeof raw.role === 'string' ? raw.role : '',
      backstory: typeof raw.backstory === 'string' ? raw.backstory : '',
      goal: goalText,
      core_objective:
        typeof raw.core_objective === 'string'
          ? raw.core_objective.trim()
          : goalText || 'Deliver role outcomes.',
      individual_vision:
        typeof raw.individual_vision === 'string' ? raw.individual_vision.trim() : goalText || '',
      role_non_negotiable_requirements: parseTags(
        raw.role_non_negotiable_requirements as string | undefined
      ),
      objectives_long_term: parseTags(raw.objectives_long_term as string | undefined),
      personality_traits: parseTags(raw.personality_traits as string | undefined),
      interaction_style:
        typeof raw.interaction_style === 'string' ? raw.interaction_style : 'Structured, concise.',
      constraints: parseTags(raw.constraints as string | undefined),
      skills: parseTags(raw.skills as string | undefined),
      kpis: parseTags(raw.kpis as string | undefined),
      data: dataRefs,
      data_requirements: dataRefs,
      protocols: parseTags(raw.protocols as string | undefined),
      workflows: parseTags(raw.workflows as string | undefined)
    }
  } catch (error) {
    console.error(`Failed to load agent: ${agentPath}`, error)
    return null
  }
}

/**
 * Check if an agent exists.
 */
export const agentExists = (agentPath: string): boolean => {
  return pathExists(resolveEntityPath(agentPath))
}

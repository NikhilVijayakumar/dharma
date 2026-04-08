/**
 * Load workflow definitions.
 */

import { readFileSync } from 'node:fs'
import { basename, extname } from 'node:path'
import { load as parseYaml } from 'js-yaml'
import { resolveEntityPath, pathExists } from './paths.js'
import type { WorkflowDefinition } from './types.js'

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
 * Load a workflow from a path (relative to registry root).
 */
export const loadWorkflow = (workflowPath: string): WorkflowDefinition | null => {
  const fullPath = resolveEntityPath(workflowPath)

  if (!pathExists(fullPath)) {
    console.warn(`Workflow not found: ${fullPath}`)
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
      console.error(`Unsupported workflow file extension: ${extension}`)
      return null
    }

    // Parse dependencies
    const deps = raw.dependencies as Record<string, unknown> | undefined

    // Parse collaborators
    const collaboratorsRaw = raw.collaborators as unknown[] | undefined
    const collaborators = collaboratorsRaw
      ?.map((c) => {
        const col = c as Record<string, unknown>
        return {
          agent_id: typeof col.agent_id === 'string' ? col.agent_id.trim() : '',
          role: typeof col.role === 'string' ? col.role.trim() : '',
          responsibility: typeof col.responsibility === 'string' ? col.responsibility.trim() : '',
          required: typeof col.required === 'boolean' ? col.required : undefined
        }
      })
      .filter((c) => c.agent_id && c.role && c.responsibility)

    // Parse steps
    let steps: string[] = []
    if (Array.isArray(raw.steps)) {
      steps = raw.steps.filter((s): s is string => typeof s === 'string')
    }

    // Build path parts for fallback IDs
    const pathParts = workflowPath.split('/')
    const fallbackAgentId = pathParts.length > 1 ? pathParts[0] : ''

    return {
      id: typeof raw.id === 'string' ? raw.id : basename(fullPath, extension),
      agent_id: typeof raw.agent_id === 'string' ? raw.agent_id : fallbackAgentId,
      trigger: typeof raw.trigger === 'string' ? raw.trigger : 'Manual invocation',
      workflow_mode: raw.workflow_mode as 'single-agent' | 'global-collaborative' | undefined,
      steps,
      dependencies: {
        required_skills: parseTags(deps?.required_skills as string | undefined),
        required_kpis: parseTags(deps?.required_kpis as string | undefined),
        data_inputs: parseTags(deps?.data_inputs as string | undefined),
        required_agents: parseTags(deps?.required_agents as string | undefined),
        protocols: parseTags(deps?.protocols as string | undefined)
      },
      collaborators: collaborators.length > 0 ? collaborators : undefined,
      expected_output:
        typeof raw.expected_output === 'string'
          ? raw.expected_output
          : 'Deliver the workflow result with explicit status and rationale.',
      sourceFile: workflowPath
    }
  } catch (error) {
    console.error(`Failed to load workflow: ${workflowPath}`, error)
    return null
  }
}

/**
 * Check if a workflow exists.
 */
export const workflowExists = (workflowPath: string): boolean => {
  return pathExists(resolveEntityPath(workflowPath))
}

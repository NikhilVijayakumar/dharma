/**
 * Load protocol documents.
 */

import { readFileSync } from 'node:fs'
import { basename, extname } from 'node:path'
import { load as parseYaml } from 'js-yaml'
import { resolveEntityPath, pathExists } from './paths.js'
import type { ProtocolDoc } from './types.js'

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
 * Load a protocol from a path (relative to registry root).
 */
export const loadProtocol = (protocolPath: string): ProtocolDoc | null => {
  const fullPath = resolveEntityPath(protocolPath)

  if (!pathExists(fullPath)) {
    console.warn(`Protocol not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    const extension = extname(fullPath).toLowerCase()

    // Handle markdown protocols
    if (extension === '.md') {
      const frontMatterMatch = content.match(/^---\n([\s\S]*?)\n---\n?/)
      let id = basename(fullPath, '.md')
      let title = id
      let tags: string[] = []

      if (frontMatterMatch) {
        const frontMatter = frontMatterMatch[1]
        const idMatch = frontMatter.match(/^id:\s*(.+)$/m)
        const titleMatch = frontMatter.match(/^title:\s*(.+)$/m)
        const tagsMatch = frontMatter.match(/^tags:\s*\[(.*)\]\s*$/m)

        if (idMatch?.[1]) id = idMatch[1].trim()
        if (titleMatch?.[1]) title = titleMatch[1].trim()
        if (tagsMatch?.[1]) tags = parseTags(tagsMatch[1])
      }

      const rules = Array.from(content.matchAll(/^\s*-\s+(.+)$/gm)).map((match) => match[1].trim())

      return {
        id,
        title,
        tags,
        format: 'markdown',
        content,
        sourceFile: protocolPath,
        rules
      }
    }

    // Handle YAML protocols
    if (extension === '.yaml' || extension === '.yml') {
      const parsed = parseYaml(content) as Record<string, unknown>
      const raw = parsed as Record<string, unknown>

      const rules = Array.from(content.matchAll(/^\s*-\s+(.+)$/gm)).map((match) => match[1].trim())

      return {
        id: typeof raw.id === 'string' ? raw.id : basename(fullPath, extension),
        title: typeof raw.title === 'string' ? raw.title : basename(fullPath, extension),
        tags: parseTags(raw.tags as string | undefined),
        format: 'yaml',
        content,
        sourceFile: protocolPath,
        rules
      }
    }

    console.error(`Unsupported protocol file extension: ${extension}`)
    return null
  } catch (error) {
    console.error(`Failed to load protocol: ${protocolPath}`, error)
    return null
  }
}

/**
 * Check if a protocol exists.
 */
export const protocolExists = (protocolPath: string): boolean => {
  return pathExists(resolveEntityPath(protocolPath))
}

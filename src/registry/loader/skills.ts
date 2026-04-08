/**
 * Load skill documents.
 */

import { readFileSync } from 'node:fs'
import { basename } from 'node:path'
import { resolveEntityPath, pathExists } from './paths.js'
import type { SkillDoc } from './types.js'

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
 * Load a skill from a path (relative to registry root).
 */
export const loadSkill = (skillPath: string): SkillDoc | null => {
  const fullPath = resolveEntityPath(skillPath)

  if (!pathExists(fullPath)) {
    console.warn(`Skill not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')

    // Parse front matter
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

    return {
      id,
      title,
      tags,
      content,
      sourceFile: skillPath
    }
  } catch (error) {
    console.error(`Failed to load skill: ${skillPath}`, error)
    return null
  }
}

/**
 * Check if a skill exists.
 */
export const skillExists = (skillPath: string): boolean => {
  return pathExists(resolveEntityPath(skillPath))
}

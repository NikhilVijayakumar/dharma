/**
 * Schema validation utilities for the registry loader.
 */

import { readFileSync } from 'node:fs'
import Ajv, { ValidateFunction } from 'ajv'

let ajvInstance: Ajv | null = null
const schemaCache: Map<string, object> = new Map()
const validatorCache: Map<string, ValidateFunction> = new Map()

/**
 * Get the shared Ajv instance.
 */
export const getAjv = (): Ajv => {
  if (!ajvInstance) {
    ajvInstance = new Ajv({ allErrors: true, strict: false })
  }
  return ajvInstance
}

/**
 * Load and cache a JSON schema.
 */
export const loadSchema = (schemaPath: string): object | null => {
  if (schemaCache.has(schemaPath)) {
    return schemaCache.get(schemaPath)!
  }

  try {
    const content = readFileSync(schemaPath, 'utf-8').replace(/^\uFEFF/, '')
    const schema = JSON.parse(content)
    schemaCache.set(schemaPath, schema)
    return schema
  } catch {
    console.error(`Failed to load schema: ${schemaPath}`)
    return null
  }
}

/**
 * Get or create a validator for a schema.
 */
export const getValidator = (schemaPath: string): ValidateFunction | null => {
  if (validatorCache.has(schemaPath)) {
    return validatorCache.get(schemaPath)!
  }

  const schema = loadSchema(schemaPath)
  if (!schema) {
    return null
  }

  const ajv = getAjv()
  const validator = ajv.compile(schema)
  validatorCache.set(schemaPath, validator)
  return validator
}

/**
 * Validate data against a schema.
 */
export const validateJson = <T = unknown>(
  data: T,
  schemaPath: string
): { valid: boolean; errors: string[] } => {
  const validator = getValidator(schemaPath)
  if (!validator) {
    return { valid: false, errors: [`Schema not found: ${schemaPath}`] }
  }

  const valid = validator(data)
  if (valid) {
    return { valid: true, errors: [] }
  }

  const errors = (validator.errors ?? []).map(
    (err) => `${err.instancePath || '(root)'} ${err.message ?? 'invalid'}`
  )
  return { valid: false, errors }
}

/**
 * Validate data and throw if invalid.
 */
export const validateOrThrow = <T = unknown>(data: T, schemaPath: string): void => {
  const result = validateJson(data, schemaPath)
  if (!result.valid) {
    throw new Error(`Validation failed: ${result.errors.join('; ')}`)
  }
}

/**
 * Clear cached schemas and validators.
 */
export const clearValidationCache = (): void => {
  schemaCache.clear()
  validatorCache.clear()
}

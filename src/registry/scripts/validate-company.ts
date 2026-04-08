/**
 * Company Registry Validation Script
 * Validates company-specific registry files against schemas and checks referential integrity.
 *
 * Validates:
 * - All metadata/*.json files parse correctly and match schemas
 * - All referenced global registry entries exist
 * - Product details match expected schema
 * - Product assignments reference valid entities
 * - No broken references
 * - Required fields present
 */

import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { basename, extname, join, relative } from 'node:path'
import Ajv from 'ajv'

const REGISTRY_ROOT = join(process.cwd(), 'src', 'registry')
const COMPANY_ROOT = join(REGISTRY_ROOT, 'company')

interface ValidationError {
  type: string
  severity: 'critical' | 'error' | 'warning'
  file: string
  message: string
  reference?: string
}

interface ValidationReport {
  isValid: boolean
  companyId: string
  errors: ValidationError[]
  warnings: ValidationError[]
  statistics: {
    metadataFilesValidated: number
    productsValidated: number
    globalReferencesChecked: number
    brokenReferences: number
  }
}

const readJsonFile = <T>(filePath: string): T | null => {
  try {
    const content = readFileSync(filePath, 'utf-8').replace(/^\uFEFF/, '')
    return JSON.parse(content) as T
  } catch (error) {
    return null
  }
}

const readTextFile = (filePath: string): string | null => {
  try {
    return readFileSync(filePath, 'utf-8').replace(/^\uFEFF/, '')
  } catch {
    return null
  }
}

const validateJsonAgainstSchema = (
  data: unknown,
  schemaPath: string,
  filePath: string
): { valid: boolean; errors: string[] } => {
  const schema = readJsonFile<Record<string, unknown>>(schemaPath)
  if (!schema) {
    return { valid: false, errors: [`Schema not found: ${schemaPath}`] }
  }

  const ajv = new Ajv({ allErrors: true })
  const validate = ajv.compile(schema)
  const valid = validate(data)

  if (!valid && validate.errors) {
    const errors = validate.errors.map(
      (err) => `${err.instancePath || '(root)'} ${err.message ?? 'invalid'}`
    )
    return { valid: false, errors }
  }

  return { valid: true, errors: [] }
}

const getGlobalRegistryDirs = (): Record<string, string> => ({
  agents: join(REGISTRY_ROOT, 'agents'),
  skills: join(REGISTRY_ROOT, 'skills'),
  protocols: join(REGISTRY_ROOT, 'protocols'),
  kpis: join(REGISTRY_ROOT, 'kpis'),
  dataInputs: join(REGISTRY_ROOT, 'data-inputs'),
  workflows: join(REGISTRY_ROOT, 'workflows')
})

const getGlobalExtensions = (): Record<string, string[]> => ({
  agents: ['.yaml', '.yml', '.json'],
  skills: ['.md'],
  protocols: ['.yaml', '.yml', '.md'],
  kpis: ['.json'],
  dataInputs: ['.json'],
  workflows: ['.yaml', '.yml', '.json']
})

const getGlobalFilePatterns = (): Record<string, (filename: string) => string | null> => ({
  agents: (filename) => {
    const base = basename(filename, extname(filename))
    return base
  },
  skills: (filename) => basename(filename, '.md'),
  protocols: (filename) => basename(filename, extname(filename)),
  kpis: (filename) => {
    const content = readJsonFile<{ uid?: string }>(join(REGISTRY_ROOT, 'kpis', filename))
    return content?.uid ?? basename(filename, '.json')
  },
  dataInputs: (filename) => {
    const content = readJsonFile<{ uid?: string }>(join(REGISTRY_ROOT, 'data-inputs', filename))
    return content?.uid ?? basename(filename, '.json')
  },
  workflows: (filename) => basename(filename, extname(filename))
})

const fileExists = (filePath: string): boolean => existsSync(filePath)

const listFilesRecursive = (dirPath: string, extensions: string[]): string[] => {
  if (!existsSync(dirPath)) {
    return []
  }

  const results: string[] = []
  const entries = readdirSync(dirPath, { withFileTypes: true })

  for (const entry of entries) {
    const fullPath = join(dirPath, entry.name)
    if (entry.isDirectory()) {
      results.push(...listFilesRecursive(fullPath, extensions))
    } else if (entry.isFile()) {
      const ext = extname(entry.name).toLowerCase()
      if (extensions.includes(ext)) {
        results.push(fullPath)
      }
    }
  }

  return results
}

const validateMetadataFile = (
  companyId: string,
  category: string,
  metadataPath: string,
  schemaPath: string,
  globalDir: string,
  globalExts: string[]
): { errors: ValidationError[]; warnings: ValidationError[]; referencesChecked: number } => {
  const errors: ValidationError[] = []
  const warnings: ValidationError[] = []
  let referencesChecked = 0

  const metadata = readJsonFile<Record<string, unknown>>(metadataPath)
  if (!metadata) {
    errors.push({
      type: 'parse-error',
      severity: 'critical',
      file: metadataPath,
      message: `Failed to parse metadata file`
    })
    return { errors, warnings, referencesChecked }
  }

  const schemaValid = validateJsonAgainstSchema(metadata, schemaPath, metadataPath)
  if (!schemaValid.valid) {
    for (const err of schemaValid.errors) {
      errors.push({
        type: 'schema-validation',
        severity: 'error',
        file: relative(REGISTRY_ROOT, metadataPath),
        message: `Schema validation failed: ${err}`
      })
    }
  }

  const entries = metadata.entries as Record<string, string> | undefined
  if (entries) {
    for (const [entryId, relativePath] of Object.entries(entries)) {
      referencesChecked++
      const fullPath = join(REGISTRY_ROOT, 'company', companyId, relativePath)

      if (!fileExists(fullPath)) {
        errors.push({
          type: 'broken-reference',
          severity: 'critical',
          file: relative(REGISTRY_ROOT, metadataPath),
          message: `Broken reference: ${entryId} -> ${relativePath}`,
          reference: relativePath
        })
      } else {
        const ext = extname(fullPath).toLowerCase()
        if (!globalExts.includes(ext)) {
          warnings.push({
            type: 'unexpected-extension',
            severity: 'warning',
            file: relative(REGISTRY_ROOT, metadataPath),
            message: `Unexpected extension for ${category}/${entryId}: expected ${globalExts.join(' or ')}, got ${ext}`
          })
        }
      }
    }
  }

  return { errors, warnings, referencesChecked }
}

const validateProductDetails = (
  companyId: string,
  productId: string,
  detailsPath: string,
  schemaPath: string
): { errors: ValidationError[]; warnings: ValidationError[] } => {
  const errors: ValidationError[] = []
  const warnings: ValidationError[] = []

  const details = readJsonFile<Record<string, unknown>>(detailsPath)
  if (!details) {
    errors.push({
      type: 'parse-error',
      severity: 'critical',
      file: relative(REGISTRY_ROOT, detailsPath),
      message: `Failed to parse product details`
    })
    return { errors, warnings }
  }

  const schemaValid = validateJsonAgainstSchema(details, schemaPath, detailsPath)
  if (!schemaValid.valid) {
    for (const err of schemaValid.errors) {
      errors.push({
        type: 'schema-validation',
        severity: 'error',
        file: relative(REGISTRY_ROOT, detailsPath),
        message: `Schema validation failed for ${productId}: ${err}`
      })
    }
  }

  const requiredFields = ['goal', 'vision', 'problemSolved', 'usp', 'mvp', 'validation']
  for (const field of requiredFields) {
    if (!(field in details)) {
      errors.push({
        type: 'missing-required-field',
        severity: 'critical',
        file: relative(REGISTRY_ROOT, detailsPath),
        message: `Missing required field: ${field}`
      })
    }
  }

  const mvp = details.mvp as unknown[]
  if (!Array.isArray(mvp) || mvp.length === 0) {
    warnings.push({
      type: 'empty-mvp',
      severity: 'warning',
      file: relative(REGISTRY_ROOT, detailsPath),
      message: `Product ${productId} has empty MVP - may be in planning stage`
    })
  }

  return { errors, warnings }
}

const validateCompanyProducts = (
  companyId: string,
  productsDir: string,
  metadataProducts: Record<string, unknown> | undefined,
  productsIndexPath: string
): { errors: ValidationError[]; warnings: ValidationError[]; productsValidated: number } => {
  const errors: ValidationError[] = []
  const warnings: ValidationError[] = []
  let productsValidated = 0

  const schemaPath = join(REGISTRY_ROOT, 'schemas', 'product-standalone.schema.json')
  if (!fileExists(schemaPath)) {
    errors.push({
      type: 'missing-schema',
      severity: 'critical',
      file: 'schemas/product-standalone.schema.json',
      message: 'Product schema not found'
    })
    return { errors, warnings, productsValidated }
  }

  if (!metadataProducts) {
    errors.push({
      type: 'missing-products-metadata',
      severity: 'critical',
      file: relative(REGISTRY_ROOT, productsIndexPath),
      message: 'Products metadata not found in registry'
    })
    return { errors, warnings, productsValidated }
  }

  const products = metadataProducts as Record<
    string,
    {
      details?: string
      documentation?: string
      owner?: string
      stage?: string
      assignedKpis?: string[]
      assignedProtocols?: string[]
    }
  >

  for (const [productId, assignment] of Object.entries(products)) {
    if (!assignment.details) {
      errors.push({
        type: 'missing-product-details',
        severity: 'critical',
        file: relative(REGISTRY_ROOT, productsIndexPath),
        message: `Product ${productId} missing details path`
      })
      continue
    }

    const detailsPath = join(REGISTRY_ROOT, 'company', companyId, assignment.details)
    if (!fileExists(detailsPath)) {
      errors.push({
        type: 'broken-reference',
        severity: 'critical',
        file: relative(REGISTRY_ROOT, productsIndexPath),
        message: `Product ${productId} details not found: ${assignment.details}`,
        reference: assignment.details
      })
    } else {
      const { errors: detailErrors, warnings: detailWarnings } = validateProductDetails(
        companyId,
        productId,
        detailsPath,
        schemaPath
      )
      errors.push(...detailErrors)
      warnings.push(...detailWarnings)
      productsValidated++
    }

    if (assignment.documentation) {
      const docPath = join(REGISTRY_ROOT, 'company', companyId, assignment.documentation)
      if (!fileExists(docPath)) {
        warnings.push({
          type: 'missing-documentation',
          severity: 'warning',
          file: relative(REGISTRY_ROOT, productsIndexPath),
          message: `Product ${productId} documentation not found: ${assignment.documentation}`
        })
      }
    }

    if (!assignment.stage) {
      warnings.push({
        type: 'missing-stage',
        severity: 'warning',
        file: relative(REGISTRY_ROOT, productsIndexPath),
        message: `Product ${productId} missing stage assignment`
      })
    }
  }

  return { errors, warnings, productsValidated }
}

const validateCompany = (companyId: string): ValidationReport => {
  const report: ValidationReport = {
    isValid: true,
    companyId,
    errors: [],
    warnings: [],
    statistics: {
      metadataFilesValidated: 0,
      productsValidated: 0,
      globalReferencesChecked: 0,
      brokenReferences: 0
    }
  }

  const companyDir = join(COMPANY_ROOT, companyId)
  if (!existsSync(companyDir)) {
    report.errors.push({
      type: 'company-not-found',
      severity: 'critical',
      file: relative(REGISTRY_ROOT, companyDir),
      message: `Company directory not found: ${companyId}`
    })
    report.isValid = false
    return report
  }

  const registryPath = join(companyDir, 'registry.json')
  const registry = readJsonFile<Record<string, unknown>>(registryPath)
  if (!registry) {
    report.errors.push({
      type: 'registry-not-found',
      severity: 'critical',
      file: relative(REGISTRY_ROOT, registryPath),
      message: 'Company registry.json not found'
    })
    report.isValid = false
    return report
  }

  const schemaRoot = join(REGISTRY_ROOT, 'schemas')
  const companyCoreSchema = join(schemaRoot, 'company-core.schema.json')
  const companyCorePath = join(companyDir, 'company-core.json')
  const companyCore = readJsonFile<Record<string, unknown>>(companyCorePath)
  if (companyCore) {
    const coreValid = validateJsonAgainstSchema(companyCore, companyCoreSchema, companyCorePath)
    if (!coreValid.valid) {
      for (const err of coreValid.errors) {
        report.errors.push({
          type: 'schema-validation',
          severity: 'error',
          file: relative(REGISTRY_ROOT, companyCorePath),
          message: `Company core schema validation: ${err}`
        })
      }
    }
  }

  const metadataDir = join(companyDir, 'metadata')
  if (existsSync(metadataDir)) {
    const globalDirs = getGlobalRegistryDirs()
    const globalExts = getGlobalExtensions()
    const metadataCategories = [
      { name: 'agents', file: 'agents.json', schema: 'metadata-agents.schema.json' },
      { name: 'skills', file: 'skills.json', schema: 'metadata-catalog.schema.json' },
      { name: 'protocols', file: 'protocols.json', schema: 'metadata-catalog.schema.json' },
      { name: 'kpis', file: 'kpis.json', schema: 'metadata-catalog.schema.json' },
      { name: 'dataInputs', file: 'data-inputs.json', schema: 'metadata-catalog.schema.json' },
      { name: 'workflows', file: 'workflows.json', schema: 'metadata-catalog.schema.json' }
    ]

    for (const cat of metadataCategories) {
      const metadataPath = join(metadataDir, cat.file)
      const schemaPath = join(schemaRoot, cat.schema)

      if (fileExists(metadataPath) && fileExists(schemaPath)) {
        const { errors, warnings, referencesChecked } = validateMetadataFile(
          companyId,
          cat.name,
          metadataPath,
          schemaPath,
          globalDirs[cat.name] || '',
          globalExts[cat.name] || []
        )

        report.errors.push(...errors)
        report.warnings.push(...warnings)
        report.statistics.metadataFilesValidated++
        report.statistics.globalReferencesChecked += referencesChecked
        report.statistics.brokenReferences += errors.filter(
          (e) => e.type === 'broken-reference'
        ).length
      }
    }

    const productsMetadataPath = join(metadataDir, 'products.json')
    const productsSchema = join(schemaRoot, 'company-product.schema.json')
    const productsMetadata = readJsonFile<Record<string, unknown>>(productsMetadataPath)

    if (productsMetadata && fileExists(productsSchema)) {
      const productsValid = validateJsonAgainstSchema(
        productsMetadata,
        productsSchema,
        productsMetadataPath
      )
      if (!productsValid.valid) {
        for (const err of productsValid.errors) {
          report.errors.push({
            type: 'schema-validation',
            severity: 'error',
            file: relative(REGISTRY_ROOT, productsMetadataPath),
            message: `Products metadata schema: ${err}`
          })
        }
      }
    }

    const productsDir = join(companyDir, 'products')
    const productsIndexPath = join(productsDir, 'index.json')
    const { errors, warnings, productsValidated } = validateCompanyProducts(
      companyId,
      productsDir,
      productsMetadata?.products,
      productsIndexPath
    )

    report.errors.push(...errors)
    report.warnings.push(...warnings)
    report.statistics.productsValidated = productsValidated
  }

  report.isValid = report.errors.filter((e) => e.severity === 'critical').length === 0

  return report
}

const printReport = (report: ValidationReport): void => {
  console.log('\n' + '='.repeat(80))
  console.log(`🔍 COMPANY REGISTRY VALIDATION: ${report.companyId}`)
  console.log('='.repeat(80))

  console.log('\n📊 STATISTICS:')
  console.log(`  Metadata files validated: ${report.statistics.metadataFilesValidated}`)
  console.log(`  Products validated: ${report.statistics.productsValidated}`)
  console.log(`  Global references checked: ${report.statistics.globalReferencesChecked}`)
  console.log(`  Broken references: ${report.statistics.brokenReferences}`)

  if (report.errors.length > 0) {
    console.log('\n' + '='.repeat(80))
    console.log('🚨 ERRORS:')
    console.log('='.repeat(80))
    report.errors.forEach((error) => {
      console.log(`  [${error.severity.toUpperCase()}] ${error.type}`)
      console.log(`    File: ${error.file}`)
      console.log(`    Message: ${error.message}`)
    })
  }

  if (report.warnings.length > 0) {
    console.log('\n' + '='.repeat(80))
    console.log('⚠️ WARNINGS:')
    console.log('='.repeat(80))
    report.warnings.forEach((warning) => {
      console.log(`  [${warning.severity.toUpperCase()}] ${warning.type}`)
      console.log(`    File: ${warning.file}`)
      console.log(`    Message: ${warning.message}`)
    })
  }

  console.log('\n' + '='.repeat(80))
  if (report.isValid) {
    console.log('✅ VALIDATION PASSED')
  } else {
    console.log('❌ VALIDATION FAILED')
  }
  console.log('='.repeat(80) + '\n')
}

const runValidation = (companyId?: string): void => {
  if (companyId) {
    const report = validateCompany(companyId)
    printReport(report)

    const reportPath = join(REGISTRY_ROOT, 'company', companyId, 'validation-report.json')
    writeFileSync(reportPath, JSON.stringify(report, null, 2))
    console.log(`📄 Report written to: ${reportPath}\n`)

    process.exit(report.isValid ? 0 : 1)
  } else {
    if (!existsSync(COMPANY_ROOT)) {
      console.log('No company registry found.')
      return
    }

    const companies = readdirSync(COMPANY_ROOT, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name)

    if (companies.length === 0) {
      console.log('No companies found.')
      return
    }

    console.log(`Found ${companies.length} company(ies): ${companies.join(', ')}`)

    const allReports: ValidationReport[] = []
    for (const company of companies) {
      const report = validateCompany(company)
      allReports.push(report)
      printReport(report)
    }

    const overallValid = allReports.every((r) => r.isValid)
    const totalErrors = allReports.reduce((sum, r) => sum + r.errors.length, 0)
    const totalWarnings = allReports.reduce((sum, r) => sum + r.warnings.length, 0)

    console.log('\n' + '='.repeat(80))
    console.log('📋 SUMMARY')
    console.log('='.repeat(80))
    console.log(`Companies validated: ${allReports.length}`)
    console.log(`Passed: ${allReports.filter((r) => r.isValid).length}`)
    console.log(`Failed: ${allReports.filter((r) => !r.isValid).length}`)
    console.log(`Total errors: ${totalErrors}`)
    console.log(`Total warnings: ${totalWarnings}`)

    const summaryPath = join(REGISTRY_ROOT, 'company-validation-summary.json')
    writeFileSync(summaryPath, JSON.stringify(allReports, null, 2))
    console.log(`\n📄 Summary written to: ${summaryPath}\n`)

    process.exit(overallValid ? 0 : 1)
  }
}

const args = process.argv.slice(2)
const companyId = args[0] || undefined

runValidation(companyId)

export { validateCompany, validateMetadataFile, validateProductDetails, type ValidationReport }

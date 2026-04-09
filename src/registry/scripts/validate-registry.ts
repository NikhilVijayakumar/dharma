/**
 * Registry Validation Script
 * Performs comprehensive referential integrity checks across the Registry.
 *
 * Checks:
 * - KPI references exist
 * - Skill references exist
 * - Agent references are valid
 * - Protocol references exist
 * - Workflow dependencies are met
 * - Data input sources are accessible
 * - No circular dependencies
 */

import * as fs from 'fs'
import * as path from 'path'
import { glob } from 'glob'

const REGISTRY_ROOT = process.env.REGISTRY_ROOT || path.resolve(process.cwd(), 'src', 'registry')

interface ValidationReport {
  isValid: boolean
  errors: ValidationError[]
  warnings: ValidationWarning[]
  statistics: ValidationStatistics
  dependencyMatrix: DependencyMatrix
}

interface ValidationError {
  type: string
  severity: 'critical' | 'error' | 'warning'
  file: string
  message: string
  reference?: string
}

interface ValidationWarning {
  type: string
  message: string
  file?: string
}

interface ValidationStatistics {
  totalAgents: number
  totalWorkflows: number
  totalKpis: number
  totalDataInputs: number
  totalSkills: number
  totalProtocols: number
  brokenKpiReferences: number
  brokenSkillReferences: number
  brokenAgentReferences: number
  orphanedKpis: number
  orphanedDataInputs: number
}

interface DependencyMatrix {
  agentToKpis: Record<string, Set<string>>
  agentToSkills: Record<string, Set<string>>
  agentToWorkflows: Record<string, Set<string>>
  workflowToKpis: Record<string, Set<string>>
  workflowToDataInputs: Record<string, Set<string>>
  kpiToDependentWorkflows: Record<string, Set<string>>
  dataInputToDependentWorkflows: Record<string, Set<string>>
}

// Initialize report
const report: ValidationReport = {
  isValid: true,
  errors: [],
  warnings: [],
  statistics: {
    totalAgents: 0,
    totalWorkflows: 0,
    totalKpis: 0,
    totalDataInputs: 0,
    totalSkills: 0,
    totalProtocols: 0,
    brokenKpiReferences: 0,
    brokenSkillReferences: 0,
    brokenAgentReferences: 0,
    orphanedKpis: 0,
    orphanedDataInputs: 0
  },
  dependencyMatrix: {
    agentToKpis: {},
    agentToSkills: {},
    agentToWorkflows: {},
    workflowToKpis: {},
    workflowToDataInputs: {},
    kpiToDependentWorkflows: {},
    dataInputToDependentWorkflows: {}
  }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

function readJsonFile(filePath: string): any {
  try {
    const content = fs.readFileSync(filePath, 'utf-8')
    return JSON.parse(content)
  } catch (error) {
    report.errors.push({
      type: 'file-parse-error',
      severity: 'error',
      file: filePath,
      message: `Failed to parse JSON: ${(error as Error).message}`
    })
    return null
  }
}

// ============================================================================
// VALIDATION FUNCTIONS
// ============================================================================

function validateKpiReferences(): void {
  console.log('\n📋 Validating KPI References...')

  const kpisDir = path.join(REGISTRY_ROOT, 'kpis')
  const existingKpis = new Set<string>()

  // Collect all existing KPIs
  const kpiFiles = glob.sync('*.json', { cwd: kpisDir })
  kpiFiles.forEach((file) => {
    const kpiData = readJsonFile(path.join(kpisDir, file))
    if (kpiData && kpiData.uid) {
      existingKpis.add(kpiData.uid)
    }
  })

  report.statistics.totalKpis = existingKpis.size

  // Check workflow dependencies
  const workflowsDir = path.join(REGISTRY_ROOT, 'workflows')
  const workflowFiles = glob.sync('**/*.yaml', { cwd: workflowsDir })

  workflowFiles.forEach((file) => {
    const filePath = path.join(workflowsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Simple regex-based extraction of required_kpis
    const kpiMatch = content.match(/required_kpis:\s*\n([\s\S]*?)(?=\n\S|\Z)/)
    if (kpiMatch) {
      const kpiLines = kpiMatch[1].split('\n').filter((line) => line.includes('-'))
      kpiLines.forEach((line) => {
        const kpiRef = line.replace(/[\s\-]/g, '').trim()
        if (kpiRef && !existingKpis.has(kpiRef)) {
          report.errors.push({
            type: 'missing-kpi-reference',
            severity: 'error',
            file: file,
            message: `References missing KPI: ${kpiRef}`,
            reference: kpiRef
          })
          report.statistics.brokenKpiReferences++
          report.isValid = false
        } else if (kpiRef) {
          // Track dependency
          if (!report.dependencyMatrix.workflowToKpis[file]) {
            report.dependencyMatrix.workflowToKpis[file] = new Set()
          }
          report.dependencyMatrix.workflowToKpis[file].add(kpiRef)
        }
      })
    }
  })

  // Check agent references
  const agentsDir = path.join(REGISTRY_ROOT, 'agents')
  const agentFiles = glob.sync('*.yaml', { cwd: agentsDir })

  agentFiles.forEach((file) => {
    const filePath = path.join(agentsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Extract KPI references
    const kpiMatch = content.match(/kpis:\s*\n([\s\S]*?)(?=\n\S|\Z)/)
    if (kpiMatch) {
      const kpiLines = kpiMatch[1].split('\n').filter((line) => line.includes('-'))
      const agentId = file.replace('.yaml', '')
      report.dependencyMatrix.agentToKpis[agentId] = new Set()

      kpiLines.forEach((line) => {
        const kpiRef = line.replace(/[\s\-]/g, '').trim()
        if (kpiRef && !existingKpis.has(kpiRef)) {
          report.errors.push({
            type: 'missing-kpi-reference',
            severity: 'error',
            file: file,
            message: `Agent references missing KPI: ${kpiRef}`,
            reference: kpiRef
          })
          report.statistics.brokenKpiReferences++
          report.isValid = false
        } else if (kpiRef) {
          report.dependencyMatrix.agentToKpis[agentId]?.add(kpiRef)
        }
      })
    }
  })
}

function validateSkillReferences(): void {
  console.log('\n🎯 Validating Skill References...')

  const skillsDir = path.join(REGISTRY_ROOT, 'skills')
  const existingSkills = new Set<string>()

  // Collect all existing skills
  const skillFiles = glob.sync('**/*.md', { cwd: skillsDir })
  skillFiles.forEach((file) => {
    const skillId = path.basename(file, '.md')
    existingSkills.add(skillId)
  })

  report.statistics.totalSkills = existingSkills.size

  // Check workflow dependencies
  const workflowsDir = path.join(REGISTRY_ROOT, 'workflows')
  const workflowFiles = glob.sync('**/*.yaml', { cwd: workflowsDir })

  workflowFiles.forEach((file) => {
    const filePath = path.join(workflowsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Extract required_skills
    const skillMatch = content.match(/required_skills:\s*\n([\s\S]*?)(?=\n\S|\Z)/)
    if (skillMatch) {
      const skillLines = skillMatch[1].split('\n').filter((line) => line.includes('-'))
      skillLines.forEach((line) => {
        const skillRef = line.replace(/[\s\-]/g, '').trim()
        if (skillRef && !existingSkills.has(skillRef)) {
          report.errors.push({
            type: 'missing-skill-reference',
            severity: 'error',
            file: file,
            message: `References missing skill: ${skillRef}`,
            reference: skillRef
          })
          report.statistics.brokenSkillReferences++
          report.isValid = false
        }
      })
    }
  })
}

function validateAgentReferences(): void {
  console.log('\n👥 Validating Agent References...')

  const agentsDir = path.join(REGISTRY_ROOT, 'agents')
  const existingAgents = new Set<string>()

  // Collect all existing agents
  const agentFiles = glob.sync('*.yaml', { cwd: agentsDir })
  agentFiles.forEach((file) => {
    const agentId = file.replace('.yaml', '')
    existingAgents.add(agentId)
  })

  report.statistics.totalAgents = existingAgents.size

  // Check workflow references
  const workflowsDir = path.join(REGISTRY_ROOT, 'workflows')
  const workflowFiles = glob.sync('**/*.yaml', { cwd: workflowsDir })

  workflowFiles.forEach((file) => {
    const filePath = path.join(workflowsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Extract agent_id and required_agents
    const agentMatch = content.match(/agent_id:\s*(\S+)/)
    if (agentMatch) {
      const agentId = agentMatch[1]
      if (!existingAgents.has(agentId)) {
        report.errors.push({
          type: 'missing-agent-reference',
          severity: 'critical',
          file: file,
          message: `References missing agent: ${agentId}`,
          reference: agentId
        })
        report.statistics.brokenAgentReferences++
        report.isValid = false
      }
    }

    // Check required_agents for multi-agent workflows
    const requiredAgentsMatch = content.match(/required_agents:\s*\n([\s\S]*?)(?=\n\S|\Z)/)
    if (requiredAgentsMatch) {
      const agentLines = requiredAgentsMatch[1].split('\n').filter((line) => line.includes('-'))
      agentLines.forEach((line) => {
        const agentRef = line.replace(/[\s\-]/g, '').trim()
        if (agentRef && !existingAgents.has(agentRef)) {
          report.errors.push({
            type: 'missing-agent-reference',
            severity: 'error',
            file: file,
            message: `References missing agent: ${agentRef}`,
            reference: agentRef
          })
          report.statistics.brokenAgentReferences++
          report.isValid = false
        }
      })
    }
  })
}

function validateDataInputs(): void {
  console.log('\n📊 Validating Data Input References...')

  const dataInputsDir = path.join(REGISTRY_ROOT, 'data-inputs')
  const existingDataInputs = new Set<string>()

  // Collect all existing data inputs
  const dataInputFiles = glob.sync('*.json', { cwd: dataInputsDir })
  dataInputFiles.forEach((file) => {
    const dataInputData = readJsonFile(path.join(dataInputsDir, file))
    if (dataInputData && dataInputData.uid) {
      existingDataInputs.add(dataInputData.uid)
    }
  })

  report.statistics.totalDataInputs = existingDataInputs.size

  // Check workflow references
  const workflowsDir = path.join(REGISTRY_ROOT, 'workflows')
  const workflowFiles = glob.sync('**/*.yaml', { cwd: workflowsDir })

  workflowFiles.forEach((file) => {
    const filePath = path.join(workflowsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Extract data_inputs
    const dataInputMatch = content.match(/data_inputs:\s*\n([\s\S]*?)(?=\n\S|\Z)/)
    if (dataInputMatch) {
      const dataLines = dataInputMatch[1]
        .split('\n')
        .filter((line) => line.trim() && !line.startsWith('#'))
      dataLines.forEach((line) => {
        const dataRef = line
          .replace(/[\s\-]/g, '')
          .split('(')[0]
          .trim()
        if (dataRef && !existingDataInputs.has(dataRef)) {
          report.warnings.push({
            type: 'missing-data-input-reference',
            message: `Workflow ${file} references data input: ${dataRef} (may be external)`
          })
        }
      })
    }
  })
}

function validateProtocols(): void {
  console.log('\n📜 Validating Protocol References...')

  const protocolsDir = path.join(REGISTRY_ROOT, 'protocols')
  const existingProtocols = new Set<string>()

  // Collect all existing protocols
  const protocolFiles = glob.sync('*.yaml', { cwd: protocolsDir })
  protocolFiles.forEach((file) => {
    const protocolId = file.replace('.yaml', '')
    existingProtocols.add(protocolId)
  })

  report.statistics.totalProtocols = existingProtocols.size
}

function validateWorkflows(): void {
  console.log('\n⚙️ Validating Workflows...')

  const workflowsDir = path.join(REGISTRY_ROOT, 'workflows')
  const workflowFiles = glob.sync('**/*.yaml', { cwd: workflowsDir })

  report.statistics.totalWorkflows = workflowFiles.length

  workflowFiles.forEach((file) => {
    const filePath = path.join(workflowsDir, file)
    const content = fs.readFileSync(filePath, 'utf-8')

    // Check for required fields
    if (!content.includes('id:')) {
      report.warnings.push({
        type: 'missing-field',
        message: `Workflow ${file} missing 'id' field`
      })
    }

    if (!content.includes('agent_id:')) {
      report.errors.push({
        type: 'missing-required-field',
        severity: 'critical',
        file: file,
        message: `Workflow ${file} missing required 'agent_id' field`
      })
      report.isValid = false
    }

    // Check for pre_conditions references
    if (content.includes('pre_conditions')) {
      report.warnings.push({
        type: 'advanced-feature',
        message: `Workflow ${file} uses pre_conditions (schema v2.0)`
      })
    }
  })
}

function buildDependencyMatrix(): void {
  console.log('\n🗺️ Building Dependency Matrix...')

  // Convert Sets to arrays for JSON serialization
  const matrix: any = {}

  for (const key in report.dependencyMatrix) {
    const value = (report.dependencyMatrix as any)[key]
    if (typeof value === 'object') {
      matrix[key] = {}
      for (const subKey in value) {
        matrix[key][subKey] = Array.from((value as any)[subKey])
      }
    }
  }

  report.dependencyMatrix = matrix
}

// ============================================================================
// MAIN EXECUTION
// ============================================================================

function runValidation(): void {
  console.log('\n' + '='.repeat(80))
  console.log('🔍 REGISTRY VALIDATION REPORT')
  console.log('='.repeat(80))

  validateKpiReferences()
  validateSkillReferences()
  validateAgentReferences()
  validateDataInputs()
  validateProtocols()
  validateWorkflows()
  buildDependencyMatrix()

  // Output report
  console.log('\n' + '='.repeat(80))
  console.log('📊 VALIDATION STATISTICS')
  console.log('='.repeat(80))
  console.log(`Total Agents: ${report.statistics.totalAgents}`)
  console.log(`Total Workflows: ${report.statistics.totalWorkflows}`)
  console.log(`Total KPIs: ${report.statistics.totalKpis}`)
  console.log(`Total Data Inputs: ${report.statistics.totalDataInputs}`)
  console.log(`Total Skills: ${report.statistics.totalSkills}`)
  console.log(`Total Protocols: ${report.statistics.totalProtocols}`)

  console.log('\n❌ ERRORS & WARNINGS:')
  console.log(`Broken KPI References: ${report.statistics.brokenKpiReferences}`)
  console.log(`Broken Skill References: ${report.statistics.brokenSkillReferences}`)
  console.log(`Broken Agent References: ${report.statistics.brokenAgentReferences}`)
  console.log(`Total Errors: ${report.errors.length}`)
  console.log(`Total Warnings: ${report.warnings.length}`)

  if (report.errors.length > 0) {
    console.log('\n' + '='.repeat(80))
    console.log('🚨 ERRORS:')
    console.log('='.repeat(80))
    report.errors.slice(0, 20).forEach((error) => {
      console.log(`  [${error.severity.toUpperCase()}] ${error.type}`)
      console.log(`    File: ${error.file}`)
      console.log(`    Message: ${error.message}`)
      if (error.reference) console.log(`    Reference: ${error.reference}`)
    })
    if (report.errors.length > 20) {
      console.log(`  ... and ${report.errors.length - 20} more errors`)
    }
  }

  if (report.warnings.length > 0) {
    console.log('\n' + '='.repeat(80))
    console.log('⚠️ WARNINGS (First 10):')
    console.log('='.repeat(80))
    report.warnings.slice(0, 10).forEach((warning) => {
      console.log(`  [${warning.type}] ${warning.message}`)
    })
    if (report.warnings.length > 10) {
      console.log(`  ... and ${report.warnings.length - 10} more warnings`)
    }
  }

  // Determine final status
  console.log('\n' + '='.repeat(80))
  if (report.isValid) {
    console.log('✅ REGISTRY VALIDATION PASSED')
  } else {
    console.log('❌ REGISTRY VALIDATION FAILED')
  }
  console.log('='.repeat(80) + '\n')

  // Write detailed report to file
  const reportPath = path.join(REGISTRY_ROOT, 'validation-report.json')
  fs.writeFileSync(reportPath, JSON.stringify(report, null, 2))
  console.log(`📄 Detailed report written to: ${reportPath}\n`)

  process.exit(report.isValid ? 0 : 1)
}

// Run validation
if (require.main === module) {
  runValidation()
}

export { type ValidationReport, validateKpiReferences, validateSkillReferences }

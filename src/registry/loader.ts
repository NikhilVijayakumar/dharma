import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { basename, extname, join, relative } from 'node:path';
import Ajv from 'ajv';
import { load as parseYaml } from 'js-yaml';
import {
  RegistryAgentDefinition,
  RegistryAgentAlignmentCheck,
  RegistryAgentIntelligence,
  RegistryAgentTemplate,
  RegistryBusinessContext,
  RegistryProductDetails,
  RegistryDataInputDefinition,
  RegistryKpiDefinition,
  RegistryKpiMinimumRequirements,
  RegistryModuleManifest,
  RegistryProtocolDoc,
  RegistryCompanyCore,
  RegistryResolvedAgent,
  RegistrySkillDoc,
  RegistrySnapshot,
  RegistryWorkflowDefinition,
} from './types';

const REGISTRY_ROOT = join(process.cwd(), 'src', 'core', 'registry');
const LOCAL_OVERRIDES_ROOT = join(process.cwd(), 'local', 'overrides');

const readText = (filePath: string): string => readFileSync(filePath, 'utf8').replace(/^\uFEFF/, '');

const readJson = <T>(filePath: string): T => {
  const raw = readText(filePath);
  return JSON.parse(raw) as T;
};

const readStructured = (filePath: string): unknown => {
  const extension = extname(filePath).toLowerCase();
  const raw = readText(filePath);

  if (extension === '.json') {
    return JSON.parse(raw) as unknown;
  }

  if (extension === '.yaml' || extension === '.yml') {
    let parsed: unknown;
    try {
      parsed = parseYaml(raw);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);

      // Some generated YAML files can include duplicate keys during transitions.
      // In that case, fall back to JSON-compatible parsing where the last key wins
      // so runtime onboarding does not hard-freeze.
      if (/duplicated mapping key/i.test(message)) {
        parsed = parseYaml(raw, { json: true });
      } else {
        throw error;
      }
    }

    if (parsed === null || typeof parsed !== 'object') {
      throw new Error(`Expected object-like YAML at ${relative(REGISTRY_ROOT, filePath)}`);
    }
    return parsed;
  }

  throw new Error(`Unsupported structured file extension: ${extension}`);
};

const listFilesWithExtensions = (dirPath: string, extensions: string[]): string[] => {
  if (!existsSync(dirPath)) {
    return [];
  }

  const entries = readdirSync(dirPath, { withFileTypes: true });
  const output: string[] = [];

  for (const entry of entries) {
    const absolutePath = join(dirPath, entry.name);
    if (entry.isDirectory()) {
      output.push(...listFilesWithExtensions(absolutePath, extensions));
      continue;
    }
    if (entry.isFile() && extensions.some((extension) => absolutePath.endsWith(extension))) {
      output.push(absolutePath);
    }
  }

  return output.sort((a, b) => a.localeCompare(b));
};

const listFiles = (dirPath: string, extension: string): string[] => listFilesWithExtensions(dirPath, [extension]);

const COMPANY_CORE_FILE = join(REGISTRY_ROOT, 'company', 'company-core.json');
const PRODUCT_DETAILS_FILE = join(REGISTRY_ROOT, 'company', 'product-details.json');

const parseTags = (raw: string | undefined): string[] => {
  if (!raw) {
    return [];
  }
  return raw
    .split(',')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
};

const parseSkillDoc = (filePath: string): RegistrySkillDoc => {
  const content = readText(filePath);
  const frontMatterMatch = content.match(/^---\n([\s\S]*?)\n---\n?/);

  let id = relative(join(REGISTRY_ROOT, 'skills'), filePath).replace(/\\/g, '/').replace(/\.md$/, '');
  let title = id;
  let tags: string[] = [];

  if (frontMatterMatch) {
    const frontMatter = frontMatterMatch[1];
    const idMatch = frontMatter.match(/^id:\s*(.+)$/m);
    const titleMatch = frontMatter.match(/^title:\s*(.+)$/m);
    const tagsMatch = frontMatter.match(/^tags:\s*\[(.*)\]\s*$/m);

    if (idMatch?.[1]) {
      id = idMatch[1].trim();
    }
    if (titleMatch?.[1]) {
      title = titleMatch[1].trim();
    }
    tags = parseTags(tagsMatch?.[1]);
  }

  return {
    id,
    title,
    tags,
    content,
    sourceFile: relative(REGISTRY_ROOT, filePath).replace(/\\/g, '/'),
  };
};

const parseProtocolMarkdownDoc = (filePath: string): RegistryProtocolDoc => {
  const content = readText(filePath);
  const frontMatterMatch = content.match(/^---\n([\s\S]*?)\n---\n?/);

  let id = basename(filePath, '.md');
  let title = id;
  let tags: string[] = [];

  if (frontMatterMatch) {
    const frontMatter = frontMatterMatch[1];
    const idMatch = frontMatter.match(/^id:\s*(.+)$/m);
    const titleMatch = frontMatter.match(/^title:\s*(.+)$/m);
    const tagsMatch = frontMatter.match(/^tags:\s*\[(.*)\]\s*$/m);

    if (idMatch?.[1]) {
      id = idMatch[1].trim();
    }
    if (titleMatch?.[1]) {
      title = titleMatch[1].trim();
    }
    tags = parseTags(tagsMatch?.[1]);
  }

  const rules = Array.from(content.matchAll(/^\s*-\s+(.+)$/gm)).map((match) => match[1].trim());

  return {
    id,
    title,
    tags,
    format: 'markdown',
    content,
    sourceFile: relative(REGISTRY_ROOT, filePath).replace(/\\/g, '/'),
    rules,
  };
};

const normalizeReference = (value: string): string => {
  return value
    .toLowerCase()
    .replace(/\([^)]*\)/g, ' ')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '');
};

const toStringArray = (input: unknown): string[] => {
  if (!Array.isArray(input)) {
    return [];
  }

  return input
    .filter((item): item is string => typeof item === 'string')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
};

const tokenizeAlignmentText = (value: string): string[] => {
  return value
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .map((entry) => entry.trim())
    .filter((entry) => entry.length >= 5);
};

const uniqueStrings = (values: string[]): string[] => {
  return Array.from(new Set(values.map((entry) => entry.trim()).filter((entry) => entry.length > 0)));
};

const buildBusinessContext = (
  company: RegistryCompanyCore,
  product: RegistryProductDetails,
): RegistryBusinessContext => {
  const nonNegotiables = toStringArray(company.global_non_negotiables);
  const operationalRules = company.operational_rules
    ? Object.values(company.operational_rules).flatMap((rules) => toStringArray(rules))
    : [];

  const productRules = [
    'No mixing of children education and mature fiction tracks in any workflow output.',
    'Human review is mandatory before publishing any content variant.',
    'Audio-first distribution decisions must preserve age-context boundaries and trust-first publishing.',
  ];

  const companyProtocolRules = uniqueStrings([
    ...nonNegotiables,
    ...operationalRules,
    ...productRules,
  ]);

  const companyText = [
    company.vision,
    ...toStringArray(company.core_values),
    ...companyProtocolRules,
  ].join(' ');
  const productText = [
    ...toStringArray(product.product_and_ecosystem?.philosophy),
    ...toStringArray(product.product_and_ecosystem?.core_content_units),
    String(product.product_and_ecosystem?.current_focus?.primary ?? ''),
  ].join(' ');

  return {
    company,
    product,
    companyProtocolRules,
    alignmentKeywords: uniqueStrings([
      ...tokenizeAlignmentText(companyText),
      ...tokenizeAlignmentText(productText),
    ]),
  };
};

const ensureBusinessAlignedAgentTemplate = (
  template: RegistryAgentTemplate,
  context: RegistryBusinessContext,
): RegistryAgentTemplate => {
  const requiredRoleRequirement = 'Enforce company global non-negotiables and human approval gates for critical actions';
  const productBoundaryRequirement = 'Preserve strict separation between children education outputs and mature fiction outputs';

  const roleRequirements = uniqueStrings([
    ...template.role_non_negotiable_requirements,
    requiredRoleRequirement,
    productBoundaryRequirement,
  ]);

  const missionPrefix = `Align role execution with company vision: ${context.company.vision}`;
  const hasVisionAnchor = template.individual_vision.toLowerCase().includes('company vision')
    || context.alignmentKeywords.some((keyword) => template.individual_vision.toLowerCase().includes(keyword));
  const hasCoreObjectiveAnchor = template.core_objective.toLowerCase().includes('company')
    || context.alignmentKeywords.some((keyword) => template.core_objective.toLowerCase().includes(keyword));

  return {
    ...template,
    core_objective: hasCoreObjectiveAnchor
      ? template.core_objective
      : `${missionPrefix}. ${template.core_objective}`,
    individual_vision: hasVisionAnchor
      ? template.individual_vision
      : `${missionPrefix}. ${template.individual_vision}`,
    role_non_negotiable_requirements: roleRequirements,
    objectives_long_term: uniqueStrings([
      ...template.objectives_long_term,
      'Maintain measurable alignment to company mission and product-track constraints',
    ]),
  };
};

const evaluateAgentAlignment = (
  agent: RegistryAgentDefinition,
  context: RegistryBusinessContext,
): RegistryAgentAlignmentCheck => {
  const issues: RegistryAgentAlignmentCheck['issues'] = [];

  const checkField = (
    field: RegistryAgentAlignmentCheck['issues'][number]['field'],
    value: string,
    threshold: number,
    reasonPrefix: string,
  ): void => {
    const normalizedValue = value.toLowerCase();
    const overlap = context.alignmentKeywords.filter((keyword) => normalizedValue.includes(keyword)).length;
    const aligned = overlap >= threshold;
    issues.push({
      field,
      aligned,
      reason: aligned
        ? `${reasonPrefix} aligned (${overlap} keyword matches)`
        : `${reasonPrefix} not aligned (${overlap} keyword matches, expected >= ${threshold})`,
    });
  };

  checkField('goal', agent.goal, 2, 'Goal');
  checkField('backstory', agent.backstory, 2, 'Backstory');
  checkField('core_objective', agent.core_objective, 3, 'Core objective');
  checkField('individual_vision', agent.individual_vision, 3, 'Individual vision');

  const roleReqText = agent.role_non_negotiable_requirements.join(' ').toLowerCase();
  const roleAligned = roleReqText.includes('non-negotiable')
    && roleReqText.includes('human')
    && roleReqText.includes('separation');
  issues.push({
    field: 'role_non_negotiable_requirements',
    aligned: roleAligned,
    reason: roleAligned
      ? 'Role non-negotiables aligned with company protocol boundaries'
      : 'Role non-negotiables missing one or more of: non-negotiables, human-review gate, product-track separation',
  });

  return {
    agentId: agent.uid,
    aligned: issues.every((issue) => issue.aligned),
    issues,
  };
};

const injectCompanyProtocolsIntoWorkflow = (
  workflow: RegistryWorkflowDefinition,
  context: RegistryBusinessContext,
): RegistryWorkflowDefinition => {
  const protocolSteps = [
    `Company Protocol Gate: ${context.companyProtocolRules[0] ?? 'Honor company non-negotiables before execution'}`,
    `Business Context Gate: Keep outputs aligned with product focus (${String(context.product.product_and_ecosystem.current_focus?.primary ?? 'active focus')}) and avoid cross-track content mixing`,
  ];

  const existingSteps = workflow.steps ?? [];
  const mergedSteps = uniqueStrings([...protocolSteps, ...existingSteps]);

  const dependencyProtocols = uniqueStrings([
    ...(workflow.dependencies.protocols ?? []),
    'company-non-negotiables-protocol',
    'human-review-required-protocol',
    'product-track-separation-protocol',
  ]);

  return {
    ...workflow,
    steps: mergedSteps,
    dependencies: {
      ...workflow.dependencies,
      protocols: dependencyProtocols,
    },
  };
};

const upsertDraftField = (
  records: Array<{ key: string; value: string }>,
  key: string,
  value: string,
): Array<{ key: string; value: string }> => {
  const existingIndex = records.findIndex((record) => record.key === key);
  if (existingIndex >= 0) {
    const next = [...records];
    next[existingIndex] = { key, value };
    return next;
  }

  return [...records, { key, value }];
};

const toGoalString = (goal: unknown): string => {
  if (typeof goal === 'string') {
    return goal.trim();
  }

  if (Array.isArray(goal)) {
    const values = goal.filter((entry): entry is string => typeof entry === 'string').map((entry) => entry.trim());
    return values.join(' | ');
  }

  if (goal && typeof goal === 'object') {
    const entries = Object.entries(goal as Record<string, unknown>)
      .filter(([, value]) => typeof value === 'string')
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([, value]) => (value as string).trim());
    return entries.join(' | ');
  }

  return '';
};

const toGoalObjectives = (goal: unknown): string[] => {
  if (Array.isArray(goal)) {
    return goal.filter((entry): entry is string => typeof entry === 'string').map((entry) => entry.trim());
  }

  if (goal && typeof goal === 'object') {
    return Object.entries(goal as Record<string, unknown>)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([, value]) => value)
      .filter((value): value is string => typeof value === 'string')
      .map((value) => value.trim());
  }

  if (typeof goal === 'string' && goal.trim().length > 0) {
    return [goal.trim()];
  }

  return [];
};

const normalizeAgentTemplate = (raw: unknown): RegistryAgentTemplate => {
  const source = raw as Record<string, unknown>;
  const goalText = toGoalString(source.goal);
  const inferredObjectives = toGoalObjectives(source.goal);
  const workflowRefs = Array.isArray(source.workflows)
    ? toStringArray(source.workflows)
    : (source.workflows && typeof source.workflows === 'object'
      ? Object.keys(source.workflows as Record<string, unknown>).map((key) => key.trim()).filter((key) => key.length > 0)
      : []);
  const dataRefs = toStringArray(source.data_requirements).length > 0
    ? toStringArray(source.data_requirements)
    : toStringArray(source.data);
  const derivedObjective = goalText.length > 0 ? goalText : 'Deliver role outcomes aligned with company mission.';
  const individualVision = typeof source.individual_vision === 'string' && source.individual_vision.trim().length > 0
    ? source.individual_vision.trim()
    : derivedObjective;
  const roleNonNegotiables = toStringArray(source.role_non_negotiable_requirements).length > 0
    ? toStringArray(source.role_non_negotiable_requirements)
    : toStringArray(source.constraints);

  return {
    name: typeof source.name === 'string' ? source.name : '',
    role: typeof source.role === 'string' ? source.role : '',
    backstory: typeof source.backstory === 'string' ? source.backstory : '',
    goal: goalText,
    core_objective: typeof source.core_objective === 'string' && source.core_objective.trim().length > 0
      ? source.core_objective.trim()
      : derivedObjective,
    individual_vision: individualVision,
    role_non_negotiable_requirements: roleNonNegotiables.length > 0
      ? roleNonNegotiables
      : ['Align role execution with global non-negotiables and approved protocols'],
    objectives_long_term: toStringArray(source.objectives_long_term).length > 0
      ? toStringArray(source.objectives_long_term)
      : inferredObjectives,
    personality_traits: toStringArray(source.personality_traits).length > 0
      ? toStringArray(source.personality_traits)
      : ['analytical', 'collaborative'],
    interaction_style: typeof source.interaction_style === 'string'
      ? source.interaction_style
      : 'Structured, concise, and action-oriented communication.',
    constraints: toStringArray(source.constraints).length > 0
      ? toStringArray(source.constraints)
      : ['Follow registry governance rules', 'Escalate high-risk decisions before execution'],
    skills: toStringArray(source.skills),
    kpis: toStringArray(source.kpis),
    data: dataRefs,
    data_requirements: dataRefs,
    protocols: toStringArray(source.protocols),
    workflows: workflowRefs,
  };
};

const normalizeWorkflow = (
  raw: unknown,
  fallbackId: string,
  fallbackAgentId: string,
): Omit<RegistryWorkflowDefinition, 'sourceFile'> => {
  const source = raw as Record<string, unknown>;

  const objectSteps = source.steps && typeof source.steps === 'object' && !Array.isArray(source.steps)
    ? Object.entries(source.steps as Record<string, unknown>)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([, value]) => value)
      .filter((value): value is string => typeof value === 'string')
      .map((value) => value.trim())
      .filter((value) => value.length > 0)
    : [];

  const numberedSteps = Object.entries(source)
    .filter(([key, value]) => /^step[_-]?\d+$/i.test(key) && typeof value === 'string')
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([, value]) => value as string)
    .map((value) => value.trim())
    .filter((value) => value.length > 0);

  const steps = toStringArray(source.steps).length > 0
    ? toStringArray(source.steps)
    : (objectSteps.length > 0 ? objectSteps : numberedSteps);

  const dependenciesSource = source.dependencies && typeof source.dependencies === 'object'
    ? (source.dependencies as Record<string, unknown>)
    : {};

  const inferredWorkflowId = typeof source.id === 'string'
    ? source.id
    : fallbackId;

  const inferredAgentId = typeof source.agent_id === 'string'
    ? source.agent_id
    : fallbackAgentId;

  const collaborators = Array.isArray(source.collaborators)
    ? source.collaborators
      .filter((entry): entry is Record<string, unknown> => Boolean(entry) && typeof entry === 'object')
      .map((entry) => {
        const collaborator: {
          agent_id: string;
          role: string;
          responsibility: string;
          required?: boolean;
        } = {
          agent_id: typeof entry.agent_id === 'string' ? entry.agent_id.trim() : '',
          role: typeof entry.role === 'string' ? entry.role.trim() : '',
          responsibility: typeof entry.responsibility === 'string' ? entry.responsibility.trim() : '',
        };

        if (typeof entry.required === 'boolean') {
          collaborator.required = entry.required;
        }

        return collaborator;
      })
      .filter((entry) => entry.agent_id.length > 0 && entry.role.length > 0 && entry.responsibility.length > 0)
    : [];

  const handoffRulesSource = source.handoff_rules && typeof source.handoff_rules === 'object'
    ? source.handoff_rules as Record<string, unknown>
    : null;

  const transferPoints = handoffRulesSource && Array.isArray(handoffRulesSource.transfer_points)
    ? handoffRulesSource.transfer_points
      .filter((entry): entry is Record<string, unknown> => Boolean(entry) && typeof entry === 'object')
      .map((entry) => {
        const transferPoint: {
          id: string;
          from_agent: string;
          to_agent: string;
          condition: string;
          context_packet_required_fields: string[];
          rfi_allowed?: boolean;
          director_approval_required?: boolean;
        } = {
          id: typeof entry.id === 'string' ? entry.id.trim() : '',
          from_agent: typeof entry.from_agent === 'string' ? entry.from_agent.trim() : '',
          to_agent: typeof entry.to_agent === 'string' ? entry.to_agent.trim() : '',
          condition: typeof entry.condition === 'string' ? entry.condition.trim() : '',
          context_packet_required_fields: toStringArray(entry.context_packet_required_fields),
        };

        if (typeof entry.rfi_allowed === 'boolean') {
          transferPoint.rfi_allowed = entry.rfi_allowed;
        }

        if (typeof entry.director_approval_required === 'boolean') {
          transferPoint.director_approval_required = entry.director_approval_required;
        }

        return transferPoint;
      })
      .filter((entry) => entry.id.length > 0 && entry.from_agent.length > 0 && entry.to_agent.length > 0 && entry.condition.length > 0 && entry.context_packet_required_fields.length > 0)
    : [];

  const handoffRules = transferPoints.length > 0
    ? {
      transfer_points: transferPoints,
      milestone_approval_roles: toStringArray(handoffRulesSource?.milestone_approval_roles),
    }
    : undefined;

  return {
    id: inferredWorkflowId,
    agent_id: inferredAgentId,
    trigger: typeof source.trigger === 'string'
      ? source.trigger
      : (typeof source.mode_trigger === 'string' ? source.mode_trigger : 'Manual invocation'),
    workflow_mode: source.workflow_mode === 'global-collaborative' || source.workflow_mode === 'single-agent'
      ? source.workflow_mode
      : undefined,
    steps,
    dependencies: {
      required_skills: toStringArray(dependenciesSource.required_skills),
      required_kpis: toStringArray(dependenciesSource.required_kpis),
      data_inputs: toStringArray(dependenciesSource.data_inputs),
      required_agents: toStringArray(dependenciesSource.required_agents),
      protocols: toStringArray(dependenciesSource.protocols),
    },
    collaborators: collaborators.length > 0 ? collaborators : undefined,
    handoffRules: handoffRules
      ? {
        transfer_points: handoffRules.transfer_points,
        milestone_approval_roles: handoffRules.milestone_approval_roles,
      }
      : undefined,
    expected_output: typeof source.expected_output === 'string'
      ? source.expected_output
      : 'Deliver the workflow result with explicit status and rationale.',
  };
};

const buildFingerprint = (paths: string[]): string => {
  const parts = paths.map((path) => {
    const stats = statSync(path);
    return `${relative(REGISTRY_ROOT, path)}:${stats.mtimeMs}`;
  });
  return parts.join('|');
};

const resolveOverridePath = (category: 'kpis' | 'data-inputs', uid: string): string | null => {
  const overridePath = join(LOCAL_OVERRIDES_ROOT, category, `${uid}.json`);
  if (!existsSync(overridePath)) {
    return null;
  }

  return overridePath;
};

const applyDefinitionOverrides = <T extends { uid: string }>(
  category: 'kpis' | 'data-inputs',
  entries: T[],
): Array<{ sourcePath: string; value: T }> => {
  return entries.map((entry) => {
    const overridePath = resolveOverridePath(category, entry.uid);
    if (!overridePath) {
      return {
        sourcePath: join(REGISTRY_ROOT, category, `${entry.uid}.json`),
        value: entry,
      };
    }

    const parsedOverride = readJson<T>(overridePath);
    return {
      sourcePath: overridePath,
      value: parsedOverride,
    };
  });
};

const buildAgentSetupFieldSchema = (agents: RegistryResolvedAgent[]): Array<Record<string, unknown>> => {
  return agents.flatMap((agent) => [
    {
      key: `${agent.uid}.goal`,
      mandatoryForEfficiency: true,
      guidance: `${agent.name} goal should be role-specific, outcome-oriented, and measurable.`,
      minWords: 14,
    },
    {
      key: `${agent.uid}.backstory`,
      mandatoryForEfficiency: true,
      guidance: `${agent.name} backstory should explain professional context and operating style.`,
      minWords: 18,
    },
    {
      key: `${agent.uid}.skills`,
      mandatoryForEfficiency: true,
      guidance: `${agent.name} skill list should map to core capabilities in the registry skill library.`,
      minWords: 2,
    },
    {
      key: `${agent.uid}.required_kpis`,
      mandatoryForEfficiency: true,
      guidance: `${agent.name} KPI linkage should map to: ${agent.kpis.map((kpi) => kpi.name).join(', ')}.`,
      minWords: 2,
    },
  ]);
};

const buildKpiInputFieldSchema = (kpis: RegistryKpiDefinition[]): Array<Record<string, unknown>> => {
  return kpis.flatMap((kpi) => [
    {
      key: `${kpi.uid}.target`,
      mandatoryForEfficiency: true,
      guidance: `Set a business-specific target for ${kpi.name} (${kpi.unit}).`,
      minWords: 1,
    },
    {
      key: `${kpi.uid}.value`,
      mandatoryForEfficiency: false,
      guidance: `Optional baseline/current value for ${kpi.name}.`,
      minWords: 1,
    },
  ]);
};

const buildAgentSetupDraft = (agents: RegistryResolvedAgent[]): Array<{ key: string; value: string }> => {
  return agents.flatMap((agent) => [
    {
      key: `${agent.uid}.goal`,
      value: agent.goal,
    },
    {
      key: `${agent.uid}.backstory`,
      value: agent.backstory,
    },
    {
      key: `${agent.uid}.skills`,
      value: agent.skills.map((skill) => skill.id).join(', '),
    },
    {
      key: `${agent.uid}.required_kpis`,
      value: agent.kpis.map((kpi) => kpi.uid).join(', '),
    },
  ]);
};

const buildKpiInputDraft = (kpis: RegistryKpiDefinition[]): Array<{ key: string; value: string }> => {
  return kpis.flatMap((kpi) => [
    {
      key: `${kpi.uid}.target`,
      value: kpi.target,
    },
    {
      key: `${kpi.uid}.value`,
      value: kpi.value,
    },
  ]);
};

const resolveByReference = <T>(
  value: string,
  entries: T[],
  keys: (entry: T) => string[],
): T | null => {
  const normalizedValue = normalizeReference(value);

  for (const entry of entries) {
    const normalizedKeys = keys(entry).map(normalizeReference);
    if (normalizedKeys.includes(normalizedValue)) {
      return entry;
    }
  }

  for (const entry of entries) {
    const normalizedKeys = keys(entry).map(normalizeReference);
    if (normalizedKeys.some((key) => key.includes(normalizedValue) || normalizedValue.includes(key))) {
      return entry;
    }
  }

  return null;
};

const parseProtocolYamlDoc = (filePath: string): RegistryProtocolDoc => {
  const parsed = readStructured(filePath) as Record<string, unknown>;

  const rules = toStringArray(parsed.rules).length > 0
    ? toStringArray(parsed.rules)
    : Array.from(readText(filePath).matchAll(/^\s*-\s+(.+)$/gm)).map((match) => match[1].trim());

  return {
    id: typeof parsed.id === 'string' ? parsed.id : basename(filePath, extname(filePath)),
    title: typeof parsed.title === 'string' ? parsed.title : basename(filePath, extname(filePath)),
    tags: toStringArray(parsed.tags),
    format: 'yaml',
    content: readText(filePath),
    sourceFile: relative(REGISTRY_ROOT, filePath).replace(/\\/g, '/'),
    rules,
  };
};

export interface RegistryLoadResult {
  snapshot: RegistrySnapshot;
  fingerprint: string;
}

export const getRegistryRoot = (): string => REGISTRY_ROOT;

export const getRegistryFileFingerprint = (): string => {
  const allTracked = [
    ...listFiles(join(REGISTRY_ROOT, 'modules'), '.manifest.json'),
    ...listFilesWithExtensions(join(REGISTRY_ROOT, 'agents'), ['.json', '.yaml', '.yml']),
    ...listFilesWithExtensions(join(REGISTRY_ROOT, 'workflows'), ['.json', '.yaml', '.yml']),
    ...listFilesWithExtensions(join(REGISTRY_ROOT, 'protocols'), ['.md', '.yaml', '.yml']),
    ...listFiles(join(REGISTRY_ROOT, 'kpis'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'data-inputs'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'kpi-schemas'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'skills'), '.md'),
    ...listFiles(join(REGISTRY_ROOT, 'onboarding', 'defaults'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'onboarding', 'schemas'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'company'), '.json'),
    ...listFiles(join(REGISTRY_ROOT, 'schemas'), '.json'),
    ...listFiles(join(LOCAL_OVERRIDES_ROOT, 'kpis'), '.json'),
    ...listFiles(join(LOCAL_OVERRIDES_ROOT, 'data-inputs'), '.json'),
  ];

  return buildFingerprint(allTracked);
};

export const loadRegistrySnapshot = (version: number): RegistryLoadResult => {
  const schemaRoot = join(REGISTRY_ROOT, 'schemas');
  const moduleManifestSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'module-manifest.schema.json'));
  const agentTemplateSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'agent-template.schema.json'));
  const kpiMinimumSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'kpi-minimum-requirements.schema.json'));
  const kpiLibrarySchema = readJson<Record<string, unknown>>(join(schemaRoot, 'kpi-definition.schema.json'));
  const dataInputLibrarySchema = readJson<Record<string, unknown>>(join(schemaRoot, 'data-input-definition.schema.json'));
  const protocolSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'protocol.schema.json'));
  const workflowSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'workflow.schema.json'));
  const companyCoreSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'company-core.schema.json'));
  const productDetailsSchema = readJson<Record<string, unknown>>(join(schemaRoot, 'product-details.schema.json'));
  const defaultCompany: RegistryCompanyCore = {
    vision: 'Define and approve company vision before onboarding agents.',
    context: 'Company context is required to evaluate onboarding alignment.',
    core_values: [],
    global_non_negotiables: [],
  };
  const defaultProduct: RegistryProductDetails = {
    product_and_ecosystem: {
      philosophy: [],
      core_content_units: [],
      educational_track: {},
      fiction_track: {},
      distribution_system: {},
      content_flow_engine: {},
      ai_influencer_system: {},
      monetization_model: {},
      current_focus: {},
    },
  };
  const company = existsSync(COMPANY_CORE_FILE)
    ? readJson<RegistryCompanyCore>(COMPANY_CORE_FILE)
    : defaultCompany;
  const product = existsSync(PRODUCT_DETAILS_FILE)
    ? readJson<RegistryProductDetails>(PRODUCT_DETAILS_FILE)
    : defaultProduct;
  const businessContext = buildBusinessContext(company, product);

  const ajv = new Ajv({ allErrors: true });

  const validateManifest = ajv.compile(moduleManifestSchema);
  const validateAgent = ajv.compile(agentTemplateSchema);
  const validateKpi = ajv.compile(kpiMinimumSchema);
  const validateKpiLibraryEntry = ajv.compile(kpiLibrarySchema);
  const validateDataInputLibraryEntry = ajv.compile(dataInputLibrarySchema);
  const validateProtocol = ajv.compile(protocolSchema);
  const validateWorkflow = ajv.compile(workflowSchema);
  const validateCompanyCore = ajv.compile(companyCoreSchema);
  const validateProductDetails = ajv.compile(productDetailsSchema);

  const validationErrors: string[] = [];

  if (!validateCompanyCore(company)) {
    const details = (validateCompanyCore.errors ?? [])
      .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
      .join('; ');
    validationErrors.push(`Invalid company definition company/company-core.json: ${details}`);
  }

  if (!validateProductDetails(product)) {
    const details = (validateProductDetails.errors ?? [])
      .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
      .join('; ');
    validationErrors.push(`Invalid product definition company/product-details.json: ${details}`);
  }

  const manifestFiles = listFiles(join(REGISTRY_ROOT, 'modules'), '.manifest.json');
  const manifests: RegistryModuleManifest[] = manifestFiles.map((filePath) => {
    const parsed = readJson<RegistryModuleManifest>(filePath);
    const valid = validateManifest(parsed);
    if (!valid) {
      const details = (validateManifest.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid module manifest ${relative(REGISTRY_ROOT, filePath)}: ${details}`);
    }
    return parsed;
  });

  const kpiLibraryFiles = listFiles(join(REGISTRY_ROOT, 'kpis'), '.json');
  const defaultKpis: RegistryKpiDefinition[] = kpiLibraryFiles.map((filePath) => {
    const parsed = readJson<RegistryKpiDefinition>(filePath);
    return parsed;
  });

  const kpiWithSources = applyDefinitionOverrides('kpis', defaultKpis);
  const kpis: RegistryKpiDefinition[] = kpiWithSources.map(({ sourcePath, value }) => {
    const valid = validateKpiLibraryEntry(value);
    if (!valid) {
      const details = (validateKpiLibraryEntry.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid KPI definition ${relative(process.cwd(), sourcePath)}: ${details}`);
    }
    return value;
  });

  const dataInputFiles = listFiles(join(REGISTRY_ROOT, 'data-inputs'), '.json');
  const defaultDataInputs: RegistryDataInputDefinition[] = dataInputFiles.map((filePath) => {
    const parsed = readJson<RegistryDataInputDefinition>(filePath);
    return parsed;
  });

  const dataInputsWithSources = applyDefinitionOverrides('data-inputs', defaultDataInputs);
  const dataInputs: RegistryDataInputDefinition[] = dataInputsWithSources.map(({ sourcePath, value }) => {
    const valid = validateDataInputLibraryEntry(value);
    if (!valid) {
      const details = (validateDataInputLibraryEntry.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid data input definition ${relative(process.cwd(), sourcePath)}: ${details}`);
    }
    return value;
  });

  const agentFiles = listFilesWithExtensions(join(REGISTRY_ROOT, 'agents'), ['.json', '.yaml', '.yml']);
  const agents: RegistryAgentDefinition[] = agentFiles.map((filePath) => {
    const normalized = normalizeAgentTemplate(readStructured(filePath));
    const alignedTemplate = ensureBusinessAlignedAgentTemplate(normalized, businessContext);
    const valid = validateAgent(alignedTemplate);
    if (!valid) {
      const details = (validateAgent.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid agent template ${relative(REGISTRY_ROOT, filePath)}: ${details}`);
    }

    const uid = basename(filePath, extname(filePath));
    return {
      uid,
      ...alignedTemplate,
    };
  });

  const alignmentChecks = agents.map((agent) => evaluateAgentAlignment(agent, businessContext));
  alignmentChecks
    .filter((check) => !check.aligned)
    .forEach((check) => {
      const reasons = check.issues.filter((issue) => !issue.aligned).map((issue) => `${issue.field}: ${issue.reason}`).join('; ');
      validationErrors.push(`Business alignment issue for agent ${check.agentId}: ${reasons}`);
    });

  const kpiFiles = listFiles(join(REGISTRY_ROOT, 'kpi-schemas'), '.json');
  const kpiRequirements: RegistryKpiMinimumRequirements[] = kpiFiles.map((filePath) => {
    const parsed = readJson<RegistryKpiMinimumRequirements>(filePath);
    const valid = validateKpi(parsed);
    if (!valid) {
      const details = (validateKpi.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid KPI schema ${relative(REGISTRY_ROOT, filePath)}: ${details}`);
    }
    return parsed;
  });

  const skillFiles = listFiles(join(REGISTRY_ROOT, 'skills'), '.md');
  const skills: RegistrySkillDoc[] = skillFiles.map(parseSkillDoc);

  const productContextKeywords = uniqueStrings([
    ...toStringArray(product.product_and_ecosystem?.core_content_units),
    ...toStringArray(product.product_and_ecosystem?.philosophy),
    ...toStringArray(company.core_values),
    ...toStringArray(company.global_non_negotiables),
    'education',
    'fiction',
    'story',
    'poem',
    'audio',
    'review',
    'consent',
    'governance',
  ]).map((entry) => normalizeReference(entry));

  const directorApprovalWarnings = skills
    .map((skill) => {
      const haystack = normalizeReference(`${skill.id} ${skill.title} ${skill.tags.join(' ')} ${skill.content}`);
      const hasProductTag = skill.tags.some((tag) => normalizeReference(tag) === 'product-specific');
      const hasContextSignal = productContextKeywords.some((keyword) => keyword.length > 0 && haystack.includes(keyword));

      if (hasProductTag || hasContextSignal) {
        return null;
      }

      return `Director Approval Warning: skill '${skill.id}' is general-purpose and lacks explicit product context mapping.`;
    })
    .filter((warning): warning is string => warning !== null)
    .slice(0, 40);

  const protocolFiles = [
    ...listFilesWithExtensions(join(REGISTRY_ROOT, 'protocols'), ['.yaml', '.yml']),
    ...listFilesWithExtensions(join(REGISTRY_ROOT, 'protocols'), ['.md']),
  ];

  const protocolDocs: RegistryProtocolDoc[] = protocolFiles.map((filePath) => {
    if (filePath.endsWith('.md')) {
      return parseProtocolMarkdownDoc(filePath);
    }

    const protocol = parseProtocolYamlDoc(filePath);
    const valid = validateProtocol({
      id: protocol.id,
      title: protocol.title,
      description: 'Global protocol entry',
      tags: protocol.tags,
      rules: protocol.rules,
    });

    if (!valid) {
      const details = (validateProtocol.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid protocol ${relative(REGISTRY_ROOT, filePath)}: ${details}`);
    }

    return protocol;
  });

  const workflowFiles = listFilesWithExtensions(join(REGISTRY_ROOT, 'workflows'), ['.json', '.yaml', '.yml']);
  const baseWorkflows: RegistryWorkflowDefinition[] = workflowFiles.map((filePath) => {
    const relativePath = relative(join(REGISTRY_ROOT, 'workflows'), filePath).replace(/\\/g, '/');
    const pathParts = relativePath.split('/');
    const fallbackAgentId = pathParts.length > 1 ? pathParts[0] : '';
    const fallbackWorkflowId = basename(filePath, extname(filePath));

    const normalizedWorkflow = normalizeWorkflow(readStructured(filePath), fallbackWorkflowId, fallbackAgentId);
    const valid = validateWorkflow(normalizedWorkflow);
    if (!valid) {
      const details = (validateWorkflow.errors ?? [])
        .map((error) => `${error.dataPath || '(root)'} ${error.message ?? 'invalid'}`)
        .join('; ');
      validationErrors.push(`Invalid workflow ${relative(REGISTRY_ROOT, filePath)}: ${details}`);
    }

    return {
      ...normalizedWorkflow,
      sourceFile: relative(REGISTRY_ROOT, filePath).replace(/\\/g, '/'),
    };
  });

  const workflows = baseWorkflows.map((workflow) => injectCompanyProtocolsIntoWorkflow(workflow, businessContext));

  const protocolById = new Map(protocolDocs.map((entry) => [entry.id, entry]));
  const skillByUid = new Map(skills.map((entry) => [entry.id, entry]));

  const resolvedAgents: RegistryResolvedAgent[] = agents.map((agent) => {
    const resolvedSkills = agent.skills
      .map((reference) => {
        const direct = skillByUid.get(reference);
        if (direct) {
          return direct;
        }

        const resolved = resolveByReference(reference, skills, (entry) => [entry.id, entry.title]);
        if (!resolved) {
          validationErrors.push(`Invalid agent template agents/${agent.uid}: missing skill ${reference}`);
          return null;
        }
        return resolved;
      })
      .filter((entry): entry is RegistrySkillDoc => entry !== null);

    const resolvedKpis = agent.kpis
      .map((reference) => {
        const resolved = resolveByReference(reference, kpis, (entry) => [entry.uid, entry.name]);
        if (!resolved) {
          validationErrors.push(`Invalid agent template agents/${agent.uid}: missing KPI ${reference}`);
          return null;
        }
        return resolved;
      })
      .filter((entry): entry is RegistryKpiDefinition => entry !== null);

    const resolvedDataInputs = agent.data_requirements
      .map((reference) => {
        const resolved = resolveByReference(reference, dataInputs, (entry) => [entry.uid, entry.name]);
        if (!resolved) {
          validationErrors.push(`Invalid agent template agents/${agent.uid}: missing data input ${reference}`);
          return null;
        }
        return resolved;
      })
      .filter((entry): entry is RegistryDataInputDefinition => entry !== null);

    return {
      uid: agent.uid,
      name: agent.name,
      role: agent.role,
      backstory: agent.backstory,
      goal: agent.goal,
      coreObjective: agent.core_objective,
      individualVision: agent.individual_vision,
      roleNonNegotiableRequirements: agent.role_non_negotiable_requirements,
      objectivesLongTerm: agent.objectives_long_term,
      personalityTraits: agent.personality_traits,
      interactionStyle: agent.interaction_style,
      constraints: agent.constraints,
      skills: resolvedSkills,
      kpis: resolvedKpis,
      dataRequirements: resolvedDataInputs,
    };
  });

  const workflowsByAgent = new Map<string, RegistryWorkflowDefinition[]>();
  for (const workflow of workflows) {
    const entries = workflowsByAgent.get(workflow.agent_id) ?? [];
    entries.push(workflow);
    workflowsByAgent.set(workflow.agent_id, entries);
  }

  const intelligence: RegistryAgentIntelligence[] = agents.map((agent) => {
    const profile = resolvedAgents.find((entry) => entry.uid === agent.uid);
    if (!profile) {
      validationErrors.push(`Missing resolved profile for agent ${agent.uid}`);
      return {
        agentId: agent.uid,
        profile: {
          uid: agent.uid,
          name: agent.name,
          role: agent.role,
          backstory: agent.backstory,
          goal: agent.goal,
          coreObjective: agent.core_objective,
          individualVision: agent.individual_vision,
          roleNonNegotiableRequirements: agent.role_non_negotiable_requirements,
          objectivesLongTerm: agent.objectives_long_term,
          personalityTraits: agent.personality_traits,
          interactionStyle: agent.interaction_style,
          constraints: agent.constraints,
          skills: [],
          kpis: [],
          dataRequirements: [],
        },
        protocols: [],
        workflows: [],
      };
    }

    const inheritedProtocols = agent.protocols
      .map((protocolId) => {
        const protocol = protocolById.get(protocolId);
        if (!protocol) {
          validationErrors.push(`Agent ${agent.uid} references missing protocol ${protocolId}`);
          return null;
        }
        return protocol;
      })
      .filter((entry): entry is RegistryProtocolDoc => entry !== null);

    const agentWorkflows = workflowsByAgent.get(agent.uid) ?? [];
    const workflowById = new Map(agentWorkflows.map((workflow) => [workflow.id, workflow]));
    const declaredWorkflowRefs = new Set((agent.workflows ?? []).map((workflowRef) => normalizeReference(workflowRef)));
    const selectedAgentWorkflows = (agent.workflows ?? []).map((workflowRef) => {
      const direct = workflowById.get(workflowRef);
      if (direct) {
        return direct;
      }

      const normalizedWorkflowRef = normalizeReference(workflowRef);
      const resolved = agentWorkflows.find((workflow) => normalizeReference(workflow.id) === normalizedWorkflowRef);
      if (!resolved) {
        validationErrors.push(`Agent ${agent.uid} references missing workflow ${workflowRef}`);
        return null;
      }

      return resolved;
    }).filter((workflow): workflow is RegistryWorkflowDefinition => workflow !== null);

    const declaredSkillRefs = new Set(agent.skills.map(normalizeReference));
    const declaredKpiRefs = new Set(agent.kpis.map(normalizeReference));

    for (const workflow of selectedAgentWorkflows) {
      for (const requiredSkill of workflow.dependencies.required_skills) {
        const normalizedSkill = normalizeReference(requiredSkill);
        if (!declaredSkillRefs.has(normalizedSkill)) {
          validationErrors.push(
            `Workflow ${workflow.id} for agent ${agent.uid} depends on unknown skill ${requiredSkill}`,
          );
        }
      }

      for (const requiredKpi of workflow.dependencies.required_kpis) {
        const normalizedKpi = normalizeReference(requiredKpi);
        if (!declaredKpiRefs.has(normalizedKpi)) {
          validationErrors.push(
            `Workflow ${workflow.id} for agent ${agent.uid} depends on unknown KPI ${requiredKpi}`,
          );
        }
      }
    }

    for (const workflow of agentWorkflows) {
      const normalizedWorkflowId = normalizeReference(workflow.id);
      if (declaredWorkflowRefs.size > 0 && !declaredWorkflowRefs.has(normalizedWorkflowId)) {
        validationErrors.push(`Workflow ${workflow.id} exists for agent ${agent.uid} but is not declared in workflows[]`);
      }
    }

    if (agent.skills.length < 3) {
      validationErrors.push(`Agent ${agent.uid} fails MVS: requires at least 3 skills`);
    }
    if (agent.protocols.length < 1) {
      validationErrors.push(`Agent ${agent.uid} fails MVS: requires at least 1 protocol`);
    }
    if ((agent.workflows ?? []).length < 1) {
      validationErrors.push(`Agent ${agent.uid} fails MVS: requires at least 1 workflow`);
    }

    return {
      agentId: agent.uid,
      profile,
      protocols: inheritedProtocols,
      workflows: selectedAgentWorkflows,
    };
  });

  const initialDrafts = readJson<Record<string, Array<{ key: string; value: string }>>>(
    join(REGISTRY_ROOT, 'onboarding', 'defaults', 'initial-drafts.json'),
  );
  const fieldSchema = readJson<Record<string, unknown[]>>(
    join(REGISTRY_ROOT, 'onboarding', 'schemas', 'field-schema.json'),
  );

  const onboardingFieldSchema: Record<string, unknown[]> = {
    ...fieldSchema,
    'virtual-employees': buildAgentSetupFieldSchema(resolvedAgents),
    'kpi-input': buildKpiInputFieldSchema(kpis),
  };

  const companyCoreDraft = initialDrafts['company-core'] ?? [];
  const companyContextAsText = typeof company.context === 'string' ? company.context : JSON.stringify(company.context);
  const companyDraftWithContext = upsertDraftField(companyCoreDraft, 'company_vision', company.vision);
  const companyDraftWithStage = upsertDraftField(companyDraftWithContext, 'company_context', companyContextAsText);
  const companyDraftWithValues = upsertDraftField(companyDraftWithStage, 'core_values', company.core_values.join(', '));
  const companyDraftWithRules = upsertDraftField(
    companyDraftWithValues,
    'global_non_negotiables',
    company.global_non_negotiables.join(', '),
  );

  const globalAssetsDraft = initialDrafts['global-assets'] ?? [];
  const globalAssetsWithSkills = upsertDraftField(globalAssetsDraft, 'approved_skills', skills.map((entry) => entry.id).join(', '));
  const globalAssetsWithKpis = upsertDraftField(globalAssetsWithSkills, 'approved_kpis', kpis.map((entry) => entry.uid).join(', '));
  const globalAssetsWithProtocols = upsertDraftField(globalAssetsWithKpis, 'approved_protocols', protocolDocs.map((entry) => entry.id).join(', '));
  const globalAssetsAligned = upsertDraftField(globalAssetsWithProtocols, 'approved_data_inputs', dataInputs.map((entry) => entry.uid).join(', '));

  const onboardingInitialDrafts: Record<string, Array<{ key: string; value: string }>> = {
    ...initialDrafts,
    'company-core': companyDraftWithRules,
    'global-assets': globalAssetsAligned,
    'virtual-employees': buildAgentSetupDraft(resolvedAgents),
    'kpi-input': buildKpiInputDraft(kpis),
  };

  // Preserve validation errors for UI and diagnostics without hard-failing
  // snapshot creation. This prevents onboarding and lifecycle IPC routes from
  // freezing when registry content drifts from strict schemas.

  const fingerprint = getRegistryFileFingerprint();

  return {
    fingerprint,
    snapshot: {
      version,
      loadedAt: new Date().toISOString(),
      sourceRoot: REGISTRY_ROOT,
      manifests,
      onboarding: {
        company,
        product,
        businessContext,
        alignmentChecks,
        directorApprovalWarnings,
        agentSetup: resolvedAgents,
        initialDrafts: onboardingInitialDrafts,
        fieldSchema: onboardingFieldSchema,
      },
      agents,
      kpis,
      dataInputs,
      kpiRequirements,
      skills,
      protocols: protocolDocs,
      workflows,
      agentIntelligence: intelligence,
      validationErrors,
    },
  };
};

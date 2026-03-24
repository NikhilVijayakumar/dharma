export type ModelProviderId = 'lmstudio' | 'openrouter' | 'gemini';

export type ModelFailureReason =
  | 'rate_limit'
  | 'overloaded'
  | 'billing'
  | 'auth'
  | 'auth_permanent'
  | 'model_not_found'
  | 'network'
  | 'unknown';

export interface ModelProviderStatus {
  provider: ModelProviderId;
  model: string;
  healthy: boolean;
  status: 'healthy' | 'cooldown' | 'unavailable';
  message: string;
  latencyMs: number | null;
  reason: ModelFailureReason | null;
  cooldownUntil: number | null;
  cooldownRemainingMs: number;
  fromCooldownProbe: boolean;
}

export interface ModelGatewayProbePayload {
  activeProvider: ModelProviderId | null;
  activeModel: string | null;
  fallbackOrder: ModelProviderId[];
  statuses: ModelProviderStatus[];
  checkedAt: string;
}

export interface QueueTask {
  id: string;
  agentProcess: string;
  description: string;
  enqueuedAt: string;
  status: 'PENDING' | 'RUNNING' | 'SUCCEEDED' | 'FAILED' | 'PAUSED';
  duration?: string;
}

export interface QueueSubagentTreeNode {
  id: string;
  agentName: string;
  model: string;
  status: 'RUNNING' | 'COMPLETED' | 'FAILED' | 'CANCELLED' | 'TIMED_OUT';
  depth: number;
  startedAt: string;
  endedAt: string | null;
  summary: string | null;
  children: QueueSubagentTreeNode[];
}

export interface QueueMonitorPayload {
  activeCount: number;
  pendingCount: number;
  gateway: ModelGatewayProbePayload;
  context: {
    activeSessions: number;
    totalMessages: number;
    totalTokens: number;
    totalCompactions: number;
    hottestSessionId: string | null;
    hottestSessionTokens: number;
    recentEvents: Array<{
      id: string;
      sessionId: string;
      type: 'threshold_reached' | 'compaction_started' | 'compaction_completed' | 'new_context_prepared' | 'new_context_started';
      message: string;
      createdAt: string;
    }>;
  };
  hooks: {
    hookCount: number;
    enabledHooks: number;
    notificationsGenerated: number;
    totalExecutions: number;
    succeeded: number;
    failed: number;
    timedOut: number;
    skipped: number;
    lastRunAt: string | null;
  };
  toolPolicy: {
    totalEvaluations: number;
    allowed: number;
    denied: number;
    approvalRequired: number;
    loopBlocks: number;
    pathBlocks: number;
    lastDecisionAt: string | null;
  };
  toolPolicyAudits: Array<{
    id: string;
    timestamp: string;
    actor: string;
    action: string;
    target: string;
    decision: 'ALLOW' | 'DENY' | 'REQUIRE_APPROVAL';
    reasonCode:
      | 'allowed'
      | 'director_approval_required'
      | 'loop_detected'
      | 'depth_limit_exceeded'
      | 'path_restricted'
      | 'policy_denied';
    message: string;
  }>;
  subagents: {
    telemetry: {
      total: number;
      running: number;
      completed: number;
      failed: number;
      cancelled: number;
      timedOut: number;
      maxDepthObserved: number;
      roots: number;
    };
    tree: QueueSubagentTreeNode[];
  };
  tasks: QueueTask[];
}

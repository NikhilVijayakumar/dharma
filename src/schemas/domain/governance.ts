export interface AuditLogEntry {
  id: string;
  timestamp: string;
  actor: string;
  action: string;
  target: string;
  result: 'SUCCESS' | 'BLOCKED' | 'FLAGGED';
}

export interface GovernanceDecision {
  id: string;
  source: string;
  title: string;
  status: 'DRAFT' | 'APPROVED' | 'REJECTED' | 'DEFERRED' | 'COMMITTED';
}

export interface GovernancePayload {
  logs: AuditLogEntry[];
  decisions: GovernanceDecision[];
}

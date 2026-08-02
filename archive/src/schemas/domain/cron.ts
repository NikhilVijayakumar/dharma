export interface ScheduleEntry {
  id: string;
  name: string;
  expression: string;
  enabled: boolean;
  nextRunAt: string | null;
  lastRunAt: string | null;
  lastRunStatus: 'SUCCESS' | 'FAILED' | 'SKIPPED_OVERLAP' | null;
  running: boolean;
}

export interface CronProposalEntry {
  proposalId: string;
  jobId: string;
  name: string;
  expression: string;
  retentionDays: number;
  maxRuntimeMs: number;
  status: 'PENDING' | 'APPROVED' | 'REJECTED' | 'OVERRIDDEN';
  createdAt: string;
  updatedAt: string;
  reviewedAt: string | null;
  reviewer: string | null;
  reviewNote: string | null;
}

export interface TaskAuditLogEntry {
  id: number;
  eventType: string;
  jobId: string | null;
  taskId: string | null;
  details: string;
  createdAt: string;
}

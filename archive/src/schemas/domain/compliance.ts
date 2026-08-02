export interface CompliancePayload {
  overallStatus: 'secure' | 'warning' | 'critical';
  violationsCount: number;
  lastAudit: string;
  adherenceScore: number;
  checks: {
    id: string;
    name: string;
    status: 'pass' | 'fail' | 'warn';
    details: string;
  }[];
}

export interface SettingsPayload {
  language: string;
  preferredModelProvider: 'lmstudio' | 'openrouter' | 'gemini';
  themeMode: 'system' | 'light' | 'dark';
  reducedMotion: boolean;
  syncPushIntervalMs?: number;
  syncCronEnabled?: boolean;
  syncPushCronEnabled?: boolean;
  syncPullCronEnabled?: boolean;
  syncPushCronExpression?: string;
  syncPullCronExpression?: string;
  syncHealthAutoRefreshEnabled?: boolean;
  syncHealthAutoRefreshIntervalMs?: number;
}

export interface SyncStatusSnapshot {
  initialized: boolean;
  pushTimerActive: boolean;
  pushIntervalMs: number;
  machineLockWarning: string | null;
  lastPull: {
    at: string | null;
    status: 'SUCCESS' | 'SKIPPED' | 'FAILED' | null;
    message: string | null;
  };
  lastPush: {
    at: string | null;
    status: 'SUCCESS' | 'SKIPPED' | 'FAILED' | null;
    message: string | null;
  };
  lastIntegrityCheck: {
    at: string | null;
    valid: boolean | null;
    issues: string[];
  };
  queue: {
    pendingOrFailed: number;
    running: number;
    completed: number;
  };
}

export interface RuntimeChannelConfigurationPayload {
  provider: string;
  allowedChannels: string[];
  approvedAgentsForChannels: Record<string, string[]>;
  channelAccessRules: string;
  telegramChannelId: string;
  webhookSubscriptionUri: string;
  providerCredentials: string;
}

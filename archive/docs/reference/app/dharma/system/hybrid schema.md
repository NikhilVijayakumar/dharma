# Hybrid Schema - Atomic Universal Specification

## A. Operational Intent
Define strict separation between high-speed operational data in SQLite and durable secure state in Vault with explicit commit boundaries.

## B. Registry Dependency
- Agent Profiles: nora, maya, eva, julia
- Skills: schema-design-and-migration, governance-enforcement, security-audit
- Protocols: database-schema-governance-protocol, privacy-by-design-protocol, audit-trail-integrity-protocol
- Workflows: maya/seed-autonomous-alignment, nora/human-in-loop-fiscal-governance

## C. The Triple-Engine Extraction Logic
### OpenCLAW
Reason over storage placement, retention policy, and security tier compliance before persistence.

### Goose
Extract data artifacts and map each to SQLite projection or Vault commit lane.

### NemoClaw
Present storage-tier decisions and commit previews through schema-driven persistence UI.

## D. Hybrid Data Lifecycle
### SQLite (High-Performance)
Provide vector index, prompt cache, queue state, FTS, telemetry, and ephemeral projections with strict TTL.

### D1. Canonical SQLite Table Contract
The following tables are mandatory for runtime parity across module and system specs.

```sql
CREATE TABLE IF NOT EXISTS queue_jobs (
	job_id TEXT PRIMARY KEY,
	queue_lane TEXT NOT NULL,
	owner_agent_id TEXT NOT NULL,
	status TEXT NOT NULL CHECK (status IN ('pending','leased','running','retry','dead_letter','done','cancelled')),
	priority INTEGER NOT NULL DEFAULT 100,
	retry_count INTEGER NOT NULL DEFAULT 0,
	max_retries INTEGER NOT NULL DEFAULT 3,
	lease_expires_at TEXT,
	payload_json TEXT NOT NULL,
	error_json TEXT,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS queue_dead_letter (
	dead_letter_id TEXT PRIMARY KEY,
	job_id TEXT NOT NULL,
	reason_code TEXT NOT NULL,
	forensic_bundle_ref TEXT,
	moved_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cron_definitions (
	cron_id TEXT PRIMARY KEY,
	cron_name TEXT NOT NULL,
	schedule_cron TEXT NOT NULL,
	timezone TEXT NOT NULL,
	is_enabled INTEGER NOT NULL CHECK (is_enabled IN (0,1)),
	precondition_json TEXT,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cron_runs (
	run_id TEXT PRIMARY KEY,
	cron_id TEXT NOT NULL,
	run_status TEXT NOT NULL CHECK (run_status IN ('running','success','failed','skipped','timeout')),
	started_at TEXT NOT NULL,
	finished_at TEXT,
	duration_ms INTEGER,
	output_json TEXT,
	error_json TEXT,
	FOREIGN KEY (cron_id) REFERENCES cron_definitions(cron_id)
);

CREATE TABLE IF NOT EXISTS kpi_current_state (
	kpi_uid TEXT PRIMARY KEY,
	current_kpi_value REAL,
	kpi_last_eval TEXT,
	last_status TEXT CHECK (last_status IN ('ok','warning','critical','unknown')),
	eval_context_json TEXT,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kpi_eval_history (
	eval_id TEXT PRIMARY KEY,
	kpi_uid TEXT NOT NULL,
	value REAL,
	status TEXT NOT NULL CHECK (status IN ('ok','warning','critical')),
	evaluated_at TEXT NOT NULL,
	details_json TEXT,
	FOREIGN KEY (kpi_uid) REFERENCES kpi_current_state(kpi_uid)
);

CREATE TABLE IF NOT EXISTS audit_dirty_cache (
	cache_id TEXT PRIMARY KEY,
	event_id TEXT NOT NULL,
	actor_id TEXT,
	event_type TEXT NOT NULL,
	payload_json TEXT NOT NULL,
	payload_sha256 TEXT NOT NULL,
	encrypted INTEGER NOT NULL DEFAULT 0 CHECK (encrypted IN (0,1)),
	flushed_to_vault INTEGER NOT NULL DEFAULT 0 CHECK (flushed_to_vault IN (0,1)),
	created_at TEXT NOT NULL,
	flushed_at TEXT
);

CREATE TABLE IF NOT EXISTS prompt_cache (
	cache_key TEXT PRIMARY KEY,
	query_hash TEXT NOT NULL,
	context_json TEXT NOT NULL,
	hit_count INTEGER NOT NULL DEFAULT 0,
	expires_at TEXT NOT NULL,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vector_chunks (
	chunk_id TEXT PRIMARY KEY,
	source_ref TEXT NOT NULL,
	content_text TEXT NOT NULL,
	metadata_json TEXT,
	created_at TEXT NOT NULL,
	updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vector_embeddings (
	embedding_id TEXT PRIMARY KEY,
	chunk_id TEXT NOT NULL,
	embedding_model TEXT NOT NULL,
	embedding_blob BLOB NOT NULL,
	dimension INTEGER NOT NULL,
	created_at TEXT NOT NULL,
	FOREIGN KEY (chunk_id) REFERENCES vector_chunks(chunk_id)
);

CREATE TABLE IF NOT EXISTS retrieval_telemetry (
	retrieval_id TEXT PRIMARY KEY,
	query_text TEXT NOT NULL,
	top_k INTEGER NOT NULL,
	latency_ms INTEGER NOT NULL,
	source_chunk_ids_json TEXT NOT NULL,
	confidence_score REAL,
	created_at TEXT NOT NULL
);
```

### D2. Required Index Contract

```sql
CREATE INDEX IF NOT EXISTS idx_queue_jobs_status_priority ON queue_jobs(status, priority, created_at);
CREATE INDEX IF NOT EXISTS idx_queue_jobs_lease ON queue_jobs(lease_expires_at);
CREATE INDEX IF NOT EXISTS idx_cron_runs_cron_started ON cron_runs(cron_id, started_at DESC);
CREATE INDEX IF NOT EXISTS idx_kpi_eval_history_uid_time ON kpi_eval_history(kpi_uid, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_dirty_cache_flush_state ON audit_dirty_cache(flushed_to_vault, created_at);
CREATE INDEX IF NOT EXISTS idx_prompt_cache_expiry ON prompt_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_vector_embeddings_chunk ON vector_embeddings(chunk_id);
CREATE INDEX IF NOT EXISTS idx_retrieval_telemetry_time ON retrieval_telemetry(created_at DESC);
```

### D3. Retention and Compaction Contract
- queue_jobs: retain done/cancelled rows for 7 days, then archive to Vault summary and purge.
- queue_dead_letter: retain 90 days locally; commit forensic bundle references to Vault before purge.
- cron_runs: retain 30 days locally; critical failures are always committed to Vault.
- kpi_eval_history: retain 180 days locally with weekly compaction.
- audit_dirty_cache: purge only when flushed_to_vault=1 and flushed_at is non-null.
- prompt_cache: strict TTL via expires_at; purge expired rows every scheduler tick.
- retrieval_telemetry: retain 30 days with daily aggregation snapshots.

### D4. Startup Recovery Contract
- Before unlocking UI, process audit_dirty_cache rows where flushed_to_vault=0.
- On startup, replay missed cron windows by comparing cron_runs latest started_at against cron_definitions schedule.
- On startup, evaluate stale KPIs by scanning kpi_current_state where kpi_last_eval exceeds each KPI frequency_of_check window.
- If vector index health fails, system enters degraded retrieval mode and logs incident to queue_dead_letter and Vault.

### Vault (Secure Commit State)
Store encrypted long-term records, signed governance actions, incident dossiers, and approved strategic artifacts.

## E. Channel and Execution
- Cronjobs: Retention enforcement, compaction, and tier-mismatch audits.
- Internal Chat: Data governance channel for schema and retention changes.
- External Channels: Telegram only for critical data-governance incidents.
- Dynamic UI Contract: System and module screens must be schema-driven by registry YAML/JSON definitions.
- No-Dead-End Navigation: Every screen must expose Back and Home controls for Electron no-URL execution.
- Manual Override: Show Current State and Proposed Improvement before every registry-impacting commit.

## F. Onboarding Governance Pipeline (Dependency-Based)
- Step 1 Company Core is mandatory and must be approved first.
- Step 2 Global Assets approval (Skills, KPIs, Protocols, Data Inputs) unlocks only after Company Core.
- Step 3 Agent Deep-Dive approval requires composite mappings to approved global assets.
- Step 4 Infrastructure approval is split into Channel ACL and Model Config approvals.
- Step 5 Master Commit is blocked until all prerequisite steps are approved.

### F1. Step Status Contract
- Allowed statuses: `PENDING`, `DRAFT`, `APPROVED`.
- Draft edits on approved entities must downgrade entity status to `DRAFT` until re-approval.
- Dashboard navigation must allow drill-down and backtracking to earlier steps without draft loss.

### F2. Validation Contract
- Agent `individual_vision` must be checked for alignment against `company_vision`.
- Alignment check uses a basic LLM verification pass with deterministic keyword-overlap fallback.
- Channel access rules must resolve to allowlisted channels and approved agents.

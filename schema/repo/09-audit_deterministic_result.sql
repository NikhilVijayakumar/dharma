-- repo.db — the deterministic audit result for one audit_run (08). Exactly
-- one row per run. `score` is the weighted_pass_rate outcome; `rules_passed`
-- / `rules_total` record the rule count; `evidence_json` carries the raw
-- evidence justifying the score (see docs/proposal/08-schema-and-crate-
-- architecture.md, "Audit Subsystem"). A human override or cancel of this
-- score is recorded in audit_override (13).

CREATE TABLE IF NOT EXISTS audit_deterministic_result (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    audit_run_id  INTEGER NOT NULL UNIQUE REFERENCES audit_run(id) ON DELETE CASCADE,
    score         REAL    NOT NULL,
    rules_passed  INTEGER NOT NULL DEFAULT 0,
    rules_total   INTEGER NOT NULL DEFAULT 0,
    evidence_json TEXT    NOT NULL DEFAULT '{}'
);

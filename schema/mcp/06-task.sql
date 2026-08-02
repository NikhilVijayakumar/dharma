-- mcp.db — one Usecase's Task set (see docs/proposal/02-task-model.md).
-- Scoped by `usecase_id` into usecase (05); the owning Usecase is itself
-- scoped to an Epic and a Domain System, so no `domain_system_id` column is
-- needed here.
--
-- `input_contract_json` / `output_contract_json` are JSON Schema
-- documents (validated by the `schemas` crate, not by SQLite). Per docs/
-- proposal/02-task-model.md's Hard Constraints, every Task requires at
-- least one entry in EACH of the three tiers of `acceptance_criteria_
-- json` before it can be assigned — enforced by the `schemas` crate at
-- write time, not by a CHECK constraint (structure varies).
--
-- acceptance_criteria_json shape:
-- { "happy_path": [...], "corner_case": [...], "edge_case": [...] }
-- each of the three arrays must hold at least one entry — none is optional.
CREATE TABLE IF NOT EXISTS task (
    id                       INTEGER PRIMARY KEY AUTOINCREMENT,
    usecase_id               INTEGER NOT NULL REFERENCES usecase(id) ON DELETE CASCADE,
    name                     TEXT    NOT NULL,
    description              TEXT    NOT NULL DEFAULT '',
    input_contract_json      TEXT    NOT NULL,
    output_contract_json     TEXT    NOT NULL,
    acceptance_criteria_json TEXT    NOT NULL,
    sort_order               INTEGER NOT NULL DEFAULT 0,
    UNIQUE(usecase_id, name)
);
CREATE INDEX IF NOT EXISTS idx_task_usecase ON task(usecase_id);

//! Content parsing into mcp.db (tables 05-19).
//!
//! Domain side (proposal 13 — Bodha's frozen format): a `section-map.yaml`
//! parses into one `domain` row plus its `section` tree; a
//! `section/profile/*.yaml` parses into one `section_profile` row.
//! Profile-default files are captured but deliberately NOT parsed (proposal
//! 13's largest named gap — still preserved losslessly in `content_asset`).
//!
//! Task side (proposal 02): a task bundle parses into `epic`/`usecase`/
//! `task`/`task_step` rows; contracts are validated by the `schemas` crate.
//! Agent side (proposal 01/03): an agent bundle parses into
//! `agent`/`agent_goal`/`skill`/`skill_prompt`/`skill_script`/
//! `skill_example`/`skill_template`/`agent_skill_binding` rows.
//!
//! Every row here traces to a captured file via `content_asset_id` — Dharma
//! is infrastructure, not an author (proposal 08).

use anyhow::{Context, Result};
use registry::McpDb;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// Section Map → domain + section (tables 05-06)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SectionMapFile {
    #[serde(default)]
    pub sections: Vec<SectionEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SectionEntry {
    pub id: String,
    #[serde(default)]
    pub title: String,
    /// "root" for top-level sections, otherwise another section's id.
    #[serde(default)]
    pub parent_id: String,
    #[serde(default)]
    pub level: i64,
    #[serde(default)]
    pub order: i64,
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default)]
    pub generated: bool,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub profile: String,
    #[serde(default)]
    pub purpose: String,
}

fn default_true() -> bool {
    true
}

/// Parse a captured section-map into a `domain` row plus its `section` tree.
/// All writes happen in one transaction. Returns the new domain id.
pub fn import_section_map(
    db: &McpDb,
    domain_system_id: i64,
    domain_name: &str,
    description: &str,
    content_asset_id: i64,
    yaml_text: &str,
) -> Result<i64> {
    let parsed: SectionMapFile = serde_yaml::from_str(yaml_text)
        .with_context(|| format!("Failed to parse section-map '{domain_name}'"))?;
    if parsed.sections.is_empty() {
        anyhow::bail!("section-map for '{domain_name}' has no sections");
    }

    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO domain (domain_system_id, name, description, content_asset_id)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![domain_system_id, domain_name, description, content_asset_id],
    )?;
    let domain_id = tx.last_insert_rowid();

    // Pass 1: insert every section (parent resolved in pass 2). Remember
    // the row id per YAML-declared section id for the parent link.
    let mut row_id_by_section: Vec<(String, i64)> = Vec::new();
    for (idx, entry) in parsed.sections.iter().enumerate() {
        tx.execute(
            "INSERT INTO section (domain_id, section_id, title, parent_id, level, sort_order,
                                  required, generated, source, profile_ref, purpose, content_asset_id)
             VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?6, ?7, ?8, NULLIF(?9, ''), ?10, ?11)",
            rusqlite::params![
                domain_id,
                entry.id,
                entry.title,
                entry.level,
                entry.order,
                entry.required as i64,
                entry.generated as i64,
                entry.source,
                entry.profile,
                entry.purpose,
                content_asset_id
            ],
        )
        .with_context(|| format!("Failed to insert section '{}'", entry.id))?;
        row_id_by_section.push((entry.id.clone(), tx.last_insert_rowid()));
        let _ = idx;
    }

    // Pass 2: link parent_id for nested sections.
    let section_ids: std::collections::HashMap<String, i64> = row_id_by_section.into_iter().collect();
    for entry in &parsed.sections {
        if entry.parent_id.is_empty() || entry.parent_id == "root" {
            continue;
        }
        let parent_row_id = section_ids.get(&entry.parent_id).ok_or_else(|| {
            anyhow::anyhow!(
                "section '{}' references unknown parent '{}' in '{}'",
                entry.id,
                entry.parent_id,
                domain_name
            )
        })?;
        let child_row_id = section_ids[&entry.id];
        tx.execute(
            "UPDATE section SET parent_id = ?1 WHERE id = ?2",
            rusqlite::params![parent_row_id, child_row_id],
        )?;
    }

    tx.commit()?;
    Ok(domain_id)
}

// ---------------------------------------------------------------------------
// Section Profile → section_profile (table 07)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SectionProfileFile {
    /// Bodha's files use a one-element list; the schema stores one TEXT.
    #[serde(default)]
    pub inherits: Vec<String>,
    #[serde(default)]
    pub section: ProfileSection,
}

#[derive(Debug, Deserialize, Default)]
pub struct ProfileSection {
    pub id: String,
    #[serde(default)]
    pub writing_objective: String,
    #[serde(default)]
    pub knowledge_goal: String,
    #[serde(default)]
    pub reader_goal: String,
    #[serde(default)]
    pub required_inputs: Vec<String>,
    #[serde(default)]
    pub expected_outputs: serde_json::Value,
    #[serde(default)]
    pub subsections: serde_json::Value,
    #[serde(default)]
    pub completion: serde_json::Value,
    #[serde(default)]
    pub review: serde_json::Value,
    #[serde(default)]
    pub validation: serde_json::Value,
}

/// Parse a section profile YAML into a `section_profile` row, scoped by the
/// section it profiles (found by `(domain_id, profile.section.id)`). Returns
/// the section_profile id. At most one profile per section.
pub fn import_section_profile(
    db: &McpDb,
    domain_id: i64,
    content_asset_id: i64,
    yaml_text: &str,
) -> Result<i64> {
    let parsed: SectionProfileFile = serde_yaml::from_str(yaml_text)
        .with_context(|| format!("Failed to parse section profile for '{}'", yaml_text.len()))?;
    let section_id = find_section_row(db, domain_id, &parsed.section.id)?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "section profile references unknown section '{}' in domain {}",
                parsed.section.id,
                domain_id
            )
        })?;

    let inherits = parsed
        .inherits
        .first()
        .cloned()
        .unwrap_or_else(|| "scientific-narrative".to_string());
    let required_inputs = serde_json::to_string(&parsed.section.required_inputs)?;
    let expected_outputs = flatten_primary(&parsed.section.expected_outputs);
    let subsection_rules = serde_json::to_string(&parsed.section.subsections)?;
    let completion_checklist = extract_list(&parsed.section.completion, "checklist");
    let review_questions = extract_list(&parsed.section.review, "questions");
    let validation_rules = extract_list(&parsed.section.validation, "rules");

    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO section_profile (section_id, inherits, writing_objective, knowledge_goal,
                                      reader_goal, required_inputs_json, expected_outputs_json,
                                      subsection_rules_json, completion_checklist_json,
                                      review_questions_json, validation_rules_json, content_asset_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            section_id,
            inherits,
            parsed.section.writing_objective,
            parsed.section.knowledge_goal,
            parsed.section.reader_goal,
            required_inputs,
            expected_outputs,
            subsection_rules,
            completion_checklist,
            review_questions,
            validation_rules,
            content_asset_id
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

fn find_section_row(db: &McpDb, domain_id: i64, section_id: &str) -> Result<Option<i64>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id FROM section WHERE domain_id = ?1 AND section_id = ?2")?;
    let mut rows = stmt.query_map(rusqlite::params![domain_id, section_id], |r| r.get::<_, i64>(0))?;
    match rows.next() {
        Some(Ok(id)) => Ok(Some(id)),
        _ => Ok(None),
    }
}

/// `expected_outputs.primary` is the meaningful list; flatten to JSON.
fn flatten_primary(value: &serde_json::Value) -> String {
    let primary = value
        .get("primary")
        .cloned()
        .unwrap_or_else(|| serde_json::Value::Null);
    serde_json::to_string(&primary).unwrap_or_else(|_| "[]".to_string())
}

fn extract_list(value: &serde_json::Value, key: &str) -> String {
    let list = value.get(key).cloned().unwrap_or_else(|| serde_json::Value::Null);
    serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string())
}

// ---------------------------------------------------------------------------
// Task bundle → epic/usecase/task/task_step (tables 08-11)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct TaskBundleFile {
    #[serde(default)]
    pub epics: Vec<TaskBundleEpic>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBundleEpic {
    pub name: String,
    #[serde(default)]
    pub objective: String,
    #[serde(default)]
    pub usecases: Vec<TaskBundleUsecase>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBundleUsecase {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tasks: Vec<TaskBundleTask>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBundleTask {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub input_contract: serde_json::Value,
    pub output_contract: serde_json::Value,
    pub acceptance_criteria: serde_json::Value,
    #[serde(default)]
    pub template_ref: Option<String>,
    #[serde(default)]
    pub steps: Vec<TaskBundleStep>,
}

#[derive(Debug, Deserialize)]
pub struct TaskBundleStep {
    #[serde(default)]
    pub order: i64,
    #[serde(default)]
    pub description: String,
    pub required_capability: String,
}

/// Parse a task bundle into epic/usecase/task/task_step rows for a Domain
/// System, validating each task's contracts via the `schemas` crate.
/// Returns `(epic_ids, usecase_ids, task_ids)`.
pub fn import_task_bundle(
    db: &McpDb,
    domain_system_id: i64,
    content_asset_id: i64,
    yaml_text: &str,
) -> Result<()> {
    let parsed: TaskBundleFile = serde_yaml::from_str(yaml_text).context("Failed to parse task bundle")?;
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;

    for (epic_idx, epic) in parsed.epics.iter().enumerate() {
        tx.execute(
            "INSERT INTO epic (domain_system_id, parent_id, name, objective, content_asset_id, sort_order)
             VALUES (?1, NULL, ?2, ?3, ?4, ?5)",
            rusqlite::params![domain_system_id, epic.name, epic.objective, content_asset_id, epic_idx as i64],
        )?;
        let epic_id = tx.last_insert_rowid();

        for (uc_idx, uc) in epic.usecases.iter().enumerate() {
            tx.execute(
                "INSERT INTO usecase (epic_id, name, description, content_asset_id, sort_order)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![epic_id, uc.name, uc.description, content_asset_id, uc_idx as i64],
            )?;
            let usecase_id = tx.last_insert_rowid();

            for (task_idx, task) in uc.tasks.iter().enumerate() {
                let input_contract = serde_json::to_string(&task.input_contract)?;
                let output_contract = serde_json::to_string(&task.output_contract)?;
                let acceptance = serde_json::to_string(&task.acceptance_criteria)?;
                schemas::validate_acceptance_criteria(&task.acceptance_criteria)
                    .with_context(|| format!("task '{}'", task.name))?;

                tx.execute(
                    "INSERT INTO task (usecase_id, name, description, input_contract_json,
                                       output_contract_json, acceptance_criteria_json, template_ref,
                                       content_asset_id, sort_order)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    rusqlite::params![
                        usecase_id,
                        task.name,
                        task.description,
                        input_contract,
                        output_contract,
                        acceptance,
                        task.template_ref,
                        content_asset_id,
                        task_idx as i64
                    ],
                )?;
                let task_id = tx.last_insert_rowid();

                for step in &task.steps {
                    // required_capability is a real FK into
                    // agent_system_registry(concern) — a bundle referencing an
                    // unregistered concern fails the whole import atomically.
                    tx.execute(
                        "INSERT INTO task_step (task_id, step_order, description, required_capability)
                         VALUES (?1, ?2, ?3, ?4)",
                        rusqlite::params![task_id, step.order, step.description, step.required_capability],
                    )
                    .with_context(|| {
                        format!(
                            "task '{}' step needs capability '{}' — is the Agent System registered?",
                            task.name, step.required_capability
                        )
                    })?;
                }
            }
        }
    }

    tx.commit()?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Agent bundle → agent/agent_goal/skill (tables 12-19)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct AgentBundleFile {
    #[serde(default)]
    pub agents: Vec<AgentBundleAgent>,
}

#[derive(Debug, Deserialize)]
pub struct AgentBundleAgent {
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub handoff_trigger_condition: String,
    #[serde(default)]
    pub handoff_candidate_role: String,
    #[serde(default)]
    pub goals: Vec<AgentBundleGoal>,
    #[serde(default)]
    pub skills: Vec<AgentBundleSkill>,
}

#[derive(Debug, Deserialize)]
pub struct AgentBundleGoal {
    pub order: i64,
    pub goal: String,
    #[serde(default)]
    pub backstory: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentBundleSkill {
    pub name: String,
    pub responsibility: String,
    #[serde(default)]
    pub is_analysis_only: bool,
    pub invocation_input: serde_json::Value,
    pub invocation_output: serde_json::Value,
    /// The mandatory prompt. Either inline text or a relative path into the
    /// provider bundle (resolved by the caller).
    pub prompt: String,
    #[serde(default)]
    pub script_ref: Option<String>,
    #[serde(default)]
    pub examples: Vec<AgentBundleSkillExample>,
    #[serde(default)]
    pub template: Option<AgentBundleSkillTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct AgentBundleSkillExample {
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    #[serde(default)]
    pub dos: Vec<String>,
    #[serde(default)]
    pub donts: Vec<String>,
    #[serde(default)]
    pub best_practices: Vec<String>,
    #[serde(default)]
    pub common_mistakes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentBundleSkillTemplate {
    pub name: String,
    pub text: String,
}

/// Parse an agent bundle into agent/agent_goal/skill rows for an Agent
/// System, validating the Skill-bundle invariants via the `schemas` crate.
/// Every agent and skill must trace to the same `content_asset_id` capture.
pub fn import_agent_bundle(
    db: &McpDb,
    agent_system_id: i64,
    content_asset_id: i64,
    yaml_text: &str,
) -> Result<()> {
    let parsed: AgentBundleFile = serde_yaml::from_str(yaml_text).context("Failed to parse agent bundle")?;
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;

    for agent in &parsed.agents {
        if agent.goals.len() > 8 {
            anyhow::bail!(
                "agent '{}' declares {} goals — cap is 8 (agent_goal CHECK)",
                agent.name,
                agent.goals.len()
            );
        }
        if agent.goals.is_empty() {
            anyhow::bail!("agent '{}' has no goals", agent.name);
        }

        tx.execute(
            "INSERT INTO agent (agent_system_id, name, role, handoff_trigger_condition,
                                handoff_candidate_role, content_asset_id, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))",
            rusqlite::params![
                agent_system_id,
                agent.name,
                agent.role,
                agent.handoff_trigger_condition,
                agent.handoff_candidate_role,
                content_asset_id
            ],
        )
        .with_context(|| format!("agent '{}' already registered for this Agent System", agent.name))?;
        let agent_id = tx.last_insert_rowid();

        for goal in &agent.goals {
            tx.execute(
                "INSERT INTO agent_goal (agent_id, goal_order, goal_text, backstory)
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![agent_id, goal.order, goal.goal, goal.backstory],
            )?;
        }

        for skill in &agent.skills {
            let analysis_only = schemas::validate_skill_bundle(
                Some(&skill.prompt),
                skill.examples.len(),
                skill.script_ref.as_deref(),
                skill.template.as_ref().map(|t| t.text.as_str()),
            )
            .with_context(|| format!("skill '{}' of agent '{}'", skill.name, agent.name))?;
            if skill.is_analysis_only != analysis_only {
                anyhow::bail!(
                    "skill '{}' declares is_analysis_only={} but its assets classify as {}",
                    skill.name,
                    skill.is_analysis_only,
                    analysis_only
                );
            }
            let invocation_input = serde_json::to_string(&skill.invocation_input)?;
            let invocation_output = serde_json::to_string(&skill.invocation_output)?;

            tx.execute(
                "INSERT INTO skill (agent_system_id, name, responsibility, is_analysis_only,
                                    invocation_input_json, invocation_output_json, content_asset_id, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))",
                rusqlite::params![
                    agent_system_id,
                    skill.name,
                    skill.responsibility,
                    skill.is_analysis_only as i64,
                    invocation_input,
                    invocation_output,
                    content_asset_id
                ],
            )
            .with_context(|| format!("skill '{}' already registered for this Agent System", skill.name))?;
            let skill_id = tx.last_insert_rowid();

            tx.execute(
                "INSERT INTO skill_prompt (skill_id, template_text, content_asset_id) VALUES (?1, ?2, ?3)",
                rusqlite::params![skill_id, skill.prompt, content_asset_id],
            )?;

            if let Some(script_ref) = &skill.script_ref {
                tx.execute(
                    "INSERT INTO skill_script (skill_id, script_ref, content_asset_id) VALUES (?1, ?2, ?3)",
                    rusqlite::params![skill_id, script_ref, content_asset_id],
                )?;
            }

            for ex in &skill.examples {
                tx.execute(
                    "INSERT INTO skill_example (skill_id, input_json, output_json, dos_json, donts_json,
                                                best_practices_json, common_mistakes_json, content_asset_id)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    rusqlite::params![
                        skill_id,
                        serde_json::to_string(&ex.input)?,
                        serde_json::to_string(&ex.output)?,
                        serde_json::to_string(&ex.dos)?,
                        serde_json::to_string(&ex.donts)?,
                        serde_json::to_string(&ex.best_practices)?,
                        serde_json::to_string(&ex.common_mistakes)?,
                        content_asset_id
                    ],
                )?;
            }

            if let Some(template) = &skill.template {
                tx.execute(
                    "INSERT INTO skill_template (skill_id, name, template_text, content_asset_id)
                     VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![skill_id, template.name, template.text, content_asset_id],
                )?;
            }

            // Pair the skill with its owning agent (same agent_system_id —
            // the service-layer invariant; a cross-system pairing is
            // rejected by the UNIQUE/agent_skill_binding shape, but the
            // "same system" rule is code-enforced here, not by a CHECK).
            tx.execute(
                "INSERT INTO agent_skill_binding (agent_id, skill_id) VALUES (?1, ?2)",
                rusqlite::params![agent_id, skill_id],
            )?;
        }
    }

    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use registry::McpDb;

    const MAP: &str = r#"
schema:
  id: bodha.paper.section-map
  name: Paper Section Map Schema
  version: 1.0.0

sections:
  - id: introduction
    title: Introduction
    parent_id: root
    level: 1
    order: 1
    required: true
    generated: true
    profile: introduction
    purpose: Introduce the research.
  - id: background
    title: Background
    parent_id: introduction
    level: 2
    order: 1
    required: true
    profile: introduction.background
    purpose: Provide context.
  - id: findings
    title: Findings
    parent_id: root
    level: 1
    order: 2
    required: true
    profile: findings
    purpose: Results.
"#;

    const PROFILE: &str = r#"
schema:
  id: bodha.section.profile.introduction
  name: Introduction Section Profile
  version: 1.0.0

inherits:
  - scientific-narrative

section:
  id: introduction
  trigger: after_research_complete
  writing_objective: Establish scientific context.
  knowledge_goal: Build context.
  reader_goal: Understand motivation.
  required_inputs:
    - metadata
    - literature_review
  expected_outputs:
    primary:
      - publication_ready_introduction
  subsections:
    background:
      objective: Introduce the domain.
      writing_guidelines:
        - Begin broadly.
      should_answer:
        - What domain?
      transition_to: problem-statement
  completion:
    checklist:
      - Context established.
  review:
    questions:
      - Is context established?
  validation:
    rules:
      - Every subsection addressed.
"#;

    fn db() -> McpDb {
        McpDb::open_in_memory().unwrap()
    }

    fn seed_domain_system(db: &McpDb) -> i64 {
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO domain_system_registry (name, version) VALUES ('test-ds', '0.1.0')",
                [],
            )
            .unwrap();
        1
    }

    fn capture(db: &McpDb, path: &str, text: &str) -> i64 {
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO content_asset (source_system, asset_kind, file_path, content_text, content_hash) VALUES ('test', 'yaml', ?1, ?2, 'h')",
                rusqlite::params![path, text],
            )
            .unwrap();
        conn.lock().unwrap().last_insert_rowid()
    }

    #[test]
    fn section_map_imports_tree_with_parents() {
        let db = db();
        let ds = seed_domain_system(&db);
        let asset = capture(&db, "section-map.yaml", MAP);
        let domain_id = import_section_map(&db, ds, "research-paper", "", asset, MAP).unwrap();

        let conn = db.conn();
        let conn = conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM section WHERE domain_id = ?1", [domain_id], |r| r.get(0)).unwrap();
        assert_eq!(count, 3);

        let (parent_ok, level): (bool, i64) = conn
            .query_row(
                "SELECT s.parent_id IS NOT NULL, s.level FROM section s JOIN section p ON s.parent_id = p.id WHERE s.section_id = 'background'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert!(parent_ok);
        assert_eq!(level, 2);
    }

    #[test]
    fn section_map_rejects_unknown_parent() {
        let db = db();
        let ds = seed_domain_system(&db);
        let asset = capture(&db, "section-map.yaml", MAP);
        let bad = MAP.replace("parent_id: introduction\n", "parent_id: does-not-exist\n");
        let result = import_section_map(&db, ds, "research-paper", "", asset, &bad);
        assert!(result.is_err());
    }

    #[test]
    fn profile_imports_into_section() {
        let db = db();
        let ds = seed_domain_system(&db);
        let asset = capture(&db, "section-map.yaml", MAP);
        let domain_id = import_section_map(&db, ds, "research-paper", "", asset, MAP).unwrap();
        let profile_asset = capture(&db, "introduction.yaml", PROFILE);
        import_section_profile(&db, domain_id, profile_asset, PROFILE).unwrap();

        let conn = db.conn();
        let conn = conn.lock().unwrap();
        let (objective, inherits, n_checklist): (String, String, String) = conn
            .query_row(
                "SELECT writing_objective, inherits, completion_checklist_json FROM section_profile",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(objective, "Establish scientific context.");
        assert_eq!(inherits, "scientific-narrative");
        assert!(n_checklist.contains("Context established."));
    }

    #[test]
    fn profile_requires_existing_section() {
        let db = db();
        let ds = seed_domain_system(&db);
        let asset = capture(&db, "section-map.yaml", MAP);
        let domain_id = import_section_map(&db, ds, "research-paper", "", asset, MAP).unwrap();
        let profile_asset = capture(&db, "orphan.yaml", PROFILE);
        let orphan = PROFILE.replace("id: introduction\n", "id: nonexistent\n");
        assert!(import_section_profile(&db, domain_id, profile_asset, &orphan).is_err());
    }

    #[test]
    fn task_bundle_imports_hierarchy() {
        let db = db();
        let ds = seed_domain_system(&db);
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO agent_system_registry (name, concern) VALUES ('rust-dev', 'rust-development')",
                [],
            )
            .unwrap();
        let asset = capture(&db, "task.yaml", "");

        let bundle = r#"
epics:
  - name: Architecture
    objective: Produce reviewable architecture.
    usecases:
      - name: Scaffold Crate
        description: Set up a crate skeleton.
        tasks:
          - name: Write Cargo.toml
            description: Author the manifest.
            input_contract: { "type": "object" }
            output_contract: { "type": "object" }
            acceptance_criteria:
              happy_path: ["file exists"]
              corner_case: ["no panic"]
              edge_case: ["empty manifest"]
            steps:
              - order: 1
                description: Create manifest
                required_capability: rust-development
"#;
        import_task_bundle(&db, ds, asset, bundle).unwrap();

        let (epics, usecases, tasks, steps): (i64, i64, i64, i64) = {
            let conn = conn.lock().unwrap();
            (
                conn.query_row("SELECT COUNT(*) FROM epic", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM usecase", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM task", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM task_step", [], |r| r.get(0)).unwrap(),
            )
        };
        assert_eq!(epics, 1);
        assert_eq!(usecases, 1);
        assert_eq!(tasks, 1);
        assert_eq!(steps, 1);
    }

    #[test]
    fn task_bundle_rejects_bad_acceptance_criteria() {
        let db = db();
        let ds = seed_domain_system(&db);
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO agent_system_registry (name, concern) VALUES ('rust-dev', 'rust-development')",
                [],
            )
            .unwrap();
        let asset = capture(&db, "task.yaml", "");

        let bundle = r#"
epics:
  - name: E
    objective: O
    usecases:
      - name: U
        tasks:
          - name: T
            input_contract: { "type": "object" }
            output_contract: { "type": "object" }
            acceptance_criteria:
              happy_path: ["ok"]
"#;
        assert!(import_task_bundle(&db, ds, asset, bundle).is_err());
    }

    #[test]
    fn agent_bundle_imports_agents_and_skills() {
        let db = db();
        let conn = db.conn();
        conn.lock().unwrap()
            .execute(
                "INSERT INTO agent_system_registry (name, concern) VALUES ('writer-sys', 'writing')",
                [],
            )
            .unwrap();
        let asset = capture(&db, "agent.yaml", "");

        let bundle = r#"
agents:
  - name: Writer
    role: Writes the document.
    handoff_trigger_condition: "when drafting"
    handoff_candidate_role: Reviewer
    goals:
      - order: 1
        goal: Produce the draft.
        backstory: Experienced writer.
    skills:
      - name: draft
        responsibility: Draft a section.
        is_analysis_only: true
        invocation_input: { "type": "object" }
        invocation_output: { "type": "object" }
        prompt: "Write the section."
        examples:
          - input: { "section": "intro" }
            output: { "text": "..." }
            dos: ["be concise"]
            donts: ["ramble"]
"#;
        import_agent_bundle(&db, 1, asset, bundle).unwrap();

        let (agents, goals, skills, prompts, bindings): (i64, i64, i64, i64, i64) = {
            let conn = conn.lock().unwrap();
            (
                conn.query_row("SELECT COUNT(*) FROM agent", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM agent_goal", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM skill", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM skill_prompt", [], |r| r.get(0)).unwrap(),
                conn.query_row("SELECT COUNT(*) FROM agent_skill_binding", [], |r| r.get(0)).unwrap(),
            )
        };
        assert_eq!(agents, 1);
        assert_eq!(goals, 1);
        assert_eq!(skills, 1);
        assert_eq!(prompts, 1);
        assert_eq!(bindings, 1);
    }

    #[test]
    fn agent_bundle_rejects_too_many_goals() {
        let db = db();
        let ds = seed_domain_system(&db);
        let asset = capture(&db, "agent.yaml", "");

        let goals: Vec<String> = (1..=9)
            .map(|i| format!("      - order: {i}\n        goal: g{i}\n        backstory: b\n"))
            .collect();
        let bundle = format!(
            "agents:\n  - name: A\n    role: R\n    goals:\n{}\n",
            goals.join("")
        );
        assert!(import_agent_bundle(&db, ds, asset, &bundle).is_err());
    }
}

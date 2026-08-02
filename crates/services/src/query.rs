//! Read-only info queries over mcp.db (tables 05-25) used by the mcp crate's
//! `*_system_info` tools. Builds nested `serde_json::Value` trees so the
//! adapter can hand them back as tool results without leaking row shape.

use anyhow::Result;
use registry::McpDb;
use rusqlite::types::Value as Sql;
use serde_json::{json, Map, Value};

fn to_value(v: rusqlite::Result<Sql>) -> Value {
    match v {
        Ok(Sql::Null) | Err(_) => Value::Null,
        Ok(Sql::Integer(i)) => json!(i),
        Ok(Sql::Real(f)) => json!(f),
        Ok(Sql::Text(s)) => json!(s),
        Ok(Sql::Blob(b)) => json!(String::from_utf8_lossy(&b).to_string()),
    }
}

fn rows_as_objects(
    conn: &rusqlite::Connection,
    sql: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<Vec<Map<String, Value>>> {
    let mut stmt = conn.prepare(sql)?;
    let col_count = stmt.column_count();
    let names: Vec<String> = (0..col_count).map(|i| stmt.column_name(i).unwrap().to_string()).collect();
    let mut out = Vec::new();
    let mut rows = stmt.query(params)?;
    while let Some(row) = rows.next()? {
        let mut map = Map::new();
        for (i, name) in names.iter().enumerate() {
            map.insert(name.clone(), to_value(row.get::<_, Sql>(i)));
        }
        out.push(map);
    }
    Ok(out)
}

/// Full picture of one Domain System: registry row, domains with their
/// Section Maps + Section Profiles, Epics -> Usecases -> Tasks -> Steps, and
/// audit definitions (rules + ensemble + calculation + template). One nested
/// `Value` ready to serialize.
pub fn domain_system_info(db: &McpDb, name: &str) -> Result<Value> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, version, description, registered_at, updated_at
         FROM domain_system_registry WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "version": row.get::<_, String>(2)?,
            "description": row.get::<_, String>(3)?,
            "registered_at": row.get::<_, String>(4)?,
            "updated_at": row.get::<_, String>(5)?,
        }))
    })?;
    let mut reg = match rows.next() {
        Some(Ok(v)) => v,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(json!({"error": format!("Domain System '{name}' is not registered")})),
    };
    let ds_id = reg["id"].as_i64().unwrap();

    let domains = rows_as_objects(
        &conn,
        "SELECT id, name, description, tier, relationship, sort_order FROM domain
         WHERE domain_system_id = ?1 ORDER BY sort_order, name",
        &[&ds_id],
    )?;

    let sections = rows_as_objects(
        &conn,
        "SELECT id, domain_id, section_id, title, parent_id, level, sort_order, required, generated, source, profile_ref, purpose
         FROM section WHERE domain_id IN (SELECT id FROM domain WHERE domain_system_id = ?1)
         ORDER BY domain_id, sort_order",
        &[&ds_id],
    )?;

    let profiles = rows_as_objects(
        &conn,
        "SELECT sp.section_id, s.section_id AS section_ref, s.domain_id, sp.inherits,
                sp.writing_objective, sp.knowledge_goal, sp.reader_goal,
                sp.required_inputs_json, sp.expected_outputs_json, sp.subsection_rules_json,
                sp.completion_checklist_json, sp.review_questions_json, sp.validation_rules_json
         FROM section_profile sp
         JOIN section s ON s.id = sp.section_id
         WHERE s.domain_id IN (SELECT id FROM domain WHERE domain_system_id = ?1)",
        &[&ds_id],
    )?;

    let epics = rows_as_objects(
        &conn,
        "SELECT e.id, e.parent_id, e.name, e.objective, e.sort_order,
                (SELECT COUNT(*) FROM usecase u WHERE u.epic_id = e.id) AS usecase_count
         FROM epic e WHERE e.domain_system_id = ?1 ORDER BY e.sort_order, e.name",
        &[&ds_id],
    )?;

    let usecases = rows_as_objects(
        &conn,
        "SELECT id, epic_id, name, description FROM usecase WHERE epic_id IN
         (SELECT id FROM epic WHERE domain_system_id = ?1) ORDER BY id",
        &[&ds_id],
    )?;

    let tasks = rows_as_objects(
        &conn,
        "SELECT t.id, t.usecase_id, t.name, t.description, t.template_ref FROM task t
         WHERE t.usecase_id IN (SELECT id FROM usecase WHERE epic_id IN
         (SELECT id FROM epic WHERE domain_system_id = ?1)) ORDER BY t.id",
        &[&ds_id],
    )?;

    let task_ids: Vec<i64> = tasks.iter().filter_map(|t| t.get("id").and_then(Value::as_i64)).collect();
    let mut steps = Vec::new();
    for tid in task_ids {
        let mut s = rows_as_objects(
            &conn,
            "SELECT id, task_id, step_order, description, required_capability FROM task_step
             WHERE task_id = ?1 ORDER BY step_order",
            &[&tid],
        )?;
        steps.append(&mut s);
    }

    let audits = rows_as_objects(
        &conn,
        "SELECT ad.id, ad.domain_id, d.name AS domain_name, ad.name, ad.kind, ad.scope, ad.standard_version
         FROM audit_definition ad LEFT JOIN domain d ON d.id = ad.domain_id
         WHERE ad.domain_system_id = ?1 ORDER BY ad.name",
        &[&ds_id],
    )?;
    let audit_ids: Vec<i64> = audits.iter().filter_map(|a| a.get("id").and_then(Value::as_i64)).collect();
    let mut audit_detail = Vec::new();
    for aid in audit_ids {
        let rules = rows_as_objects(
            &conn,
            "SELECT rule_id, description, condition, message, severity, evidence_type, evidence_target
             FROM audit_rule WHERE audit_definition_id = ?1 ORDER BY rule_id",
            &[&aid],
        )?;
        let semantic = rows_as_objects(
            &conn,
            "SELECT required_models_json, prompt_md_path, metadata_fields_json, evidence_requirements_json
             FROM audit_semantic WHERE audit_definition_id = ?1",
            &[&aid],
        )?;
        let calc = rows_as_objects(
            &conn,
            "SELECT formula_type, formula_json FROM audit_calculation WHERE audit_definition_id = ?1",
            &[&aid],
        )?;
        let templ = rows_as_objects(
            &conn,
            "SELECT kind, format FROM audit_template WHERE audit_definition_id = ?1",
            &[&aid],
        )?;
        audit_detail.push(json!({
            "audit_definition_id": aid,
            "rules": rules,
            "semantic_ensemble": semantic,
            "calculation": calc,
            "templates": templ,
        }));
    }

    reg["domains"] = json!(domains);
    reg["sections"] = json!(sections);
    reg["section_profiles"] = json!(profiles);
    reg["epics"] = json!(epics);
    reg["usecases"] = json!(usecases);
    reg["tasks"] = json!(tasks);
    reg["task_steps"] = json!(steps);
    reg["audit_definitions"] = json!(audits);
    reg["audit_detail"] = json!(audit_detail);
    Ok(reg)
}

/// Full picture of one Agent System: registry row, agents with goals,
/// skills with prompts / scripts / examples / templates, and bindings.
pub fn agent_system_info(db: &McpDb, name: &str) -> Result<Value> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], |row| {
        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "concern": row.get::<_, String>(2)?,
            "description": row.get::<_, String>(3)?,
            "is_privileged": row.get::<_, i64>(4)? != 0,
            "registered_at": row.get::<_, String>(5)?,
            "updated_at": row.get::<_, String>(6)?,
        }))
    })?;
    let mut reg = match rows.next() {
        Some(Ok(v)) => v,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(json!({"error": format!("Agent System '{name}' is not registered")})),
    };
    let as_id = reg["id"].as_i64().unwrap();

    let agents = rows_as_objects(
        &conn,
        "SELECT id, name, role, handoff_trigger_condition, handoff_candidate_role, created_at, updated_at
         FROM agent WHERE agent_system_id = ?1 ORDER BY name",
        &[&as_id],
    )?;

    let agent_ids: Vec<i64> = agents.iter().filter_map(|a| a.get("id").and_then(Value::as_i64)).collect();
    let mut goals = Vec::new();
    for aid in agent_ids {
        let mut g = rows_as_objects(
            &conn,
            "SELECT id, agent_id, goal_text, backstory FROM agent_goal WHERE agent_id = ?1 ORDER BY id",
            &[&aid],
        )?;
        goals.append(&mut g);
    }

    let skills = rows_as_objects(
        &conn,
        "SELECT id, agent_system_id, name, responsibility, invocation_input_json, invocation_output_json, created_at, updated_at
         FROM skill WHERE agent_system_id = ?1 ORDER BY name",
        &[&as_id],
    )?;

    let skill_ids: Vec<i64> = skills.iter().filter_map(|s| s.get("id").and_then(Value::as_i64)).collect();
    let mut prompts = Vec::new();
    let mut scripts = Vec::new();
    let mut examples = Vec::new();
    let mut templates = Vec::new();
    for sid in skill_ids {
        let mut p = rows_as_objects(
            &conn,
            "SELECT id, skill_id, template_text FROM skill_prompt WHERE skill_id = ?1 ORDER BY id",
            &[&sid],
        )?;
        prompts.append(&mut p);
        let mut sc = rows_as_objects(
            &conn,
            "SELECT id, skill_id, script_ref FROM skill_script WHERE skill_id = ?1 ORDER BY id",
            &[&sid],
        )?;
        scripts.append(&mut sc);
        let mut ex = rows_as_objects(
            &conn,
            "SELECT id, skill_id, input_json, output_json, dos_json, donts_json, best_practices_json, common_mistakes_json
             FROM skill_example WHERE skill_id = ?1 ORDER BY id",
            &[&sid],
        )?;
        examples.append(&mut ex);
        let mut tp = rows_as_objects(
            &conn,
            "SELECT id, skill_id, name, template_text FROM skill_template WHERE skill_id = ?1 ORDER BY name",
            &[&sid],
        )?;
        templates.append(&mut tp);
    }

    let bindings = rows_as_objects(
        &conn,
        "SELECT b.agent_id, a.name AS agent_name, b.skill_id, s.name AS skill_name
         FROM agent_skill_binding b
         JOIN agent a ON a.id = b.agent_id
         JOIN skill s ON s.id = b.skill_id
         WHERE b.agent_id IN (SELECT id FROM agent WHERE agent_system_id = ?1) ORDER BY agent_name",
        &[&as_id],
    )?;

    reg["agents"] = json!(agents);
    reg["agent_goals"] = json!(goals);
    reg["skills"] = json!(skills);
    reg["skill_prompts"] = json!(prompts);
    reg["skill_scripts"] = json!(scripts);
    reg["skill_examples"] = json!(examples);
    reg["skill_templates"] = json!(templates);
    reg["agent_skill_bindings"] = json!(bindings);
    Ok(reg)
}

//! Domain System / Agent System registries (mcp.db tables 00-01).
//!
//! A Domain System (proposal 05) is a registered provider of domain content
//! (domains, section maps, profiles, epic-usecase-task sets). An Agent
//! System (proposal 04) is a registered provider of agents and skills,
//! distinguished by a UNIQUE `concern` so `task_step.required_capability`
//! can resolve against it. `is_privileged` is set by the Agent-Management
//! Agent System, never by a provider's own request unchecked.

use anyhow::Result;
use common::id::{AgentSystemId, DomainSystemId};
use registry::McpDb;

#[derive(Debug, Clone)]
pub struct DomainSystemRow {
    pub id: DomainSystemId,
    pub name: String,
    pub version: String,
    pub description: String,
    pub registered_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct AgentSystemRow {
    pub id: AgentSystemId,
    pub name: String,
    pub concern: String,
    pub description: String,
    pub is_privileged: bool,
    pub registered_at: String,
    pub updated_at: String,
}

pub fn register_domain_system(
    db: &McpDb,
    name: &str,
    version: &str,
    description: &str,
) -> Result<DomainSystemRow> {
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO domain_system_registry (name, version, description, updated_at)
         VALUES (?1, ?2, ?3, datetime('now'))",
        rusqlite::params![name, version, description],
    )?;
    get_domain_system_by_name_locked(&mut conn, name)
}

pub fn get_domain_system(db: &McpDb, id: DomainSystemId) -> Result<Option<DomainSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    get_domain_system_by_id_locked(&conn, id.0)
}

pub fn get_domain_system_by_name(db: &McpDb, name: &str) -> Result<Option<DomainSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    get_domain_system_by_name_locked_opt(&conn, name)
}

pub fn list_domain_systems(db: &McpDb) -> Result<Vec<DomainSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, version, description, registered_at, updated_at
         FROM domain_system_registry ORDER BY name",
    )?;
    let rows = stmt.query_map([], map_domain_system)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Bump a Domain System's version — the consuming repos' pinned
/// `domain_system_version` then falls stale (repo_registration.status
/// becomes 'stale'), blocking Task assignment until re-sync (proposal 11).
pub fn update_domain_system_version(
    db: &McpDb,
    name: &str,
    new_version: &str,
) -> Result<Option<DomainSystemRow>> {
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE domain_system_registry SET version = ?1, updated_at = datetime('now') WHERE name = ?2",
        rusqlite::params![new_version, name],
    )?;
    if affected == 0 {
        return Ok(None);
    }
    get_domain_system_by_name_locked(&mut conn, name).map(Some)
}

pub fn register_agent_system(
    db: &McpDb,
    name: &str,
    concern: &str,
    description: &str,
    is_privileged: bool,
) -> Result<AgentSystemRow> {
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO agent_system_registry (name, concern, description, is_privileged, updated_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        rusqlite::params![name, concern, description, is_privileged as i64],
    )?;
    get_agent_system_by_name_locked(&mut conn, name)
}

pub fn get_agent_system(db: &McpDb, id: AgentSystemId) -> Result<Option<AgentSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    get_agent_system_by_id_locked(&conn, id.0)
}

pub fn get_agent_system_by_name(db: &McpDb, name: &str) -> Result<Option<AgentSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], map_agent_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn get_agent_system_by_concern(db: &McpDb, concern: &str) -> Result<Option<AgentSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry WHERE concern = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![concern], map_agent_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

pub fn list_agent_systems(db: &McpDb) -> Result<Vec<AgentSystemRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry ORDER BY name",
    )?;
    let rows = stmt.query_map([], map_agent_system)?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Set `is_privileged`. Only the Agent-Management Agent System may call this
/// (callers enforce that policy); a provider's `is_privileged_request` in
/// `dharma-agent.toml` is a REQUEST, never a grant (proposal 11).
pub fn set_agent_system_privileged(
    db: &McpDb,
    name: &str,
    is_privileged: bool,
) -> Result<Option<AgentSystemRow>> {
    let conn = db.conn();
    let mut conn = conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE agent_system_registry SET is_privileged = ?1, updated_at = datetime('now') WHERE name = ?2",
        rusqlite::params![is_privileged as i64, name],
    )?;
    if affected == 0 {
        return Ok(None);
    }
    get_agent_system_by_name_locked(&mut conn, name).map(Some)
}

fn map_domain_system(row: &rusqlite::Row) -> rusqlite::Result<DomainSystemRow> {
    Ok(DomainSystemRow {
        id: DomainSystemId(row.get(0)?),
        name: row.get(1)?,
        version: row.get(2)?,
        description: row.get(3)?,
        registered_at: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

fn map_agent_system(row: &rusqlite::Row) -> rusqlite::Result<AgentSystemRow> {
    Ok(AgentSystemRow {
        id: AgentSystemId(row.get(0)?),
        name: row.get(1)?,
        concern: row.get(2)?,
        description: row.get(3)?,
        is_privileged: row.get::<_, i64>(4)? != 0,
        registered_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

fn get_domain_system_by_id_locked(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<Option<DomainSystemRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, version, description, registered_at, updated_at
         FROM domain_system_registry WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_domain_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn get_domain_system_by_name_locked(
    conn: &mut rusqlite::Connection,
    name: &str,
) -> Result<DomainSystemRow> {
    get_domain_system_by_name_locked_opt(conn, name)?
        .ok_or_else(|| anyhow::anyhow!("Domain System '{name}' not found"))
}

fn get_domain_system_by_name_locked_opt(
    conn: &rusqlite::Connection,
    name: &str,
) -> Result<Option<DomainSystemRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, version, description, registered_at, updated_at
         FROM domain_system_registry WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], map_domain_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn get_agent_system_by_id_locked(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<Option<AgentSystemRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], map_agent_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

fn get_agent_system_by_name_locked(
    conn: &mut rusqlite::Connection,
    name: &str,
) -> Result<AgentSystemRow> {
    let mut stmt = conn.prepare(
        "SELECT id, name, concern, description, is_privileged, registered_at, updated_at
         FROM agent_system_registry WHERE name = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![name], map_agent_system)?;
    match rows.next() {
        Some(Ok(row)) => Ok(row),
        _ => anyhow::bail!("Agent System '{name}' not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> McpDb {
        McpDb::open_in_memory().unwrap()
    }

    #[test]
    fn register_and_get_domain_system() {
        let db = db();
        let row = register_domain_system(&db, "rust-dev-domain", "0.1.0", "Rust domain").unwrap();
        assert_eq!(row.name, "rust-dev-domain");
        assert_eq!(row.version, "0.1.0");
        let fetched = get_domain_system(&db, row.id.clone()).unwrap().unwrap();
        assert_eq!(fetched.description, "Rust domain");
        assert_eq!(get_domain_system_by_name(&db, "rust-dev-domain").unwrap().unwrap().id, row.id);
    }

    #[test]
    fn domain_system_name_is_unique() {
        let db = db();
        register_domain_system(&db, "d", "0.0.0", "").unwrap();
        assert!(register_domain_system(&db, "d", "0.0.0", "").is_err());
    }

    #[test]
    fn bump_version_changes_registry() {
        let db = db();
        register_domain_system(&db, "d", "0.1.0", "").unwrap();
        update_domain_system_version(&db, "d", "0.2.0").unwrap();
        assert_eq!(get_domain_system_by_name(&db, "d").unwrap().unwrap().version, "0.2.0");
        assert!(update_domain_system_version(&db, "missing", "0.2.0").unwrap().is_none());
    }

    #[test]
    fn register_and_get_agent_system() {
        let db = db();
        let row = register_agent_system(&db, "rust-development", "rust-development", "Rust agents", false)
            .unwrap();
        assert_eq!(get_agent_system(&db, row.id.clone()).unwrap().unwrap().name, "rust-development");
        assert_eq!(
            get_agent_system_by_concern(&db, "rust-development").unwrap().unwrap().id,
            row.id
        );
        assert!(!row.is_privileged);
    }

    #[test]
    fn concern_is_unique_across_agent_systems() {
        let db = db();
        register_agent_system(&db, "a", "shared-concern", "", false).unwrap();
        assert!(register_agent_system(&db, "b", "shared-concern", "", false).is_err());
    }

    #[test]
    fn privileged_flag_requires_service_call() {
        let db = db();
        register_agent_system(&db, "agent-mgmt", "agent-management", "", true).unwrap();
        register_agent_system(&db, "plain", "plain", "", false).unwrap();
        set_agent_system_privileged(&db, "plain", true).unwrap();
        assert!(get_agent_system_by_name(&db, "plain").unwrap().unwrap().is_privileged);
        assert!(set_agent_system_privileged(&db, "missing", true).unwrap().is_none());
    }

    #[test]
    fn list_orders_by_name() {
        let db = db();
        register_domain_system(&db, "z", "0.0.0", "").unwrap();
        register_domain_system(&db, "a", "0.0.0", "").unwrap();
        let names: Vec<String> = list_domain_systems(&db).unwrap().into_iter().map(|d| d.name).collect();
        assert_eq!(names, vec!["a".to_string(), "z".to_string()]);
    }
}

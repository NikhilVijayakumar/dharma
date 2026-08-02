//! Capture flow (mcp.db tables 02-04).
//!
//! Dharma never authors content — every `domain`/`section`/`skill`/etc. row
//! traces back to a captured provider file in `content_asset` (proposal 08,
//! "Dharma Is Infrastructure, Not an Author"). `capture_file` records the
//! full original bytes plus a sha256; re-capturing a changed file appends a
//! new row (append-only ledger). `yaml_template` and `seeder` declare how a
//! captured file is reconstructed / how repo.db rows are seeded from it.

use anyhow::{Context, Result};
use common::env::sha256_hex;
use common::id::DomainSystemId;
use registry::McpDb;

#[derive(Debug, Clone)]
pub struct ContentAssetRow {
    pub id: i64,
    pub source_system: String,
    pub asset_kind: String,
    pub file_path: String,
    pub content_hash: String,
    pub captured_at: String,
}

/// Map a file extension to a `content_asset.asset_kind`.
pub fn asset_kind_for_path(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => "yaml",
        Some(ext) if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") => "markdown",
        Some(ext) if ext.eq_ignore_ascii_case("py") => "python",
        Some(ext) if ext.eq_ignore_ascii_case("json") => "json",
        _ => "text",
    }
}

/// Capture one file from a provider bundle into the ledger. `rel_path` is
/// the file's path relative to the provider's bundle root, recorded as-is.
/// Returns the new `content_asset.id`.
pub fn capture_file(
    db: &McpDb,
    source_system: &str,
    rel_path: &str,
    content: &str,
) -> Result<i64> {
    let kind = asset_kind_for_path(std::path::Path::new(rel_path));
    let hash = sha256_hex(content);
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO content_asset (source_system, asset_kind, file_path, content_text, content_hash)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![source_system, kind, rel_path, content, hash],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Capture a whole provider bundle: walk `root`, capture every file relative
/// to it. Returns the rows captured, keyed by relative path.
pub fn capture_bundle(
    db: &McpDb,
    source_system: &str,
    root: &std::path::Path,
) -> Result<Vec<ContentAssetRow>> {
    let files = common::fs::walk_files(root)?;
    let mut out = Vec::new();
    for rel in files {
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let content = std::fs::read_to_string(root.join(&rel))
            .with_context(|| format!("Failed to read {}", root.join(&rel).display()))?;
        capture_file(db, source_system, &rel_str, &content)?;
        let row = latest_asset(db, source_system, &rel_str)?.expect("just inserted");
        out.push(row);
    }
    Ok(out)
}

/// Register a YAML reconstruction template for a captured YAML asset (03).
/// Exactly one template per captured asset (`UNIQUE(content_asset_id)`).
pub fn register_yaml_template(
    db: &McpDb,
    content_asset_id: i64,
    name: &str,
    scope: &str,
    template_text: &str,
) -> Result<()> {
    if !["domain", "agent", "audit"].contains(&scope) {
        anyhow::bail!("invalid yaml_template scope '{scope}' (expected domain|agent|audit)");
    }
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO yaml_template (content_asset_id, name, scope, template_text)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![content_asset_id, name, scope, template_text],
    )?;
    Ok(())
}

/// Register a seeder/reconstruction script (04). `domain_system_id` is NULL
/// for Dharma's generic seeders (`is_generic = 1`).
pub fn register_seeder(
    db: &McpDb,
    domain_system_id: Option<DomainSystemId>,
    name: &str,
    script_ref: &str,
    description: &str,
) -> Result<()> {
    let is_generic = if domain_system_id.is_none() { 1 } else { 0 };
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO seeder (domain_system_id, name, script_ref, is_generic, description)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![domain_system_id.map(|d| d.0), name, script_ref, is_generic, description],
    )?;
    Ok(())
}

pub fn list_seeders_for_domain_system(
    db: &McpDb,
    domain_system_id: DomainSystemId,
) -> Result<Vec<SeederRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, domain_system_id, name, script_ref, is_generic, description
         FROM seeder
         WHERE domain_system_id = ?1 OR (domain_system_id IS NULL AND is_generic = 1)
         ORDER BY name",
    )?;
    let rows = stmt.query_map(rusqlite::params![domain_system_id.0], |row| {
        Ok(SeederRow {
            id: row.get(0)?,
            domain_system_id: row.get(1)?,
            name: row.get(2)?,
            script_ref: row.get(3)?,
            is_generic: row.get::<_, i64>(4)? != 0,
            description: row.get(5)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

#[derive(Debug, Clone)]
pub struct SeederRow {
    pub id: i64,
    pub domain_system_id: Option<i64>,
    pub name: String,
    pub script_ref: String,
    pub is_generic: bool,
    pub description: String,
}

fn latest_asset(db: &McpDb, source_system: &str, rel_path: &str) -> Result<Option<ContentAssetRow>> {
    let conn = db.conn();
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, source_system, asset_kind, file_path, content_hash, captured_at
         FROM content_asset
         WHERE source_system = ?1 AND file_path = ?2
         ORDER BY id DESC LIMIT 1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![source_system, rel_path], |row| {
        Ok(ContentAssetRow {
            id: row.get(0)?,
            source_system: row.get(1)?,
            asset_kind: row.get(2)?,
            file_path: row.get(3)?,
            content_hash: row.get(4)?,
            captured_at: row.get(5)?,
        })
    })?;
    match rows.next() {
        Some(Ok(row)) => Ok(Some(row)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> McpDb {
        McpDb::open_in_memory().unwrap()
    }

    #[test]
    fn capture_file_records_kind_and_hash() {
        let db = db();
        let id = capture_file(&db, "bodha", "section/map/section-map.yaml", "schema: v1").unwrap();
        assert_eq!(id, 1);
        let conn = db.conn();
        let conn = conn.lock().unwrap();
        let (kind, hash): (String, String) = conn
            .query_row(
                "SELECT asset_kind, content_hash FROM content_asset WHERE id = 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(kind, "yaml");
        assert_eq!(hash, sha256_hex("schema: v1"));
    }

    #[test]
    fn capture_bundle_walks_and_hard_skips_build_dirs() {
        let dir = std::env::temp_dir().join(format!("dharma-cap-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("nested")).unwrap();
        std::fs::write(dir.join("a.yaml"), "a: 1").unwrap();
        std::fs::write(dir.join("nested/b.py"), "print(1)").unwrap();
        std::fs::create_dir_all(dir.join(".dharma")).unwrap();
        std::fs::write(dir.join(".dharma/repo.db"), "x").unwrap();
        std::fs::create_dir_all(dir.join("target")).unwrap();
        std::fs::write(dir.join("target/x.o"), "x").unwrap();

        let db = db();
        let rows = capture_bundle(&db, "test", &dir).unwrap();
        let paths: Vec<String> = rows.iter().map(|r| r.file_path.clone()).collect();
        assert!(paths.contains(&"a.yaml".to_string()));
        assert!(paths.contains(&"nested/b.py".to_string()));
        assert!(!paths.iter().any(|p| p.contains(".dharma") || p.contains("target")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn template_scope_validated() {
        let db = db();
        let id = capture_file(&db, "s", "section-map.yaml", "x").unwrap();
        register_yaml_template(&db, id, "sec-map", "domain", "{{.}}").unwrap();
        assert!(register_yaml_template(&db, id, "dup", "bogus", "{{.}}").is_err());
    }

    #[test]
    fn seeders_list_generic_plus_domain() {
        let db = db();
        let conn = db.conn();
        let (ds7, ds8): (i64, i64) = {
            let conn = conn.lock().unwrap();
            conn.execute(
                "INSERT INTO domain_system_registry (name, version) VALUES ('d7', '0.0.0')",
                [],
            )
            .unwrap();
            let id7 = conn.last_insert_rowid();
            conn.execute(
                "INSERT INTO domain_system_registry (name, version) VALUES ('d8', '0.0.0')",
                [],
            )
            .unwrap();
            let id8 = conn.last_insert_rowid();
            (id7, id8)
        };
        register_seeder(&db, None, "generic-sync", "sync.py", "generic").unwrap();
        register_seeder(&db, Some(DomainSystemId(ds7)), "rust-sync", "rust.py", "rust").unwrap();
        register_seeder(&db, Some(DomainSystemId(ds8)), "other", "o.py", "").unwrap();
        let rows = list_seeders_for_domain_system(&db, DomainSystemId(ds7)).unwrap();
        let names: Vec<String> = rows.iter().map(|r| r.name.clone()).collect();
        assert!(names.contains(&"generic-sync".to_string()));
        assert!(names.contains(&"rust-sync".to_string()));
        assert!(!names.contains(&"other".to_string()));
    }
}

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Validate that a path is within an allowed root directory.
///
/// Canonicalizes both paths and checks that the canonicalized target
/// path starts with the canonicalized root. Returns the canonicalized
/// path on success.
pub fn validate_path(path: &Path, root: &Path) -> Result<PathBuf> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Cannot canonicalize path: {}", path.display()))?;
    let canonical_root = root
        .canonicalize()
        .with_context(|| format!("Cannot canonicalize root: {}", root.display()))?;
    if !canonical.starts_with(&canonical_root) {
        anyhow::bail!(
            "Path {} is outside allowed root {}",
            canonical.display(),
            canonical_root.display()
        );
    }
    Ok(canonical)
}

/// Recursively walk `dir`, returning every file path relative to `dir`.
/// Skips `.git`, `target`, `node_modules`, `.dharma` by default. Mirrors the
/// capture flow's provider content walk (proposal 08).
pub fn walk_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_files_inner(dir, dir, &mut out)?;
    Ok(out)
}

fn walk_files_inner(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read dir {}", dir.display()))?
    {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == ".git" || name_str == "target" || name_str == "node_modules" || name_str == ".dharma" {
            continue;
        }
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            walk_files_inner(root, &path, out)?;
        } else {
            let rel = path.strip_prefix(root).unwrap_or(&path);
            out.push(rel.to_path_buf());
        }
    }
    Ok(())
}

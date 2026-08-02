use anyhow::{Context, Result};
use std::path::Path;

/// Default exclusion patterns for content sync — Python bytecode artifacts.
/// Always applied on top of any caller-supplied excludes.
pub const DEFAULT_EXCLUDES: &[&str] = &["**/__pycache__/**", "**/*.pyc"];

/// Recursively copy `src` into `dest`, applying glob-based exclusion
/// patterns. Returns the number of files copied. Mirrors samgraha's
/// `copy_dir_recursive` (used for provider content capture into mcp.db and
/// for `.dharma/assets/` materialization).
pub fn copy_dir_recursive(src: &Path, dest: &Path, exclude: &[&str]) -> Result<usize> {
    let mut count = 0;
    let mut all_excludes: Vec<&str> = DEFAULT_EXCLUDES.iter().copied().collect();
    all_excludes.extend(exclude.iter().copied());
    copy_dir_recursive_inner(src, dest, "", &all_excludes, &mut count)?;
    Ok(count)
}

fn copy_dir_recursive_inner(
    src: &Path,
    dest: &Path,
    rel_prefix: &str,
    exclude: &[&str],
    count: &mut usize,
) -> Result<()> {
    std::fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create dir {}", dest.display()))?;

    for entry in std::fs::read_dir(src)
        .with_context(|| format!("Failed to read dir {}", src.display()))?
    {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;

        let rel_str = if rel_prefix.is_empty() {
            file_name.to_string_lossy().to_string()
        } else {
            format!("{}/{}", rel_prefix, file_name.to_string_lossy())
        };

        if should_exclude(&rel_str, exclude) {
            continue;
        }

        let dest_path = dest.join(&file_name);

        if file_type.is_dir() {
            copy_dir_recursive_inner(&entry.path(), &dest_path, &rel_str, exclude, count)?;
        } else if file_type.is_file() {
            std::fs::copy(entry.path(), &dest_path)
                .with_context(|| format!(
                    "Failed to copy {} -> {}",
                    entry.path().display(),
                    dest_path.display()
                ))?;
            *count += 1;
        }
    }
    Ok(())
}

fn should_exclude(rel_path: &str, exclude: &[&str]) -> bool {
    for pattern in exclude {
        if crate::glob::matches_glob(pattern, rel_path) {
            return true;
        }
    }
    false
}

/// Atomically copy `src` into `dest` using a sibling temp directory and
/// `fs::rename`. If the copy fails partway, the previous `dest` tree
/// remains untouched. Returns the number of files copied.
pub fn copy_dir_atomic(src: &Path, dest: &Path, exclude: &[&str]) -> Result<usize> {
    let parent = dest
        .parent()
        .with_context(|| format!("No parent for {}", dest.display()))?;
    std::fs::create_dir_all(parent)?;

    let uuid = uuid::Uuid::new_v4();
    let tmp_name = format!(".tmp-sync-{}", uuid);
    let tmp_dir = parent.join(&tmp_name);

    if tmp_dir.exists() {
        std::fs::remove_dir_all(&tmp_dir)?;
    }

    let result = copy_dir_recursive(src, &tmp_dir, exclude);

    match result {
        Ok(count) => {
            if dest.exists() {
                std::fs::remove_dir_all(dest)?;
            }
            std::fs::rename(&tmp_dir, dest).with_context(|| format!(
                "Failed to atomically rename {} -> {}",
                tmp_dir.display(),
                dest.display()
            ))?;
            Ok(count)
        }
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tmp_dir);
            Err(e)
        }
    }
}

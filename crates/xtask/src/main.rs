//! `cargo run -p xtask -- release` — builds + packages dharma-mcp and dharma.
//!
//! Replaces scripts/build-release.{sh,ps1} duplicated bash/PowerShell logic
//! with one Rust implementation; the two scripts are now thin wrappers that
//! just invoke this binary (docs/proposal, "Dharma Build Config"). Reads
//! dharma-build.toml (repo root) + .env, same convention as samgraha.toml/.env.
//! Template: config.example/dharma-build.toml.

use anyhow::{bail, Context, Result};
use common::config::{load_build_config, BuildConfig};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const EXE_SUFFIX: &str = std::env::consts::EXE_SUFFIX;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("release") | None => release(),
        Some(other) => bail!("unknown xtask command '{other}' — expected 'release'"),
    }
}

fn release() -> Result<()> {
    let root = workspace_root()?;
    common::env::load_dotenv();

    let cfg: BuildConfig = load_build_config(&root).map_err(anyhow::Error::msg)?;
    let output_dir = cfg.resolve_output_dir(&root);
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("creating output dir {}", output_dir.display()))?;
    let output_dir = output_dir.canonicalize()?;

    // Informational only — crates/mcp/build.rs computes its own copy of this
    // same value (from the same config + .env) and bakes it into the binary;
    // the two calls land seconds apart, which is fine for a display label.
    let (expiry_days, expiry_hours) = cfg.resolve_expiry();
    let expiry_label = if expiry_days == -1 {
        "never".to_string()
    } else {
        common::env::expiry_rfc3339_from_now(expiry_days, expiry_hours)
    };
    println!("Expiry: {expiry_label}  (days={expiry_days}, hours={expiry_hours})");

    println!("Building dharma-mcp + dharma (release)...");
    let status = std::process::Command::new("cargo")
        .args(["build", "--release", "--bin", "dharma-mcp", "--bin", "dharma"])
        .arg("--manifest-path")
        .arg(root.join("Cargo.toml"))
        .status()
        .context("running cargo build")?;
    if !status.success() {
        bail!("cargo build failed");
    }

    let pkg_dir = output_dir.join("dharma");
    if pkg_dir.exists() {
        std::fs::remove_dir_all(&pkg_dir)?;
    }
    std::fs::create_dir_all(pkg_dir.join("bin"))?;

    let mcp_bin = format!("dharma-mcp{EXE_SUFFIX}");
    let cli_bin = format!("dharma{EXE_SUFFIX}");
    let target_dir = root.join("target/release");
    std::fs::copy(target_dir.join(&mcp_bin), pkg_dir.join("bin").join(&mcp_bin))?;
    std::fs::copy(target_dir.join(&cli_bin), pkg_dir.join("bin").join(&cli_bin))?;

    strip_if_possible(&pkg_dir.join("bin").join(&mcp_bin));
    strip_if_possible(&pkg_dir.join("bin").join(&cli_bin));

    copy_glob_toml(&root.join("config.example"), &pkg_dir.join("config"))?;
    println!("  -> config/*.toml (example — copy the one matching your role, rename to dharma-*.toml)");

    copy_glob_ext(&root.join("env"), &pkg_dir.join("env"), "env.example")?;
    println!("  -> env/*.env.example (matches config/, copy the one matching your role to .env)");

    copy_glob_ext(&root.join("schema/mcp"), &pkg_dir.join("schema/mcp"), "sql")?;
    copy_glob_ext(&root.join("schema/repo"), &pkg_dir.join("schema/repo"), "sql")?;
    println!("  -> schema/mcp/, schema/repo/ (reference schema)");

    write_launcher(&pkg_dir, &mcp_bin, &expiry_label)?;

    let mcp_hash = sha256_file(&pkg_dir.join("bin").join(&mcp_bin))?;
    let cli_hash = sha256_file(&pkg_dir.join("bin").join(&cli_bin))?;
    std::fs::write(
        pkg_dir.join("SHA256SUMS"),
        format!("{mcp_hash}  bin/{mcp_bin}\n{cli_hash}  bin/{cli_bin}\n"),
    )?;

    let mcp_kb = std::fs::metadata(pkg_dir.join("bin").join(&mcp_bin))?.len() / 1024;
    let cli_kb = std::fs::metadata(pkg_dir.join("bin").join(&cli_bin))?.len() / 1024;

    println!("\n=== Release packaged ===");
    println!("  Location:   {}", pkg_dir.display());
    println!("  dharma-mcp: {mcp_kb}KB  ({mcp_hash})");
    println!("  dharma:     {cli_kb}KB  ({cli_hash})");
    println!("  Expiry:     {expiry_label}");
    if EXE_SUFFIX.is_empty() {
        println!("  Use:        echo '{{\"id\":1,\"method\":\"list_domain_systems\",\"params\":{{}}}}' | ./run-mcp.sh");
    } else {
        println!("  Use:        Get-Content input.json | .\\run-mcp.cmd");
    }
    Ok(())
}

/// `xtask`'s own `CARGO_MANIFEST_DIR` is `<root>/crates/xtask` — the
/// workspace root is two levels up.
fn workspace_root() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .context("resolving workspace root from CARGO_MANIFEST_DIR")
}

fn strip_if_possible(bin: &Path) {
    if !EXE_SUFFIX.is_empty() {
        return; // no `strip` on Windows binaries here
    }
    let _ = std::process::Command::new("strip").arg(bin).status();
}

fn copy_glob_toml(src: &Path, dst: &Path) -> Result<()> {
    copy_glob_ext(src, dst, "toml")
}

fn copy_glob_ext(src: &Path, dst: &Path, ext: &str) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src).with_context(|| format!("reading {}", src.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some(ext)
            || path.to_string_lossy().ends_with(&format!(".{ext}"))
        {
            std::fs::copy(&path, dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn write_launcher(pkg_dir: &Path, mcp_bin: &str, expiry_label: &str) -> Result<()> {
    let expiry_comment = if expiry_label == "never" { "no expiry".to_string() } else { format!("expires {expiry_label}") };
    let path = pkg_dir.join("run-mcp.sh");
    std::fs::write(
        &path,
        format!(
            "#!/usr/bin/env sh\n\
             # dharma-mcp — {expiry_comment}; set DHARMA_MCP_DIR to control\n\
             # where mcp.db lives (defaults to $HOME/.dharma if unset).\n\
             exec \"$(dirname \"$0\")/bin/{mcp_bin}\" \"$@\"\n"
        ),
    )?;
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(windows)]
fn write_launcher(pkg_dir: &Path, mcp_bin: &str, expiry_label: &str) -> Result<()> {
    let expiry_comment = if expiry_label == "never" { "no expiry".to_string() } else { format!("expires {expiry_label}") };
    let path = pkg_dir.join("run-mcp.cmd");
    std::fs::write(
        &path,
        format!(
            "@echo off\r\n\
             rem dharma-mcp -- {expiry_comment}; set DHARMA_MCP_DIR to control\r\n\
             rem where mcp.db lives (defaults to %USERPROFILE%\\.dharma if unset).\r\n\
             \"%~dp0bin\\{mcp_bin}\" %*\r\n"
        ),
    )?;
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

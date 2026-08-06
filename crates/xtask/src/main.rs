//! `cargo run -p xtask -- release` — builds + packages dharma-mcp and dharma.
//!
//! Replaces scripts/build-release.{sh,ps1} duplicated bash/PowerShell logic
//! with one Rust implementation; the two scripts are now thin wrappers that
//! just invoke this binary (docs/proposal, "Dharma Build Config"). Reads
//! dharma-build.toml (repo root) + .env, same convention as samgraha.toml/.env.
//! Template: config.example/dharma-build.toml.

use anyhow::{bail, Context, Result};
use common::config::{
    interpolate, load_build_config, AgentSystemProviderConfig, BuildConfig,
    DomainSystemProviderConfig, ReleaseProviderEntry,
};
use registry::McpDb;
use services::capture::capture_bundle;
use services::registry::{register_agent_system, register_domain_system};
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

    bundle_release_providers(&root, &pkg_dir, &cfg)?;

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

/// One `[[release.providers]]` entry's outcome — registered (with capture
/// count) or skipped (with why), never a hard failure: a missing/empty
/// `content_root` is expected for a concern whose provider hasn't published
/// content yet (proposal 16, `agent-system-evaluation` before Kriti splits).
#[derive(Debug, PartialEq)]
enum BundleOutcome {
    Registered { name: String, captured_files: usize },
    Skipped { reason: String },
}

/// Register + capture one `[[release.providers]]` entry into `db`. A
/// malformed config file, an unknown `kind`, or a registration error (e.g.
/// the `agent_system_registry.concern` UNIQUE collision, `schema/mcp/
/// 01-agent_system_registry.sql:12`) is a hard error — the caller should let
/// it abort the release rather than silently skip. Only a missing/empty
/// content directory is a soft skip.
fn bundle_provider_entry(db: &McpDb, root: &Path, entry: &ReleaseProviderEntry) -> Result<BundleOutcome> {
    let config_path = resolve_path(root, &entry.config_path);
    let text = std::fs::read_to_string(&config_path)
        .with_context(|| format!("reading provider config {}", config_path.display()))?;

    match entry.kind.as_str() {
        "agent_system" => {
            let cfg: AgentSystemProviderConfig = toml::from_str(&text)
                .with_context(|| format!("parsing {} as an agent_system provider config", config_path.display()))?;
            let name = cfg.agent_system.name.clone();
            let concern = cfg.agent_system.concern.clone();
            let content_root = cfg
                .agent_system
                .content
                .as_ref()
                .and_then(|c| resolve_content_root(&c.root_dir, root));
            let Some(content_root) = content_root else {
                return Ok(BundleOutcome::Skipped {
                    reason: format!("no content at configured root_dir for concern '{concern}' ({})", entry.config_path),
                });
            };
            let row = register_agent_system(db, &name, &concern, &cfg.agent_system.description, false).with_context(|| {
                format!("registering agent_system '{name}' (concern '{concern}') from {}", entry.config_path)
            })?;
            let captured = capture_bundle(db, &name, &content_root)?.len();
            Ok(BundleOutcome::Registered { name: row.name, captured_files: captured })
        }
        "domain_system" => {
            let cfg: DomainSystemProviderConfig = toml::from_str(&text)
                .with_context(|| format!("parsing {} as a domain_system provider config", config_path.display()))?;
            let name = cfg.domain_system.name.clone();
            let content_root = cfg
                .domain_system
                .content
                .as_ref()
                .and_then(|c| resolve_content_root(&c.root_dir, root));
            let Some(content_root) = content_root else {
                return Ok(BundleOutcome::Skipped {
                    reason: format!("no content at configured root_dir for domain system '{name}' ({})", entry.config_path),
                });
            };
            let row = register_domain_system(db, &name, &cfg.domain_system.version, &cfg.domain_system.description)
                .with_context(|| format!("registering domain_system '{name}' from {}", entry.config_path))?;
            let captured = capture_bundle(db, &name, &content_root)?.len();
            Ok(BundleOutcome::Registered { name: row.name, captured_files: captured })
        }
        other => bail!("unknown release.providers kind '{other}' in {} (expected 'agent_system' or 'domain_system')", entry.config_path),
    }
}

/// Resolve a `root_dir` value — a literal path, or one containing `${VAR}`/
/// `${VAR:-default}` anywhere in the string (`common::config::interpolate`,
/// e.g. `${DHARMA_AGENT_CONTENT_DIR}/capability-provisioning`) — against
/// `root` (the workspace root) and return it only if it names a real
/// directory. An unresolved env var or a not-yet-created directory is
/// treated as "no content published yet," not an error.
fn resolve_content_root(raw: &str, root: &Path) -> Option<PathBuf> {
    let value = interpolate(raw, root).ok()?;
    if value.trim().is_empty() {
        return None;
    }
    let path = resolve_path(root, value.trim());
    path.is_dir().then_some(path)
}

fn resolve_path(base: &Path, raw: &str) -> PathBuf {
    let p = PathBuf::from(raw);
    if p.is_absolute() {
        p
    } else {
        base.join(p)
    }
}

/// The Release Bundling Step (proposal 16): register every configured
/// `[[release.providers]]` entry into `pkg_dir/data/mcp.db` so the packaged
/// release ships pre-populated instead of empty.
fn bundle_release_providers(root: &Path, pkg_dir: &Path, cfg: &BuildConfig) -> Result<()> {
    let providers = match cfg.release.as_ref() {
        Some(r) if !r.providers.is_empty() => &r.providers,
        _ => {
            println!("  -> data/mcp.db: no [[release.providers]] configured, skipping");
            return Ok(());
        }
    };
    let db = McpDb::open_at(&pkg_dir.join("data").join("mcp.db"))?;
    for entry in providers {
        match bundle_provider_entry(&db, root, entry)? {
            BundleOutcome::Registered { name, captured_files } => {
                println!("  -> data/mcp.db: registered '{name}' ({captured_files} files captured, {})", entry.config_path);
            }
            BundleOutcome::Skipped { reason } => {
                println!("  -> data/mcp.db: skipped — {reason}");
            }
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
             # where mcp.db lives (defaults to $HOME/.dharma if unset — seeded\n\
             # once from this package's data/mcp.db on first launch, if that\n\
             # file exists; see McpDb::open, proposal 16).\n\
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
             rem where mcp.db lives (defaults to %USERPROFILE%\\.dharma if unset --\r\n\
             rem seeded once from this package's data\\mcp.db on first launch, if\r\n\
             rem that file exists; see McpDb::open, proposal 16).\r\n\
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> PathBuf {
        std::env::temp_dir().join(format!("dharma-xtask-test-{}", uuid_like()))
    }

    // No `uuid` dependency in xtask — a counter is enough to keep parallel
    // test runs from colliding on the same temp path.
    fn uuid_like() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        format!("{}-{}", std::process::id(), COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    #[test]
    fn bundle_provider_entry_registers_agent_system_when_content_exists() {
        let root = temp_dir();
        let content_dir = root.join("agent-content");
        std::fs::create_dir_all(content_dir.join("agent")).unwrap();
        std::fs::write(content_dir.join("agent").join("example.yaml"), "name: example\n").unwrap();

        let config_path = root.join("provider.toml");
        std::fs::write(
            &config_path,
            format!(
                "[agent_system]\nname = \"capability-provisioning\"\nconcern = \"capability-provisioning\"\ndescription = \"test\"\n\n[agent_system.content]\nroot_dir = \"{}\"\n",
                content_dir.display()
            ),
        )
        .unwrap();

        let db = McpDb::open_in_memory().unwrap();
        let entry = ReleaseProviderEntry { kind: "agent_system".into(), config_path: "provider.toml".into() };
        let outcome = bundle_provider_entry(&db, &root, &entry).unwrap();
        match outcome {
            BundleOutcome::Registered { name, captured_files } => {
                assert_eq!(name, "capability-provisioning");
                assert_eq!(captured_files, 1);
            }
            other => panic!("expected Registered, got {other:?}"),
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn bundle_provider_entry_skips_when_content_root_missing() {
        let root = temp_dir();
        std::fs::create_dir_all(&root).unwrap();
        let config_path = root.join("provider.toml");
        std::fs::write(
            &config_path,
            "[agent_system]\nname = \"agent-system-evaluation\"\nconcern = \"agent-system-evaluation\"\n\n[agent_system.content]\nroot_dir = \"${DHARMA_AGENT_CONTENT_DIR_DOES_NOT_EXIST}\"\n",
        )
        .unwrap();

        let db = McpDb::open_in_memory().unwrap();
        let entry = ReleaseProviderEntry { kind: "agent_system".into(), config_path: "provider.toml".into() };
        let outcome = bundle_provider_entry(&db, &root, &entry).unwrap();
        match outcome {
            BundleOutcome::Skipped { reason } => assert!(reason.contains("agent-system-evaluation")),
            other => panic!("expected Skipped, got {other:?}"),
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn bundle_provider_entry_bails_on_concern_collision() {
        let root = temp_dir();
        let content_dir = root.join("agent-content");
        std::fs::create_dir_all(content_dir.join("agent")).unwrap();
        std::fs::write(content_dir.join("agent").join("example.yaml"), "name: example\n").unwrap();
        let config_path = root.join("provider.toml");
        std::fs::write(
            &config_path,
            format!(
                "[agent_system]\nname = \"dup\"\nconcern = \"dup-concern\"\n\n[agent_system.content]\nroot_dir = \"{}\"\n",
                content_dir.display()
            ),
        )
        .unwrap();

        let db = McpDb::open_in_memory().unwrap();
        register_agent_system(&db, "dup", "dup-concern", "", false).unwrap();
        let entry = ReleaseProviderEntry { kind: "agent_system".into(), config_path: "provider.toml".into() };
        let err = bundle_provider_entry(&db, &root, &entry).unwrap_err();
        assert!(err.to_string().contains("registering agent_system 'dup'"), "{err}");
        std::fs::remove_dir_all(&root).ok();
    }
}

use anyhow::Context;
use std::path::Path;

/// Load `KEY=VALUE` lines from a `.env` file, found by walking up from the
/// current directory, into the process environment. Keys already set in the
/// real environment are left untouched — the file only fills gaps, same
/// precedence as the `dotenv` convention. Mirrors samgraha's manual parser
/// (no `dotenv` crate dependency).
///
/// Called once at process startup (CLI and MCP `main()`), before any config
/// field that uses `resolve_configured_dir` is read — otherwise `${VAR}`
/// placeholders in the dharma TOMLs would never see values from `.env`.
pub fn load_dotenv() {
    let Ok(cwd) = std::env::current_dir() else { return };
    let Some(path) = find_dotenv(&cwd) else { return };
    let Ok(content) = std::fs::read_to_string(&path) else { return };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, val)) = trimmed.split_once('=') else { continue };
        let key = key.trim();
        if key.is_empty() || std::env::var(key).is_ok() {
            continue;
        }
        let val = val.trim().trim_matches('"').trim_matches('\'');
        std::env::set_var(key, val);
    }
}

/// Cross-platform home directory resolution.
/// Windows uses `USERPROFILE`; Unix uses `HOME`. Falls back to `.` if neither
/// is set.
pub fn home_dir() -> std::path::PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
}

/// MCP's own data directory — where the global `mcp.db` lives
/// (e.g. `~/.dharma/mcp.db`), mirroring samgraha's `mcp_dir()`.
/// `DHARMA_MCP_DIR` overrides for tests/dev; falls back to
/// `<home>/.dharma` if unset; falls back to `.` if even home is unresolvable.
pub fn mcp_dir() -> std::path::PathBuf {
    std::env::var("DHARMA_MCP_DIR")
        .map(std::path::PathBuf::from)
        .ok()
        .or_else(|| {
            let home = home_dir();
            if home.as_os_str().is_empty() || home == Path::new(".") {
                None
            } else {
                Some(home.join(".dharma"))
            }
        })
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}

/// Path of the global `mcp.db` — always inside `mcp_dir()`, never in a repo.
pub fn mcp_db_path() -> std::path::PathBuf {
    mcp_dir().join("mcp.db")
}

/// sha256 hex of `content` — the `content_hash` in `content_asset` and the
/// `capability_hash` in `analysis_cache`.
pub fn sha256_hex(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Build the `Command` to run a script, wrapped in its interpreter —
/// `.ps1`/`.sh`/`.py` aren't natively executable on every platform.
/// Anything else (a compiled native binary) runs directly. `args` are
/// appended after the script path.
pub fn script_command(script_path: &Path, args: &[&str]) -> std::process::Command {
    match script_path.extension().and_then(|e| e.to_str()) {
        Some("ps1") => {
            let mut cmd = pwsh_command();
            cmd.arg("-NoProfile").arg("-NonInteractive").arg("-File").arg(script_path);
            cmd.args(args);
            cmd
        }
        Some("sh") => {
            let mut cmd = sh_command();
            cmd.arg(script_path);
            cmd.args(args);
            cmd
        }
        Some("py") => {
            let mut cmd = python_command();
            cmd.arg(script_path);
            cmd.args(args);
            cmd
        }
        _ => {
            let mut cmd = std::process::Command::new(script_path);
            cmd.args(args);
            cmd
        }
    }
}

/// Resolve the Python interpreter to shell out to. Prefers `python3`;
/// falls back to `python` (common on Windows).
pub fn python_command() -> std::process::Command {
    for candidate in ["python3", "python"] {
        if std::process::Command::new(candidate)
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return std::process::Command::new(candidate);
        }
    }
    std::process::Command::new("python3")
}

fn pwsh_command() -> std::process::Command {
    for candidate in ["pwsh", "powershell"] {
        if std::process::Command::new(candidate)
            .arg("-NoProfile")
            .arg("-Command")
            .arg("exit 0")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return std::process::Command::new(candidate);
        }
    }
    std::process::Command::new("powershell")
}

fn sh_command() -> std::process::Command {
    for candidate in ["sh", "bash"] {
        if std::process::Command::new(candidate)
            .arg("-c")
            .arg("exit 0")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return std::process::Command::new(candidate);
        }
    }
    std::process::Command::new("sh")
}

/// Run `cmd` to completion, killing it and returning a `TimedOut` error if
/// `timeout_secs` elapses first. Shared by script runners.
fn run_with_optional_timeout(
    mut cmd: std::process::Command,
    timeout_secs: Option<u64>,
) -> std::io::Result<std::process::Output> {
    let Some(secs) = timeout_secs else {
        return cmd.output();
    };
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    cmd.spawn().and_then(|mut child| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => return child.wait_with_output(),
                Ok(None) => {
                    if std::time::Instant::now() >= deadline {
                        let _ = child.kill();
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            format!("Script timed out after {}s", secs),
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => return Err(e),
            }
        }
    })
}

/// Run a skill's script with the capability contract: `--repo-root`, `--in`,
/// `--out`. The script must write a JSON envelope to `--out`:
///
/// ```json
/// { "status": "ok"|"error", "message": "...", "written": [...] }
/// ```
///
/// Returns the parsed JSON envelope on success. Mirrors samgraha's
/// `run_capability_script`; this is the deterministic path of `run_skill`
/// (proposal 03/07).
pub fn run_capability_script(
    script_path: &Path,
    repo_root: &Path,
    input_json_path: &Path,
    timeout_secs: Option<u64>,
) -> anyhow::Result<serde_json::Value> {
    let out_file = std::env::temp_dir().join(format!("dharma-cap-{}.json", uuid::Uuid::new_v4()));
    let repo_root_str = repo_root.display().to_string();
    let in_str = input_json_path.display().to_string();
    let out_str = out_file.display().to_string();

    let is_ps1 = script_path.extension().and_then(|e| e.to_str()) == Some("ps1");
    let args: Vec<String> = if is_ps1 {
        vec![
            "-RepoRoot".into(), repo_root_str,
            "-In".into(), in_str,
            "-Out".into(), out_str.clone(),
        ]
    } else {
        vec![
            "--repo-root".into(), repo_root_str,
            "--in".into(), in_str,
            "--out".into(), out_str.clone(),
        ]
    };
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let mut cmd = script_command(script_path, &arg_refs);
    cmd.current_dir(repo_root);

    let output = run_with_optional_timeout(cmd, timeout_secs)
        .context("Failed to execute skill script")?;
    let content = std::fs::read_to_string(&out_file);
    let _ = std::fs::remove_file(&out_file);

    match content {
        Ok(text) => serde_json::from_str(text.trim_start_matches('\u{FEFF}'))
            .with_context(|| format!("Skill script wrote invalid JSON to {}", out_file.display())),
        Err(_) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!(
                "Skill script did not write an output file (exit {:?}); stderr: {} stdout: {}",
                output.status.code(),
                stderr.trim(),
                stdout.trim()
            )
        }
    }
}

/// Current git HEAD commit sha for a repo, or `None` if the repo has no
/// commits yet, isn't a git repo, or `git` isn't on PATH. Used to pin
/// proposal lifecycle transitions and audit runs to commits.
pub fn current_head_sha(repo_root: &Path) -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(repo_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sha = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if sha.is_empty() { None } else { Some(sha) }
}

fn find_dotenv(start: &Path) -> Option<std::path::PathBuf> {
    let mut dir = start;
    loop {
        let candidate = dir.join(".env");
        if candidate.exists() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hex_is_64_hex_chars() {
        assert_eq!(sha256_hex("dharma").len(), 64);
        assert!(sha256_hex("dharma").chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn script_command_dispatches_py_through_python_interpreter() {
        let cmd = script_command(Path::new("skill.py"), &["--in", "x.json"]);
        let program = cmd.get_program().to_string_lossy().to_string();
        assert!(program == "python3" || program == "python", "expected a python interpreter, got '{}'", program);
        let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().to_string()).collect();
        assert_eq!(args, vec!["skill.py", "--in", "x.json"]);
    }

    #[test]
    fn script_command_runs_unrecognized_extension_directly() {
        let cmd = script_command(Path::new("./checker"), &["--flag"]);
        assert_eq!(cmd.get_program().to_string_lossy(), "./checker");
    }
}

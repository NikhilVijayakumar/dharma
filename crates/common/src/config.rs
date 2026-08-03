use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Read + parse `<root>/dharma-repo.toml`, the consuming repository config.
/// The MCP adapter uses this to resolve `.dharma/` and to read the pinned
/// domain/version without re-parsing in the transport layer.
pub fn load_repo_config(root: &Path) -> Result<RepoConfig, String> {
    let path = root.join("dharma-repo.toml");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("No dharma-repo.toml at {}: {}", path.display(), e))?;
    toml::from_str(&text)
        .map_err(|e| format!("Invalid dharma-repo.toml at {}: {}", path.display(), e))
}

/// Read + parse `<root>/dharma-build.toml` — dharma's own self-tooling
/// config (`xtask`, `scripts/build-release.{sh,ps1}`). Template lives at
/// `config.example/dharma-build.toml`; copy it to the repo root to activate.
pub fn load_build_config(root: &Path) -> Result<BuildConfig, String> {
    let path = root.join("dharma-build.toml");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("No dharma-build.toml at {}: {}", path.display(), e))?;
    toml::from_str(&text)
        .map_err(|e| format!("Invalid dharma-build.toml at {}: {}", path.display(), e))
}

/// Resolve `${VAR}` or `${VAR:-default}` from the process environment.
///
/// - `"${VAR}"`: read `VAR` from the environment. Returns `None` if unset —
///   callers with their own fallback use that; callers with no fallback
///   should treat `None` as an error.
/// - `"${VAR:-default}"`: read `VAR`; if unset, use the literal `default`
///   instead (always `Some`).
/// - Anything else is returned unchanged (a literal value).
pub fn resolve_env_string(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let Some(inner) = trimmed.strip_prefix("${").and_then(|s| s.strip_suffix('}')) else {
        return Some(trimmed.to_string());
    };
    if let Some((var_name, default)) = inner.split_once(":-") {
        Some(std::env::var(var_name).unwrap_or_else(|_| default.to_string()))
    } else {
        std::env::var(inner).ok()
    }
}

/// Resolve a configured path that may be a `${VAR}` or `${VAR:-default}` placeholder.
///
/// - `"${VAR}"` (the whole string, nothing else): read `VAR`. If set, use its
///   value (joined to `root` if relative). If unset, fall back to
///   `root.join(fallback_rel)`.
/// - `"${VAR:-default}"`: read `VAR`; if unset, use `default` instead.
/// - Anything else is a literal path, used as-is (joined to `root` if relative).
pub fn resolve_configured_dir(raw: &str, root: &Path, fallback_rel: &str) -> PathBuf {
    let literal = resolve_env_string(raw);

    match literal {
        Some(value) if !value.is_empty() => {
            let p = PathBuf::from(&value);
            if p.is_absolute() {
                p
            } else {
                root.join(p)
            }
        }
        _ => root.join(fallback_rel),
    }
}

/// Substitute every `${VAR}` / `${VAR:-default}` occurrence anywhere inside
/// `input` — used for pipeline contract fields like
/// `${PROJECT_ROOT}/target/release/dharma-mcp.exe`. `PROJECT_ROOT` resolves
/// to `project_root`; everything else reads the process environment.
pub fn interpolate(input: &str, project_root: &Path) -> Result<String, String> {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            return Err(format!("Unterminated '${{' in '{}'", input));
        };
        let inner = &after[..end];
        let (var_name, default) = match inner.split_once(":-") {
            Some((n, d)) => (n, Some(d)),
            None => (inner, None),
        };
        let value = if var_name == "PROJECT_ROOT" {
            Some(project_root.to_string_lossy().to_string())
        } else {
            std::env::var(var_name).ok()
        };
        match value.or_else(|| default.map(|d| d.to_string())) {
            Some(v) => out.push_str(&v),
            None => {
                return Err(format!(
                    "Unresolved environment variable '{}' in '{}'",
                    var_name, input
                ))
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

// ---------------------------------------------------------------------------
// dharma-repo.toml — the CONSUMING repository config (proposal 06/11).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoConfig {
    #[serde(default)]
    pub repository: RepoSection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<ReportConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RepoSection {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uuid: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainSelect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dharma: Option<DharmaDirConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<DocumentationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tests: Option<TestsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsConfig>,
    #[serde(default)]
    pub ignore: IgnoreConfig,
}

/// `[repository.domain]` — selects a Domain System by name, with an optional
/// version pin. Never inline domain shape content (proposal 06/11).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainSelect {
    pub select: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// `[repository.dharma]` — where this repo's `.dharma/` lives and which MCP
/// instance it registers with.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DharmaDirConfig {
    #[serde(default = "default_dharma_dir")]
    pub dir: String,
    #[serde(default = "default_mcp_dir")]
    pub mcp_dir: String,
}

fn default_dharma_dir() -> String {
    "${DHARMA_DIR}".to_string()
}

fn default_mcp_dir() -> String {
    "${DHARMA_MCP_DIR}".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentationConfig {
    #[serde(default)]
    pub root_dir: String,
    /// Doc-type domains this repo maintains (dharma-build.toml).
    #[serde(default)]
    pub domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImplementationConfig {
    #[serde(default)]
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestsConfig {
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScriptsConfig {
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IgnoreConfig {
    #[serde(default)]
    pub patterns: Vec<String>,
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            patterns: vec![
                "**/node_modules/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/.dharma/**".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReportConfig {
    #[serde(default = "default_report_dir")]
    pub dir: String,
}

fn default_report_dir() -> String {
    "${DHARMA_REPORT_DIR}".to_string()
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self { dir: default_report_dir() }
    }
}

impl RepoConfig {
    /// Resolve this repo's `.dharma/` directory (holds repo.db, assets/).
    pub fn resolve_dharma_dir(&self, root: &Path) -> PathBuf {
        let raw = self
            .repository
            .dharma
            .as_ref()
            .map(|d| d.dir.as_str())
            .unwrap_or("${DHARMA_DIR}");
        resolve_configured_dir(raw, root, ".dharma")
    }

    /// Resolve the MCP data directory this repo registers with.
    pub fn resolve_mcp_dir(&self, root: &Path) -> PathBuf {
        let raw = self
            .repository
            .dharma
            .as_ref()
            .map(|d| d.mcp_dir.as_str())
            .unwrap_or("${DHARMA_MCP_DIR}");
        let resolved = resolve_configured_dir(raw, root, ".dharma");
        if resolved == root.join(".dharma") {
            // No explicit mcp_dir — fall back to the standard MCP data dir.
            crate::env::mcp_dir()
        } else {
            resolved
        }
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            repository: RepoSection {
                id: String::new(),
                name: String::new(),
                uuid: None,
                domain: None,
                dharma: None,
                documentation: None,
                implementation: None,
                tests: None,
                scripts: None,
                ignore: IgnoreConfig::default(),
            },
            report: None,
        }
    }
}

// ---------------------------------------------------------------------------
// dharma-domain.toml — the Domain System PROVIDER config (proposal 05/11).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainSystemProviderConfig {
    #[serde(default)]
    pub domain_system: DomainSystemSection,
    #[serde(default)]
    pub repository: ProviderRepositorySection,
}

fn default_version() -> String {
    "0.0.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DomainSystemSection {
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<DomainContentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<ProviderMcpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainContentConfig {
    #[serde(default = "default_content_root")]
    pub root_dir: String,
    #[serde(default = "default_section_map")]
    pub section_map: String,
}

fn default_content_root() -> String {
    "${DHARMA_DOMAIN_CONTENT_DIR}".to_string()
}

fn default_section_map() -> String {
    "section-map.yaml".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderMcpConfig {
    #[serde(default = "default_mcp_dir")]
    pub mcp_dir: String,
}

/// Shared `[repository.ignore]` on provider configs (not used by capture;
/// present so the same file is a valid samgraha-style config too).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ProviderRepositorySection {
    #[serde(default)]
    pub ignore: IgnoreConfig,
}

// ---------------------------------------------------------------------------
// dharma-agent.toml — the Agent System PROVIDER config (proposal 04/11).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSystemProviderConfig {
    #[serde(default)]
    pub agent_system: AgentSystemSection,
    #[serde(default)]
    pub repository: ProviderRepositorySection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgentSystemSection {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub concern: String,
    #[serde(default)]
    pub description: String,
    /// A REQUEST, not a grant — the Agent-Management Agent System reviews and
    /// sets the real `agent_system_registry.is_privileged` flag (proposal 11).
    #[serde(default)]
    pub is_privileged_request: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<AgentContentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<ProviderMcpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentContentConfig {
    #[serde(default = "default_agent_content_root")]
    pub root_dir: String,
}

fn default_agent_content_root() -> String {
    "${DHARMA_AGENT_CONTENT_DIR}".to_string()
}

// ---------------------------------------------------------------------------
// dharma-build.toml — Dharma's OWN self-tooling config (proposal 11, no MCP
// protocol role).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildConfig {
    #[serde(default)]
    pub repository: BuildRepositorySection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<PipelineContractConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<ReportConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<PackageConfig>,
}

/// `[package]` — where `xtask` (scripts/build-release.{sh,ps1}) drops the
/// packaged release (bin/, config/, env/, schema/).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageConfig {
    #[serde(default = "default_package_output_dir")]
    pub output_dir: String,
    /// Validity window baked into `dharma-mcp` at compile time
    /// (`crates/mcp/build.rs`) — `-1` means the build never expires.
    #[serde(default = "default_expiry_days")]
    pub expiry_days: String,
    #[serde(default = "default_expiry_hours")]
    pub expiry_hours: String,
}

fn default_package_output_dir() -> String {
    "${DHARMA_BUILD_OUTPUT_DIR}".to_string()
}

fn default_expiry_days() -> String {
    "${DHARMA_BUILD_EXPIRY_DAYS:-30}".to_string()
}

fn default_expiry_hours() -> String {
    "${DHARMA_BUILD_EXPIRY_HOURS:-0}".to_string()
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            output_dir: default_package_output_dir(),
            expiry_days: default_expiry_days(),
            expiry_hours: default_expiry_hours(),
        }
    }
}

impl BuildConfig {
    /// Resolve where the packaged release goes — `[package].output_dir`
    /// (`${DHARMA_BUILD_OUTPUT_DIR}` by default), falling back to
    /// `<root>/release` if unset.
    pub fn resolve_output_dir(&self, root: &Path) -> PathBuf {
        let raw = self
            .package
            .as_ref()
            .map(|p| p.output_dir.as_str())
            .unwrap_or("${DHARMA_BUILD_OUTPUT_DIR}");
        resolve_configured_dir(raw, root, "release")
    }

    /// Resolve `(days, hours)` for the packaged build's validity window —
    /// `[package].expiry_days` / `expiry_hours`, overridden per-build via
    /// `.env` (`DHARMA_BUILD_EXPIRY_DAYS`/`DHARMA_BUILD_EXPIRY_HOURS`).
    /// `days == -1` means the build never expires. Falls back to `(30, 0)`
    /// if unset or unparseable.
    pub fn resolve_expiry(&self) -> (i64, i64) {
        let days_raw = self.package.as_ref().map(|p| p.expiry_days.clone()).unwrap_or_else(default_expiry_days);
        let hours_raw = self.package.as_ref().map(|p| p.expiry_hours.clone()).unwrap_or_else(default_expiry_hours);
        let days = resolve_env_string(&days_raw).and_then(|s| s.parse().ok()).unwrap_or(30);
        let hours = resolve_env_string(&hours_raw).and_then(|s| s.parse().ok()).unwrap_or(0);
        (days, hours)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BuildRepositorySection {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uuid: Option<Uuid>,
    #[serde(default)]
    pub ignore: IgnoreConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<DocumentationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PipelineContractConfig {
    #[serde(default = "default_pipelines_version")]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<ContractSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<ContractSpec>,
}

fn default_pipelines_version() -> String {
    "1.0".to_string()
}

/// This build's supported `[pipelines] version`.
pub const SUPPORTED_PIPELINES_MAJOR: u32 = 1;
pub const SUPPORTED_PIPELINES_MINOR: u32 = 0;

impl PipelineContractConfig {
    /// Checks `version` (`"major.minor"`) against what this build supports.
    pub fn check_version(&self) -> Result<bool, String> {
        let (major, minor) = self
            .version
            .split_once('.')
            .and_then(|(maj, min)| Some((maj.parse::<u32>().ok()?, min.parse::<u32>().ok()?)))
            .ok_or_else(|| {
                format!(
                    "Invalid [pipelines] version '{}': expected 'major.minor'",
                    self.version
                )
            })?;
        if major != SUPPORTED_PIPELINES_MAJOR {
            return Err(format!(
                "config declares [pipelines] version {}, this build supports {}.x",
                self.version, SUPPORTED_PIPELINES_MAJOR
            ));
        }
        Ok(minor > SUPPORTED_PIPELINES_MINOR)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContractSpec {
    pub command: Vec<String>,
    #[serde(default = "default_working_directory")]
    pub working_directory: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub produces: Vec<String>,
    #[serde(default)]
    pub consumes: Vec<String>,
}

fn default_working_directory() -> String {
    "${PROJECT_ROOT}".to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedContract {
    pub command: Vec<String>,
    pub working_directory: PathBuf,
}

impl ContractSpec {
    pub fn resolve(&self, project_root: &Path) -> Result<ResolvedContract, String> {
        let command = self
            .command
            .iter()
            .map(|c| interpolate(c, project_root))
            .collect::<Result<Vec<_>, _>>()?;
        let working_directory = PathBuf::from(interpolate(&self.working_directory, project_root)?);
        Ok(ResolvedContract { command, working_directory })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_var_name() -> String {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("DHARMA_TEST_VAR_{}_{}", std::process::id(), id)
    }

    #[test]
    fn literal_string_passes_through() {
        assert_eq!(resolve_env_string("literal/path"), Some("literal/path".to_string()));
    }

    #[test]
    fn bare_var_resolves_when_set() {
        let name = unique_var_name();
        std::env::set_var(&name, "resolved-value");
        assert_eq!(resolve_env_string(&format!("${{{}}}", name)), Some("resolved-value".to_string()));
        std::env::remove_var(&name);
    }

    #[test]
    fn bare_var_is_none_when_unset() {
        let name = unique_var_name();
        std::env::remove_var(&name);
        assert_eq!(resolve_env_string(&format!("${{{}}}", name)), None);
    }

    #[test]
    fn var_with_default_falls_back_when_unset() {
        let name = unique_var_name();
        std::env::remove_var(&name);
        assert_eq!(
            resolve_env_string(&format!("${{{}:-fallback}}", name)),
            Some("fallback".to_string())
        );
    }

    #[test]
    fn resolve_configured_dir_uses_fallback_rel_when_bare_var_unset() {
        let name = unique_var_name();
        std::env::remove_var(&name);
        let root = std::path::Path::new("/repo");
        let resolved = resolve_configured_dir(&format!("${{{}}}", name), root, ".dharma");
        assert_eq!(resolved, root.join(".dharma"));
    }

    #[test]
    fn interpolate_substitutes_project_root_mid_string() {
        let root = std::path::Path::new("/repo");
        let result = interpolate("${PROJECT_ROOT}/target/release/dharma-mcp.exe", root).unwrap();
        assert_eq!(result, "/repo/target/release/dharma-mcp.exe");
    }

    #[test]
    fn repo_config_parses_repo_toml_shape() {
        let toml_str = r#"
            [repository]
            id = "my-service"
            name = "My Service"
            uuid = "00000000-0000-0000-0000-000000000000"

            [repository.domain]
            select = "rust-dev-domain"

            [repository.dharma]
            dir = "${DHARMA_DIR}"
            mcp_dir = "${DHARMA_MCP_DIR}"
        "#;
        let cfg: RepoConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.repository.domain.as_ref().unwrap().select, "rust-dev-domain");
        assert_eq!(cfg.repository.uuid.unwrap().to_string(), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn domain_provider_config_parses() {
        let toml_str = r#"
            [domain_system]
            name = "rust-dev-domain"
            version = "0.1.0"
            description = "Rust development domain"

            [domain_system.content]
            root_dir = "${DHARMA_DOMAIN_CONTENT_DIR}"
            section_map = "section-map.yaml"

            [domain_system.mcp]
            mcp_dir = "${DHARMA_MCP_DIR}"
        "#;
        let cfg: DomainSystemProviderConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.domain_system.name, "rust-dev-domain");
        assert_eq!(cfg.domain_system.version, "0.1.0");
        assert_eq!(cfg.domain_system.content.as_ref().unwrap().section_map, "section-map.yaml");
    }

    #[test]
    fn agent_provider_config_parses_with_privileged_request() {
        let toml_str = r#"
            [agent_system]
            name = "rust-development"
            concern = "rust-development"
            description = "Rust agents"
            is_privileged_request = false

            [agent_system.content]
            root_dir = "${DHARMA_AGENT_CONTENT_DIR}"

            [agent_system.mcp]
            mcp_dir = "${DHARMA_MCP_DIR}"
        "#;
        let cfg: AgentSystemProviderConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.agent_system.concern, "rust-development");
        assert!(!cfg.agent_system.is_privileged_request);
    }

    #[test]
    fn build_config_parses_with_pipelines() {
        let toml_str = r#"
            [repository]
            id = "dharma"
            name = "dharma"

            [repository.documentation]
            root_dir = "${DHARMA_DOCS_DIR}"
            domain = ["architecture", "build", "crates", "proposal"]

            [pipelines.build]
            command = ["cargo", "build", "--release", "--workspace"]
            working_directory = "${PROJECT_ROOT}"

            [report]
            dir = "${DHARMA_REPORT_DIR}"
        "#;
        let cfg: BuildConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.repository.documentation.as_ref().unwrap().domain.len(), 4);
        assert_eq!(cfg.pipelines.as_ref().unwrap().build.as_ref().unwrap().command, vec!["cargo", "build", "--release", "--workspace"]);
    }
}

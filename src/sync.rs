// ABOUTME: Core sync logic for generating shell completions.
// ABOUTME: Gets installed tools from mise and generates completion files.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;

use crate::registry;
use crate::shells;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read registry at {0}: {1}")]
    RegistryRead(PathBuf, std::io::Error),

    #[error("failed to parse registry at {0}: {1}")]
    RegistryParse(PathBuf, toml::de::Error),

    #[error("failed to get installed tools from mise: {0}")]
    MiseList(String),

    #[error("failed to create completions directory {0}: {1}")]
    CreateDir(PathBuf, std::io::Error),

    #[error("failed to write completion file {0}: {1}")]
    WriteFile(PathBuf, std::io::Error),

    #[error("failed to generate completion for {0}: {1}")]
    Generate(String, String),

    #[error("unsupported shell: {0}")]
    UnsupportedShell(String),

    #[error("unknown pattern '{1}' for tool '{0}'")]
    UnknownPattern(String, String),

    #[error("registry schema version {found} is not supported (expected {expected})")]
    IncompatibleSchema { found: u32, expected: u32 },

    #[error("registry is missing schema_version field (format may be outdated)")]
    MissingSchemaVersion,
}

pub struct CompletionsDirs {
    base_dir: PathBuf,
    shell_overrides: HashMap<String, PathBuf>,
}

impl CompletionsDirs {
    pub fn from_env() -> Self {
        let base_dir = if let Ok(home) = std::env::var("MISE_COMPLETIONS_SYNC_HOME") {
            PathBuf::from(home)
        } else {
            std::env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    dirs::home_dir()
                        .unwrap_or_default()
                        .join(".local")
                        .join("share")
                })
                .join("mise-completions")
        };

        let mut shell_overrides = HashMap::new();
        for shell in ["bash", "zsh", "fish"] {
            let env_var = format!("MISE_COMPLETIONS_SYNC_{}_DIR", shell.to_uppercase());
            if let Ok(path) = std::env::var(&env_var) {
                shell_overrides.insert(shell.to_string(), PathBuf::from(path));
            }
        }

        Self {
            base_dir,
            shell_overrides,
        }
    }

    pub fn get_dir(&self, shell: &str) -> Result<PathBuf, Error> {
        match shell {
            "zsh" | "bash" | "fish" => {
                if let Some(path) = self.shell_overrides.get(shell) {
                    Ok(path.clone())
                } else {
                    Ok(self.base_dir.join(shell))
                }
            }
            _ => Err(Error::UnsupportedShell(shell.to_string())),
        }
    }
}

/// Check if a string looks like a version identifier
fn is_version_component(s: &str) -> bool {
    // Common version patterns:
    // - v1, v2, v5 (Go module versions)
    // - v1.0, v2.3.1 (semver with v prefix)
    // - 1.0.0, 2.3.1 (semver without v prefix)
    // - latest (special case)
    if s == "latest" {
        return true;
    }

    // Check for v-prefixed versions (v1, v2.3, v1.0.0)
    if let Some(rest) = s.strip_prefix('v') {
        if rest.is_empty() {
            return false;
        }
        // Check if remaining characters are digits and dots
        return rest.chars().all(|c| c.is_ascii_digit() || c == '.');
    }

    // Check for plain semver (1.0.0, 2.3.1)
    if s.chars().all(|c| c.is_ascii_digit() || c == '.') {
        // Must have at least one digit
        return s.chars().any(|c| c.is_ascii_digit());
    }

    false
}

/// Extract the binary name from a tool identifier (which may have backend prefixes)
///
/// Examples:
/// - "go:golang.org/x/tools/gopls" -> "gopls"
/// - "go:sigs.k8s.io/kustomize/kustomize/v5" -> "kustomize" (skips version)
/// - "aqua:reteps/dockerfmt" -> "dockerfmt"
/// - "github:GoogleCloudPlatform/kubectl-ai" -> "kubectl-ai"
/// - "yq" -> "yq" (no prefix, keep as-is)
fn extract_tool_name(tool_id: &str) -> String {
    if let Some(colon_pos) = tool_id.find(':') {
        // Has backend prefix, extract the last component after the last slash
        let after_colon = &tool_id[colon_pos + 1..];
        let mut parts = after_colon.rsplit('/');

        // Get the last component
        let last = parts.next().unwrap_or(after_colon);

        // If it looks like a version, use the previous component instead
        if is_version_component(last) {
            parts
                .next()
                .map(|s| s.to_string())
                .unwrap_or_else(|| last.to_string())
        } else {
            last.to_string()
        }
    } else {
        // No backend prefix, use as-is
        tool_id.to_string()
    }
}

// When two installed tools resolve to the same short name (e.g. "kubectl" and
// "aqua:org/kubectl"), prefer the bare entry — it's the one most likely to be on PATH.
fn resolve_tool_binary(tool_map: &mut std::collections::HashMap<String, String>, tool_id: &str) {
    let short_name = extract_tool_name(tool_id);
    let is_bare = !tool_id.contains(':');
    match tool_map.entry(short_name) {
        std::collections::hash_map::Entry::Vacant(e) => {
            e.insert(tool_id.to_string());
        }
        std::collections::hash_map::Entry::Occupied(mut e) if is_bare => {
            e.insert(tool_id.to_string());
        }
        std::collections::hash_map::Entry::Occupied(_) => {}
    }
}

/// Get list of installed tools from mise
/// Returns a map of stripped tool names to their original IDs (with backend prefixes)
/// This allows registry matching on short names while preserving the original ID for mise x
fn get_installed_tools() -> Result<std::collections::HashMap<String, String>, Error> {
    let output = Command::new("mise")
        .args(["ls", "--installed", "--json"])
        .output()
        .map_err(|e| Error::MiseList(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::MiseList(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let tools: serde_json::Value =
        serde_json::from_str(&stdout).map_err(|e| Error::MiseList(e.to_string()))?;

    // mise ls --json returns an object with tool names as keys
    // Tool names may include backend prefixes like "go:package" or "aqua:repo/tool"
    // We need to extract the actual binary name for registry matching
    // but keep the original ID for mise x operations.
    let mut tool_map = std::collections::HashMap::new();
    if let Some(obj) = tools.as_object() {
        for tool_id in obj.keys() {
            resolve_tool_binary(&mut tool_map, tool_id);
        }
    }

    Ok(tool_map)
}

/// Get newly installed/updated tools from MISE_INSTALLED_TOOLS environment variable
/// Format: [{"name":"node","version":"20.10.0"},{"name":"python","version":"3.12.0"}]
/// Returns empty HashMap if env var is not set or empty (no newly installed tools)
/// Returns a map of stripped tool names to their original IDs (same format as get_installed_tools)
fn get_newly_installed_tools() -> Result<std::collections::HashMap<String, String>, Error> {
    let installed_tools_json = match std::env::var("MISE_INSTALLED_TOOLS") {
        Ok(val) if !val.is_empty() => val,
        Ok(_) => return Ok(std::collections::HashMap::new()), // Empty string = no new tools
        Err(_) => return Ok(std::collections::HashMap::new()), // Env var not set
    };

    parse_installed_tools_json(&installed_tools_json)
}

/// Parse the MISE_INSTALLED_TOOLS JSON string and extract tool names
/// Returns a map of stripped tool names to their original IDs (with backend prefixes)
fn parse_installed_tools_json(
    json: &str,
) -> Result<std::collections::HashMap<String, String>, Error> {
    let tools: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| Error::MiseList(format!("failed to parse MISE_INSTALLED_TOOLS: {e}")))?;

    let mut tool_map = std::collections::HashMap::new();

    if let Some(arr) = tools.as_array() {
        for item in arr {
            if let Some(name) = item.get("name") {
                if let Some(s) = name.as_str() {
                    resolve_tool_binary(&mut tool_map, s);
                }
            }
        }
    }

    Ok(tool_map)
}

/// Generate completion for a single tool and shell
fn generate_completion(
    tool_id: &str,   // Original ID with backend prefix (for mise x)
    tool_name: &str, // Stripped name (for filename)
    command: &str,
    shell: &str,
    output_dir: &PathBuf,
) -> Result<(), Error> {
    // Create output directory if needed
    std::fs::create_dir_all(output_dir).map_err(|e| Error::CreateDir(output_dir.clone(), e))?;

    // Run the completion command wrapped with mise to ensure the tool is available
    let wrapped_command = format!("mise x {tool_id} -- {command}");
    let output = Command::new("sh")
        .args(["-c", &wrapped_command])
        .output()
        .map_err(|e| Error::Generate(tool_name.to_string(), e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Generate(tool_name.to_string(), stderr.to_string()));
    }

    // Write the completion file using the stripped name (not the original ID)
    let filename = shells::completion_filename(shell, tool_name);
    let filepath = output_dir.join(&filename);

    std::fs::write(&filepath, &output.stdout).map_err(|e| Error::WriteFile(filepath.clone(), e))?;

    println!("  {tool_name} -> {filename}");
    Ok(())
}

/// Sync completions for the given shells and tools
pub fn sync_completions(
    dirs: &CompletionsDirs,
    shells: &[String],
    specific_tools: &[String],
    new_only: bool,
) -> Result<(), Error> {
    let registry = registry::load_registry()?;

    // Determine which tools to sync
    let tools_map: std::collections::HashMap<String, String> = if specific_tools.is_empty() {
        if new_only {
            // Only sync newly installed tools from MISE_INSTALLED_TOOLS env var
            get_newly_installed_tools()?
        } else {
            // Get all installed tools from mise (maps short name -> original ID)
            get_installed_tools()?
        }
    } else {
        // For specific tools, short name equals original ID
        specific_tools
            .iter()
            .cloned()
            .map(|t| (t.clone(), t))
            .collect()
    };

    // Filter to only tools in our registry (match on short names)
    let mut tools_in_registry: Vec<(&String, &String)> = tools_map
        .iter()
        .filter(|(short_name, _)| registry.tools.contains_key(*short_name))
        .collect();

    // Sort alphabetically by short name for consistent output
    tools_in_registry.sort_by(|a, b| a.0.cmp(b.0));

    if tools_in_registry.is_empty() {
        if new_only {
            println!("No newly-installed tools have completion support in registry.");
        } else {
            println!("No installed tools have completion support in registry.");
        }
        return Ok(());
    }

    println!(
        "Syncing completions for {} tools...",
        tools_in_registry.len()
    );

    for shell in shells {
        let output_dir = dirs.get_dir(shell)?;
        println!("\n[{shell}] -> {}", output_dir.display());

        for (short_name, original_id) in &tools_in_registry {
            if let Some(completions) = registry.tools.get(*short_name) {
                if let Some(cmd) = completions.get(shell) {
                    // Use the original tool ID (with backend prefix) for mise x
                    // and the stripped name for the filename
                    if let Err(e) =
                        generate_completion(original_id, short_name, cmd, shell, &output_dir)
                    {
                        eprintln!("  {short_name}: {e}");
                    }
                }
            }
        }
    }

    println!("\nDone!");
    Ok(())
}

/// Remove completions for tools that are no longer installed
pub fn clean_stale_completions(dirs: &CompletionsDirs) -> Result<(), Error> {
    let registry = registry::load_registry()?;
    let installed_map = get_installed_tools()?;
    let installed_set: HashSet<_> = installed_map.keys().collect();

    let shells = ["zsh", "bash", "fish"];
    let mut removed = 0;

    for shell in shells {
        let dir = dirs.get_dir(shell)?;
        if !dir.exists() {
            continue;
        }

        let entries = std::fs::read_dir(&dir).map_err(|e| Error::CreateDir(dir.clone(), e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                // Extract tool name from filename
                let tool = shells::tool_from_filename(shell, filename);
                if let Some(tool) = tool {
                    if registry.tools.contains_key(&tool)
                        && !installed_set.contains(&tool)
                        && std::fs::remove_file(&path).is_ok()
                    {
                        println!("Removed: {}", path.display());
                        removed += 1;
                    }
                }
            }
        }
    }

    println!("Cleaned {removed} stale completion files.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dirs_with_base(base: &str) -> CompletionsDirs {
        CompletionsDirs {
            base_dir: PathBuf::from(base),
            shell_overrides: HashMap::new(),
        }
    }

    #[test]
    fn test_get_dir_with_custom_base() {
        let dirs = dirs_with_base("/custom/base");

        assert_eq!(
            dirs.get_dir("bash").unwrap(),
            PathBuf::from("/custom/base/bash")
        );
        assert_eq!(
            dirs.get_dir("zsh").unwrap(),
            PathBuf::from("/custom/base/zsh")
        );
        assert_eq!(
            dirs.get_dir("fish").unwrap(),
            PathBuf::from("/custom/base/fish")
        );
    }

    #[test]
    fn test_get_dir_with_shell_overrides() {
        let dirs = CompletionsDirs {
            base_dir: PathBuf::from("/ignored"),
            shell_overrides: HashMap::from([
                (
                    "bash".to_string(),
                    PathBuf::from("/custom/bash-completion/completions"),
                ),
                (
                    "zsh".to_string(),
                    PathBuf::from("/custom/zsh/site-functions"),
                ),
                (
                    "fish".to_string(),
                    PathBuf::from("/custom/fish/vendor_completions.d"),
                ),
            ]),
        };

        assert_eq!(
            dirs.get_dir("bash").unwrap(),
            PathBuf::from("/custom/bash-completion/completions")
        );
        assert_eq!(
            dirs.get_dir("zsh").unwrap(),
            PathBuf::from("/custom/zsh/site-functions")
        );
        assert_eq!(
            dirs.get_dir("fish").unwrap(),
            PathBuf::from("/custom/fish/vendor_completions.d")
        );
    }

    #[test]
    fn test_get_dir_falls_back_to_base() {
        let dirs = dirs_with_base("/default/base");

        assert_eq!(
            dirs.get_dir("bash").unwrap(),
            PathBuf::from("/default/base/bash")
        );
        assert_eq!(
            dirs.get_dir("zsh").unwrap(),
            PathBuf::from("/default/base/zsh")
        );
        assert_eq!(
            dirs.get_dir("fish").unwrap(),
            PathBuf::from("/default/base/fish")
        );
    }

    #[test]
    fn test_get_dir_unsupported_shell() {
        let dirs = dirs_with_base("/any");
        assert!(dirs.get_dir("tcsh").is_err());
    }

    #[test]
    fn test_get_dir_shell_override_takes_precedence() {
        let dirs = CompletionsDirs {
            base_dir: PathBuf::from("/custom/base"),
            shell_overrides: HashMap::from([(
                "zsh".to_string(),
                PathBuf::from("/custom/zsh/site-functions"),
            )]),
        };

        assert_eq!(
            dirs.get_dir("bash").unwrap(),
            PathBuf::from("/custom/base/bash")
        );
        assert_eq!(
            dirs.get_dir("zsh").unwrap(),
            PathBuf::from("/custom/zsh/site-functions")
        );
        assert_eq!(
            dirs.get_dir("fish").unwrap(),
            PathBuf::from("/custom/base/fish")
        );
    }

    #[test]
    fn test_is_version_component() {
        // v-prefixed versions
        assert!(is_version_component("v5"));
        assert!(is_version_component("v1"));
        assert!(is_version_component("v1.0"));
        assert!(is_version_component("v2.3.1"));

        // Plain semver
        assert!(is_version_component("1.0.0"));
        assert!(is_version_component("2.3.1"));
        assert!(is_version_component("5"));

        // Special cases
        assert!(is_version_component("latest"));

        // Not versions
        assert!(!is_version_component("kustomize"));
        assert!(!is_version_component("gopls"));
        assert!(!is_version_component("kubectl-ai"));
        assert!(!is_version_component("v")); // just 'v' alone
        assert!(!is_version_component("")); // empty
    }

    #[test]
    fn test_extract_tool_name_no_prefix() {
        assert_eq!(extract_tool_name("yq"), "yq");
        assert_eq!(extract_tool_name("kubectl"), "kubectl");
        assert_eq!(extract_tool_name("mise"), "mise");
    }

    #[test]
    fn test_extract_tool_name_go_backend() {
        assert_eq!(extract_tool_name("go:golang.org/x/tools/gopls"), "gopls");
        assert_eq!(extract_tool_name("go:example.com/tool"), "tool");
    }

    #[test]
    fn test_extract_tool_name_go_backend_with_version() {
        // Go module paths with version suffix
        assert_eq!(
            extract_tool_name("go:sigs.k8s.io/kustomize/kustomize/v5"),
            "kustomize"
        );
        assert_eq!(
            extract_tool_name("go:github.com/golangci/golangci-lint/cmd/golangci-lint/v2"),
            "golangci-lint"
        );
        assert_eq!(extract_tool_name("go:example.com/tool/tool/v1.0.0"), "tool");
    }

    #[test]
    fn test_extract_tool_name_aqua_backend() {
        assert_eq!(extract_tool_name("aqua:reteps/dockerfmt"), "dockerfmt");
        assert_eq!(extract_tool_name("aqua:helm/helm"), "helm");
    }

    #[test]
    fn test_extract_tool_name_github_backend() {
        assert_eq!(
            extract_tool_name("github:GoogleCloudPlatform/kubectl-ai"),
            "kubectl-ai"
        );
        assert_eq!(extract_tool_name("github:cli/cli"), "cli");
    }

    #[test]
    fn test_extract_tool_name_complex_paths() {
        // Multiple slashes in path (now with version handling)
        assert_eq!(
            extract_tool_name("go:sigs.k8s.io/kustomize/kustomize/v5"),
            "kustomize"
        );
        // Single component after colon
        assert_eq!(extract_tool_name("aqua:simple-tool"), "simple-tool");
    }

    #[test]
    fn test_parse_installed_tools_json_parses_correctly() {
        let json = r#"[{"name":"node","version":"20.10.0"},{"name":"python","version":"3.12.0"}]"#;
        let tools = parse_installed_tools_json(json).expect("should parse JSON");
        assert_eq!(tools.get("node"), Some(&"node".to_string()));
        assert_eq!(tools.get("python"), Some(&"python".to_string()));
    }

    #[test]
    fn test_parse_installed_tools_json_short_names() {
        let json =
            r#"[{"name":"gopls","version":"0.12.0"},{"name":"dockerfmt","version":"1.0.0"}]"#;
        let tools = parse_installed_tools_json(json).expect("should parse short names");
        assert_eq!(tools.get("gopls"), Some(&"gopls".to_string()));
        assert_eq!(tools.get("dockerfmt"), Some(&"dockerfmt".to_string()));
    }

    #[test]
    fn test_parse_installed_tools_json_empty_array() {
        let json = "[]";
        let tools = parse_installed_tools_json(json).expect("should return empty HashMap");
        assert!(tools.is_empty());
    }

    #[test]
    fn test_parse_installed_tools_json_invalid_json() {
        let json = "invalid json";
        let result = parse_installed_tools_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("failed to parse MISE_INSTALLED_TOOLS"));
    }
}

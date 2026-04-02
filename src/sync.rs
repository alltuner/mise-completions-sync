// ABOUTME: Core sync logic for generating shell completions.
// ABOUTME: Gets installed tools from mise and generates completion files.

use std::collections::HashSet;
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

/// Get the base directory for completions
pub fn get_completions_base_dir() -> PathBuf {
    std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".local")
                .join("share")
        })
        .join("mise-completions")
}

/// Get the directory for a specific shell's completions
pub fn get_completions_dir(shell: &str) -> Result<PathBuf, Error> {
    match shell {
        "zsh" | "bash" | "fish" => Ok(get_completions_base_dir().join(shell)),
        _ => Err(Error::UnsupportedShell(shell.to_string())),
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
    // but keep the original ID for mise x operations
    let mut tool_map = std::collections::HashMap::new();
    if let Some(obj) = tools.as_object() {
        for tool_id in obj.keys() {
            let short_name = extract_tool_name(tool_id);
            // Verify the short name resolves to an actual binary using mise which
            let which_output = Command::new("mise").args(["which", &short_name]).output();
            if let Ok(output) = which_output {
                if output.status.success() {
                    // Binary exists, add to map
                    tool_map
                        .entry(short_name)
                        .or_insert_with(|| tool_id.to_string());
                }
            }
            // If mise which fails, skip this tool (binary not available)
        }
    }

    Ok(tool_map)
}

#[cfg(test)]
mod tests {
    use super::*;

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
pub fn sync_completions(shells: &[String], specific_tools: &[String]) -> Result<(), Error> {
    let registry = registry::load_registry()?;

    // Determine which tools to sync
    let tools_map: std::collections::HashMap<String, String> = if specific_tools.is_empty() {
        // Get all installed tools from mise (maps short name -> original ID)
        get_installed_tools()?
    } else {
        // For specific tools, short name equals original ID
        specific_tools
            .iter()
            .cloned()
            .map(|t| (t.clone(), t))
            .collect()
    };

    // Filter to only tools in our registry (match on short names)
    let tools_in_registry: Vec<(&String, &String)> = tools_map
        .iter()
        .filter(|(short_name, _)| registry.tools.contains_key(*short_name))
        .collect();

    if tools_in_registry.is_empty() {
        println!("No installed tools have completion support in registry.");
        return Ok(());
    }

    println!(
        "Syncing completions for {} tools...",
        tools_in_registry.len()
    );

    for shell in shells {
        let output_dir = get_completions_dir(shell)?;
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
pub fn clean_stale_completions() -> Result<(), Error> {
    let registry = registry::load_registry()?;
    let installed_map = get_installed_tools()?;
    let installed_set: HashSet<_> = installed_map.keys().collect();

    let shells = ["zsh", "bash", "fish"];
    let mut removed = 0;

    for shell in shells {
        let dir = get_completions_dir(shell)?;
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

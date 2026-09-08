// ABOUTME: Core sync logic for generating shell completions.
// ABOUTME: Gets installed tools from mise and generates completion files.

use std::collections::HashMap;
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

/// Pass-through scope flags for `mise ls`.
///
/// These mirror the corresponding `mise ls` options and narrow which installed
/// tools are discovered. `--global` and `--local` are mutually exclusive (enforced
/// at the CLI layer); `--current` combines freely with either. With no flags set,
/// behavior is unchanged: `mise ls --installed --json`.
#[derive(Debug, Clone, Copy, Default)]
pub struct MiseLsFlags {
    global: bool,
    local: bool,
    current: bool,
}

impl MiseLsFlags {
    pub fn new(global: bool, local: bool, current: bool) -> Self {
        Self {
            global,
            local,
            current,
        }
    }

    /// Build the full `mise ls` argument list, including any scope flags.
    fn args(&self) -> Vec<&'static str> {
        let mut args = vec!["ls", "--installed", "--json"];
        if self.global {
            args.push("--global");
        }
        if self.local {
            args.push("--local");
        }
        if self.current {
            args.push("--current");
        }
        args
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
fn get_installed_tools(
    flags: MiseLsFlags,
) -> Result<std::collections::HashMap<String, String>, Error> {
    let output = Command::new("mise")
        .args(flags.args())
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

/// Build the `mise x` invocation for a completion command.
///
/// A tool listing `requires` needs a second binary on PATH to render its
/// completions. Naming it here puts both tools in the same environment.
fn wrap_command(tool_id: &str, requires: Option<&str>, command: &str) -> String {
    match requires {
        Some(required) => format!("mise x {tool_id} {required} -- {command}"),
        None => format!("mise x {tool_id} -- {command}"),
    }
}

/// Lines to report for a command that succeeded but still wrote to stderr.
///
/// mise installs a missing `requires` tool on demand and reports it here, so
/// discarding stderr on success hides software being installed.
fn diagnostics(tool_name: &str, stderr: &[u8]) -> Vec<String> {
    let stderr = String::from_utf8_lossy(stderr);
    if stderr.trim().is_empty() {
        return Vec::new();
    }
    stderr
        .trim_end()
        .lines()
        .map(|line| format!("  {tool_name}: {line}"))
        .collect()
}

/// Reject completion output that is empty or whitespace-only.
///
/// `generate_completion` trusts the command's exit status, but some tools — or a
/// wrong registry guess — exit 0 while emitting nothing useful (e.g. the old `xh
/// completion zsh`, which parsed its args as a URL). Without this guard we would
/// silently write an empty completion file. Fail loudly instead.
fn validate_completion_output(tool_name: &str, stdout: &[u8]) -> Result<(), Error> {
    if stdout.iter().all(u8::is_ascii_whitespace) {
        return Err(Error::Generate(
            tool_name.to_string(),
            "completion command produced no output (empty or whitespace-only); \
             the registry command for this tool is likely wrong"
                .to_string(),
        ));
    }
    Ok(())
}

fn completion_output_name<'a>(tool_name: &'a str, entry: &'a registry::ToolEntry) -> &'a str {
    entry
        .completions
        .completion_name
        .as_deref()
        .unwrap_or(tool_name)
}

#[derive(Debug, PartialEq, Eq)]
struct SyncTarget {
    tool_name: String,
    tool_id: String,
}

fn installed_sync_targets(
    registry: &registry::Registry,
    installed_tools: &HashMap<String, String>,
) -> Vec<SyncTarget> {
    let mut targets: Vec<_> = registry
        .tools
        .iter()
        .filter_map(|(tool_name, entry)| {
            let runtime_tool = entry.provided_by.as_deref().unwrap_or(tool_name);
            installed_tools.get(runtime_tool).map(|tool_id| SyncTarget {
                tool_name: tool_name.clone(),
                tool_id: tool_id.clone(),
            })
        })
        .collect();
    targets.sort_by(|a, b| a.tool_name.cmp(&b.tool_name));
    targets
}

fn specific_sync_targets(
    registry: &registry::Registry,
    specific_tools: &[String],
) -> Vec<SyncTarget> {
    let mut targets: Vec<_> = specific_tools
        .iter()
        .filter_map(|tool_name| {
            registry.tools.get(tool_name).map(|entry| SyncTarget {
                tool_name: tool_name.clone(),
                tool_id: entry
                    .provided_by
                    .as_deref()
                    .unwrap_or(tool_name)
                    .to_string(),
            })
        })
        .collect();
    targets.sort_by(|a, b| a.tool_name.cmp(&b.tool_name));
    targets.dedup_by(|a, b| a.tool_name == b.tool_name);
    targets
}

fn is_tool_installed(
    tool_name: &str,
    entry: &registry::ToolEntry,
    installed_tools: &HashMap<String, String>,
) -> bool {
    let runtime_tool = entry.provided_by.as_deref().unwrap_or(tool_name);
    installed_tools.contains_key(runtime_tool)
}

/// Generate completion for a single tool and shell
/// Locate a bundled completion file somewhere beneath a tool's install directory.
///
/// The directory between the root and the file encodes the version and platform
/// (`hyperfine-v1.20.0-x86_64-apple-darwin/autocomplete`), and not consistently,
/// so the file is found by name rather than by path. Breadth-first, so the
/// shallowest match wins over a copy vendored deeper in the tree.
fn find_bundled(root: &std::path::Path, filename: &str) -> Option<PathBuf> {
    let mut queue = std::collections::VecDeque::from([root.to_path_buf()]);

    while let Some(dir) = queue.pop_front() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };

        let mut subdirs = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                subdirs.push(path);
            } else if entry.file_name() == filename {
                return Some(path);
            }
        }
        subdirs.sort();
        queue.extend(subdirs);
    }

    None
}

/// Where mise installed a tool.
fn tool_install_dir(tool_id: &str) -> Result<PathBuf, Error> {
    let output = Command::new("mise")
        .args(["where", tool_id])
        .output()
        .map_err(|e| Error::Generate(tool_id.to_string(), e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Generate(
            tool_id.to_string(),
            stderr.trim().to_string(),
        ));
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}

/// Copy a completion file that ships inside the tool's own download.
fn install_bundled_completion(
    tool_id: &str,
    tool_name: &str,
    entry: &registry::ToolEntry,
    filename: &str,
    shell: &str,
    output_dir: &PathBuf,
) -> Result<(), Error> {
    std::fs::create_dir_all(output_dir).map_err(|e| Error::CreateDir(output_dir.clone(), e))?;

    let root = tool_install_dir(tool_id)?;
    let source = find_bundled(&root, filename).ok_or_else(|| {
        Error::Generate(
            tool_name.to_string(),
            format!(
                "bundled completion '{filename}' not found under {}",
                root.display()
            ),
        )
    })?;

    let contents = std::fs::read(&source).map_err(|e| Error::RegistryRead(source.clone(), e))?;
    if contents.iter().all(u8::is_ascii_whitespace) {
        return Err(Error::Generate(
            tool_name.to_string(),
            format!("bundled completion {} is empty", source.display()),
        ));
    }

    let target = shells::completion_filename(shell, completion_output_name(tool_name, entry));
    let filepath = output_dir.join(&target);
    std::fs::write(&filepath, &contents).map_err(|e| Error::WriteFile(filepath.clone(), e))?;

    println!("  {tool_name} -> {target}");
    Ok(())
}

fn generate_completion(
    tool_id: &str,   // Original ID with backend prefix (for mise x)
    tool_name: &str, // Stripped name (for filename)
    entry: &registry::ToolEntry,
    command: &str,
    requires: Option<&str>,
    shell: &str,
    output_dir: &PathBuf,
) -> Result<(), Error> {
    // Create output directory if needed
    std::fs::create_dir_all(output_dir).map_err(|e| Error::CreateDir(output_dir.clone(), e))?;

    // Run the completion command wrapped with mise to ensure the tool is available
    let wrapped_command = wrap_command(tool_id, requires, command);
    let output = Command::new("sh")
        .args(["-c", &wrapped_command])
        .output()
        .map_err(|e| Error::Generate(tool_name.to_string(), e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Generate(tool_name.to_string(), stderr.to_string()));
    }

    for line in diagnostics(tool_name, &output.stderr) {
        eprintln!("{line}");
    }
    // A zero exit code is not enough: a wrong command can exit 0 with no output.
    validate_completion_output(tool_name, &output.stdout)?;

    // Write the completion file using the registry name unless an override is set.
    let filename = shells::completion_filename(shell, completion_output_name(tool_name, entry));
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
    flags: MiseLsFlags,
    new_only: bool,
) -> Result<(), Error> {
    let registry = registry::load_registry()?;

    // Determine which tools to sync
    let tools_in_registry = if specific_tools.is_empty() {
        let tools_map = if new_only {
            // Only sync newly installed tools from MISE_INSTALLED_TOOLS env var
            get_newly_installed_tools()?
        } else {
            // Get all installed tools from mise (maps short name -> original ID)
            get_installed_tools(flags)?
        };
        installed_sync_targets(&registry, &tools_map)
    } else {
        specific_sync_targets(&registry, specific_tools)
    };

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

        for target in &tools_in_registry {
            if let Some(entry) = registry.tools.get(&target.tool_name) {
                if let Some(value) = entry.completions.get(shell) {
                    // Use the provider's original tool ID (with backend prefix) for mise x
                    // and the registry entry name for the filename. For a bundled
                    // entry the value is a filename to copy, not a command to run.
                    let result = if entry.completions.is_bundled() {
                        install_bundled_completion(
                            &target.tool_id,
                            &target.tool_name,
                            entry,
                            value,
                            shell,
                            &output_dir,
                        )
                    } else {
                        generate_completion(
                            &target.tool_id,
                            &target.tool_name,
                            entry,
                            value,
                            entry.completions.requires.as_deref(),
                            shell,
                            &output_dir,
                        )
                    };
                    if let Err(e) = result {
                        eprintln!("  {}: {e}", target.tool_name);
                    }
                }
            }
        }
    }

    println!("\nDone!");
    Ok(())
}

/// Group registry entries by the name their completion files are written under.
///
/// Several entries can share one name: `rg` and `ripgrep` are the same binary
/// reached through two mise tool names, and both write `_rg`.
fn entries_by_completion_name(
    registry: &registry::Registry,
) -> std::collections::HashMap<&str, Vec<(&String, &registry::ToolEntry)>> {
    let mut by_name: std::collections::HashMap<&str, Vec<_>> = std::collections::HashMap::new();
    for (tool_name, entry) in &registry.tools {
        by_name
            .entry(completion_output_name(tool_name, entry))
            .or_default()
            .push((tool_name, entry));
    }
    by_name
}

/// Remove completions for tools that are no longer installed
pub fn clean_stale_completions(dirs: &CompletionsDirs, flags: MiseLsFlags) -> Result<(), Error> {
    let registry = registry::load_registry()?;
    let installed_map = get_installed_tools(flags)?;
    let writers = entries_by_completion_name(&registry);

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
                    // The file belongs to every entry that would write it, so it
                    // is stale only once none of them is installed.
                    if writers.get(tool.as_str()).is_some_and(|entries| {
                        entries
                            .iter()
                            .all(|(name, entry)| !is_tool_installed(name, entry, &installed_map))
                    }) && std::fs::remove_file(&path).is_ok()
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

    /// Build a throwaway directory tree; returns the root, caller removes it.
    fn scratch_tree(name: &str, files: &[&str]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("misecompsync-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for file in files {
            let path = root.join(file);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, "contents").unwrap();
        }
        root
    }

    #[test]
    fn test_find_bundled_locates_a_nested_file() {
        // The directory between the root and the file carries the version and
        // platform, so it can't be named up front -- only the file can.
        let root = scratch_tree(
            "nested",
            &["hyperfine-v1.20.0-x86_64-apple-darwin/autocomplete/_hyperfine"],
        );

        let found = find_bundled(&root, "_hyperfine").expect("should find the file");
        assert!(found.ends_with("autocomplete/_hyperfine"));

        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_find_bundled_returns_none_when_absent() {
        let root = scratch_tree("absent", &["bin/hyperfine"]);
        assert!(find_bundled(&root, "_hyperfine").is_none());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_find_bundled_prefers_the_shallowest_match() {
        let root = scratch_tree(
            "shallow",
            &["completions/_tool", "vendor/copy/deep/completions/_tool"],
        );

        let found = find_bundled(&root, "_tool").expect("should find a file");
        assert!(
            found.ends_with("completions/_tool") && !found.to_string_lossy().contains("vendor"),
            "expected the shallowest match, got {found:?}"
        );

        std::fs::remove_dir_all(&root).unwrap();
    }

    fn dirs_with_base(base: &str) -> CompletionsDirs {
        CompletionsDirs {
            base_dir: PathBuf::from(base),
            shell_overrides: HashMap::new(),
        }
    }

    fn registry_with_provider() -> registry::Registry {
        let completions = || registry::ToolCompletions {
            zsh: Some("completion zsh".to_string()),
            bash: None,
            fish: None,
            completion_name: None,
            requires: None,
            bundled: None,
        };

        registry::Registry {
            tools: HashMap::from([
                (
                    "uv".to_string(),
                    registry::ToolEntry {
                        completions: completions(),
                        provided_by: None,
                    },
                ),
                (
                    "uvx".to_string(),
                    registry::ToolEntry {
                        completions: completions(),
                        provided_by: Some("uv".to_string()),
                    },
                ),
            ]),
        }
    }

    #[test]
    fn test_diagnostics_empty_when_nothing_written() {
        assert!(diagnostics("fnox", b"").is_empty());
        assert!(diagnostics("fnox", b"  \n\t \n").is_empty());
    }

    #[test]
    fn test_diagnostics_attribute_each_line_to_the_tool() {
        assert_eq!(
            diagnostics(
                "fnox",
                b"mise usage@4.0.0 install\nmise usage@4.0.0 download\n"
            ),
            vec![
                "  fnox: mise usage@4.0.0 install",
                "  fnox: mise usage@4.0.0 download",
            ]
        );
    }

    #[test]
    fn test_diagnostics_handle_invalid_utf8() {
        assert_eq!(
            diagnostics("fnox", b"warn: \xff\xfe"),
            vec!["  fnox: warn: ��"]
        );
    }

    #[test]
    fn test_wrap_command_without_requires() {
        assert_eq!(
            wrap_command("yq", None, "yq completion zsh"),
            "mise x yq -- yq completion zsh"
        );
    }

    #[test]
    fn test_wrap_command_with_requires() {
        // The helper tool joins the same `mise x` invocation, so it lands on PATH
        // for the command without a nested `mise x`.
        assert_eq!(
            wrap_command("fnox", Some("usage"), "fnox completion zsh"),
            "mise x fnox usage -- fnox completion zsh"
        );
    }

    #[test]
    fn test_wrap_command_preserves_backend_prefix() {
        assert_eq!(
            wrap_command("pipx:ipython", Some("pipx:argcomplete"), "ipython x"),
            "mise x pipx:ipython pipx:argcomplete -- ipython x"
        );
    }

    #[test]
    fn test_validate_completion_output_rejects_empty() {
        // Regression: a wrong registry command can exit 0 with no output (the old
        // `xh completion zsh` did exactly this). We must fail loudly, not write an
        // empty completion file.
        assert!(validate_completion_output("xh", b"").is_err());
        assert!(validate_completion_output("xh", b"   \n\t  ").is_err());
    }

    #[test]
    fn test_validate_completion_output_accepts_real_script() {
        assert!(validate_completion_output("xh", b"#compdef xh\n...").is_ok());
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
    fn test_mise_ls_flags_default() {
        // No scope flags -> unchanged behavior.
        let flags = MiseLsFlags::default();
        assert_eq!(flags.args(), vec!["ls", "--installed", "--json"]);
    }

    #[test]
    fn test_mise_ls_flags_global() {
        let flags = MiseLsFlags::new(true, false, false);
        assert_eq!(
            flags.args(),
            vec!["ls", "--installed", "--json", "--global"]
        );
    }

    #[test]
    fn test_mise_ls_flags_local() {
        let flags = MiseLsFlags::new(false, true, false);
        assert_eq!(flags.args(), vec!["ls", "--installed", "--json", "--local"]);
    }

    #[test]
    fn test_mise_ls_flags_current() {
        let flags = MiseLsFlags::new(false, false, true);
        assert_eq!(
            flags.args(),
            vec!["ls", "--installed", "--json", "--current"]
        );
    }

    #[test]
    fn test_mise_ls_flags_global_and_current() {
        // --current combines freely with --global.
        let flags = MiseLsFlags::new(true, false, true);
        assert_eq!(
            flags.args(),
            vec!["ls", "--installed", "--json", "--global", "--current"]
        );
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

    #[test]
    fn test_installed_sync_targets_include_provider_children() {
        let registry = registry_with_provider();
        let installed = HashMap::from([("uv".to_string(), "aqua:astral-sh/uv".to_string())]);

        assert_eq!(
            installed_sync_targets(&registry, &installed),
            vec![
                SyncTarget {
                    tool_name: "uv".to_string(),
                    tool_id: "aqua:astral-sh/uv".to_string(),
                },
                SyncTarget {
                    tool_name: "uvx".to_string(),
                    tool_id: "aqua:astral-sh/uv".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_installed_sync_targets_exclude_missing_provider() {
        let registry = registry_with_provider();
        let installed = HashMap::from([("node".to_string(), "node".to_string())]);

        assert!(installed_sync_targets(&registry, &installed).is_empty());
    }

    #[test]
    fn test_specific_sync_targets_do_not_expand_provider() {
        let registry = registry_with_provider();

        assert_eq!(
            specific_sync_targets(&registry, &["uvx".to_string(), "uvx".to_string()]),
            vec![SyncTarget {
                tool_name: "uvx".to_string(),
                tool_id: "uv".to_string(),
            }]
        );
        assert_eq!(
            specific_sync_targets(&registry, &["uv".to_string()]),
            vec![SyncTarget {
                tool_name: "uv".to_string(),
                tool_id: "uv".to_string(),
            }]
        );
    }

    #[test]
    fn test_provider_controls_installed_liveness() {
        let registry = registry_with_provider();
        let installed = HashMap::from([("uv".to_string(), "uv".to_string())]);
        let uv = registry.tools.get("uv").unwrap();
        let uvx = registry.tools.get("uvx").unwrap();

        assert!(is_tool_installed("uv", uv, &installed));
        assert!(is_tool_installed("uvx", uvx, &installed));
        assert!(!is_tool_installed("uvx", uvx, &HashMap::new()));
    }

    fn entry_named(completion_name: Option<&str>) -> registry::ToolEntry {
        registry::ToolEntry {
            completions: registry::ToolCompletions {
                zsh: Some("completion zsh".to_string()),
                bash: None,
                fish: None,
                completion_name: completion_name.map(str::to_string),
                requires: None,
                bundled: None,
            },
            provided_by: None,
        }
    }

    #[test]
    fn test_completion_name_decides_where_an_entry_is_grouped() {
        let registry = registry::Registry {
            tools: HashMap::from([("television".to_string(), entry_named(Some("tv")))]),
        };

        let by_name = entries_by_completion_name(&registry);
        assert!(by_name.contains_key("tv"));
        assert!(!by_name.contains_key("television"));
    }

    #[test]
    fn test_entries_sharing_a_completion_name_are_grouped_together() {
        // rg and ripgrep are the same binary under two mise tool names.
        let registry = registry::Registry {
            tools: HashMap::from([
                ("rg".to_string(), entry_named(None)),
                ("ripgrep".to_string(), entry_named(Some("rg"))),
            ]),
        };

        let by_name = entries_by_completion_name(&registry);
        let mut writers: Vec<_> = by_name["rg"]
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        writers.sort_unstable();
        assert_eq!(writers, ["rg", "ripgrep"]);
    }

    #[test]
    fn test_a_shared_completion_file_is_stale_only_when_no_writer_is_installed() {
        let registry = registry::Registry {
            tools: HashMap::from([
                ("rg".to_string(), entry_named(None)),
                ("ripgrep".to_string(), entry_named(Some("rg"))),
            ]),
        };
        let by_name = entries_by_completion_name(&registry);
        let stale = |installed: &HashMap<String, String>| {
            by_name["rg"]
                .iter()
                .all(|(name, entry)| !is_tool_installed(name, entry, installed))
        };

        let via_ripgrep = HashMap::from([("ripgrep".to_string(), "ripgrep".to_string())]);
        assert!(!stale(&via_ripgrep));
        assert!(stale(&HashMap::new()));
    }
}

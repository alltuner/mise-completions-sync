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

/// Get list of installed tools from mise
fn get_installed_tools() -> Result<Vec<String>, Error> {
    let output = Command::new("mise")
        .args(["ls", "--installed", "--json"])
        .output()
        .map_err(|e| Error::MiseList(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::MiseList(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let tools: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| Error::MiseList(e.to_string()))?;

    // mise ls --json returns an object with tool names as keys
    let tool_names: Vec<String> = tools
        .as_object()
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default();

    Ok(tool_names)
}

/// Generate completion for a single tool and shell
fn generate_completion(
    tool: &str,
    command: &str,
    shell: &str,
    output_dir: &PathBuf,
) -> Result<(), Error> {
    // Create output directory if needed
    std::fs::create_dir_all(output_dir)
        .map_err(|e| Error::CreateDir(output_dir.clone(), e))?;

    // Run the completion command
    let output = Command::new("sh")
        .args(["-c", command])
        .output()
        .map_err(|e| Error::Generate(tool.to_string(), e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Generate(tool.to_string(), stderr.to_string()));
    }

    // Write the completion file
    let filename = shells::completion_filename(shell, tool);
    let filepath = output_dir.join(&filename);

    std::fs::write(&filepath, &output.stdout)
        .map_err(|e| Error::WriteFile(filepath.clone(), e))?;

    println!("  {tool} -> {filename}");
    Ok(())
}

/// Sync completions for the given shells and tools
pub fn sync_completions(shells: &[String], specific_tools: &[String]) -> Result<(), Error> {
    let registry = registry::load_registry()?;

    // Determine which tools to sync
    let tools_to_sync: Vec<String> = if specific_tools.is_empty() {
        // Get all installed tools from mise
        get_installed_tools()?
    } else {
        specific_tools.to_vec()
    };

    // Filter to only tools in our registry
    let tools_in_registry: Vec<&String> = tools_to_sync
        .iter()
        .filter(|t| registry.tools.contains_key(*t))
        .collect();

    if tools_in_registry.is_empty() {
        println!("No installed tools have completion support in registry.");
        return Ok(());
    }

    println!("Syncing completions for {} tools...", tools_in_registry.len());

    for shell in shells {
        let output_dir = get_completions_dir(shell)?;
        println!("\n[{shell}] -> {}", output_dir.display());

        for tool in &tools_in_registry {
            if let Some(completions) = registry.tools.get(*tool) {
                if let Some(cmd) = completions.get(shell) {
                    if let Err(e) = generate_completion(tool, cmd, shell, &output_dir) {
                        eprintln!("  {tool}: {e}");
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
    let installed = get_installed_tools()?;
    let installed_set: HashSet<_> = installed.iter().collect();

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

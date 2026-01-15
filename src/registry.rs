// ABOUTME: Loads the tool completion registry from registry.toml.
// ABOUTME: Maps tool names to their shell-specific completion commands.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::sync::Error;

const EMBEDDED_REGISTRY: &str = include_str!("../registry.toml");

/// Parsed registry format with patterns and tools sections
#[derive(Debug, Deserialize)]
struct RawRegistry {
    #[serde(default)]
    patterns: HashMap<String, ToolCompletions>,
    #[serde(default)]
    tools: HashMap<String, ToolEntry>,
}

/// A tool entry: either a pattern name or explicit shell commands
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ToolEntry {
    Pattern(String),
    Explicit(ToolCompletions),
}

/// Expanded registry with all patterns resolved
#[derive(Debug)]
pub struct Registry {
    pub tools: HashMap<String, ToolCompletions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCompletions {
    pub zsh: Option<String>,
    pub bash: Option<String>,
    pub fish: Option<String>,
}

impl ToolCompletions {
    pub fn get(&self, shell: &str) -> Option<&String> {
        match shell {
            "zsh" => self.zsh.as_ref(),
            "bash" => self.bash.as_ref(),
            "fish" => self.fish.as_ref(),
            _ => None,
        }
    }

    /// Expand pattern placeholders with tool name
    fn expand(&self, tool_name: &str) -> Self {
        Self {
            zsh: self.zsh.as_ref().map(|s| s.replace("{}", tool_name)),
            bash: self.bash.as_ref().map(|s| s.replace("{}", tool_name)),
            fish: self.fish.as_ref().map(|s| s.replace("{}", tool_name)),
        }
    }
}

/// Try to load registry from external file, with fallback to embedded
fn get_registry_content() -> Result<(String, Option<PathBuf>), Error> {
    // Check for registry.toml next to the executable (allows user customization)
    if let Ok(exe_path) = std::env::current_exe() {
        let alongside = exe_path.parent().unwrap().join("registry.toml");
        if alongside.exists() {
            let content = std::fs::read_to_string(&alongside)
                .map_err(|e| Error::RegistryRead(alongside.clone(), e))?;
            return Ok((content, Some(alongside)));
        }
    }

    // Check XDG data directory for user-provided registry
    if let Some(data_dir) = dirs::data_dir() {
        let user_registry = data_dir.join("mise-completions-sync").join("registry.toml");
        if user_registry.exists() {
            let content = std::fs::read_to_string(&user_registry)
                .map_err(|e| Error::RegistryRead(user_registry.clone(), e))?;
            return Ok((content, Some(user_registry)));
        }
    }

    // Use embedded registry
    Ok((EMBEDDED_REGISTRY.to_string(), None))
}

pub fn load_registry() -> Result<Registry, Error> {
    let (content, path) = get_registry_content()?;
    let path_for_error = path.clone().unwrap_or_else(|| PathBuf::from("<embedded>"));

    let raw: RawRegistry =
        toml::from_str(&content).map_err(|e| Error::RegistryParse(path_for_error.clone(), e))?;

    let mut tools = HashMap::new();

    for (tool_name, entry) in raw.tools {
        let completions = match entry {
            ToolEntry::Pattern(pattern_name) => {
                let pattern = raw.patterns.get(&pattern_name).ok_or_else(|| {
                    Error::UnknownPattern(tool_name.clone(), pattern_name.clone())
                })?;
                pattern.expand(&tool_name)
            }
            ToolEntry::Explicit(completions) => completions,
        };
        tools.insert(tool_name, completions);
    }

    Ok(Registry { tools })
}

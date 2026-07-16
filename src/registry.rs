// ABOUTME: Loads the tool completion registry from registry.toml.
// ABOUTME: Maps tool names to their shell-specific completion commands.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::sync::Error;

const EMBEDDED_REGISTRY: &str = include_str!("../registry.toml");
const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Parsed registry format with patterns and tools sections
#[derive(Debug, Deserialize)]
struct RawRegistry {
    schema_version: Option<u32>,
    #[serde(default)]
    patterns: HashMap<String, ToolCompletions>,
    #[serde(default)]
    tools: HashMap<String, RawToolEntry>,
}

/// A tool entry: either a pattern name or explicit shell commands
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawToolEntry {
    Pattern(String),
    Explicit(ExplicitToolEntry),
}

#[derive(Debug, Deserialize)]
struct ExplicitToolEntry {
    #[serde(flatten)]
    completions: ToolCompletions,
    provided_by: Option<String>,
}

/// Expanded registry with all patterns resolved
#[derive(Debug)]
pub struct Registry {
    pub tools: HashMap<String, ToolEntry>,
}

#[derive(Debug, Clone)]
pub struct ToolEntry {
    pub completions: ToolCompletions,
    pub provided_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCompletions {
    pub zsh: Option<String>,
    pub bash: Option<String>,
    pub fish: Option<String>,
    /// Another mise tool that must be on PATH for the command to work, because
    /// the tool shells out to it to render completions (e.g. fnox needs `usage`).
    pub requires: Option<String>,
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
            requires: self.requires.clone(),
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

    parse_registry(&content, path_for_error)
}

fn parse_registry(content: &str, path_for_error: PathBuf) -> Result<Registry, Error> {
    let raw: RawRegistry =
        toml::from_str(content).map_err(|e| Error::RegistryParse(path_for_error.clone(), e))?;

    // Check schema version
    match raw.schema_version {
        None => return Err(Error::MissingSchemaVersion),
        Some(v) if v != CURRENT_SCHEMA_VERSION => {
            return Err(Error::IncompatibleSchema {
                found: v,
                expected: CURRENT_SCHEMA_VERSION,
            })
        }
        Some(_) => {}
    }

    let mut tools = HashMap::new();

    for (tool_name, entry) in raw.tools {
        let entry = match entry {
            RawToolEntry::Pattern(pattern_name) => {
                let pattern = raw.patterns.get(&pattern_name).ok_or_else(|| {
                    Error::UnknownPattern(tool_name.clone(), pattern_name.clone())
                })?;
                ToolEntry {
                    completions: pattern.expand(&tool_name),
                    provided_by: None,
                }
            }
            RawToolEntry::Explicit(entry) => ToolEntry {
                completions: entry.completions,
                provided_by: entry.provided_by,
            },
        };
        tools.insert(tool_name, entry);
    }

    Ok(Registry { tools })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prek_in_registry() {
        let registry = load_registry().expect("Failed to load registry");
        let prek = registry
            .tools
            .get("prek")
            .expect("prek should be in registry");
        assert_eq!(
            prek.completions.zsh.as_deref(),
            Some("prek util generate-shell-completion zsh")
        );
        assert_eq!(
            prek.completions.bash.as_deref(),
            Some("prek util generate-shell-completion bash")
        );
        assert_eq!(
            prek.completions.fish.as_deref(),
            Some("prek util generate-shell-completion fish")
        );
        assert_eq!(prek.provided_by, None);
    }

    #[test]
    fn test_xh_in_registry() {
        // Regression test: xh does not support `xh completion <shell>` (it treats the
        // args as URLs and tries to send HTTP requests). It uses `xh --generate=complete-<shell>`.
        let registry = load_registry().expect("Failed to load registry");
        let xh = registry.tools.get("xh").expect("xh should be in registry");
        assert_eq!(
            xh.completions.zsh.as_deref(),
            Some("xh --generate=complete-zsh")
        );
        assert_eq!(
            xh.completions.bash.as_deref(),
            Some("xh --generate=complete-bash")
        );
        assert_eq!(
            xh.completions.fish.as_deref(),
            Some("xh --generate=complete-fish")
        );
    }

    #[test]
    fn test_fnox_requires_usage() {
        // fnox renders completions by shelling out to the `usage` CLI, which is not
        // on PATH inside `mise x fnox`. The command itself stays plain; `requires`
        // is what puts usage there.
        let registry = load_registry().expect("Failed to load registry");
        let fnox = registry
            .tools
            .get("fnox")
            .expect("fnox should be in registry");
        assert_eq!(fnox.completions.requires.as_deref(), Some("usage"));
        assert_eq!(fnox.completions.zsh.as_deref(), Some("fnox completion zsh"));
        assert_eq!(
            fnox.completions.bash.as_deref(),
            Some("fnox completion bash")
        );
        assert_eq!(
            fnox.completions.fish.as_deref(),
            Some("fnox completion fish")
        );
    }

    #[test]
    fn test_requires_defaults_to_none() {
        let registry = load_registry().expect("Failed to load registry");
        let yq = registry.tools.get("yq").expect("yq should be in registry");
        assert_eq!(yq.completions.requires, None);
    }

    #[test]
    fn test_self_in_registry() {
        let registry = load_registry().expect("Failed to load registry");
        let entry = registry
            .tools
            .get("mise-completions-sync")
            .expect("mise-completions-sync should be in registry");
        assert_eq!(
            entry.completions.zsh.as_deref(),
            Some("misecompsync completion zsh")
        );
        assert_eq!(
            entry.completions.bash.as_deref(),
            Some("misecompsync completion bash")
        );
        assert_eq!(
            entry.completions.fish.as_deref(),
            Some("misecompsync completion fish")
        );
    }

    #[test]
    fn test_uvx_in_registry() {
        let registry = load_registry().expect("Failed to load registry");
        let uvx = registry
            .tools
            .get("uvx")
            .expect("uvx should be in registry");

        assert_eq!(uvx.provided_by.as_deref(), Some("uv"));
        assert_eq!(
            uvx.completions.zsh.as_deref(),
            Some("uvx --generate-shell-completion zsh")
        );
        assert_eq!(
            uvx.completions.bash.as_deref(),
            Some("uvx --generate-shell-completion bash")
        );
        assert_eq!(
            uvx.completions.fish.as_deref(),
            Some("uvx --generate-shell-completion fish")
        );
    }

    #[test]
    fn test_explicit_entry_with_provider() {
        let registry = parse_registry(
            r#"
schema_version = 1

[patterns]
standard = { zsh = "{} completion zsh" }

[tools]
parent = "standard"
child = { provided_by = "parent", zsh = "child completion zsh", bash = "child completion bash" }
"#,
            PathBuf::from("<test>"),
        )
        .expect("Failed to parse registry");

        let parent = registry.tools.get("parent").expect("parent should exist");
        assert_eq!(parent.provided_by, None);
        assert_eq!(
            parent.completions.zsh.as_deref(),
            Some("parent completion zsh")
        );

        let child = registry.tools.get("child").expect("child should exist");
        assert_eq!(child.provided_by.as_deref(), Some("parent"));
        assert_eq!(
            child.completions.zsh.as_deref(),
            Some("child completion zsh")
        );
        assert_eq!(
            child.completions.bash.as_deref(),
            Some("child completion bash")
        );
        assert_eq!(child.completions.fish, None);
    }
}

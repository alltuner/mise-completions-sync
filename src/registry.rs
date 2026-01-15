// ABOUTME: Loads the tool completion registry from registry.toml.
// ABOUTME: Maps tool names to their shell-specific completion commands.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::sync::Error;

const EMBEDDED_REGISTRY: &str = include_str!("../registry.toml");

#[derive(Debug, Deserialize)]
pub struct Registry {
    #[serde(flatten)]
    pub tools: HashMap<String, ToolCompletions>,
}

#[derive(Debug, Deserialize)]
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
        let user_registry = data_dir
            .join("mise-completions-sync")
            .join("registry.toml");
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
    let registry: Registry = toml::from_str(&content)
        .map_err(|e| Error::RegistryParse(path.unwrap_or_else(|| PathBuf::from("<embedded>")), e))?;
    Ok(registry)
}

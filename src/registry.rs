// ABOUTME: Loads the tool completion registry from registry.toml.
// ABOUTME: Maps tool names to their shell-specific completion commands.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::sync::Error;

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

/// Get the path to registry.toml bundled with the binary
fn get_registry_path() -> Result<PathBuf, Error> {
    // First, check if there's a registry.toml next to the executable
    if let Ok(exe_path) = std::env::current_exe() {
        let alongside = exe_path.parent().unwrap().join("registry.toml");
        if alongside.exists() {
            return Ok(alongside);
        }
    }

    // Fall back to embedded registry (compiled into binary)
    // For now, we'll look in common locations
    let candidates = [
        // Development: in the project directory
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("registry.toml"),
        // Installed via cargo
        dirs::data_dir()
            .unwrap_or_default()
            .join("mise-completions-sync")
            .join("registry.toml"),
    ];

    for path in candidates {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(Error::RegistryNotFound)
}

pub fn load_registry() -> Result<Registry, Error> {
    let path = get_registry_path()?;
    let content = std::fs::read_to_string(&path)
        .map_err(|e| Error::RegistryRead(path.clone(), e))?;
    let registry: Registry = toml::from_str(&content)
        .map_err(|e| Error::RegistryParse(path, e))?;
    Ok(registry)
}

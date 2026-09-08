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
    /// Override the shell command name used for the written completion file.
    /// This is useful when the mise tool name and the user-facing command name
    /// differ, such as tealdeer installing the `tldr` command.
    pub completion_name: Option<String>,
    /// Another mise tool that must be on PATH for the command to work, because
    /// the tool shells out to it to render completions (e.g. fnox needs `usage`).
    pub requires: Option<String>,
    /// The tool ships completion files in its download instead of generating
    /// them. Each shell's value is then the filename to look for, not a command.
    pub bundled: Option<bool>,
}

impl ToolCompletions {
    pub fn is_bundled(&self) -> bool {
        self.bundled.unwrap_or(false)
    }

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
            completion_name: self.completion_name.clone(),
            requires: self.requires.clone(),
            bundled: self.bundled,
        }
    }
}

/// Find a user-provided registry to lay over the built-in one, if there is one.
///
/// The executable's own directory wins over the XDG one; only a single user
/// registry applies.
fn user_registry_content() -> Result<Option<(String, PathBuf)>, Error> {
    let mut candidates = Vec::new();

    if let Ok(exe_path) = std::env::current_exe() {
        candidates.push(exe_path.parent().unwrap().join("registry.toml"));
    }
    if let Some(data_dir) = dirs::data_dir() {
        candidates.push(data_dir.join("mise-completions-sync").join("registry.toml"));
    }

    for path in candidates {
        if path.exists() {
            let content =
                std::fs::read_to_string(&path).map_err(|e| Error::RegistryRead(path.clone(), e))?;
            return Ok(Some((content, path)));
        }
    }

    Ok(None)
}

pub fn load_registry() -> Result<Registry, Error> {
    let user = user_registry_content()?;
    build_registry(
        EMBEDDED_REGISTRY,
        user.as_ref()
            .map(|(content, path)| (content.as_str(), path.clone())),
    )
}

fn parse_raw(content: &str, path_for_error: PathBuf) -> Result<RawRegistry, Error> {
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

    Ok(raw)
}

/// Parse the built-in registry and lay a user registry over it.
///
/// Merging happens before patterns are resolved, so a user entry can reference
/// a built-in pattern, and redefining a pattern reaches the tools that use it.
fn build_registry(embedded: &str, user: Option<(&str, PathBuf)>) -> Result<Registry, Error> {
    let mut raw = parse_raw(embedded, PathBuf::from("<embedded>"))?;

    if let Some((content, path)) = user {
        let overlay = parse_raw(content, path)?;
        raw.patterns.extend(overlay.patterns);
        raw.tools.extend(overlay.tools);
    }

    expand(raw)
}

fn expand(raw: RawRegistry) -> Result<Registry, Error> {
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
    fn test_mdbook_uses_completions_subcommand() {
        // Regression: mdbook was mapped to `generate_shell`, but it has no
        // `generate-shell-completion` subcommand -- it uses `mdbook completions`.
        let registry = load_registry().expect("Failed to load registry");
        let mdbook = registry
            .tools
            .get("mdbook")
            .expect("mdbook should be in registry");
        assert_eq!(
            mdbook.completions.zsh.as_deref(),
            Some("mdbook completions zsh")
        );
    }

    #[test]
    fn test_tools_without_completion_support_are_absent() {
        // gitu and gitui generate no completions at all, so they must not be
        // listed -- an entry here means sync will try and fail every run.
        let registry = load_registry().expect("Failed to load registry");
        assert!(!registry.tools.contains_key("gitu"));
        assert!(!registry.tools.contains_key("gitui"));
    }

    #[test]
    fn test_pitchfork_requires_usage() {
        // Regression: pitchfork renders through `usage`, like fnox, so the bare
        // `standard` pattern failed with "No version is set for shim: usage".
        let registry = load_registry().expect("Failed to load registry");
        let pitchfork = registry
            .tools
            .get("pitchfork")
            .expect("pitchfork should be in registry");
        assert_eq!(pitchfork.completions.requires.as_deref(), Some("usage"));
    }

    #[test]
    fn test_saml2aws_uses_completion_flags() {
        // Regression: saml2aws is kingpin-based and has no `completion` command.
        let registry = load_registry().expect("Failed to load registry");
        let saml2aws = registry
            .tools
            .get("saml2aws")
            .expect("saml2aws should be in registry");
        assert_eq!(
            saml2aws.completions.zsh.as_deref(),
            Some("saml2aws --completion-script-zsh")
        );
        // kingpin has no fish support
        assert_eq!(saml2aws.completions.fish, None);
    }

    #[test]
    fn test_vercel_removed() {
        // vercel dropped its `completion` command; nothing replaced it.
        let registry = load_registry().expect("Failed to load registry");
        assert!(!registry.tools.contains_key("vercel"));
    }

    #[test]
    fn test_hyperfine_is_bundled() {
        // hyperfine ships completion files in its download rather than having a
        // command, so the per-shell values are filenames to find, not commands.
        let registry = load_registry().expect("Failed to load registry");
        let hyperfine = registry
            .tools
            .get("hyperfine")
            .expect("hyperfine should be in registry");
        assert!(hyperfine.completions.is_bundled());
        assert_eq!(hyperfine.completions.zsh.as_deref(), Some("_hyperfine"));
        assert_eq!(
            hyperfine.completions.bash.as_deref(),
            Some("hyperfine.bash")
        );
        assert_eq!(
            hyperfine.completions.fish.as_deref(),
            Some("hyperfine.fish")
        );
    }

    #[test]
    fn test_tealdeer_uses_tldr_completion_name() {
        // tealdeer installs the `tldr` command, so the completion files need to
        // land under `_tldr` even though mise knows the package as `tealdeer`.
        let registry = load_registry().expect("Failed to load registry");
        let tealdeer = registry
            .tools
            .get("tealdeer")
            .expect("tealdeer should be in registry");
        assert!(tealdeer.completions.is_bundled());
        assert_eq!(
            tealdeer.completions.completion_name.as_deref(),
            Some("tldr")
        );
        assert_eq!(tealdeer.completions.zsh.as_deref(), Some("zsh_tealdeer"));
        assert_eq!(tealdeer.completions.bash.as_deref(), Some("bash_tealdeer"));
        assert_eq!(tealdeer.completions.fish.as_deref(), Some("fish_tealdeer"));
    }

    #[test]
    fn test_command_entries_are_not_bundled() {
        let registry = load_registry().expect("Failed to load registry");
        assert!(!registry.tools["yq"].completions.is_bundled());
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

    const BASE: &str = r#"
schema_version = 1

[patterns]
standard = { zsh = "{} completion zsh", bash = "{} completion bash" }

[tools]
builtin = "standard"
override_me = { zsh = "original zsh" }
"#;

    fn overlaid(user: &str) -> Registry {
        build_registry(BASE, Some((user, PathBuf::from("<user>"))))
            .expect("Failed to build registry")
    }

    #[test]
    fn test_user_registry_adds_a_tool_without_losing_builtins() {
        // The whole point: a three-line user file must not switch off the other
        // entries, which is what replacing the registry used to do.
        let registry = overlaid(
            r#"
schema_version = 1

[tools]
graphite-cli = { zsh = "gt completion zsh" }
"#,
        );

        assert_eq!(
            registry.tools["graphite-cli"].completions.zsh.as_deref(),
            Some("gt completion zsh")
        );
        assert_eq!(
            registry.tools["builtin"].completions.zsh.as_deref(),
            Some("builtin completion zsh")
        );
    }

    #[test]
    fn test_user_entry_overrides_builtin() {
        let registry = overlaid(
            r#"
schema_version = 1

[tools]
override_me = { zsh = "user zsh" }
"#,
        );

        assert_eq!(
            registry.tools["override_me"].completions.zsh.as_deref(),
            Some("user zsh")
        );
    }

    #[test]
    fn test_user_entry_can_reference_a_builtin_pattern() {
        // Patterns are resolved after merging, so a user never has to redeclare
        // `standard` just to use it.
        let registry = overlaid(
            r#"
schema_version = 1

[tools]
mytool = "standard"
"#,
        );

        assert_eq!(
            registry.tools["mytool"].completions.zsh.as_deref(),
            Some("mytool completion zsh")
        );
    }

    #[test]
    fn test_user_can_redefine_a_builtin_pattern() {
        let registry = overlaid(
            r#"
schema_version = 1

[patterns]
standard = { zsh = "{} --fixed-completions zsh" }
"#,
        );

        assert_eq!(
            registry.tools["builtin"].completions.zsh.as_deref(),
            Some("builtin --fixed-completions zsh")
        );
    }

    #[test]
    fn test_no_user_registry_leaves_builtins_untouched() {
        let registry = build_registry(BASE, None).expect("Failed to build registry");

        assert_eq!(registry.tools.len(), 2);
        assert_eq!(
            registry.tools["override_me"].completions.zsh.as_deref(),
            Some("original zsh")
        );
    }

    #[test]
    fn test_user_registry_still_needs_a_schema_version() {
        let err = build_registry(
            BASE,
            Some(("[tools]\nfoo = \"standard\"\n", PathBuf::from("<user>"))),
        )
        .expect_err("a user registry without schema_version should be rejected");

        assert!(matches!(err, Error::MissingSchemaVersion));
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
        let registry = build_registry(
            r#"
schema_version = 1

[patterns]
standard = { zsh = "{} completion zsh" }

[tools]
parent = "standard"
child = { provided_by = "parent", zsh = "child completion zsh", bash = "child completion bash" }
"#,
            None,
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

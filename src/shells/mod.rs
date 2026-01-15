// ABOUTME: Shell-specific handling for completion files.
// ABOUTME: Each shell has different naming conventions for completions.

/// Get the completion filename for a given shell and tool
pub fn completion_filename(shell: &str, tool: &str) -> String {
    match shell {
        // ZSH: completions are named _toolname
        "zsh" => format!("_{tool}"),
        // Bash: completions are named toolname or toolname.bash
        "bash" => tool.to_string(),
        // Fish: completions are named toolname.fish
        "fish" => format!("{tool}.fish"),
        _ => tool.to_string(),
    }
}

/// Extract tool name from a completion filename
pub fn tool_from_filename(shell: &str, filename: &str) -> Option<String> {
    match shell {
        "zsh" => filename.strip_prefix('_').map(String::from),
        "bash" => Some(filename.to_string()),
        "fish" => filename.strip_suffix(".fish").map(String::from),
        _ => Some(filename.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zsh_filename() {
        assert_eq!(completion_filename("zsh", "kubectl"), "_kubectl");
        assert_eq!(tool_from_filename("zsh", "_kubectl"), Some("kubectl".to_string()));
    }

    #[test]
    fn test_bash_filename() {
        assert_eq!(completion_filename("bash", "kubectl"), "kubectl");
        assert_eq!(tool_from_filename("bash", "kubectl"), Some("kubectl".to_string()));
    }

    #[test]
    fn test_fish_filename() {
        assert_eq!(completion_filename("fish", "kubectl"), "kubectl.fish");
        assert_eq!(tool_from_filename("fish", "kubectl.fish"), Some("kubectl".to_string()));
    }
}

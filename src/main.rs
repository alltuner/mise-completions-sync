// ABOUTME: CLI entry point for mise-completions-sync.
// ABOUTME: Syncs shell completions for tools managed by mise.

mod registry;
mod shells;
mod sync;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "misecompsync")]
#[command(about = "Sync shell completions for tools managed by mise")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Only sync completions for specific shell(s)
    #[arg(long, short, value_delimiter = ',')]
    shell: Option<Vec<String>>,

    /// Only tools in global mise config files (passes through to `mise ls --global`)
    #[arg(long, short, conflicts_with_all = ["local", "tools"])]
    global: bool,

    /// Only tools in local mise config files (passes through to `mise ls --local`)
    #[arg(long, short, conflicts_with_all = ["global", "tools"])]
    local: bool,

    /// Only tools in current mise config files (passes through to `mise ls --current`)
    #[arg(long, short, conflicts_with = "tools")]
    current: bool,

    /// Specific tool(s) to sync (default: all installed)
    #[arg(value_name = "TOOL")]
    tools: Vec<String>,

    /// Only sync completions for newly installed/updated tools (reads MISE_INSTALLED_TOOLS env var)
    #[arg(long, conflicts_with_all = ["tools", "global", "local", "current"])]
    new_only: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all tools in the registry
    List,
    /// Show completions output directory
    Dir {
        /// Shell to show directory for
        #[arg(default_value = "zsh")]
        shell: String,
    },
    /// Remove completions for tools no longer installed. Use caution with --global/--local/--current — may remove for tools not in scope
    Clean,
    /// Print shell completions for misecompsync itself to stdout
    Completion {
        /// Shell to generate completions for
        shell: Shell,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), sync::Error> {
    let cli = Cli::parse();
    let dirs = sync::CompletionsDirs::from_env();
    let flags = sync::MiseLsFlags::new(cli.global, cli.local, cli.current);

    match cli.command {
        Some(Commands::List) => {
            let registry = registry::load_registry()?;
            println!("Tools with completion support:");
            for tool in registry.tools.keys() {
                println!("  {tool}");
            }
            Ok(())
        }
        Some(Commands::Dir { shell }) => {
            let dir = dirs.get_dir(&shell)?;
            println!("{}", dir.display());
            Ok(())
        }
        Some(Commands::Clean) => sync::clean_stale_completions(&dirs, flags),
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
            Ok(())
        }
        None => {
            let shells = cli
                .shell
                .unwrap_or_else(|| vec!["zsh".to_string(), "bash".to_string(), "fish".to_string()]);
            sync::sync_completions(&dirs, &shells, &cli.tools, flags, cli.new_only)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(args)
    }

    #[test]
    fn test_global_and_local_conflict() {
        // Mirrors `mise ls --global --local`, which mise rejects.
        assert!(parse(&["misecompsync", "--global", "--local"]).is_err());
    }

    #[test]
    fn test_scope_flag_conflicts_with_specific_tools() {
        // Scope flags are a no-op when specific tools are given, so reject the combo.
        assert!(parse(&["misecompsync", "--global", "kubectl"]).is_err());
        assert!(parse(&["misecompsync", "--local", "kubectl"]).is_err());
        assert!(parse(&["misecompsync", "--current", "kubectl"]).is_err());
    }

    #[test]
    fn test_global_and_current_allowed() {
        // --current combines freely with --global (and --local).
        let cli = parse(&["misecompsync", "--global", "--current"]).unwrap();
        assert!(cli.global);
        assert!(cli.current);
        assert!(!cli.local);

        assert!(parse(&["misecompsync", "--local", "--current"]).is_ok());
    }

    #[test]
    fn test_short_flags() {
        let cli = parse(&["misecompsync", "-g"]).unwrap();
        assert!(cli.global);

        let cli = parse(&["misecompsync", "-l"]).unwrap();
        assert!(cli.local);

        let cli = parse(&["misecompsync", "-c"]).unwrap();
        assert!(cli.current);
    }

    #[test]
    fn test_no_flags_defaults() {
        // Default invocation stays valid with no scope flags set.
        let cli = parse(&["misecompsync"]).unwrap();
        assert!(!cli.global);
        assert!(!cli.local);
        assert!(!cli.current);
        assert!(cli.tools.is_empty());
    }

    #[test]
    fn test_new_only_conflicts_with_scope_flags() {
        // --new-only bypasses mise ls, so scope flags are meaningless.
        assert!(parse(&["misecompsync", "--new-only", "--global"]).is_err());
        assert!(parse(&["misecompsync", "--new-only", "--local"]).is_err());
        assert!(parse(&["misecompsync", "--new-only", "--current"]).is_err());
    }

    #[test]
    fn test_new_only_conflicts_with_tools() {
        assert!(parse(&["misecompsync", "--new-only", "kubectl"]).is_err());
    }
}

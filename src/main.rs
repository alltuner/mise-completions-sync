// ABOUTME: CLI entry point for mise-completions-sync.
// ABOUTME: Syncs shell completions for tools managed by mise.

mod registry;
mod shells;
mod sync;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mise-completions-sync")]
#[command(about = "Sync shell completions for tools managed by mise")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Only sync completions for specific shell(s)
    #[arg(long, short, value_delimiter = ',')]
    shell: Option<Vec<String>>,

    /// Specific tool(s) to sync (default: all installed)
    #[arg(value_name = "TOOL")]
    tools: Vec<String>,
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
    /// Remove completions for tools no longer installed
    Clean,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), sync::Error> {
    let cli = Cli::parse();

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
            let dir = sync::get_completions_dir(&shell)?;
            println!("{}", dir.display());
            Ok(())
        }
        Some(Commands::Clean) => sync::clean_stale_completions(),
        None => {
            let shells = cli
                .shell
                .unwrap_or_else(|| vec!["zsh".to_string(), "bash".to_string(), "fish".to_string()]);
            sync::sync_completions(&shells, &cli.tools)
        }
    }
}

# mise-completions-sync

Automatically sync shell completions for tools managed by [mise](https://mise.jdx.dev/).

## The Problem

When mise installs a tool like `kubectl` or `gh`, you don't automatically get shell completions. You'd have to manually run the tool's completion command and configure your shell.

## The Solution

`mise-completions-sync` automatically generates completions for all your mise-installed tools.

## Installation

### Using mise (recommended)

```bash
mise use -g github:alltuner/mise-completions-sync
```

### From GitHub Releases

Download the appropriate binary for your platform from the [releases page](https://github.com/alltuner/mise-completions-sync/releases).

### From Source

```bash
cargo install --git https://github.com/alltuner/mise-completions-sync
```

## Shell Setup

Add the completions directory to your shell configuration.

### ZSH

Add to `~/.zshrc` **before** `compinit`:

```zsh
fpath=(${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/zsh $fpath)
```

### Bash

Add to `~/.bashrc`:

```bash
for f in ${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/bash/*; do
  [[ -f "$f" ]] && source "$f"
done
```

### Fish

Add to `~/.config/fish/config.fish`:

```fish
set -gx fish_complete_path $fish_complete_path ~/.local/share/mise-completions/fish
```

## Automatic Sync

Set up a mise hook to automatically sync completions when tools are installed:

```bash
mkdir -p ~/.config/mise && cat >> ~/.config/mise/config.toml << 'EOF'

[hooks]
postinstall = "mise-completions-sync"
EOF
```

## Initial Sync

After setup, run the initial sync:

```bash
mise-completions-sync
```

## Usage

```bash
# Sync completions for all installed tools (all shells)
mise-completions-sync

# Sync only for specific shell
mise-completions-sync --shell zsh

# Sync specific tool(s)
mise-completions-sync kubectl helm

# List tools with completion support
mise-completions-sync list

# Show completion directory for a shell
mise-completions-sync dir zsh

# Clean up completions for uninstalled tools
mise-completions-sync clean
```

## License

MIT

---

Built at [All Tuner Labs](https://alltuner.com) by [David Poblador i Garcia](https://davidpoblador.com)

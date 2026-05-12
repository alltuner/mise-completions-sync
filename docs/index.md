# mise-completions-sync

Automatically sync shell completions for tools managed by [mise](https://mise.jdx.dev/).

## The Problem

When mise installs a tool like `kubectl` or `gh`, you don't automatically get shell completions. You'd have to manually run the tool's completion command and configure your shell.

## The Solution

mise-completions-sync automatically generates completions for all your mise-installed tools. The package is named `mise-completions-sync`; the binary it installs is `misecompsync` (mise reserves `mise-*` binary names for itself, so the shim can't forward to a binary that starts with `mise-`).

## Installation

### Homebrew

```bash
brew install alltuner/tap/mise-completions-sync
```

### Cargo

```bash
cargo install mise-completions-sync
```

### Using mise

```bash
mise use -g github:alltuner/mise-completions-sync
```

### From GitHub Releases

Download the appropriate binary for your platform from the [releases page](https://github.com/alltuner/mise-completions-sync/releases).

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

## Updating

### Homebrew

```bash
brew upgrade mise-completions-sync
```

### Cargo

```bash
cargo install --force mise-completions-sync
```

### mise

```bash
mise upgrade github:alltuner/mise-completions-sync
```

Or pin a specific version with mise:

```bash
mise use -g github:alltuner/mise-completions-sync@0.5.1
```

## Automatic Sync

Set up a mise hook to automatically sync completions when tools are installed:

```bash
mkdir -p ~/.config/mise && cat >> ~/.config/mise/config.toml << 'EOF'

[hooks]
postinstall = "misecompsync"
EOF
```

## Initial Sync

After setup, run the initial sync:

```bash
misecompsync
```

## Usage

```bash
# Sync completions for all installed tools (all shells)
misecompsync

# Sync only for specific shell
misecompsync --shell zsh

# Sync specific tool(s)
misecompsync kubectl helm

# List tools with completion support
misecompsync list

# Show completion directory for a shell
misecompsync dir zsh

# Clean up completions for uninstalled tools
misecompsync clean
```

## License

MIT

---

Built at [All Tuner Labs](https://alltuner.com) by [David Poblador i Garcia](https://davidpoblador.com)

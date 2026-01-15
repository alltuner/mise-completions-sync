# mise-completions-sync

Automatically sync shell completions for tools managed by [mise](https://mise.jdx.dev/).

## The Problem

When mise installs a tool like `kubectl` or `gh`, you don't automatically get shell completions. You'd have to manually run the tool's completion command and configure your shell.

## The Solution

`mise-completions-sync` automatically generates completions for all your mise-installed tools.

## Quick Setup

### 1. Install the tool

```bash
mise use -g ubi:alltuner/mise-completions-sync
```

### 2. Add to your shell config

**ZSH** (add to `~/.zshrc` BEFORE `compinit`):
```zsh
fpath=(${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/zsh $fpath)
```

**Bash** (add to `~/.bashrc`):
```bash
for f in ${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/bash/*; do
  [[ -f "$f" ]] && source "$f"
done
```

**Fish** (add to `~/.config/fish/config.fish`):
```fish
set -gx fish_complete_path $fish_complete_path ~/.local/share/mise-completions/fish
```

### 3. Set up automatic sync

```bash
mise settings set hooks.postinstall "mise-completions-sync"
```

### 4. Initial sync

```bash
mise-completions-sync
```

That's it! From now on, completions will be synced automatically whenever you install tools with mise.

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

## How It Works

1. Gets list of installed tools via `mise ls --installed --json`
2. Looks up each tool in the built-in registry
3. Runs the tool's completion command (e.g., `kubectl completion zsh`)
4. Saves output to `~/.local/share/mise-completions/<shell>/`

## Supported Tools

See [registry.toml](registry.toml) for the full list. Popular tools include:

- **Kubernetes**: kubectl, helm, k9s, kind, minikube, kustomize, argocd, flux
- **Cloud**: aws, gcloud, terraform, vault
- **Development**: mise, just, task, docker, gh, glab
- **Rust**: rustup, cargo, deno, bat, eza, fd, rg, starship
- **Python**: uv, ruff, poetry, pdm, pipx
- **Node**: npm, pnpm, yarn, bun

## Adding Tools

If a tool you use isn't in the registry, you can:

1. Check if the tool supports completions (usually `tool completion --help`)
2. Add an entry to `registry.toml`
3. Submit a PR

## Development

```bash
# Install dev tools
cd ~/repos/mise-completions-sync
mise install

# Build
mise run build

# Test
mise run test

# Generate registry entries from mise's registry
mise run generate-registry
```

## License

MIT

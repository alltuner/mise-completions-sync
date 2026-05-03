# mise-completions-sync

Automatically sync shell completions for tools managed by [mise](https://mise.jdx.dev/).

## Installation

**Homebrew:**
```bash
brew install alltuner/tap/mise-completions-sync
```

**Cargo:**
```bash
cargo install mise-completions-sync
```

**mise:**
```bash
mise use -g github:alltuner/mise-completions-sync
```

Or download a prebuilt binary from [releases](https://github.com/alltuner/mise-completions-sync/releases).

## Shell Setup

Add the completions directory to your shell config.

**ZSH** - add to `~/.zshrc` before `compinit`:
```zsh
fpath=(${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/zsh $fpath)
```

**Bash** - add to `~/.bashrc`:
```bash
for f in ${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/bash/*; do
  [[ -f "$f" ]] && source "$f"
done
```

**Fish** - add to `~/.config/fish/config.fish`:
```fish
set -gx fish_complete_path $fish_complete_path ~/.local/share/mise-completions/fish
```

## Usage

```bash
# Sync completions for all installed tools
mise-completions-sync

# Sync only for specific shell
mise-completions-sync --shell zsh

# Sync specific tools
mise-completions-sync kubectl helm

# List supported tools
mise-completions-sync list

# Clean up completions for uninstalled tools
mise-completions-sync clean
```

## Updating

**Homebrew:**
```bash
brew upgrade mise-completions-sync
```

**Cargo:**
```bash
cargo install --force mise-completions-sync
```

**mise:**
```bash
mise upgrade github:alltuner/mise-completions-sync
```

Or pin a specific version with mise:
```bash
mise use -g github:alltuner/mise-completions-sync@0.5.1
```

## Automatic Sync

Set up a mise hook to sync completions when tools are installed:

```bash
mkdir -p ~/.config/mise && cat >> ~/.config/mise/config.toml << 'EOF'

[hooks]
postinstall = "mise-completions-sync"
EOF
```

## Documentation

See the [full documentation](https://alltuner.github.io/mise-completions-sync/) for supported tools and more details.

## License

MIT

---

Built at [All Tuner Labs](https://alltuner.com) by [David Poblador i Garcia](https://davidpoblador.com)

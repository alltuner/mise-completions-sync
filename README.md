<h1 align="center">mise-completions-sync</h1>

<p align="center">
  <strong>Sync shell completions for tools managed by mise.</strong><br>
  One command keeps Bash, Zsh, and Fish completions current as <a href="https://mise.jdx.dev/">mise</a> installs and removes tools.
</p>

<p align="center">
  <a href="https://alltuner.github.io/mise-completions-sync/">Docs</a> &middot;
  <a href="https://alltuner.com/sponsor">Sponsor</a>
</p>

<p align="center">
  <img src="https://img.shields.io/crates/v/mise-completions-sync?color=5B2333" alt="crates.io">
  <img src="https://img.shields.io/github/license/alltuner/mise-completions-sync?color=5B2333" alt="License">
  <img src="https://img.shields.io/github/stars/alltuner/mise-completions-sync?color=5B2333" alt="Stars">
</p>

---

## Get Started

Install via Homebrew, Cargo, mise, or grab a [prebuilt binary](https://github.com/alltuner/mise-completions-sync/releases):

```bash
brew install alltuner/tap/mise-completions-sync
```

```bash
cargo install mise-completions-sync
```

```bash
mise use -g github:alltuner/mise-completions-sync
```

Then add the completions directory to your shell config:

| Shell | Where to add | Snippet |
|---|---|---|
| Zsh  | `~/.zshrc` (before `compinit`) | `fpath=(${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/zsh $fpath)` |
| Bash | `~/.bashrc` | `for f in ${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/bash/*; do [[ -f "$f" ]] && source "$f"; done` |
| Fish | `~/.config/fish/config.fish` | `set -gx fish_complete_path $fish_complete_path ~/.local/share/mise-completions/fish` |

---

## What is mise-completions-sync?

mise installs language and tool versions per project, but it doesn't touch your shell completion files. As versions change, completions get stale or missing. `mise-completions-sync` walks your installed mise tools, generates the right completion file for each one (Bash, Zsh, Fish), and writes them under `${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/<shell>/`. Run it once after installing tools, or wire it into a mise post-install hook.

## Usage

```bash
# Sync completions for all installed tools
mise-completions-sync

# Sync only for a specific shell
mise-completions-sync --shell zsh

# Sync specific tools
mise-completions-sync kubectl helm

# List supported tools
mise-completions-sync list

# Clean up completions for uninstalled tools
mise-completions-sync clean
```

### Automatic sync

Wire it into a mise post-install hook so new tool installs get completions automatically:

```bash
mkdir -p ~/.config/mise && cat >> ~/.config/mise/config.toml << 'EOF'

[hooks]
postinstall = "mise-completions-sync"
EOF
```

## Updating

```bash
# Homebrew
brew upgrade mise-completions-sync

# Cargo
cargo install --force mise-completions-sync

# mise
mise upgrade github:alltuner/mise-completions-sync

# Pin a specific version with mise
mise use -g github:alltuner/mise-completions-sync@0.5.1
```

## Documentation

Full docs at [alltuner.github.io/mise-completions-sync](https://alltuner.github.io/mise-completions-sync/) — supported tools, completion details, and troubleshooting.

## Support the project

mise-completions-sync is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://www.alltuner.com/).

If this project was useful to you, [consider supporting its development](https://alltuner.com/sponsor).

## License

[MIT](LICENSE)

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>

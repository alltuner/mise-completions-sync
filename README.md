<p align="center">
  <img src="https://brand.alltuner.com/logos/mise-completions-sync/horizontal.png" alt="mise-completions-sync" width="500">
</p>

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

The installed binary is named `misecompsync` (mise reserves `mise-*` names for itself, so the shim can't forward to a binary that starts with `mise-`).

Then add the completions directory to your shell config:

| Shell | Where to add | Snippet |
|---|---|---|
| Zsh  | `~/.zshrc` (before `compinit`) | `fpath=(${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/zsh $fpath)` |
| Bash | `~/.bashrc` | `for f in ${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/bash/*; do [[ -f "$f" ]] && source "$f"; done` |
| Fish | `~/.config/fish/config.fish` | `set -gx fish_complete_path $fish_complete_path ~/.local/share/mise-completions/fish` |

---

## What is mise-completions-sync?

mise installs language and tool versions per project, but it doesn't touch your shell completion files. As versions change, completions get stale or missing. mise-completions-sync walks your installed mise tools, generates the right completion file for each one (Bash, Zsh, Fish), and writes them under `${XDG_DATA_HOME:-$HOME/.local/share}/mise-completions/<shell>/`. Run it once after installing tools, or wire it into a mise post-install hook.

## Usage

```bash
# Sync completions for all installed tools
misecompsync

# Sync only for a specific shell
misecompsync --shell zsh

# Sync specific tools
misecompsync kubectl helm

# List supported tools
misecompsync list

# Clean up completions for uninstalled tools
misecompsync clean

# Print misecompsync's own completions to stdout
misecompsync completion zsh
```

### Additional Flags

By default, completions are synced for every installed tool. You can narrow the set with
the following scope flags that `mise ls` accepts — they're passed straight through:

```bash
# Only tools in global mise config files
misecompsync --global   # or -g

# Only tools in local (project) mise config files
misecompsync --local    # or -l

# Only tools currently in mise config files (not just with `mise install`)
misecompsync --current  # or -c
```

* `--global` and `--local` are mutually exclusive (same as `mise ls`)
* Scope flags also apply to `clean` — **caution**: `misecompsync --global clean` would remove completions for tools _not_ in the global config, which may include locally-installed tools if they both use the same `MISE_COMPLETIONS_SYNC_HOME`.
* Scope flags conflict with explicit tool args and `--new-only`

### Automatic sync

Wire it into a mise post-install hook so new tool installs get completions automatically:

```bash
mkdir -p ~/.config/mise && cat >> ~/.config/mise/config.toml << 'EOF'

[hooks]
postinstall = "misecompsync"
EOF
```

## Custom Output Dirs

By default, completions are synced to `$XDG_DATA_HOME/mise-completions/<shell>`. However, you can override the output directories using environment variables:

```shell
# Override default base output directory
export MISE_COMPLETIONS_SYNC_HOME="$XDG_DATA_HOME/custom-vendor-completions"
```

Or you can override output targets on a per-shell basis (these take precedence over the base override above):

```shell
# Bash completions to standard bash location
export MISE_COMPLETIONS_SYNC_BASH_DIR="$XDG_DATA_HOME/bash-completion/completions"

# ZSH completions to standard zsh location
export MISE_COMPLETIONS_SYNC_ZSH_DIR="$XDG_DATA_HOME/zsh/site-functions"

# Fish completions to standard fish locations.
# (pick one or the other, both are autoloaded by fish)
# export MISE_COMPLETIONS_SYNC_FISH_DIR="$XDG_CONFIG_HOME/fish/completions"
export MISE_COMPLETIONS_SYNC_FISH_DIR="$XDG_DATA_HOME/fish/vendor_completions.d"
```

Note: Target directories will be created if they don't already exist. Don't forget to update your shell setup above.

If you want to only generate completions for newly installed or updated tools, you can add the flag `--new-only`:

```toml
[hooks]
postinstall = "misecompsync --new-only"
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

## License

[MIT](LICENSE)

## Support the project

mise-completions-sync is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://www.alltuner.com/).

If this project was useful to you, [consider supporting its development](https://alltuner.com/sponsor).

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>

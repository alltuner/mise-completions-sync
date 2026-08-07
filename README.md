<p align="center">
  <img src="https://brand.alltuner.com/logos/mise-completions-sync/horizontal.png" alt="mise-completions-sync" width="500">
</p>

<p align="center">
  <strong>Sync shell completions for tools managed by mise.</strong><br>
  One command keeps Bash, Zsh, and Fish completions current as <a href="https://mise.jdx.dev/">mise</a> installs and removes tools.
</p>

<p align="center">
  <a href="https://mise-completions.alltuner.com/">Docs</a> &middot;
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

# Sync a tool and its direct companion binaries
misecompsync --children uv

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

### Companion binaries

By default, explicitly named tools are parent-only: `misecompsync uv` syncs
only `uv`. Use `misecompsync --children uv` to also sync direct companion
binaries that the registry identifies as provided by `uv`. Expansion is
downward and one hop only. With multiple explicit tools, misecompsync syncs
the sorted, deduplicated union of those tools and their direct children.

Automatic sync and `--new-only` are unchanged: they include a companion binary
when its provider is installed.

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

## Custom Registry

The list of supported tools is built into the binary, but you don't have to wait
for a release (or send a PR) to add your own. Drop a `registry.toml` at
`$XDG_DATA_HOME/mise-completions-sync/registry.toml`, or next to the
`misecompsync` executable, and it is laid on top of the built-in registry:

```toml
schema_version = 1

[tools]
# a tool the built-in registry doesn't cover
graphite-cli = { zsh = "gt completion zsh", bash = "gt completion bash" }

# built-in patterns are available to your own entries
mytool = "standard"

# override a built-in entry
yq = { zsh = "yq shell-completion zsh" }
```

Your entries are merged with the built-in ones rather than replacing them, so a
short file like the above adds `graphite-cli` and `mytool` and changes `yq`,
while every other tool keeps working. `schema_version` is required.

If an entry turns out to be generally useful, [open a PR](https://github.com/alltuner/mise-completions-sync/blob/main/registry.toml)
so everyone gets it.

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

Full docs at [mise-completions.alltuner.com](https://mise-completions.alltuner.com/) — supported tools, completion details, and troubleshooting.

## License

[MIT](LICENSE)

## Support the project

mise-completions-sync is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://alltuner.com/).

If this project was useful to you, [consider supporting its development](https://alltuner.com/sponsor).

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>

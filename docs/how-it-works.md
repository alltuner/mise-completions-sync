# How It Works

mise-completions-sync follows a simple process to generate shell completions:

1. **Discover installed tools** - Gets list of installed tools via `mise ls --installed --json` (or optional flags `--global`, `--local`, `--current`)
2. **Look up registry entries** - Each tool is matched against the built-in registry
3. **Generate completions** - Runs the tool's completion command (e.g., `kubectl completion zsh`)
4. **Save output** - Writes completions to `~/.local/share/mise-completions/<shell>/`

## Registry

The registry (`registry.toml`) maps each tool to the command that prints its
completion script. Most tools share a handful of conventions, so those live in
`[patterns]`, where `{}` stands in for the tool name:

```toml
[patterns]
standard = { zsh = "{} completion zsh", bash = "{} completion bash", fish = "{} completion fish" }

[tools]
kubectl = "standard"
```

A tool whose command doesn't fit a pattern spells it out per shell. Omitting a
shell means the tool doesn't support it:

```toml
[tools]
npm = { zsh = "npm completion", bash = "npm completion" }
```

Two optional fields cover tools that need more than a command:

- `requires` names another mise tool that must be on PATH while generating,
  for tools that shell out to a helper. `fnox` renders through `usage`:

  ```toml
  fnox = { requires = "usage", zsh = "fnox completion zsh" }
  ```

- `provided_by` marks a binary that arrives as part of another mise tool rather
  than being one itself, like `uvx` from `uv`. Its completions are generated
  whenever the provider is installed.

Commands run inside `mise x <tool> -- …`, so they resolve against the version
mise has installed rather than whatever is on your PATH.

## Custom Registry

The built-in registry is embedded in the binary, but you can extend it. If a
`registry.toml` exists next to the executable, or at
`$XDG_DATA_HOME/mise-completions-sync/registry.toml`, it is laid **on top of**
the built-in one rather than replacing it.

```toml
schema_version = 1

[tools]
# a tool the built-in registry doesn't know about
graphite-cli = { zsh = "gt completion zsh", bash = "gt completion bash" }

# built-in patterns are available to your entries
mytool = "standard"

# and you can override a built-in entry
yq = { zsh = "yq shell-completion zsh" }
```

Merging happens before patterns are resolved, so your entries can use the
built-in patterns, and redefining a pattern reaches every tool that references
it. `schema_version` is required. Entries can be added or overridden; there is
no way to remove a built-in entry.

The executable's directory takes precedence over the XDG location, and only one
user registry applies.

## Output Locations

By default, completions are saved to `$XDG_DATA_HOME/mise-completions/<shell>`.

Each tool gets its own completion file named `_<tool>` (for zsh) or `<tool>.bash`/`<tool>.fish` for other shells.

You can override the default output directory using the `MISE_COMPLETIONS_SYNC_HOME` environment variable:

```shell
export MISE_COMPLETIONS_SYNC_HOME="$XDG_DATA_HOME/custom-vendor-completions"

misecompsync kubectl
#  [fish] -> ~/.local/share/custom-vendor-completions/fish/kubectl.fish
#  [zsh]  -> ~/.local/share/custom-vendor-completions/zsh/_kubectl
#  [bash] -> ~/.local/share/custom-vendor-completions/bash/kubectl
```

Or you can override output directories for one (or more) shells (e.g., `MISE_COMPLETIONS_SYNC_{SHELL}_DIR`)

```shell
export MISE_COMPLETIONS_SYNC_FISH_DIR="$XDG_CONFIG_HOME/fish/completions"

misecompsync kubectl
#  [fish] -> ~/.config/fish/completions/kubectl.fish
#  [zsh]  -> ~/.local/share/mise-completions/zsh/_kubectl
#  [bash] -> ~/.local/share/mise-completions/bash/kubectl
```

Or a shell dir with base dir default (shell takes precedence):

```shell
export MISE_COMPLETIONS_SYNC_HOME="$XDG_DATA_HOME/custom-vendor-completions"
export MISE_COMPLETIONS_SYNC_ZSH_DIR="$XDG_DATA_HOME/zsh/site-functions"

misecompsync kubectl
#  [fish] -> ~/.local/share/custom-vendor-completions/fish/kubectl.fish
#  [zsh]  -> ~/.local/share/zsh/site-functions/_kubectl
#  [bash] -> ~/.local/share/custom-vendor-completions/bash/kubectl
```

Note: Target directories will be created if they don't already exist.

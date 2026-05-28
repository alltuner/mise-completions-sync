# How It Works

mise-completions-sync follows a simple process to generate shell completions:

1. **Discover installed tools** - Gets list of installed tools via `mise ls --installed --json`
2. **Look up registry entries** - Each tool is matched against the built-in registry
3. **Generate completions** - Runs the tool's completion command (e.g., `kubectl completion zsh`)
4. **Save output** - Writes completions to `~/.local/share/mise-completions/<shell>/`

## Registry

The registry (`registry.toml`) contains patterns for generating completions. Each entry specifies:

- The tool name (matching mise's tool name)
- A format pattern for the completion command
- Which shells are supported

Example entry:

```toml
[tools.kubectl]
format = "{bin} completion {shell}"
shells = ["zsh", "bash", "fish"]
```

The format pattern supports these placeholders:

- `{bin}` - The tool's binary path
- `{shell}` - The target shell (zsh, bash, fish)

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

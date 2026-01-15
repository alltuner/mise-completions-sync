# mise-completions-sync

Automatically sync shell completions for tools managed by [mise](https://mise.jdx.dev/).

## The Problem

When mise installs a tool like `kubectl` or `gh`, you don't automatically get shell completions. You'd have to manually run the tool's completion command and configure your shell.

## The Solution

`mise-completions-sync` automatically generates completions for all your mise-installed tools.

## How It Works

1. Gets list of installed tools via `mise ls --installed --json`
2. Looks up each tool in the built-in registry
3. Runs the tool's completion command (e.g., `kubectl completion zsh`)
4. Saves output to `~/.local/share/mise-completions/<shell>/`

---

Built at [All Tuner Labs](https://alltuner.com) by [David Poblador i Garcia](https://davidpoblador.com)

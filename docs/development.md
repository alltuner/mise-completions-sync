# Development

## Prerequisites

Install development tools:

```bash
cd ~/repos/mise-completions-sync
mise install
```

This installs:

- `rust` - For building the project
- `uv` - For running Python scripts
- `prek` - For pre-commit hooks

## Tasks

The project uses mise tasks for common operations. Run `mise tasks` to see all available tasks.

| Task | Description |
|------|-------------|
| `mise run build` | Build the release binary |
| `mise run test` | Run tests |
| `mise run lint` | Run clippy lints |
| `mise run format` | Format code |
| `mise run install-dev` | Install locally for testing |
| `mise run install-hooks` | Install pre-commit hooks |
| `mise run generate-registry` | Generate registry.toml from mise's registry |
| `mise run validate-registry` | Validate registry entries against installed tools |
| `mise run docs-tools` | Generate tools documentation from registry |

## Adding Tools to the Registry

If a tool you use isn't in the registry:

1. Check if the tool supports completions (usually `tool completion --help`)
2. Add an entry to `registry.toml`
3. Run `mise run validate-registry` to verify it works
4. Submit a PR

### Registry Entry Format

```toml
[tools.mytool]
format = "{bin} completion {shell}"
shells = ["zsh", "bash", "fish"]
```

Some tools use different subcommands or flags:

```toml
# Tool uses --shell flag
[tools.othertool]
format = "{bin} completions --shell {shell}"
shells = ["zsh", "bash"]
```

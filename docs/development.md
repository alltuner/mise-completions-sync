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
| `mise run audit-registry` | Install every registry tool and verify it (slow) |
| `mise run docs-tools` | Generate tools documentation from registry |

## Adding Tools to the Registry

If a tool you use isn't in the registry:

1. Check if the tool supports completions (usually `tool completion --help`)
2. Add an entry to `registry.toml`
3. Run `mise run validate-registry` to verify it works
4. Submit a PR

### Registry Entry Format

Most tools follow one of the shared conventions in `[patterns]`, where `{}`
stands in for the tool name:

```toml
[tools]
mytool = "standard"      # {} completion <shell>
```

A tool that doesn't fit a pattern spells out each shell. Omitting a shell means
the tool doesn't support it:

```toml
[tools]
othertool = { zsh = "othertool completions --shell zsh", bash = "othertool completions --shell bash" }
```

Optional fields cover the awkward cases: `requires` names a helper binary the
command needs on PATH, `provided_by` marks a binary that ships inside another
mise tool, and `bundled` means the tool ships completion files rather than
generating them (each shell's value is then the filename to look for). See
[How It Works](how-it-works.md) for details.

### Registry Audit

`mise run validate-registry` only checks tools that happen to be installed on
the machine running it, so an entry that is wrong for everyone still passes. The
`Registry Audit` workflow runs weekly, installs every tool in the registry and
verifies each command, and opens an issue when something is broken. Tools that
won't install on the runner are reported separately — that says nothing about
whether the entry is correct.

Run it locally with `mise run audit-registry`. It installs every tool in the
registry, so expect it to be slow and to leave a lot behind.

A few tools need a working environment — not just an install — before they will
print a completion script, so the audit can't judge them. `kubeseal` wants a
reachable cluster config even to emit a static file. Those entries carry
`audit_skip` with a reason and are reported as unverified rather than broken:

```toml
kubeseal = { audit_skip = "needs a reachable cluster config even to print completions", zsh = "kubeseal completion zsh" }
```

Use it sparingly: it silences the only check that would catch the entry going
stale. `mise run validate-registry` still tests these normally, since a
developer running it locally usually does have the environment.

#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
# ABOUTME: Generates TOOLS.md with supported and pending tools for tracking.
# ABOUTME: Helps Claude Code sessions continue adding support for more tools.
"""
Generates TOOLS.md with supported and pending tool lists.

Usage: uv run scripts/generate-tools-status.py > TOOLS.md
"""

import json
import subprocess
import tomllib
from pathlib import Path

# Tools known to NOT support shell completions (validated manually)
NO_COMPLETION_SUPPORT = {
    "air",
    "aws",  # Uses runtime completer, not script generator
    "biome",
    "consul",  # HashiCorp autocomplete-install pattern
    "croc",
    "dust",
    "evans",
    "eza",
    "nomad",  # HashiCorp autocomplete-install pattern
    "terraform",  # HashiCorp autocomplete-install pattern
    "tokei",
    "vault",  # HashiCorp autocomplete-install pattern
    "wrangler",
    "yarn",  # Requires project context
}

# Common completion patterns to check
COMPLETION_HINTS = [
    "completion",
    "completions",
    "--completions",
    "generate-completion",
    "shell-completion",
]


def get_mise_registry() -> dict[str, dict]:
    """Fetch tool metadata from mise's registry."""
    result = subprocess.run(
        ["mise", "registry", "--json"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return {}

    try:
        data = json.loads(result.stdout)
        return {item["short"]: item for item in data}
    except (json.JSONDecodeError, KeyError):
        return {}


def load_supported_tools() -> set[str]:
    """Load tools from our registry.toml."""
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        registry = tomllib.load(f)
    return set(registry.get("tools", {}).keys())


def get_github_url(backends: list[str]) -> str | None:
    """Extract GitHub URL from backend info if possible."""
    for backend in backends:
        if backend.startswith("aqua:"):
            parts = backend.replace("aqua:", "").split("/")
            if len(parts) >= 2:
                return f"https://github.com/{parts[0]}/{parts[1]}"
    return None


def main():
    supported = load_supported_tools()
    mise_registry = get_mise_registry()

    print("# Tool Support Status")
    print()
    print("This document tracks shell completion support status for tools in mise's registry.")
    print("Use this to identify tools that could be added to mise-completions-sync.")
    print()
    print("## Supported Tools")
    print()
    print(f"Currently **{len(supported)} tools** have completion support.")
    print()
    print("See [docs/tools.md](docs/tools.md) for the full list with shell support details.")
    print()

    # Find candidate tools (popular tools that might have completions)
    print("## Pending Tools (Candidates)")
    print()
    print("These tools are in mise's registry but not yet supported. They may have completion support.")
    print()
    print("| Tool | Description | Status |")
    print("|------|-------------|--------|")

    # Get all tools from mise registry, filter out supported and known-no-support
    candidates = []
    for tool, meta in sorted(mise_registry.items()):
        if tool in supported:
            continue
        if tool in NO_COMPLETION_SUPPORT:
            continue
        # Skip aliases (tools that point to another tool)
        if meta.get("aliases") and tool in meta.get("aliases", []):
            continue

        description = meta.get("description", "")
        if len(description) > 50:
            description = description[:47] + "..."

        github_url = get_github_url(meta.get("backends", []))
        tool_display = f"[{tool}]({github_url})" if github_url else tool

        candidates.append((tool_display, description))

    # Show first 50 candidates
    for tool_display, description in candidates[:50]:
        print(f"| {tool_display} | {description} | Needs testing |")

    if len(candidates) > 50:
        print()
        print(f"*...and {len(candidates) - 50} more tools in mise registry*")

    print()
    print("## Tools Without Completion Support")
    print()
    print("These tools have been tested and confirmed to NOT output shell completion scripts:")
    print()
    for tool in sorted(NO_COMPLETION_SUPPORT):
        meta = mise_registry.get(tool, {})
        description = meta.get("description", "")
        if len(description) > 50:
            description = description[:47] + "..."
        print(f"- **{tool}**: {description}")

    print()
    print("## How to Add a Tool")
    print()
    print("1. Check if the tool supports shell completions:")
    print("   ```bash")
    print("   mise x <tool> -- <tool> completion --help")
    print("   mise x <tool> -- <tool> --help | grep -i complet")
    print("   ```")
    print()
    print("2. Find the correct completion command pattern")
    print()
    print("3. Add entry to `registry.toml`:")
    print("   ```toml")
    print("   # If it matches an existing pattern:")
    print('   newtool = "standard"  # for: newtool completion <shell>')
    print()
    print("   # Or with explicit commands:")
    print('   newtool = { zsh = "newtool completions zsh", bash = "newtool completions bash" }')
    print("   ```")
    print()
    print("4. Test the entry:")
    print("   ```bash")
    print("   uv run scripts/validate-registry.py --installed-only")
    print("   ```")
    print()
    print("5. Submit a PR")


if __name__ == "__main__":
    main()

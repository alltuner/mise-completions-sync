#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
# ABOUTME: Generates the tools documentation from registry.toml.
# ABOUTME: Enriches output with metadata from mise's registry (descriptions, etc).
"""
Generates the tools documentation from registry.toml.

Usage: uv run scripts/generate-tools-docs.py > docs/tools.md
"""

import json
import subprocess
import tomllib
from pathlib import Path


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
        # Convert list to dict keyed by short name
        return {item["short"]: item for item in data}
    except (json.JSONDecodeError, KeyError):
        return {}


def get_github_url(backends: list[str]) -> str | None:
    """Extract GitHub URL from backend info if possible."""
    for backend in backends:
        if backend.startswith("aqua:"):
            # aqua format: aqua:owner/repo or aqua:owner/repo/cmd
            parts = backend.replace("aqua:", "").split("/")
            if len(parts) >= 2:
                return f"https://github.com/{parts[0]}/{parts[1]}"
    return None


def main():
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        registry = tomllib.load(f)

    patterns = registry.get("patterns", {})
    tools = registry.get("tools", {})
    mise_registry = get_mise_registry()

    print("# Supported Tools")
    print()
    print("The following tools have shell completion support in mise-completions-sync.")
    print()
    print("| Tool | Description | ZSH | Bash | Fish |")
    print("|------|-------------|-----|------|------|")

    for tool in sorted(tools.keys()):
        config = tools[tool]

        if isinstance(config, str):
            # Pattern reference - look up the pattern
            pattern = patterns.get(config, {})
            zsh = "✓" if pattern.get("zsh") else ""
            bash = "✓" if pattern.get("bash") else ""
            fish = "✓" if pattern.get("fish") else ""
        else:
            # Explicit commands dict
            zsh = "✓" if config.get("zsh") else ""
            bash = "✓" if config.get("bash") else ""
            fish = "✓" if config.get("fish") else ""

        # Get metadata from mise registry
        meta = mise_registry.get(tool, {})
        description = meta.get("description", "")
        # Truncate long descriptions
        if len(description) > 50:
            description = description[:47] + "..."

        # Try to create a linked tool name
        github_url = get_github_url(meta.get("backends", []))
        if github_url:
            tool_display = f"[{tool}]({github_url})"
        else:
            tool_display = tool

        print(f"| {tool_display} | {description} | {zsh} | {bash} | {fish} |")

    print()
    print(f"**Total: {len(tools)} tools**")
    print()
    print("## Shell Support Legend")
    print()
    print("- **✓** = Full completion support")
    print("- Empty = Not supported by the tool for this shell")
    print()
    print("## Adding New Tools")
    print()
    print("If a tool you use isn't listed:")
    print()
    print("1. Check if the tool supports shell completions (`tool completion --help`)")
    print("2. Add an entry to `registry.toml` using an existing pattern or explicit commands")
    print("3. Test with `uv run scripts/validate-registry.py --installed-only`")
    print("4. Submit a PR")


if __name__ == "__main__":
    main()

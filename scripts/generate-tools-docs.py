#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""
Generates the tools documentation from registry.toml.

Usage: uv run scripts/generate-tools-docs.py > docs/tools.md
"""

import tomllib
from pathlib import Path


def main():
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        registry = tomllib.load(f)

    patterns = registry.get("patterns", {})
    tools = registry.get("tools", {})

    print("# Supported Tools")
    print()
    print("The following tools have completion support in mise-completions-sync.")
    print()
    print("| Tool | ZSH | Bash | Fish |")
    print("|------|-----|------|------|")

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

        print(f"| {tool} | {zsh} | {bash} | {fish} |")

    print()
    print(f"**Total: {len(tools)} tools**")
    print()
    print("## Adding Tools")
    print()
    print("If a tool you use isn't in the registry:")
    print()
    print("1. Check if the tool supports completions (usually `tool completion --help`)")
    print("2. Add an entry to `registry.toml`")
    print("3. Submit a PR")


if __name__ == "__main__":
    main()

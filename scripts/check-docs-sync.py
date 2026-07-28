#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
# ABOUTME: Checks that docs/tools.md lists exactly the tools in registry.toml.
# ABOUTME: Compares tool names only, so upstream description changes never fail it.
"""
Checks docs/tools.md is in sync with registry.toml.

Usage: uv run scripts/check-docs-sync.py

Compares the set of tool names in both files and exits non-zero if they
differ. Descriptions and links come from `mise registry` and drift on their
own schedule, so they are deliberately not compared -- regenerate with
`mise run docs-tools` to fix a reported mismatch.
"""

import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).parent.parent

# Table rows look like "| tool | desc | ✓ | ✓ | ✓ |" where the tool cell is
# either a bare name or a markdown link.
ROW = re.compile(r"^\|\s*(?:\[([^\]]+)\]\([^)]*\)|([^|\s]+))\s*\|")


def registry_tools() -> set[str]:
    with open(ROOT / "registry.toml", "rb") as f:
        return set(tomllib.load(f).get("tools", {}).keys())


def documented_tools() -> set[str]:
    tools = set()
    for line in (ROOT / "docs" / "tools.md").read_text().splitlines():
        if line.startswith("|-") or line.startswith("| Tool"):
            continue
        match = ROW.match(line)
        if match:
            tools.add(match.group(1) or match.group(2))
    return tools


def main() -> int:
    in_registry = registry_tools()
    in_docs = documented_tools()

    undocumented = in_registry - in_docs
    stale = in_docs - in_registry

    if not undocumented and not stale:
        print(f"docs/tools.md is in sync with registry.toml ({len(in_registry)} tools)")
        return 0

    if undocumented:
        print("In registry.toml but missing from docs/tools.md:")
        for tool in sorted(undocumented):
            print(f"  + {tool}")
    if stale:
        print("In docs/tools.md but no longer in registry.toml:")
        for tool in sorted(stale):
            print(f"  - {tool}")

    print("\nRegenerate with: mise run docs-tools")
    return 1


if __name__ == "__main__":
    sys.exit(main())

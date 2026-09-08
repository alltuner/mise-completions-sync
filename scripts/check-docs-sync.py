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
TOTAL = re.compile(r"^\*\*Total: (\d+) tools\*\*$")


def registry_tools() -> set[str]:
    with open(ROOT / "registry.toml", "rb") as f:
        return set(tomllib.load(f).get("tools", {}).keys())


def documented_tools() -> tuple[set[str], int | None]:
    tools = set()
    total = None
    for line in (ROOT / "docs" / "tools.md").read_text().splitlines():
        if line.startswith("|-") or line.startswith("| Tool"):
            continue
        match = ROW.match(line)
        if match:
            tools.add(match.group(1) or match.group(2))
            continue
        match = TOTAL.match(line)
        if match:
            total = int(match.group(1))
    return tools, total


def main() -> int:
    in_registry = registry_tools()
    in_docs, documented_total = documented_tools()

    undocumented = in_registry - in_docs
    stale = in_docs - in_registry
    # Two PRs each adding a tool both bump this line from the same starting
    # number, so it survives the merge one short of the real count.
    miscounted = documented_total != len(in_registry)

    if not undocumented and not stale and not miscounted:
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
    if miscounted:
        print(
            f"docs/tools.md says '**Total: {documented_total} tools**' "
            f"but registry.toml has {len(in_registry)}"
        )

    print("\nRegenerate with: mise run docs-tools")
    return 1


if __name__ == "__main__":
    sys.exit(main())

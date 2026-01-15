#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "httpx",
# ]
# ///
"""
Fetches mise's registry.toml and generates completion entries for tools
that follow common patterns.

Usage: uv run scripts/generate-registry.py > registry.toml.generated

This script helps bootstrap the registry by identifying tools in mise's
registry that likely support completions. The output should be reviewed
and merged with manual entries.
"""

import tomllib
import httpx
import sys

MISE_REGISTRY_URL = "https://raw.githubusercontent.com/jdx/mise/main/registry.toml"

# Common completion command patterns
# Pattern name -> (zsh_cmd, bash_cmd, fish_cmd)
KNOWN_PATTERNS = {
    "standard": (
        "{tool} completion zsh",
        "{tool} completion bash",
        "{tool} completion fish",
    ),
    "completions": (
        "{tool} completions zsh",
        "{tool} completions bash",
        "{tool} completions fish",
    ),
    "gh_style": (
        "{tool} completion -s zsh",
        "{tool} completion -s bash",
        "{tool} completion -s fish",
    ),
    "generate_shell": (
        "{tool} generate-shell-completion zsh",
        "{tool} generate-shell-completion bash",
        "{tool} generate-shell-completion fish",
    ),
    "gen_completions": (
        "{tool} gen-completions --shell zsh",
        "{tool} gen-completions --shell bash",
        "{tool} gen-completions --shell fish",
    ),
}

# Map tool names to their known pattern
# Add tools here as you discover their completion patterns
TOOL_PATTERNS = {
    # Kubernetes ecosystem
    "kubectl": "standard",
    "helm": "standard",
    "k9s": "standard",
    "kind": "standard",
    "minikube": "standard",
    "kustomize": "standard",
    "argocd": "standard",
    "flux": "standard",
    # Cloud CLI tools
    "gh": "gh_style",
    "glab": "gh_style",
    # Development tools
    "mise": "standard",
    "just": "completions",
    "task": "standard",
    "goreleaser": "standard",
    "hugo": "standard",
    # Rust tools
    "rustup": "completions",
    "deno": "completions",
    "bat": "generate_shell",
    "starship": "completions",
    "zellij": "standard",
    "atuin": "gen_completions",
    # Python tools
    "uv": "generate_shell",
    "ruff": "generate_shell",
    "poetry": "completions",
    "pdm": "standard",
    # Node tools
    "pnpm": "standard",
    # Cloud platforms
    "flyctl": "standard",
    # Containers
    "docker": "standard",
}


def fetch_mise_registry():
    """Fetch and parse mise's registry.toml."""
    response = httpx.get(MISE_REGISTRY_URL)
    response.raise_for_status()
    return tomllib.loads(response.text)


def generate_entry(tool: str, pattern_name: str) -> list[str]:
    """Generate TOML entry for a tool."""
    pattern = KNOWN_PATTERNS[pattern_name]
    lines = [f"[{tool}]"]
    lines.append(f'zsh = "{pattern[0].format(tool=tool)}"')
    lines.append(f'bash = "{pattern[1].format(tool=tool)}"')
    lines.append(f'fish = "{pattern[2].format(tool=tool)}"')
    return lines


def main():
    mise_registry = fetch_mise_registry()
    output_lines = [
        "# Auto-generated tool completion registry",
        "# Review and merge with manual entries in registry.toml",
        "",
    ]

    # Find tools that are in both mise registry and our pattern mapping
    matched_tools = []
    for tool_name in sorted(mise_registry.keys()):
        if tool_name in TOOL_PATTERNS:
            matched_tools.append(tool_name)

    # Generate entries for matched tools
    for tool in matched_tools:
        pattern = TOOL_PATTERNS[tool]
        output_lines.extend(generate_entry(tool, pattern))
        output_lines.append("")

    # Print summary to stderr
    print(f"# Generated entries for {len(matched_tools)} tools", file=sys.stderr)
    print(f"# Tools in mise registry: {len(mise_registry)}", file=sys.stderr)
    print(f"# Tools with known patterns: {len(TOOL_PATTERNS)}", file=sys.stderr)

    # Print the TOML to stdout
    print("\n".join(output_lines))


if __name__ == "__main__":
    main()

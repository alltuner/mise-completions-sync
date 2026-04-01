#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "httpx",
# ]
# ///
"""
Discovers new tools in mise's registry that might support completions.

Usage: uv run scripts/generate-registry.py

Fetches tools from https://mise-versions.jdx.dev/api/tools and compares
against our registry.toml to suggest new tools to add.
"""

import tomllib
import httpx
from pathlib import Path

MISE_API_URL = "https://mise-versions.jdx.dev/api/tools"

# Map tool names to their likely completion pattern
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
    "k3d": "standard",
    "kubeseal": "standard",
    "krew": "standard",
    "stern": "standard",
    "velero": "standard",
    "istioctl": "standard",
    "cilium": "standard",
    "oc": "standard",
    "linkerd": "standard",
    "skaffold": "standard",
    "tilt": "standard",
    # Cloud CLI tools
    "gh": "gh_style",
    "glab": "gh_style",
    "tea": "gh_style",
    # Development tools
    "mise": "standard",
    "task": "standard",
    "goreleaser": "standard",
    "hugo": "standard",
    "pulumi": "standard",
    "turso": "standard",
    "wails": "standard",
    "air": "standard",
    "golangci-lint": "standard",
    "ko": "standard",
    "cue": "standard",
    "dagger": "standard",
    "restic": "standard",
    "chezmoi": "standard",
    "lazygit": "standard",
    "gh-dash": "gh_style",
    # Rust tools with completions pattern
    "rustup": "completions",
    "deno": "completions",
    "starship": "completions",
    "poetry": "completions",
    "wrangler": "completions",
    "lefthook": "completions",
    "bacon": "completions",
    # Rust tools with generate-shell-completion pattern
    "uv": "generate_shell",
    "ruff": "generate_shell",
    "bat": "generate_shell",
    "ty": "generate_shell",
    "procs": "generate_shell",
    "dust": "generate_shell",
    "ouch": "generate_shell",
    "hyperfine": "generate_shell",
    "tokei": "generate_shell",
    "miniserve": "generate_shell",
    "mdbook": "generate_shell",
    "cargo-watch": "generate_shell",
    # Rust tools with gen-completions pattern
    "atuin": "gen_completions",
    "gitui": "gen_completions",
    "gitu": "gen_completions",
    # Node tools
    "pnpm": "standard",
    "biome": "standard",
    # Cloud platforms
    "flyctl": "standard",
    "doctl": "standard",
    "oci": "standard",
    "scaleway-cli": "standard",
    # Containers
    "docker": "standard",
    "podman": "standard",
    "nerdctl": "standard",
    "buildah": "standard",
    "skopeo": "standard",
    "trivy": "standard",
    "cosign": "standard",
    # Other tools
    "saml2aws": "standard",
    "croc": "standard",
    "httpie": "standard",
    "xh": "standard",
    "grpcurl": "standard",
    "evans": "standard",
    "mkcert": "standard",
    "step": "standard",
}


def fetch_mise_api() -> list[dict]:
    """Fetch all tools from mise versions API (handles pagination)."""
    all_tools = []
    page = 1
    limit = 100  # Max per page

    while True:
        url = f"{MISE_API_URL}?page={page}&limit={limit}"
        response = httpx.get(url)
        response.raise_for_status()
        data = response.json()

        tools = data.get("tools", [])
        all_tools.extend(tools)

        total_pages = data.get("total_pages", 1)
        if page >= total_pages:
            break
        page += 1

    return all_tools


def extract_tool_name(tool_entry: dict) -> str:
    """
    Extract the canonical tool name from a mise API entry.

    The API returns entries with a 'name' field that's the canonical name.
    For example: "kubectl", "golangci-lint", "ripgrep"
    """
    return tool_entry.get("name", "")


def load_our_registry() -> set[str]:
    """Load our registry.toml and return set of tool names."""
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        registry = tomllib.load(f)
    return set(registry.get("tools", {}).keys())


def main():
    tools_data = fetch_mise_api()
    our_tools = load_our_registry()

    # Extract tool names from API response
    mise_tools = {
        extract_tool_name(tool) for tool in tools_data if extract_tool_name(tool)
    }

    # Find tools in mise that we don't have
    missing = mise_tools - our_tools

    # Find missing tools that have known patterns
    suggestions = []
    for tool in sorted(missing):
        if tool in TOOL_PATTERNS:
            suggestions.append((tool, TOOL_PATTERNS[tool]))

    # Report
    print(f"Tools in mise API: {len(mise_tools)}")
    print(f"Tools in our registry: {len(our_tools)}")
    print(f"Missing from our registry: {len(missing)}")
    print()

    if suggestions:
        print("Suggested additions (tools with known patterns):")
        print()
        for tool, pattern in suggestions:
            print(f'{tool} = "{pattern}"')
        print()
        print("Add these lines to the [tools] section in registry.toml")
    else:
        print("No new tools with known patterns found.")

    # List unknown tools for reference (limit to first 20)
    unknown = missing - set(TOOL_PATTERNS.keys())
    if unknown:
        print()
        unknown_list = sorted(unknown)[:20]
        print("Tools without known patterns (may need explicit entries):")
        for tool in unknown_list:
            print(f"  - {tool}")
        if len(unknown) > 20:
            print(f"  ... and {len(unknown) - 20} more")


if __name__ == "__main__":
    main()

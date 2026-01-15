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

Compares mise's registry against our registry.toml and suggests new tools
to add based on known completion patterns.
"""

import tomllib
import httpx
import sys
from pathlib import Path

MISE_REGISTRY_URL = "https://raw.githubusercontent.com/jdx/mise/main/registry.toml"

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


def fetch_mise_registry() -> dict:
    """Fetch and parse mise's registry.toml."""
    response = httpx.get(MISE_REGISTRY_URL)
    response.raise_for_status()
    registry = tomllib.loads(response.text)
    if "tools" in registry:
        return registry["tools"]
    return registry


def load_our_registry() -> set[str]:
    """Load our registry.toml and return set of tool names."""
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        registry = tomllib.load(f)
    return set(registry.get("tools", {}).keys())


def main():
    mise_registry = fetch_mise_registry()
    our_tools = load_our_registry()

    # Find tools in mise that we don't have
    mise_tools = set(mise_registry.keys())
    missing = mise_tools - our_tools

    # Find missing tools that have known patterns
    suggestions = []
    for tool in sorted(missing):
        if tool in TOOL_PATTERNS:
            suggestions.append((tool, TOOL_PATTERNS[tool]))

    # Report
    print(f"Tools in mise registry: {len(mise_tools)}")
    print(f"Tools in our registry: {len(our_tools)}")
    print(f"Missing from our registry: {len(missing)}")
    print()

    if suggestions:
        print("Suggested additions (tools with known patterns):")
        print()
        for tool, pattern in suggestions:
            print(f'{tool} = "{pattern}"')
        print()
        print(f"Add these lines to the [tools] section in registry.toml")
    else:
        print("No new tools with known patterns found.")

    # List unknown tools for reference
    unknown = missing - set(TOOL_PATTERNS.keys())
    if unknown and len(unknown) <= 20:
        print()
        print("Tools without known patterns (may need explicit entries):")
        for tool in sorted(unknown):
            print(f"  - {tool}")


if __name__ == "__main__":
    main()

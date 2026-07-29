#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""
Validates registry.toml entries by testing completion commands.

Usage: uv run scripts/validate-registry.py [--installed-only]

Tests each tool's completion command to verify it works. By default tests
all entries; use --installed-only to skip tools not installed via mise.
"""

import subprocess
import sys
import tomllib
from pathlib import Path


def get_installed_tools() -> set[str]:
    """Get list of tools installed via mise."""
    result = subprocess.run(
        ["mise", "ls", "--installed", "--json"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print(f"Warning: couldn't get mise tools: {result.stderr}", file=sys.stderr)
        return set()

    import json

    try:
        data = json.loads(result.stdout)
        return set(data.keys())
    except json.JSONDecodeError:
        return set()


def load_registry() -> dict[str, tuple[str, dict[str, str]]]:
    """Load registry.toml and expand patterns to get tool completions."""
    registry_path = Path(__file__).parent.parent / "registry.toml"
    with open(registry_path, "rb") as f:
        raw = tomllib.load(f)

    patterns = raw.get("patterns", {})
    tools_raw = raw.get("tools", {})

    expanded: dict[str, tuple[str, dict[str, str]]] = {}
    for tool_name, entry in tools_raw.items():
        if isinstance(entry, str):
            # Pattern reference
            pattern = patterns.get(entry)
            if pattern is None:
                print(f"Warning: unknown pattern '{entry}' for tool '{tool_name}'", file=sys.stderr)
                continue
            # Expand {} placeholder with tool name
            completions = {
                shell: cmd.replace("{}", tool_name) for shell, cmd in pattern.items()
            }
            expanded[tool_name] = (tool_name, completions)
        else:
            # Explicit commands, optionally provided by another mise tool
            provider = entry.get("provided_by", tool_name)
            completions = {
                shell: entry[shell]
                for shell in ("zsh", "bash", "fish")
                if shell in entry
            }
            # `requires` names a helper binary, not a shell, but it has to survive
            # here so the invocation can put it on PATH.
            if "requires" in entry:
                completions["requires"] = entry["requires"]
            # A bundled entry's shell values are filenames shipped in the
            # download, not commands, so they are checked differently.
            if entry.get("bundled"):
                completions["bundled"] = True
            expanded[tool_name] = (provider, completions)

    return expanded


def find_bundled(provider: str, filename: str) -> tuple[bool, str]:
    """Check a bundled completion file exists in the tool's install directory."""
    where = subprocess.run(
        ["mise", "where", provider], capture_output=True, text=True, timeout=30
    )
    if where.returncode != 0:
        return False, where.stderr.strip() or "mise where failed"

    root = Path(where.stdout.strip())
    for path in sorted(root.rglob(filename)):
        if path.is_file() and path.stat().st_size > 0:
            return True, ""
    return False, f"{filename} not found under {root}"


def test_completion(
    provider: str, shell: str, command: str, requires: str | None = None
) -> tuple[bool, str]:
    """Test a completion command. Returns (success, error_message)."""
    tools = f"{provider} {requires}" if requires else provider
    wrapped = f"mise x {tools} -- {command}"
    result = subprocess.run(
        ["sh", "-c", wrapped],
        capture_output=True,
        text=True,
        timeout=30,
    )

    if result.returncode == 0 and result.stdout.strip():
        return True, ""

    error = result.stderr.strip() or result.stdout.strip() or "empty output"
    return False, error


def main():
    installed_only = "--installed-only" in sys.argv

    registry = load_registry()
    installed = get_installed_tools() if installed_only else set()

    results: dict[str, dict[str, tuple[bool, str]]] = {}
    shells = ["zsh", "bash", "fish"]

    tools = sorted(registry.keys())
    total = len(tools)

    print(f"Validating {total} tools...\n")

    for i, tool in enumerate(tools, 1):
        provider, completions = registry[tool]
        if installed_only and provider not in installed:
            continue

        requires = completions.get("requires")
        results[tool] = {}

        print(f"[{i}/{total}] {tool}...", end=" ", flush=True)
        tool_ok = True

        for shell in shells:
            if shell not in completions:
                continue

            command = completions[shell]
            try:
                if completions.get("bundled"):
                    ok, err = find_bundled(provider, command)
                else:
                    ok, err = test_completion(provider, shell, command, requires)
                results[tool][shell] = (ok, err)
                if not ok:
                    tool_ok = False
            except subprocess.TimeoutExpired:
                results[tool][shell] = (False, "timeout")
                tool_ok = False
            except Exception as e:
                results[tool][shell] = (False, str(e))
                tool_ok = False

        print("✓" if tool_ok else "✗")

    # Summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)

    failures: dict[str, list[tuple[str, str, str]]] = {}
    successes = 0
    total_tests = 0

    for tool, shell_results in results.items():
        for shell, (ok, err) in shell_results.items():
            total_tests += 1
            if ok:
                successes += 1
            else:
                if tool not in failures:
                    failures[tool] = []
                _, completions = registry[tool]
                failures[tool].append((shell, completions[shell], err))

    print(f"\nPassed: {successes}/{total_tests}")

    if failures:
        print(f"\nFailed tools ({len(failures)}):\n")
        for tool, errs in sorted(failures.items()):
            print(f"  [{tool}]")
            for shell, cmd, err in errs:
                # Truncate long errors
                err_short = err[:60] + "..." if len(err) > 60 else err
                print(f"    {shell}: {err_short}")
            print()

    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())

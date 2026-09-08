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
            # Some tools need a working environment (a cluster config, say) even
            # to print a static script, so the audit cannot judge them.
            if "audit_skip" in entry:
                completions["audit_skip"] = entry["audit_skip"]
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


def install_tool(target: str) -> tuple[bool, str]:
    """Install a tool with mise. Returns (installed, reason_if_not)."""
    try:
        result = subprocess.run(
            ["mise", "install", f"{target}@latest"],
            capture_output=True,
            text=True,
            timeout=600,
        )
    except subprocess.TimeoutExpired:
        return False, "install timed out"

    if result.returncode == 0:
        return True, ""

    for line in result.stderr.splitlines():
        if "ERROR" in line:
            return False, line.split("ERROR", 1)[1].strip()
    return False, result.stderr.strip().splitlines()[0] if result.stderr.strip() else "install failed"


def main():
    installed_only = "--installed-only" in sys.argv
    install = "--install" in sys.argv
    only = [a for a in sys.argv[1:] if not a.startswith("--")]

    registry = load_registry()
    installed = get_installed_tools() if installed_only else set()

    results: dict[str, dict[str, tuple[bool, str]]] = {}
    unavailable: dict[str, str] = {}
    shells = ["zsh", "bash", "fish"]

    tools = [t for t in sorted(registry.keys()) if not only or t in only]
    total = len(tools)

    print(f"Validating {total} tools{' (installing first)' if install else ''}...\n")

    for i, tool in enumerate(tools, 1):
        provider, completions = registry[tool]
        if installed_only and provider not in installed:
            continue

        print(f"[{i}/{total}] {tool}...", end=" ", flush=True)

        skip_reason = completions.get("audit_skip")
        if install and skip_reason:
            unavailable[tool] = f"skipped: {skip_reason}"
            print("skipped (cannot be audited)")
            continue

        if install:
            ok, reason = install_tool(provider)
            if not ok:
                # Not installable here, so the entry is untested rather than wrong.
                unavailable[tool] = reason
                print("skipped (not installable)")
                continue

        requires = completions.get("requires")
        results[tool] = {}
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

        print("\u2713" if tool_ok else "\u2717")

    failures: dict[str, list[tuple[str, str, str]]] = {}
    successes = 0
    total_tests = 0

    for tool, shell_results in results.items():
        for shell, (ok, err) in shell_results.items():
            total_tests += 1
            if ok:
                successes += 1
            else:
                failures.setdefault(tool, [])
                _, completions = registry[tool]
                failures[tool].append((shell, completions[shell], err))

    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"\nPassed: {successes}/{total_tests}")

    if failures:
        print(f"\n## Broken entries ({len(failures)})\n")
        for tool, errs in sorted(failures.items()):
            print(f"  [{tool}]")
            for shell, cmd, err in errs:
                err_short = err[:60] + "..." if len(err) > 60 else err
                print(f"    {shell}: `{cmd}` -> {err_short}")
            print()

    if unavailable:
        # Reported separately: these say nothing about whether the entry is right.
        print(f"\n## Could not verify ({len(unavailable)})\n")
        for tool, reason in sorted(unavailable.items()):
            print(f"  {tool}: {reason[:70]}")
        print()

    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())

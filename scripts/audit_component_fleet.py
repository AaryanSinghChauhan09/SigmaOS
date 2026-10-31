#!/usr/bin/env python3
"""Read-only audit of the repositories declared in COMPONENTS.toml.

The audit deliberately does not clone, fetch, modify, commit, or push anything.
It reports the scope and hygiene gaps that must be resolved before fleet-wide
automation is safe.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tomllib
from pathlib import Path
from urllib.parse import urlsplit


REQUIRED_FILES = ("README.md", "README", "README.rst", "readme.md")
LICENSE_FILES = ("LICENSE", "LICENSE.md", "COPYING")


def git(repo: Path, *args: str) -> str:
    result = subprocess.run(
        ("git", "-C", str(repo), *args),
        capture_output=True,
        text=True,
        check=False,
    )
    return result.stdout.strip() if result.returncode == 0 else ""


def has_file(repo: Path, candidates: tuple[str, ...]) -> bool:
    return any((repo / candidate).is_file() for candidate in candidates)


def has_license(repo: Path) -> bool:
    """Recognize single-file notices and Cargo's conventional dual-license pair."""
    if has_file(repo, LICENSE_FILES):
        return True
    return (repo / "LICENSE-MIT").is_file() and (repo / "LICENSE-APACHE").is_file()


def normalize_remote(remote: str) -> str:
    """Normalize common HTTPS and SSH GitHub remotes for URL comparisons."""
    remote = remote.strip()
    if remote.startswith("git@github.com:"):
        path = remote.removeprefix("git@github.com:")
        host = "github.com"
    else:
        parsed = urlsplit(remote)
        host = parsed.hostname or ""
        path = parsed.path
    path = path.strip("/")
    if path.endswith(".git"):
        path = path[:-4]
    return f"{host.lower()}/{path}".lower() if host and path else ""


def audit(root: Path) -> dict[str, object]:
    registry_path = root / "COMPONENTS.toml"
    data = tomllib.loads(registry_path.read_text(encoding="utf-8"))
    entries = data.get("component", [])

    invalid_entries = [
        index
        for index, entry in enumerate(entries, start=1)
        if not isinstance(entry.get("name"), str) or not entry["name"].strip()
    ]
    names = [
        entry["name"].strip()
        for entry in entries
        if isinstance(entry.get("name"), str) and entry["name"].strip()
    ]
    groups: dict[str, list[dict[str, object]]] = {}
    for entry in entries:
        raw_name = entry.get("name")
        if not isinstance(raw_name, str) or not raw_name.strip():
            continue
        name = raw_name.strip()
        if name:
            groups.setdefault(name, []).append(entry)
    duplicates = sorted(name for name, group in groups.items() if len(group) > 1)
    exact_duplicates = sorted(
        name
        for name, group in groups.items()
        if len(group) > 1
        and len({tuple(sorted((key, repr(value)) for key, value in item.items())) for item in group}) == 1
    )
    components: list[dict[str, object]] = []

    for entry in entries:
        name = entry.get("name", "")
        repo = root.parent / name
        exists = (repo / ".git").is_dir()
        expected_origin = entry.get("repo", "")
        origin = git(repo, "remote", "get-url", "origin") if exists else ""
        record: dict[str, object] = {
            "name": name,
            "path": str(repo),
            "checkout": exists,
            "origin": origin,
            "expected_origin": expected_origin,
            "origin_matches_registry": bool(origin)
            and normalize_remote(origin) == normalize_remote(expected_origin),
            "dirty": bool(git(repo, "status", "--porcelain")) if exists else False,
            "readme": has_file(repo, REQUIRED_FILES) if exists else False,
            "license": has_license(repo) if exists else False,
            "ci": (repo / ".github" / "workflows").is_dir() if exists else False,
        }
        components.append(record)

    return {
        "registry_entries": len(entries),
        "unique_components": len(set(names)),
        "invalid_entries": invalid_entries,
        "missing_local_checkouts": sorted(
            set(names) - {item["name"] for item in components if item["checkout"]}
        ),
        "duplicate_names": duplicates,
        "exact_duplicate_names": exact_duplicates,
        "variant_duplicate_names": sorted(set(duplicates) - set(exact_duplicates)),
        "components": components,
    }


def summary(report: dict[str, object]) -> str:
    components = report["components"]
    assert isinstance(components, list)
    checkout = sum(bool(item["checkout"]) for item in components)
    missing_local = len(report["missing_local_checkouts"])
    no_origin = sum(bool(item["checkout"]) and not item["origin"] for item in components)
    wrong_origin = sum(
        bool(item["checkout"] and item["origin"] and not item["origin_matches_registry"])
        for item in components
    )
    dirty = sum(bool(item["dirty"]) for item in components)
    readme = sum(bool(item["readme"]) for item in components)
    license_count = sum(bool(item["license"]) for item in components)
    ci = sum(bool(item["ci"]) for item in components)
    return "\n".join(
        (
            f"registry entries: {report['registry_entries']}",
            f"unique components: {report['unique_components']}",
            f"invalid unnamed entries: {len(report['invalid_entries'])}",
            f"local git checkouts: {checkout}",
            f"missing local checkouts: {missing_local}",
            f"missing origin: {no_origin}",
            f"origin differs from registry URL: {wrong_origin}",
            f"dirty worktrees: {dirty}",
            f"README present: {readme}",
            f"license present: {license_count}",
            f"GitHub workflows directory: {ci}",
            f"duplicate registry names: {len(report['duplicate_names'])}",
            f"exact duplicate names: {len(report['exact_duplicate_names'])}",
            f"metadata-variant duplicate names: {len(report['variant_duplicate_names'])}",
        )
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the full report as JSON")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    report = audit(root)
    print(json.dumps(report, indent=2) if args.json else summary(report))
    return 0


if __name__ == "__main__":
    sys.exit(main())

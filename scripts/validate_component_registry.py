#!/usr/bin/env python3
"""Validate SigmaOS component ownership and repository metadata."""

from pathlib import Path
import sys
import tomllib


def main() -> int:
    path = Path(__file__).resolve().parents[1] / "components" / "SIGMAOS_COMPONENT_REGISTRY.toml"
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    components = {entry["name"]: entry for entry in data["component"]}
    published = set(data["canonical_repositories_published"])
    owners = data["component_owners"]

    if set(components) != set(owners):
        missing = sorted(set(components) ^ set(owners))
        raise SystemExit(f"component ownership mismatch: {', '.join(missing)}")
    unknown = sorted(set(owners.values()) - published)
    if unknown:
        raise SystemExit(f"owners are not published repositories: {', '.join(unknown)}")

    repositories = [entry["repository"] for entry in components.values()]
    if len(repositories) != len(set(repositories)):
        raise SystemExit("duplicate component repository names")
    print(f"validated {len(components)} components and {len(published)} published repositories")
    return 0


if __name__ == "__main__":
    sys.exit(main())

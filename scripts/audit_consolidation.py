#!/usr/bin/env python3
"""Read-only audit of consolidation-suite references against COMPONENTS.toml."""

from __future__ import annotations

import collections
import tomllib
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    components = tomllib.loads((root / "COMPONENTS.toml").read_text(encoding="utf-8"))["component"]
    suites = tomllib.loads((root / "CONSOLIDATION.toml").read_text(encoding="utf-8"))["suites"]
    names = {entry["name"] for entry in components}
    canonical_names = [suite.get("canonical", "") for suite in suites.values()]
    constituents = {
        (suite_name, name)
        for suite_name, suite in suites.items()
        for name in suite.get("consolidates", [])
    }
    missing_canonical = sorted(set(canonical_names) - names)
    missing_constituents = sorted({name for _, name in constituents} - names)
    self_references = sorted(
        (suite_name, suite.get("canonical"))
        for suite_name, suite in suites.items()
        if suite.get("canonical") in suite.get("consolidates", [])
    )
    constituent_owners: dict[str, list[str]] = collections.defaultdict(list)
    for suite_name, name in constituents:
        constituent_owners[name].append(suite_name)
    overlaps = sorted((name, sorted(owners)) for name, owners in constituent_owners.items() if len(owners) > 1)
    missing_checkouts = sorted(
        {name for _, name in constituents} - {name for _, name in constituents if (root.parent / name / ".git").exists()}
    )

    print(f"consolidation suites: {len(suites)}")
    print(f"canonical names missing from component registry: {len(missing_canonical)}")
    print(f"constituent names: {len({name for _, name in constituents})}")
    print(f"constituents missing from component registry: {len(missing_constituents)}")
    print(f"constituents without local checkouts: {len(missing_checkouts)}")
    print(f"canonical self-references: {len(self_references)}")
    print(f"constituents assigned to multiple suites: {len(overlaps)}")
    for label, values in (
        ("missing canonical", missing_canonical),
        ("missing constituent", missing_constituents),
    ):
        if values:
            print(f"{label}: " + ", ".join(values[:100]))
            if len(values) > 100:
                print(f"... and {len(values) - 100} more")
    for suite, name in self_references:
        print(f"self-reference: {suite} -> {name}")
    for name, owners in overlaps:
        print(f"overlap: {name} -> {', '.join(owners)}")
    return int(bool(missing_canonical or self_references or overlaps))


if __name__ == "__main__":
    raise SystemExit(main())

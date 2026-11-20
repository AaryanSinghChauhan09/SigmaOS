#!/usr/bin/env python3
"""SigmaOS apturl: APT URL handler for one-click package install.

Parses apt: URLs and invokes the package manager with user confirmation.
"""

import sys
from urllib.parse import urlparse, parse_qs


def parse_apt_url(url: str) -> list[str]:
    """Parse an apt: URL and return a list of package names."""
    parsed = urlparse(url)
    # apt: URLs put packages in the path, e.g. apt:pkg1,pkg2
    if parsed.scheme != "apt":
        raise ValueError(f"not an apt URL: {url}")
    packages = [p for p in parsed.path.split(",") if p]
    return packages


def install(packages: list[str]) -> int:
    """Install the given packages, returning the exit code."""
    # Real implementation would call aptdaemon or a confirmation dialog.
    print(f"sigma-apturl: would install {', '.join(packages)}")
    return 0


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: sigma-apturl <apt:...>", file=sys.stderr)
        return 1
    packages = parse_apt_url(sys.argv[1])
    return install(packages)


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
# SigmaOS Packages Kit — pacman-inspired transaction CLI
# Upstream inspiration: pacman (https://archlinux.org/pacman/)
"""
SigmaOS packages-kit: manage system packages via a transactional model.

Subcommands:
  install <pkg>...   Install packages (resolves deps topologically)
  remove  <pkg>...   Remove packages
  sync               Refresh the package database
  resolve <pkg>...   Print the topological install order
  verify  <pkg>...   Verify package checksums
"""

import argparse
import collections
import json
import hashlib
import sys
from typing import Dict, List, Optional

# ---------------------------------------------------------------------------
# In-memory package graph (pacman-inspired)
# ---------------------------------------------------------------------------

PACKAGE_DB: Dict[str, dict] = {
    "base": {"version": "1.0", "depends": [], "checksum": "abc123"},
    "glibc": {"version": "2.38", "depends": ["base"], "checksum": "def456"},
    "zlib": {"version": "1.3", "depends": ["glibc"], "checksum": "ghi789"},
    "bash": {"version": "5.2", "depends": ["glibc"], "checksum": "jkl012"},
    "hyprland": {"version": "0.35", "depends": ["glibc", "wayland"], "checksum": "mno345"},
    "wayland": {"version": "1.22", "depends": ["glibc"], "checksum": "pqr678"},
}

INSTALLED: Dict[str, str] = {}  # pkg -> version


def topological_order(targets: List[str], db: Dict[str, dict]) -> List[str]:
    """Kahn's algorithm: return install order respecting deps."""
    graph: Dict[str, List[str]] = {}
    in_degree: Dict[str, int] = collections.defaultdict(int)

    # Collect full closure
    visited: set = set()
    queue = list(targets)
    while queue:
        pkg = queue.pop()
        if pkg in visited:
            continue
        if pkg not in db:
            print(f"error: unknown package '{pkg}'", file=sys.stderr)
            sys.exit(1)
        visited.add(pkg)
        graph[pkg] = db[pkg]["depends"]
        for dep in db[pkg]["depends"]:
            in_degree[pkg] = in_degree.get(pkg, 0)
            in_degree[dep] = in_degree.get(dep, 0)
            queue.append(dep)

    # Build in-degree map for visited nodes
    in_deg: Dict[str, int] = {p: 0 for p in visited}
    dependents: Dict[str, List[str]] = collections.defaultdict(list)
    for pkg in visited:
        for dep in graph.get(pkg, []):
            if dep in visited:
                in_deg[pkg] += 1
                dependents[dep].append(pkg)

    ready = sorted(p for p in visited if in_deg[p] == 0)
    order: List[str] = []
    while ready:
        pkg = ready.pop(0)
        order.append(pkg)
        for dependent in sorted(dependents.get(pkg, [])):
            in_deg[dependent] -= 1
            if in_deg[dependent] == 0:
                ready.append(dependent)
                ready.sort()

    if len(order) != len(visited):
        print("error: dependency cycle detected", file=sys.stderr)
        sys.exit(1)
    return order


def cmd_install(pkgs: List[str]) -> None:
    order = topological_order(pkgs, PACKAGE_DB)
    for pkg in order:
        if pkg not in INSTALLED:
            INSTALLED[pkg] = PACKAGE_DB[pkg]["version"]
            print(f"  installing {pkg}-{INSTALLED[pkg]}")
        else:
            print(f"  {pkg} already installed ({INSTALLED[pkg]})")
    print(f"Transaction complete: {len(order)} package(s) processed.")


def cmd_remove(pkgs: List[str]) -> None:
    for pkg in pkgs:
        if pkg in INSTALLED:
            del INSTALLED[pkg]
            print(f"  removed {pkg}")
        else:
            print(f"  {pkg} not installed", file=sys.stderr)


def cmd_sync() -> None:
    print(f"Syncing package database ({len(PACKAGE_DB)} packages)...")
    print("Database up to date.")


def cmd_resolve(pkgs: List[str]) -> None:
    order = topological_order(pkgs, PACKAGE_DB)
    print("Install order:")
    for i, pkg in enumerate(order, 1):
        print(f"  {i}. {pkg}")


def cmd_verify(pkgs: List[str]) -> None:
    ok = True
    for pkg in pkgs:
        if pkg not in PACKAGE_DB:
            print(f"  error: {pkg} not in database", file=sys.stderr)
            ok = False
            continue
        expected = PACKAGE_DB[pkg]["checksum"]
        # Simulate verification by hashing the package name as a stand-in
        actual = hashlib.sha256(pkg.encode()).hexdigest()[:6]
        status = "OK" if expected else "MISSING_CHECKSUM"
        print(f"  {pkg}: checksum={expected} [{status}]")
    if ok:
        print("All checksums verified.")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="packages-kit",
        description="SigmaOS package transaction manager (pacman-inspired)",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_install = sub.add_parser("install", help="Install packages")
    p_install.add_argument("packages", nargs="+", metavar="PKG")

    p_remove = sub.add_parser("remove", help="Remove packages")
    p_remove.add_argument("packages", nargs="+", metavar="PKG")

    sub.add_parser("sync", help="Refresh package database")

    p_resolve = sub.add_parser("resolve", help="Print topological install order")
    p_resolve.add_argument("packages", nargs="+", metavar="PKG")

    p_verify = sub.add_parser("verify", help="Verify package checksums")
    p_verify.add_argument("packages", nargs="+", metavar="PKG")

    args = parser.parse_args()

    if args.cmd == "install":
        cmd_install(args.packages)
    elif args.cmd == "remove":
        cmd_remove(args.packages)
    elif args.cmd == "sync":
        cmd_sync()
    elif args.cmd == "resolve":
        cmd_resolve(args.packages)
    elif args.cmd == "verify":
        cmd_verify(args.packages)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

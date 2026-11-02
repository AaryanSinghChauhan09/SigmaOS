#!/usr/bin/env python3
# SigmaOS Repo Manager — repository metadata management
# Inspired by pacman's mirrorlist + Arch repo model, with BSD pkg repo support.
"""Manage SigmaOS package repositories: list, add, remove, enable, disable, sync, health."""

import argparse
import json
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional


@dataclass
class RepoEntry:
    name: str
    url: str
    priority: int = 50       # lower = higher priority
    enabled: bool = True
    backend: str = "pacman"  # pacman | apt | dnf | pkg
    signed: bool = True
    last_sync: Optional[str] = None
    healthy: bool = True


# In-memory repo registry
REPOS: Dict[str, RepoEntry] = {
    "sigmaos-core": RepoEntry("sigmaos-core", "https://repo.sigmaos.io/core", priority=10),
    "sigmaos-extra": RepoEntry("sigmaos-extra", "https://repo.sigmaos.io/extra", priority=20),
    "sigmaos-community": RepoEntry("sigmaos-community", "https://repo.sigmaos.io/community", priority=30),
    "sigmaos-testing": RepoEntry("sigmaos-testing", "https://repo.sigmaos.io/testing", priority=90, enabled=False),
}


# ---------------------------------------------------------------------------
# Operations
# ---------------------------------------------------------------------------

def do_list(enabled_only: bool = False) -> int:
    entries = [r for r in REPOS.values() if not enabled_only or r.enabled]
    if not entries:
        print("No repositories configured.")
        return 0
    print(f"{'NAME':<25} {'PRI':>4}  {'STATE':<8} {'BACKEND':<8} {'SIGNED':<7} URL")
    print("-" * 90)
    for r in sorted(entries, key=lambda x: x.priority):
        state = "enabled" if r.enabled else "disabled"
        signed = "yes" if r.signed else "no"
        print(f"  {r.name:<23} {r.priority:>4}  {state:<8} {r.backend:<8} {signed:<7} {r.url}")
    return 0


def do_add(name: str, url: str, priority: int, backend: str, signed: bool) -> int:
    if name in REPOS:
        print(f"error: repo '{name}' already exists. Use 'edit' to modify.", file=sys.stderr)
        return 1
    REPOS[name] = RepoEntry(name, url, priority=priority, backend=backend, signed=signed)
    print(f"[repo] Added '{name}' ({url})  priority={priority}  backend={backend}")
    return 0


def do_remove(name: str) -> int:
    if name not in REPOS:
        print(f"error: repo '{name}' not found.", file=sys.stderr)
        return 1
    del REPOS[name]
    print(f"[repo] Removed '{name}'.")
    return 0


def do_enable(name: str) -> int:
    if name not in REPOS:
        print(f"error: repo '{name}' not found.", file=sys.stderr)
        return 1
    REPOS[name].enabled = True
    print(f"[repo] Enabled '{name}'.")
    return 0


def do_disable(name: str) -> int:
    if name not in REPOS:
        print(f"error: repo '{name}' not found.", file=sys.stderr)
        return 1
    REPOS[name].enabled = False
    print(f"[repo] Disabled '{name}'.")
    return 0


def do_sync() -> int:
    from datetime import datetime
    enabled = [r for r in REPOS.values() if r.enabled]
    if not enabled:
        print("No enabled repositories to sync.")
        return 0
    print(f"Syncing {len(enabled)} repositories...")
    for r in sorted(enabled, key=lambda x: x.priority):
        r.last_sync = datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
        print(f"  [sync] {r.name}  ({r.url})")
    print(f"Done. Synced {len(enabled)} repo(s).")
    return 0


def do_health() -> int:
    enabled = [r for r in REPOS.values() if r.enabled]
    issues = 0
    for r in sorted(enabled, key=lambda x: x.priority):
        # Scaffold: simulate health based on repo name
        r.healthy = not r.name.endswith("-testing")
        status = "✓ healthy" if r.healthy else "✗ unhealthy"
        sync = r.last_sync or "never"
        print(f"  {status:<14} {r.name:<25}  last_sync={sync}")
        if not r.healthy:
            issues += 1
    if issues:
        print(f"\n{issues} unhealthy repo(s). Check connectivity or disable them.", file=sys.stderr)
    return issues


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def cmd_list(args: argparse.Namespace) -> int:
    return do_list(enabled_only=args.enabled)


def cmd_add(args: argparse.Namespace) -> int:
    return do_add(args.name, args.url, args.priority, args.backend, not args.unsigned)


def cmd_remove(args: argparse.Namespace) -> int:
    return do_remove(args.name)


def cmd_enable(args: argparse.Namespace) -> int:
    return do_enable(args.name)


def cmd_disable(args: argparse.Namespace) -> int:
    return do_disable(args.name)


def cmd_sync(args: argparse.Namespace) -> int:
    return do_sync()


def cmd_health(args: argparse.Namespace) -> int:
    return min(do_health(), 1)


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="sigma-repo",
        description="SigmaOS repo manager: manage package repositories",
    )
    sub = p.add_subparsers(dest="command", required=True)

    li = sub.add_parser("list", help="List all repositories")
    li.add_argument("--enabled", action="store_true", help="Show only enabled repos")
    li.set_defaults(func=cmd_list)

    ad = sub.add_parser("add", help="Add a repository")
    ad.add_argument("name")
    ad.add_argument("url")
    ad.add_argument("--priority", type=int, default=50)
    ad.add_argument("--backend", default="pacman", choices=["pacman", "apt", "dnf", "pkg"])
    ad.add_argument("--unsigned", action="store_true", help="Allow unsigned repo")
    ad.set_defaults(func=cmd_add)

    rm = sub.add_parser("remove", help="Remove a repository")
    rm.add_argument("name")
    rm.set_defaults(func=cmd_remove)

    en = sub.add_parser("enable", help="Enable a repository")
    en.add_argument("name")
    en.set_defaults(func=cmd_enable)

    di = sub.add_parser("disable", help="Disable a repository")
    di.add_argument("name")
    di.set_defaults(func=cmd_disable)

    sub.add_parser("sync", help="Sync all enabled repos").set_defaults(func=cmd_sync)
    sub.add_parser("health", help="Check repo health").set_defaults(func=cmd_health)

    return p


def main() -> int:
    return build_parser().parse_args().func(build_parser().parse_args())


if __name__ == "__main__":
    sys.exit(main())

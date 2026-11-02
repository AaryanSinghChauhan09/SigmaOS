#!/usr/bin/env python3
# SigmaOS Plugin Marketplace — AUR-inspired trusted community marketplace
# Upstream inspiration: Arch User Repository (https://aur.archlinux.org/)
"""
SigmaOS plugin-marketplace: community plugin discovery and install.

Subcommands:
  search  <query>   Fuzzy-search the marketplace catalogue
  install <name>    Install a plugin by exact name
  list              List installed plugins
  info    <name>    Show plugin details
"""

import argparse
import sys
from typing import Dict, List

# ---------------------------------------------------------------------------
# In-memory catalogue (AUR-inspired: trust_score, votes)
# ---------------------------------------------------------------------------

CATALOGUE: Dict[str, dict] = {
    "weather-widget": {
        "description": "Quickshell weather overlay using wttr.in",
        "version": "1.3.0",
        "author": "community",
        "trust_score": 4.9,
        "votes": 312,
        "tags": ["weather", "widget", "quickshell"],
    },
    "gpu-telemetry": {
        "description": "NVIDIA/AMD GPU gauges for Waybar",
        "version": "2.1.0",
        "author": "sigmaos-core",
        "trust_score": 5.0,
        "votes": 541,
        "tags": ["gpu", "waybar", "monitoring"],
    },
    "sigma-theme-catppuccin": {
        "description": "Catppuccin Mocha colour scheme for SigmaOS",
        "version": "0.4.2",
        "author": "community",
        "trust_score": 4.8,
        "votes": 228,
        "tags": ["theme", "catppuccin", "colour"],
    },
    "hypr-scratchpad": {
        "description": "Hyprland scratchpad manager for quickshell",
        "version": "0.2.1",
        "author": "community",
        "trust_score": 4.5,
        "votes": 87,
        "tags": ["hyprland", "scratchpad", "wm"],
    },
    "sigma-notch": {
        "description": "MacBook-style notch overlay for SigmaOS bar",
        "version": "1.0.0",
        "author": "community",
        "trust_score": 4.2,
        "votes": 45,
        "tags": ["bar", "waybar", "notch"],
    },
}

INSTALLED: Dict[str, str] = {}  # plugin_name -> version


def fuzzy_match(query: str, text: str) -> bool:
    q, t = query.lower(), text.lower()
    qi = 0
    for ch in t:
        if qi < len(q) and ch == q[qi]:
            qi += 1
    return qi == len(q)


def cmd_search(query: str) -> None:
    results = []
    for name, info in CATALOGUE.items():
        if (fuzzy_match(query, name) or
                fuzzy_match(query, info["description"]) or
                any(fuzzy_match(query, tag) for tag in info["tags"])):
            results.append((info["trust_score"], info["votes"], name, info))
    results.sort(key=lambda x: (x[0], x[1]), reverse=True)
    if not results:
        print(f"No results for '{query}'.")
        return
    print(f"{'Trust':<7} {'Votes':<6} {'Name':<28} {'Version':<10} Description")
    print("-" * 80)
    for trust, votes, name, info in results:
        installed = " [installed]" if name in INSTALLED else ""
        print(f"  {trust:<5.1f} {votes:<6} {name:<28} {info['version']:<10} "
              f"{info['description'][:32]}{installed}")


def cmd_install(name: str) -> None:
    if name not in CATALOGUE:
        print(f"error: plugin '{name}' not found in marketplace", file=sys.stderr)
        sys.exit(1)
    info = CATALOGUE[name]
    if name in INSTALLED:
        print(f"  '{name}' ({INSTALLED[name]}) already installed.")
        return
    trust = info["trust_score"]
    if trust < 3.0:
        print(f"warning: low trust score ({trust}) for '{name}'", file=sys.stderr)
    INSTALLED[name] = info["version"]
    print(f"  installed '{name}' v{info['version']} "
          f"(trust={trust}, votes={info['votes']})")


def cmd_list() -> None:
    if not INSTALLED:
        print("No plugins installed.")
        return
    print(f"{'Name':<28} {'Version'}")
    print("-" * 40)
    for name, ver in sorted(INSTALLED.items()):
        print(f"  {name:<26} {ver}")


def cmd_info(name: str) -> None:
    if name not in CATALOGUE:
        print(f"error: plugin '{name}' not found", file=sys.stderr)
        sys.exit(1)
    info = CATALOGUE[name]
    print(f"Name:        {name}")
    print(f"Version:     {info['version']}")
    print(f"Author:      {info['author']}")
    print(f"Trust score: {info['trust_score']}/5.0")
    print(f"Votes:       {info['votes']}")
    print(f"Tags:        {', '.join(info['tags'])}")
    print(f"Description: {info['description']}")
    print(f"Installed:   {'yes' if name in INSTALLED else 'no'}")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="plugin-marketplace",
        description="SigmaOS AUR-inspired plugin marketplace",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_search = sub.add_parser("search", help="Search the catalogue")
    p_search.add_argument("query", help="Search query")

    p_install = sub.add_parser("install", help="Install a plugin")
    p_install.add_argument("name", help="Plugin name")

    sub.add_parser("list", help="List installed plugins")

    p_info = sub.add_parser("info", help="Show plugin details")
    p_info.add_argument("name", help="Plugin name")

    args = parser.parse_args()

    if args.cmd == "search":
        cmd_search(args.query)
    elif args.cmd == "install":
        cmd_install(args.name)
    elif args.cmd == "list":
        cmd_list()
    elif args.cmd == "info":
        cmd_info(args.name)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

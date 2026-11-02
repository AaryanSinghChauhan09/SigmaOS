#!/usr/bin/env python3
# SigmaOS Plugin Registry — AUR-inspired registry: register/list/resolve
# Upstream inspiration: Arch User Repository (https://aur.archlinux.org/)
"""
SigmaOS plugin-registry: package registry for community plugins.

Subcommands:
  register <name> <version> <source>   Register a plugin entry
  list                                  List all registered plugins
  resolve  <name>                       Resolve name -> version -> source
  unregister <name>                     Remove a plugin from the registry
"""

import argparse
import sys
from typing import Dict, Optional, Tuple

# ---------------------------------------------------------------------------
# In-memory registry: name -> (version, source_url)
# ---------------------------------------------------------------------------

_REGISTRY: Dict[str, Tuple[str, str]] = {
    "weather-widget": ("1.3.0", "https://plugins.sigmaos.dev/weather-widget"),
    "gpu-telemetry":  ("2.1.0", "https://plugins.sigmaos.dev/gpu-telemetry"),
    "sigma-theme-catppuccin": ("0.4.2",
                               "https://plugins.sigmaos.dev/sigma-theme-catppuccin"),
    "hypr-scratchpad": ("0.2.1",
                        "https://github.com/community/hypr-scratchpad"),
}


def cmd_register(name: str, version: str, source: str) -> None:
    if name in _REGISTRY:
        old_ver, _ = _REGISTRY[name]
        _REGISTRY[name] = (version, source)
        print(f"  updated '{name}': {old_ver} -> {version} ({source})")
    else:
        _REGISTRY[name] = (version, source)
        print(f"  registered '{name}' v{version} ({source})")


def cmd_list() -> None:
    if not _REGISTRY:
        print("Registry is empty.")
        return
    print(f"{'Name':<32} {'Version':<12} Source")
    print("-" * 72)
    for name in sorted(_REGISTRY):
        ver, src = _REGISTRY[name]
        print(f"  {name:<30} {ver:<12} {src}")


def cmd_resolve(name: str) -> None:
    if name not in _REGISTRY:
        print(f"error: '{name}' not found in registry", file=sys.stderr)
        sys.exit(1)
    ver, src = _REGISTRY[name]
    print(f"name:    {name}")
    print(f"version: {ver}")
    print(f"source:  {src}")


def cmd_unregister(name: str) -> None:
    if name not in _REGISTRY:
        print(f"error: '{name}' not in registry", file=sys.stderr)
        sys.exit(1)
    _REGISTRY.pop(name)
    print(f"  unregistered '{name}'")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="plugin-registry",
        description="SigmaOS AUR-inspired plugin registry",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_reg = sub.add_parser("register", help="Register a plugin")
    p_reg.add_argument("name", help="Plugin name")
    p_reg.add_argument("version", help="Version string")
    p_reg.add_argument("source", help="Source URL or path")

    sub.add_parser("list", help="List all registered plugins")

    p_resolve = sub.add_parser("resolve", help="Resolve name to version+source")
    p_resolve.add_argument("name", help="Plugin name")

    p_unreg = sub.add_parser("unregister", help="Remove a plugin")
    p_unreg.add_argument("name", help="Plugin name")

    args = parser.parse_args()

    if args.cmd == "register":
        cmd_register(args.name, args.version, args.source)
    elif args.cmd == "list":
        cmd_list()
    elif args.cmd == "resolve":
        cmd_resolve(args.name)
    elif args.cmd == "unregister":
        cmd_unregister(args.name)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

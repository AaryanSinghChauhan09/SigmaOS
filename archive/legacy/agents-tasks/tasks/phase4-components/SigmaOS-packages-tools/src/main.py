#!/usr/bin/env python3
# SigmaOS Packages Tools — package install/upgrade/verify-sig CLI
# Upstream inspiration: pacman tooling (https://archlinux.org/pacman/)
"""
SigmaOS packages-tools: low-level package operations.

Subcommands:
  install   <pkg>...   Install packages into the in-memory record
  upgrade   <pkg>...   Upgrade packages to latest version
  verify-sig <pkg>...  Verify package signatures
  list                 List installed packages
"""

import argparse
import sys
from typing import Dict, Tuple

# In-memory package record: pkg -> (installed_version, latest_version, sig_valid)
_AVAILABLE: Dict[str, Tuple[str, bool]] = {
    "hyprland": ("0.35.0", True),
    "waybar": ("0.9.24", True),
    "mako": ("1.8.0", True),
    "foot": ("1.17.0", True),
    "grim": ("1.4.1", True),
    "slurp": ("1.5.0", True),
    "pipewire": ("1.0.0", True),
    "wireplumber": ("0.4.17", True),
}

_INSTALLED: Dict[str, str] = {}  # pkg -> installed_version


def cmd_install(pkgs: list) -> None:
    for pkg in pkgs:
        if pkg not in _AVAILABLE:
            print(f"  error: '{pkg}' not found in repository", file=sys.stderr)
            continue
        version, _ = _AVAILABLE[pkg]
        if pkg in _INSTALLED:
            print(f"  {pkg} ({_INSTALLED[pkg]}) already installed")
        else:
            _INSTALLED[pkg] = version
            print(f"  installed {pkg}-{version}")
    print(f"Done: {len(_INSTALLED)} package(s) installed total.")


def cmd_upgrade(pkgs: list) -> None:
    targets = pkgs if pkgs else list(_INSTALLED.keys())
    upgraded = 0
    for pkg in targets:
        if pkg not in _AVAILABLE:
            print(f"  error: '{pkg}' not in repository", file=sys.stderr)
            continue
        latest, _ = _AVAILABLE[pkg]
        current = _INSTALLED.get(pkg)
        if current is None:
            print(f"  {pkg}: not installed, skipping")
        elif current == latest:
            print(f"  {pkg}: already at {latest}")
        else:
            _INSTALLED[pkg] = latest
            print(f"  upgraded {pkg}: {current} -> {latest}")
            upgraded += 1
    print(f"Upgrade complete: {upgraded} package(s) upgraded.")


def cmd_verify_sig(pkgs: list) -> None:
    all_ok = True
    for pkg in pkgs:
        if pkg not in _AVAILABLE:
            print(f"  error: '{pkg}' not in repository", file=sys.stderr)
            all_ok = False
            continue
        _, sig_valid = _AVAILABLE[pkg]
        status = "VALID" if sig_valid else "INVALID"
        print(f"  {pkg}: signature {status}")
        if not sig_valid:
            all_ok = False
    if all_ok:
        print("All signatures verified.")
    else:
        print("WARNING: some signatures could not be verified.", file=sys.stderr)
        sys.exit(1)


def cmd_list() -> None:
    if not _INSTALLED:
        print("No packages installed.")
        return
    print(f"{'Package':<24} {'Version':<12}")
    print("-" * 36)
    for pkg, ver in sorted(_INSTALLED.items()):
        print(f"  {pkg:<22} {ver:<12}")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="packages-tools",
        description="SigmaOS low-level package operations",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_install = sub.add_parser("install", help="Install packages")
    p_install.add_argument("packages", nargs="+", metavar="PKG")

    p_upgrade = sub.add_parser("upgrade", help="Upgrade packages")
    p_upgrade.add_argument("packages", nargs="*", metavar="PKG",
                           help="Leave empty to upgrade all")

    p_verifysig = sub.add_parser("verify-sig", help="Verify package signatures")
    p_verifysig.add_argument("packages", nargs="+", metavar="PKG")

    sub.add_parser("list", help="List installed packages")

    args = parser.parse_args()

    if args.cmd == "install":
        cmd_install(args.packages)
    elif args.cmd == "upgrade":
        cmd_upgrade(args.packages)
    elif args.cmd == "verify-sig":
        cmd_verify_sig(args.packages)
    elif args.cmd == "list":
        cmd_list()
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

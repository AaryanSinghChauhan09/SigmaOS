"""
sigma_forge.py — SigmaOS Forge SDK (v1.0 Apex)
================================================
Backward-compat shim. Real implementation lives in sigma_forge/ package.
"""
import os
import sys
import argparse

from sigma_forge.forge_dispatcher import forge
from sigma_forge.list_templates import list_templates


class SigmaForge:
    """Thin facade over the modular sigma_forge package."""

    def forge(self, template_type, name, output_dir="userland/apps"):
        return forge(template_type, name, output_dir)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="SigmaForge: Sovereign SDK")
    parser.add_argument("cmd", choices=["new", "list"], help="Forge command")
    parser.add_argument("--type", choices=list_templates(), default="app", help="Type to forge")
    parser.add_argument("--name", help="Name of the forged object")

    args = parser.parse_args()
    f = SigmaForge()
    if args.cmd == "new" and args.name:
        print(f.forge(args.type, args.name))
    elif args.cmd == "list":
        print("Available Templates:", ", ".join(list_templates()))
    else:
        parser.print_help()

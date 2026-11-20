#!/usr/bin/env python3
# Sigma — SigmaOS meta-package and system integration entry point.
import argparse, sys

META = {
    "name": "sigma", "version": "0.9.0-alpha", "description": "SigmaOS meta-package — installs and coordinates the full SigmaOS system",
    "components": ["sigmaos-core","sigmaos-kernel","sigmaos-desktop","sigmaos-package-manager","sigmaos-security","sigmaos-network","sigmaos-theme","sigmaos-docs"],
}

def do_info(): print(f"Sigma {META['version']} — {META['description']}"); print(f"  Components ({len(META['components'])}):"); [print(f"    - {c}") for c in META["components"]]
def do_version(): print(f"sigma {META['version']}")

def build_parser():
    p = argparse.ArgumentParser(prog="sigma", description="SigmaOS meta-package entry point")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("info").set_defaults(func=lambda _: do_info() or 0)
    sub.add_parser("version").set_defaults(func=lambda _: do_version() or 0)
    sub.add_parser("components").set_defaults(func=lambda _: [print(f"  {c}") for c in META["components"]] or 0)
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))

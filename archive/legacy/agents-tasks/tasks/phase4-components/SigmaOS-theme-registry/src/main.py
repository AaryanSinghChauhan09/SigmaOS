#!/usr/bin/env python3
# SigmaOS Theme Registry — catalog of available themes with metadata and validation.
import argparse, sys
from dataclasses import dataclass, field
from typing import Dict, List

@dataclass
class ThemeEntry:
    name: str
    variant: str
    author: str
    supports: List[str]
    version: str = "1.0.0"
    compatible: bool = True

REGISTRY: Dict[str, ThemeEntry] = {
    "sigma-dark":    ThemeEntry("sigma-dark",    "dark",  "SigmaOS Team", ["gtk4","qt6","hyprland"], "1.2.0"),
    "sigma-light":   ThemeEntry("sigma-light",   "light", "SigmaOS Team", ["gtk4","qt6","hyprland"], "1.2.0"),
    "sigma-nord":    ThemeEntry("sigma-nord",    "dark",  "Community",    ["gtk4","gtk3"],            "2.1.0"),
    "sigma-gruvbox": ThemeEntry("sigma-gruvbox", "dark",  "Community",    ["gtk4"],                   "1.0.3"),
}

def do_list(variant: str = "") -> int:
    for name, t in REGISTRY.items():
        if variant and t.variant != variant:
            continue
        backends = ",".join(t.supports)
        print("  {:<25} {:<7} {:<8} [{}]".format(name, t.variant, t.version, backends))
    return 0

def do_info(name: str) -> int:
    t = REGISTRY.get(name)
    if not t:
        print("error: theme '{}' not found".format(name), file=sys.stderr)
        return 1
    print("Theme: " + t.name)
    print("  Variant  : " + t.variant)
    print("  Author   : " + t.author)
    print("  Version  : " + t.version)
    print("  Supports : " + ", ".join(t.supports))
    print("  Compatible: " + str(t.compatible))
    return 0

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigma-theme-registry")
    sub = p.add_subparsers(dest="command", required=True)
    li = sub.add_parser("list"); li.add_argument("--variant", default="")
    li.set_defaults(func=lambda a: do_list(a.variant))
    inf = sub.add_parser("info"); inf.add_argument("name")
    inf.set_defaults(func=lambda a: do_info(a.name))
    return p

def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)

if __name__ == "__main__":
    sys.exit(main())

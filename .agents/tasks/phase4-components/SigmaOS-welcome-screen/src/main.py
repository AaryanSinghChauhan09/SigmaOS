#!/usr/bin/env python3
# SigmaOS Welcome Screen — first-boot welcome and orientation guide.
import argparse, sys

SECTIONS = {
    "overview": (
        "Welcome to SigmaOS!\n"
        "  A fast, secure, multi-kernel OS built on Arch foundations with BSD backend support.\n"
        "  Performance-first. Curated defaults. Your system, your rules."
    ),
    "quick-start": (
        "Quick Start:\n"
        "  1. sigma-pkg install <package>   -- install software\n"
        "  2. sigma-theme apply sigma-dark  -- change theme\n"
        "  3. sigma-control-center          -- system settings\n"
        "  4. sigma-launcher                -- launch apps (Super key)\n"
        "  5. sigma-update check            -- check for updates"
    ),
    "learn-more": (
        "Learn More:\n"
        "  Wiki:   https://github.com/AaryanSinghChauhan09/SigmaOS/wiki\n"
        "  GitHub: https://github.com/AaryanSinghChauhan09/SigmaOS"
    ),
}

def do_show(section: str = "overview") -> int:
    text = SECTIONS.get(section)
    if not text:
        print("Unknown section '{}'. Available: {}".format(section, ", ".join(SECTIONS)))
        return 1
    print(text)
    return 0

def do_list() -> int:
    for k in SECTIONS:
        print("  " + k)
    return 0

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigma-welcome")
    sub = p.add_subparsers(dest="command", required=True)
    sh = sub.add_parser("show"); sh.add_argument("section", nargs="?", default="overview")
    sh.set_defaults(func=lambda a: do_show(a.section))
    sub.add_parser("list").set_defaults(func=lambda _: do_list())
    return p

def main() -> int:
    return build_parser().parse_args().func(build_parser().parse_args())

if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
# SigmaOS Upgrade Info — release notes, compatibility matrix, migration checks.
import argparse, sys
from dataclasses import dataclass, field
from typing import List

@dataclass
class ReleaseNote:
    version: str; codename: str; date: str; highlights: List[str] = field(default_factory=list); known_issues: List[str] = field(default_factory=list); breaking: List[str] = field(default_factory=list)

RELEASES = [
    ReleaseNote("0.9.0","Argon","2026-12-01",["8-layer architecture","Linux+BSD backends","CachyOS performance layer"],["QEMU smoke test pending","BSD backend alpha only"],["Service management API changed (see platform-contracts/service-management.md)"]),
    ReleaseNote("0.8.0","Xenon","2026-06-01",["Arch/Omarchy inspiration","162 component repos","Wiki live"],["Some component scaffolds still in prototype"],["Package manager CLI flags changed"]),
]

def do_list(): [print(f"  {r.version:<10} ({r.codename})  {r.date}") for r in RELEASES]
def do_show(ver):
    r=next((r for r in RELEASES if r.version==ver),None)
    if not r: print(f"No release notes for {ver}",file=sys.stderr); return 1
    print(f"Release {r.version} — {r.codename} ({r.date})")
    print("  Highlights:"); [print(f"    + {h}") for h in r.highlights]
    if r.breaking: print("  Breaking:"); [print(f"    ! {b}") for b in r.breaking]
    if r.known_issues: print("  Known issues:"); [print(f"    - {i}") for i in r.known_issues]
    return 0

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-upgrade-info")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list").set_defaults(func=lambda _: do_list() or 0)
    sh = sub.add_parser("show"); sh.add_argument("version"); sh.set_defaults(func=lambda a: do_show(a.version))
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))

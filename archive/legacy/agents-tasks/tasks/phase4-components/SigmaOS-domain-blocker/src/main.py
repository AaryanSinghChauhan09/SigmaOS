#!/usr/bin/env python3
# SigmaOS Domain Blocker — network domain blocking and filtering tool.
# Manages a hosts-file-based blocklist for privacy and security filtering.
"""Block/unblock domains, manage blocklists, and show blocking status."""

import argparse
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional


@dataclass
class BlocklistSource:
    name: str
    url: str
    enabled: bool = True
    entry_count: int = 0


@dataclass
class BlockEntry:
    domain: str
    reason: str = "manual"
    source: str = "user"


# In-memory state
BLOCKLIST_SOURCES: Dict[str, BlocklistSource] = {
    "hagezi-pro":   BlocklistSource("hagezi-pro",   "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/hosts/pro.txt", entry_count=400000),
    "oisd-basic":   BlocklistSource("oisd-basic",   "https://hosts.oisd.nl/basic", entry_count=150000),
    "stevenblack":  BlocklistSource("stevenblack",  "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts", entry_count=100000),
}
MANUAL_BLOCKS: Dict[str, BlockEntry] = {}
MANUAL_ALLOWLIST: List[str] = []


# ---------------------------------------------------------------------------
# Operations
# ---------------------------------------------------------------------------

def do_block(domain: str, reason: str = "manual") -> int:
    MANUAL_BLOCKS[domain] = BlockEntry(domain=domain, reason=reason)
    print(f"[blocker] Blocked: {domain}  (reason: {reason})")
    print(f"  (Real impl: adds '0.0.0.0 {domain}' to /etc/hosts.d/sigma-blocklist)")
    return 0


def do_unblock(domain: str) -> int:
    if domain not in MANUAL_BLOCKS:
        print(f"warning: '{domain}' not in manual block list.", file=sys.stderr)
        return 0
    del MANUAL_BLOCKS[domain]
    print(f"[blocker] Unblocked: {domain}")
    return 0


def do_allow(domain: str) -> int:
    if domain not in MANUAL_ALLOWLIST:
        MANUAL_ALLOWLIST.append(domain)
    print(f"[blocker] Allowlisted: {domain}  (overrides all blocklists)")
    return 0


def do_status() -> int:
    total = sum(s.entry_count for s in BLOCKLIST_SOURCES.values() if s.enabled)
    print(f"Domain blocker status:")
    print(f"  Active sources  : {sum(1 for s in BLOCKLIST_SOURCES.values() if s.enabled)}")
    print(f"  Blocked domains : ~{total:,}")
    print(f"  Manual blocks   : {len(MANUAL_BLOCKS)}")
    print(f"  Allowlisted     : {len(MANUAL_ALLOWLIST)}")
    print()
    for s in BLOCKLIST_SOURCES.values():
        state = "✓" if s.enabled else "✗"
        print(f"  {state} {s.name:<20} ~{s.entry_count:>8,} entries")
    if MANUAL_BLOCKS:
        print(f"\nManual blocks ({len(MANUAL_BLOCKS)}):")
        for b in MANUAL_BLOCKS.values():
            print(f"  {b.domain}  [{b.reason}]")
    return 0


def do_update() -> int:
    for s in BLOCKLIST_SOURCES.values():
        if s.enabled:
            print(f"[blocker] Updating '{s.name}' from {s.url}")
    print(f"  (Real impl: downloads list, deduplicates, refreshes /etc/hosts.d/)")
    print("Blocklists updated.")
    return 0


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="sigma-blocker",
        description="SigmaOS domain blocker: hosts-based DNS-level blocking",
    )
    sub = p.add_subparsers(dest="command", required=True)

    bl = sub.add_parser("block", help="Block a domain")
    bl.add_argument("domain")
    bl.add_argument("--reason", default="manual")
    bl.set_defaults(func=lambda a: do_block(a.domain, a.reason))

    ub = sub.add_parser("unblock", help="Remove a manual block")
    ub.add_argument("domain")
    ub.set_defaults(func=lambda a: do_unblock(a.domain))

    al = sub.add_parser("allow", help="Allowlist a domain (overrides blocklists)")
    al.add_argument("domain")
    al.set_defaults(func=lambda a: do_allow(a.domain))

    sub.add_parser("status", help="Show blocking status").set_defaults(func=lambda _: do_status())
    sub.add_parser("update", help="Update blocklists").set_defaults(func=lambda _: do_update())

    return p


def main() -> int:
    args = build_parser().parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())

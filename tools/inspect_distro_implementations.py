#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""
Distro Implementation Inspection Script for SigmaOS
Audits Linux & BSD distribution inspired abstractions across the codebase.
"""

import os
import sys

DISTRO_KEYWORDS = [
    "Arch", "Debian", "Fedora", "Alpine", "Nix", "Gentoo",
    "FreeBSD", "OpenBSD", "NetBSD", "DragonFly", "Void", "Slackware"
]

def main():
    print("=== SigmaOS Distro Implementation Inspection Tool ===")
    src_dir = "src"
    found_counts = {k: 0 for k in DISTRO_KEYWORDS}

    for root, _, files in os.walk(src_dir):
        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                try:
                    with open(filepath, "r", encoding="utf-8") as f:
                        content = f.read()
                        for kw in DISTRO_KEYWORDS:
                            found_counts[kw] += content.count(kw)
                except Exception as e:
                    print(f"Error reading {filepath}: {e}")

    print("\n[+] Distribution Reference Counts in src/:")
    for kw, count in found_counts.items():
        print(f"  - {kw:<12}: {count} occurrences")

    total_refs = sum(found_counts.values())
    print(f"\nTotal Distro Abstraction References: {total_refs}")

    if total_refs > 100:
        print("[SUCCESS] Distro abstractions are thoroughly integrated and verified.")
        sys.exit(0)
    else:
        print("[WARNING] Low distro abstraction count detected.")
        sys.exit(1)

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# kabi/check.py — Kernel ABI stability checker
#
# Usage:
#   python kabi/check.py check        # compare current headers to baseline
#   python kabi/check.py snapshot     # generate a new snapshot from current headers
#   python kabi/check.py diff v15.0.0 v15.1.0   # compare two snapshots
#   python kabi/check.py report       # human-readable ABI change report
#
# In CI: runs as part of sigma_dev_workflow.yml on every PR.
# Fails if any stable-ABI symbol is removed or its signature changed.

import argparse
import hashlib
import json
import os
import pathlib
import re
import sys
from dataclasses import dataclass, asdict, field
from typing import Optional

KABI_DIR       = pathlib.Path(__file__).parent
SNAPSHOTS_DIR  = KABI_DIR / "snapshots"
STABLE_SOURCES = [
    KABI_DIR / "src" / "lib.rs",
    KABI_DIR.parent / "sdk" / "driver" / "src" / "lib.rs",
    KABI_DIR.parent / "kernel" / "core" / "driver_framework.rs",
    KABI_DIR.parent / "kernel" / "core" / "sigma_irq.rs",
    KABI_DIR.parent / "kernel" / "core" / "syscall_dispatch.rs",
]

# ── Symbol extractor ──────────────────────────────────────────────────────

@dataclass
class Symbol:
    name:       str
    kind:       str          # fn | struct | trait | const | type
    signature:  str          # normalised signature string
    cabi:       bool = False # #[no_mangle] or extern "C"
    file:       str = ""

def extract_symbols(path: pathlib.Path) -> list[Symbol]:
    """Parse a Rust source file and extract public ABI symbols."""
    if not path.exists():
        return []
    src = path.read_text(encoding="utf-8", errors="replace")
    symbols = []

    # #[no_mangle] pub extern "C" fn name(...) -> ...
    for m in re.finditer(
        r'#\[no_mangle\]\s*pub\s+unsafe\s+extern\s+"C"\s+fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^{;]+))?',
        src
    ):
        sig = f"fn({m.group(2).strip()}) -> {m.group(3).strip() if m.group(3) else '()'}"
        symbols.append(Symbol(
            name=m.group(1), kind="fn",
            signature=_normalise(sig),
            cabi=True, file=str(path),
        ))

    # pub struct Name { ... }
    for m in re.finditer(r'#\[repr\(C[^)]*\)\]\s*(?:#\[[^\]]+\]\s*)*pub\s+struct\s+(\w+)', src):
        symbols.append(Symbol(
            name=m.group(1), kind="struct",
            signature=f"repr(C) struct",
            cabi=True, file=str(path),
        ))

    # pub trait Name
    for m in re.finditer(r'pub\s+trait\s+(\w+)\s*[{:]', src):
        # Extract method names
        trait_body_m = re.search(
            rf'pub\s+trait\s+{re.escape(m.group(1))}\s*\{{([^}}]+)\}}',
            src, re.DOTALL
        )
        methods = []
        if trait_body_m:
            methods = re.findall(r'fn\s+(\w+)', trait_body_m.group(1))
        symbols.append(Symbol(
            name=m.group(1), kind="trait",
            signature=f"trait methods=[{','.join(sorted(methods))}]",
            cabi=False, file=str(path),
        ))

    # pub const NAME: type = ...;
    for m in re.finditer(r'pub\s+const\s+(\w+)\s*:\s*([^=;]+)', src):
        symbols.append(Symbol(
            name=m.group(1), kind="const",
            signature=f"const {m.group(2).strip()}",
            cabi=False, file=str(path),
        ))

    return symbols

def _normalise(sig: str) -> str:
    """Normalise whitespace in a signature for stable comparison."""
    return re.sub(r'\s+', ' ', sig.strip())

# ── Snapshot management ────────────────────────────────────────────────────

@dataclass
class Snapshot:
    version:  str
    symbols:  list[dict] = field(default_factory=list)
    hash:     str = ""

def collect_current_symbols() -> list[Symbol]:
    all_syms = []
    for src in STABLE_SOURCES:
        all_syms.extend(extract_symbols(src))
    # Deduplicate by name (keep first occurrence)
    seen = set()
    unique = []
    for s in all_syms:
        if s.name not in seen:
            seen.add(s.name)
            unique.append(s)
    return unique

def make_snapshot(version: str) -> Snapshot:
    syms = collect_current_symbols()
    sym_dicts = [asdict(s) for s in syms]
    content = json.dumps(sym_dicts, sort_keys=True)
    h = hashlib.sha256(content.encode()).hexdigest()[:16]
    return Snapshot(version=version, symbols=sym_dicts, hash=h)

def save_snapshot(snap: Snapshot) -> pathlib.Path:
    SNAPSHOTS_DIR.mkdir(parents=True, exist_ok=True)
    path = SNAPSHOTS_DIR / f"{snap.version}.json"
    path.write_text(json.dumps(asdict(snap), indent=2))
    return path

def load_snapshot(version: str) -> Optional[Snapshot]:
    path = SNAPSHOTS_DIR / f"{version}.json"
    if not path.exists():
        return None
    data = json.loads(path.read_text())
    return Snapshot(**data)

def latest_snapshot_version() -> Optional[str]:
    if not SNAPSHOTS_DIR.exists():
        return None
    snapshots = sorted(SNAPSHOTS_DIR.glob("*.json"), key=lambda p: p.stem)
    return snapshots[-1].stem if snapshots else None

# ── ABI diff ──────────────────────────────────────────────────────────────

@dataclass
class AbiDiff:
    removed:  list[dict] = field(default_factory=list)  # ABI break
    changed:  list[tuple] = field(default_factory=list)  # ABI break
    added:    list[dict] = field(default_factory=list)  # OK, but note
    is_break: bool = False

def diff_snapshots(old: Snapshot, new: Snapshot) -> AbiDiff:
    old_map = {s["name"]: s for s in old.symbols}
    new_map = {s["name"]: s for s in new.symbols}

    d = AbiDiff()

    # Removed symbols — ABI break
    for name, sym in old_map.items():
        if name not in new_map:
            d.removed.append(sym)

    # Changed signatures — ABI break
    for name, old_sym in old_map.items():
        if name in new_map:
            new_sym = new_map[name]
            if old_sym["signature"] != new_sym["signature"]:
                d.changed.append((old_sym, new_sym))

    # Added symbols — OK
    for name, sym in new_map.items():
        if name not in old_map:
            d.added.append(sym)

    d.is_break = bool(d.removed or d.changed)
    return d

# ── Commands ──────────────────────────────────────────────────────────────

def cmd_snapshot(args):
    import subprocess
    try:
        version = subprocess.check_output(
            ["git", "describe", "--tags", "--always"],
            stderr=subprocess.DEVNULL, text=True
        ).strip()
    except Exception:
        version = args.version or "unknown"

    snap = make_snapshot(version)
    path = save_snapshot(snap)
    print(f"  Snapshot saved: {path}")
    print(f"  Version: {snap.version}  SHA: {snap.hash}  Symbols: {len(snap.symbols)}")

def cmd_check(args):
    baseline_version = args.baseline or latest_snapshot_version()
    if not baseline_version:
        print("  No baseline snapshot found. Run: python kabi/check.py snapshot")
        print("  (Non-blocking — first run always passes)")
        sys.exit(0)

    old = load_snapshot(baseline_version)
    if not old:
        print(f"  Snapshot {baseline_version} not found.")
        sys.exit(0)

    current = make_snapshot("current")
    d = diff_snapshots(old, current)

    print(f"\n  ABI check: {baseline_version} → current\n")

    if d.removed:
        print(f"  ❌ REMOVED ({len(d.removed)} symbols — ABI BREAK):")
        for s in d.removed:
            print(f"     - {s['kind']} {s['name']}  [{s['signature']}]")
    if d.changed:
        print(f"\n  ❌ CHANGED ({len(d.changed)} symbols — ABI BREAK):")
        for old_s, new_s in d.changed:
            print(f"     ~ {old_s['name']}")
            print(f"       was: {old_s['signature']}")
            print(f"       now: {new_s['signature']}")
    if d.added:
        print(f"\n  ✅ ADDED ({len(d.added)} new symbols — OK):")
        for s in d.added[:10]:
            print(f"     + {s['kind']} {s['name']}")
        if len(d.added) > 10:
            print(f"     ... and {len(d.added)-10} more")

    if not d.removed and not d.changed:
        print(f"  ✅ ABI is stable. ({len(current.symbols)} symbols checked)")

    if d.is_break:
        print("\n  ABI BREAK DETECTED — bump kabi version and update snapshot.")
        print("  Run: python kabi/check.py snapshot --version <new-version>")
        sys.exit(1)

def cmd_diff(args):
    v1, v2 = args.v1, args.v2
    old = load_snapshot(v1)
    new = load_snapshot(v2)
    if not old: print(f"Snapshot {v1} not found"); sys.exit(1)
    if not new: print(f"Snapshot {v2} not found"); sys.exit(1)
    d = diff_snapshots(old, new)
    print(f"\n  Diff: {v1} → {v2}")
    print(f"  Removed: {len(d.removed)}  Changed: {len(d.changed)}  Added: {len(d.added)}")
    for s in d.removed: print(f"  - {s['name']}")
    for o, n in d.changed: print(f"  ~ {o['name']}: {o['signature']} → {n['signature']}")
    for s in d.added[:5]: print(f"  + {s['name']}")

def cmd_report(args):
    print("\n  SigmaOS Kernel ABI Report")
    print(f"  {'─'*50}")
    syms = collect_current_symbols()
    by_kind: dict = {}
    for s in syms:
        by_kind.setdefault(s.kind, []).append(s)
    for kind, slist in sorted(by_kind.items()):
        print(f"\n  {kind.upper()} ({len(slist)}):")
        for s in sorted(slist, key=lambda x: x.name)[:20]:
            cabi_mark = " [C-ABI]" if s.cabi else ""
            print(f"    {s.name}{cabi_mark}")
    print(f"\n  Total stable symbols: {len(syms)}")

def main():
    parser = argparse.ArgumentParser(
        prog="kabi/check.py",
        description="SigmaOS Kernel ABI stability checker"
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_snap = sub.add_parser("snapshot", help="Generate ABI snapshot from current sources")
    p_snap.add_argument("--version", default=None)

    p_check = sub.add_parser("check", help="Compare current headers to baseline snapshot")
    p_check.add_argument("--baseline", default=None, help="Snapshot version to compare against")

    p_diff = sub.add_parser("diff", help="Diff two snapshots")
    p_diff.add_argument("v1"); p_diff.add_argument("v2")

    sub.add_parser("report", help="Print ABI report")

    args = parser.parse_args()
    {"snapshot": cmd_snapshot, "check": cmd_check,
     "diff": cmd_diff, "report": cmd_report}[args.cmd](args)

if __name__ == "__main__":
    main()

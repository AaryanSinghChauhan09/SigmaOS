#!/usr/bin/env python3
# SigmaOS Renamer Bulk — batch file renaming CLI scaffold
# Upstream: Thunar bulk-rename, KRename (https://krename.net/)
"""
SigmaOS renamer-bulk: batch-rename files using patterns and rules.
Subcommands:
  preview  <dir> [options]   Show what renames would occur (dry-run)
  apply    <dir> [options]   Apply the rename rules to files in <dir>
  undo     <log-file>        Undo a previous bulk-rename using its log
  rules                      List available rename rule types

Options (for preview / apply):
  --pattern  <p>    Regex pattern to match filenames
  --replace  <r>    Replacement string (supports backreferences \1, \2…)
  --prefix   <s>    Add prefix to each name
  --suffix   <s>    Add suffix before the extension
  --numbering       Append sequential number: name_001, name_002, …
  --start    <n>    Starting number for --numbering (default 1)
  --lower           Lowercase all filenames
  --upper           Uppercase all filenames
  --ext      <e>    Only rename files with this extension
  --dry-run         Alias for preview subcommand
"""
import argparse
import os
import re
import sys
from typing import List, Tuple, Optional


def compute_renames(
    directory: str,
    pattern: Optional[str],
    replace: Optional[str],
    prefix: str,
    suffix: str,
    numbering: bool,
    start: int,
    lower: bool,
    upper: bool,
    ext_filter: Optional[str],
) -> List[Tuple[str, str]]:
    """Return list of (old_name, new_name) pairs."""
    try:
        entries = sorted(os.listdir(directory))
    except OSError as e:
        print(f"[renamer] Cannot read directory: {e}", file=sys.stderr)
        return []

    renames: List[Tuple[str, str]] = []
    counter = start

    for name in entries:
        full = os.path.join(directory, name)
        if os.path.isdir(full):
            continue
        base, ext = os.path.splitext(name)

        # Extension filter
        if ext_filter and ext.lstrip(".").lower() != ext_filter.lower():
            continue

        new_base = base

        # Regex replace
        if pattern and replace is not None:
            new_base = re.sub(pattern, replace, new_base)

        # Case
        if lower:
            new_base = new_base.lower()
        elif upper:
            new_base = new_base.upper()

        # Prefix / suffix
        new_base = f"{prefix}{new_base}{suffix}"

        # Numbering
        if numbering:
            new_base = f"{new_base}_{counter:03d}"
            counter += 1

        new_name = f"{new_base}{ext}"
        if new_name != name:
            renames.append((name, new_name))

    return renames


def cmd_preview(args) -> None:
    renames = compute_renames(
        args.dir, args.pattern, args.replace,
        args.prefix or "", args.suffix or "",
        args.numbering, args.start, args.lower, args.upper, args.ext,
    )
    if not renames:
        print("[renamer] No files would be renamed")
        return
    print(f"[renamer] {len(renames)} rename(s) would occur:")
    for old, new in renames:
        print(f"  {old:40s} → {new}")


def cmd_apply(args) -> None:
    renames = compute_renames(
        args.dir, args.pattern, args.replace,
        args.prefix or "", args.suffix or "",
        args.numbering, args.start, args.lower, args.upper, args.ext,
    )
    if not renames:
        print("[renamer] Nothing to rename")
        return
    log_path = os.path.join(args.dir, ".sigma-renamer-undo.log")
    applied = 0
    with open(log_path, "w") as log:
        for old, new in renames:
            src = os.path.join(args.dir, old)
            dst = os.path.join(args.dir, new)
            if os.path.exists(dst):
                print(f"[renamer] Skip (target exists): {new}", file=sys.stderr)
                continue
            os.rename(src, dst)
            log.write(f"{new}\t{old}\n")
            print(f"[renamer] Renamed: {old} → {new}")
            applied += 1
    print(f"[renamer] Done: {applied} file(s) renamed. Undo log: {log_path}")


def cmd_undo(args) -> None:
    try:
        with open(args.log_file) as f:
            lines = f.readlines()
    except OSError as e:
        print(f"[renamer] Cannot read log: {e}", file=sys.stderr)
        return
    directory = os.path.dirname(os.path.abspath(args.log_file))
    for line in reversed(lines):
        parts = line.strip().split("\t")
        if len(parts) != 2:
            continue
        current, original = parts
        src = os.path.join(directory, current)
        dst = os.path.join(directory, original)
        if os.path.exists(src):
            os.rename(src, dst)
            print(f"[renamer] Restored: {current} → {original}")
    print("[renamer] Undo complete")


def cmd_rules(_args) -> None:
    print("Available rename rules:")
    print("  --pattern / --replace  Regex substitution (Python re.sub)")
    print("  --prefix               Prepend string to basename")
    print("  --suffix               Append string before extension")
    print("  --numbering            Append sequential 3-digit counter")
    print("  --lower                Lowercase the entire basename")
    print("  --upper                Uppercase the entire basename")
    print("  --ext                  Filter by file extension")


def add_rename_opts(p: argparse.ArgumentParser) -> None:
    p.add_argument("dir",                        help="Target directory")
    p.add_argument("--pattern",  default=None,   help="Regex pattern to match")
    p.add_argument("--replace",  default=None,   help="Replacement string")
    p.add_argument("--prefix",   default=None)
    p.add_argument("--suffix",   default=None)
    p.add_argument("--numbering", action="store_true")
    p.add_argument("--start",    type=int, default=1)
    p.add_argument("--lower",    action="store_true")
    p.add_argument("--upper",    action="store_true")
    p.add_argument("--ext",      default=None,   help="Filter by extension (without dot)")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-renamer-bulk",
                                description="SigmaOS batch file renamer")
    sub = p.add_subparsers(dest="cmd", required=True)
    add_rename_opts(sub.add_parser("preview", help="Preview renames (dry-run)"))
    add_rename_opts(sub.add_parser("apply",   help="Apply renames"))
    undo_p = sub.add_parser("undo", help="Undo a previous bulk-rename")
    undo_p.add_argument("log_file", help="Undo log file produced by apply")
    sub.add_parser("rules", help="List available rename rule types")
    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    {"preview": cmd_preview, "apply": cmd_apply,
     "undo": cmd_undo, "rules": cmd_rules}[args.cmd](args)
    return 0


if __name__ == "__main__":
    sys.exit(main())

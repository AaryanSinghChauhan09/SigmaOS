#!/usr/bin/env python3
# SigmaOS Renamer File — single-file interactive rename CLI scaffold
# Upstream: Thunar custom-action rename, Dolphin inline-rename
"""
SigmaOS renamer-file: rename a single file interactively or via rules.
Subcommands:
  rename  <file> <new-name>        Rename a single file directly
  regex   <file> --pattern <p> --replace <r>   Regex rename
  trim    <file> [--leading <n>] [--trailing <n>]  Trim characters
  case    <file> --mode lower|upper|title|camel   Change case
  number  <file> --pos prefix|suffix --start <n>  Add number
  ext     <file> <new-ext>         Change the extension
  info    <file>                   Print current name analysis
"""
import argparse
import os
import re
import sys
from typing import List, Optional


def safe_rename(src: str, dst: str) -> bool:
    if not os.path.exists(src):
        print(f"[renamer] Source not found: {src}", file=sys.stderr)
        return False
    if os.path.exists(dst):
        print(f"[renamer] Target already exists: {dst}", file=sys.stderr)
        return False
    os.rename(src, dst)
    print(f"[renamer] {os.path.basename(src)} → {os.path.basename(dst)}")
    return True


def cmd_rename(args) -> None:
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, args.new_name)
    safe_rename(args.file, dst)


def cmd_regex(args) -> None:
    base, ext = os.path.splitext(os.path.basename(args.file))
    new_base = re.sub(args.pattern, args.replace, base)
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, f"{new_base}{ext}")
    safe_rename(args.file, dst)


def cmd_trim(args) -> None:
    base, ext = os.path.splitext(os.path.basename(args.file))
    new_base = base
    if args.leading:
        new_base = new_base[args.leading:]
    if args.trailing:
        new_base = new_base[:-args.trailing] if args.trailing else new_base
    if not new_base:
        print("[renamer] Trim would produce an empty name", file=sys.stderr)
        return
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, f"{new_base}{ext}")
    safe_rename(args.file, dst)


def cmd_case(args) -> None:
    base, ext = os.path.splitext(os.path.basename(args.file))
    mode = args.mode
    if mode == "lower":
        new_base = base.lower()
    elif mode == "upper":
        new_base = base.upper()
    elif mode == "title":
        new_base = base.title()
    elif mode == "camel":
        words = re.split(r"[\s_\-]+", base)
        new_base = words[0].lower() + "".join(w.title() for w in words[1:])
    else:
        print(f"[renamer] Unknown case mode: {mode}", file=sys.stderr)
        return
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, f"{new_base}{ext}")
    safe_rename(args.file, dst)


def cmd_number(args) -> None:
    base, ext = os.path.splitext(os.path.basename(args.file))
    n = f"{args.start:03d}"
    new_base = f"{n}_{base}" if args.pos == "prefix" else f"{base}_{n}"
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, f"{new_base}{ext}")
    safe_rename(args.file, dst)


def cmd_ext(args) -> None:
    base = os.path.splitext(os.path.basename(args.file))[0]
    new_ext = args.new_ext if args.new_ext.startswith(".") else f".{args.new_ext}"
    directory = os.path.dirname(os.path.abspath(args.file))
    dst = os.path.join(directory, f"{base}{new_ext}")
    safe_rename(args.file, dst)


def cmd_info(args) -> None:
    path = args.file
    base, ext = os.path.splitext(os.path.basename(path))
    print(f"Path      : {os.path.abspath(path)}")
    print(f"Directory : {os.path.dirname(os.path.abspath(path))}")
    print(f"Basename  : {os.path.basename(path)}")
    print(f"Stem      : {base}")
    print(f"Extension : {ext or '(none)'}")
    print(f"Length    : {len(base)} chars (stem), {len(os.path.basename(path))} total")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-renamer-file",
                                description="SigmaOS single-file renamer")
    sub = p.add_subparsers(dest="cmd", required=True)

    r = sub.add_parser("rename", help="Direct rename")
    r.add_argument("file"); r.add_argument("new_name")

    rx = sub.add_parser("regex", help="Regex rename")
    rx.add_argument("file")
    rx.add_argument("--pattern", required=True)
    rx.add_argument("--replace", required=True)

    tr = sub.add_parser("trim", help="Trim leading/trailing chars from stem")
    tr.add_argument("file")
    tr.add_argument("--leading",  type=int, default=0)
    tr.add_argument("--trailing", type=int, default=0)

    ca = sub.add_parser("case", help="Change case of stem")
    ca.add_argument("file")
    ca.add_argument("--mode", choices=["lower", "upper", "title", "camel"], required=True)

    nb = sub.add_parser("number", help="Prepend or append a number")
    nb.add_argument("file")
    nb.add_argument("--pos",   choices=["prefix", "suffix"], default="suffix")
    nb.add_argument("--start", type=int, default=1)

    ex = sub.add_parser("ext", help="Change extension")
    ex.add_argument("file"); ex.add_argument("new_ext")

    inf = sub.add_parser("info", help="Analyze current filename")
    inf.add_argument("file")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    {"rename": cmd_rename, "regex": cmd_regex, "trim": cmd_trim,
     "case": cmd_case, "number": cmd_number, "ext": cmd_ext,
     "info": cmd_info}[args.cmd](args)
    return 0


if __name__ == "__main__":
    sys.exit(main())

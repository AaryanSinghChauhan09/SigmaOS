#!/usr/bin/env python3
# SigmaOS QuickShell — keyboard-driven fuzzy launcher
# Upstream inspiration: wofi (https://hg.sr.ht/~scoopta/wofi),
#                       walker (https://github.com/abenz1267/walker),
#                       rofi (https://github.com/davatorium/rofi)
"""
SigmaOS quickshell: fuzzy-scored application launcher.

Subcommands:
  query  <text>    Fuzzy-search entries and print ranked matches
  launch <name>    Launch the best-matching entry
  list             List all known entries
"""

import argparse
import sys
from typing import List, Tuple

# ---------------------------------------------------------------------------
# .desktop-style entry catalogue
# ---------------------------------------------------------------------------

ENTRIES = [
    {"name": "Terminal", "exec": "foot", "keywords": ["term", "shell", "console"]},
    {"name": "Files", "exec": "nautilus", "keywords": ["files", "folder", "browse"]},
    {"name": "Browser", "exec": "chromium", "keywords": ["web", "internet", "chrome"]},
    {"name": "Firefox", "exec": "firefox", "keywords": ["web", "browser", "internet"]},
    {"name": "Text Editor", "exec": "helix", "keywords": ["editor", "code", "text"]},
    {"name": "Settings", "exec": "sigma-settings", "keywords": ["settings", "prefs", "config"]},
    {"name": "System Monitor", "exec": "btop", "keywords": ["cpu", "memory", "monitor", "btop"]},
    {"name": "Neovim", "exec": "nvim", "keywords": ["editor", "vim", "code"]},
    {"name": "Hyprland Config", "exec": "helix ~/.config/hypr/hyprland.conf",
     "keywords": ["hypr", "wm", "config"]},
]


# ---------------------------------------------------------------------------
# Fuzzy score (subsequence + consecutive bonus)
# ---------------------------------------------------------------------------

def fuzzy_score(query: str, name: str) -> int:
    """Subsequence match with consecutive-character bonus."""
    if not query:
        return 1
    q = query.lower()
    n = name.lower()
    score = 0
    qi = 0
    last_match = -2
    for ni, ch in enumerate(n):
        if qi < len(q) and ch == q[qi]:
            score += 10
            if ni == last_match + 1:
                score += 5  # consecutive bonus
            last_match = ni
            qi += 1
    if qi < len(q):
        return 0  # not a subsequence
    if n.startswith(q):
        score += 20  # prefix bonus
    return score


def search_entries(query: str) -> List[Tuple[int, dict]]:
    results = []
    for entry in ENTRIES:
        score = fuzzy_score(query, entry["name"])
        # Also check keywords
        for kw in entry.get("keywords", []):
            s = fuzzy_score(query, kw)
            if s > score:
                score = s
        if score > 0:
            results.append((score, entry))
    results.sort(key=lambda x: x[0], reverse=True)
    return results


# ---------------------------------------------------------------------------
# Subcommands
# ---------------------------------------------------------------------------

def cmd_query(text: str) -> None:
    results = search_entries(text)
    if not results:
        print(f"No matches for '{text}'.")
        return
    print(f"{'Score':<8} {'Name':<20} {'Exec'}")
    print("-" * 52)
    for score, entry in results:
        print(f"  {score:<6} {entry['name']:<20} {entry['exec']}")


def cmd_launch(name: str) -> None:
    results = search_entries(name)
    if not results:
        print(f"error: no match for '{name}'", file=sys.stderr)
        sys.exit(1)
    best_score, best_entry = results[0]
    print(f"Launching '{best_entry['name']}': {best_entry['exec']}")
    # In a real implementation: os.execvp / subprocess.Popen
    # Here we just print the command (test-safe).


def cmd_list() -> None:
    print(f"{'Name':<24} {'Exec'}")
    print("-" * 50)
    for entry in sorted(ENTRIES, key=lambda e: e["name"]):
        print(f"  {entry['name']:<22} {entry['exec']}")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="quickshell",
        description="SigmaOS keyboard-driven fuzzy launcher",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_query = sub.add_parser("query", help="Fuzzy-search entries")
    p_query.add_argument("text", help="Search text")

    p_launch = sub.add_parser("launch", help="Launch best-matching entry")
    p_launch.add_argument("name", help="App name or query")

    sub.add_parser("list", help="List all entries")

    args = parser.parse_args()

    if args.cmd == "query":
        cmd_query(args.text)
    elif args.cmd == "launch":
        cmd_launch(args.name)
    elif args.cmd == "list":
        cmd_list()
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

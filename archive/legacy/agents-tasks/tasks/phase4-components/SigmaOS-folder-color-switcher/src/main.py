#!/usr/bin/env python3
"""SigmaOS folder-color-switcher: folder color customizer.

Integrates with the file manager context menu to tag directories with
custom icon colors from the current theme.
"""

import sys
from dataclasses import dataclass


@dataclass
class ColorChoice:
    """A color that can be applied to a folder."""

    name: str
    icon_suffix: str


DEFAULT_COLORS = [
    ColorChoice("blue", "blue"),
    ColorChoice("green", "green"),
    ColorChoice("red", "red"),
    ColorChoice("yellow", "yellow"),
    ColorChoice("purple", "purple"),
]


def apply_color(folder: str, color: str) -> int:
    """Apply a color to a folder, returning the exit code."""
    if color not in {c.name for c in DEFAULT_COLORS}:
        print(f"unknown color: {color}", file=sys.stderr)
        return 1
    # Real implementation writes a .directory file with the themed icon.
    print(f"sigma-folder-color: would set {folder} to {color}")
    return 0


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: sigma-folder-color <folder> <color>", file=sys.stderr)
        return 1
    return apply_color(sys.argv[1], sys.argv[2])


if __name__ == "__main__":
    raise SystemExit(main())

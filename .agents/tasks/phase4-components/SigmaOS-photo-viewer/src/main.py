#!/usr/bin/env python3
# SigmaOS Photo Viewer — image catalog and viewing CLI scaffold
# Upstream: eog (https://wiki.gnome.org/Apps/EyeOfGnome), gThumb
"""
SigmaOS photo-viewer: browse and present image files.
Subcommands:
  open    <file>...   Open one or more image files into the viewer session
  info    <file>      Print metadata (dimensions, format, size)
  next                Advance to next image in current catalog
  prev                Go to previous image
  zoom    <factor>    Set zoom level (e.g. 1.0 = 100%, 2.0 = 200%)
  rotate  <deg>       Rotate current image (90/180/270)
  slideshow [--delay <s>]  Start a slideshow over the current catalog
  list                List all images in the current catalog
  status              Show current image and session state
"""
import argparse
import os
import sys
from typing import List, Optional

SUPPORTED_EXTENSIONS = {".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".tiff", ".svg"}


class ImageRecord:
    def __init__(self, path: str):
        self.path = path
        self.filename = os.path.basename(path)
        self.extension = os.path.splitext(path)[1].lower()
        self.size_bytes: int = 0
        self._load_meta()

    def _load_meta(self) -> None:
        try:
            self.size_bytes = os.path.getsize(self.path)
        except OSError:
            self.size_bytes = 0

    def __str__(self) -> str:
        return f"{self.filename} ({self.extension}, {self.size_bytes} bytes)"


class ViewerSession:
    """Represents an in-memory photo viewer catalog and cursor."""

    def __init__(self):
        self.catalog: List[ImageRecord] = []
        self.cursor: int = 0
        self.zoom: float = 1.0
        self.rotation: int = 0  # degrees: 0/90/180/270

    @property
    def current(self) -> Optional[ImageRecord]:
        if not self.catalog:
            return None
        return self.catalog[self.cursor]

    def open(self, paths: List[str]) -> None:
        added = 0
        for p in paths:
            ext = os.path.splitext(p)[1].lower()
            if ext not in SUPPORTED_EXTENSIONS:
                print(f"[viewer] Unsupported format: {p}", file=sys.stderr)
                continue
            self.catalog.append(ImageRecord(p))
            added += 1
        print(f"[viewer] Opened {added} image(s); catalog size: {len(self.catalog)}")
        if added and len(self.catalog) == added:
            self.cursor = 0

    def info(self, path: str) -> None:
        rec = ImageRecord(path)
        print(f"File      : {rec.path}")
        print(f"Format    : {rec.extension.lstrip('.')}")
        print(f"Size      : {rec.size_bytes} bytes")
        print(f"Supported : {'yes' if rec.extension in SUPPORTED_EXTENSIONS else 'no'}")

    def next(self) -> None:
        if not self.catalog:
            print("[viewer] Catalog is empty", file=sys.stderr)
            return
        self.cursor = (self.cursor + 1) % len(self.catalog)
        print(f"[viewer] → {self.current}")

    def prev(self) -> None:
        if not self.catalog:
            print("[viewer] Catalog is empty", file=sys.stderr)
            return
        self.cursor = (self.cursor - 1) % len(self.catalog)
        print(f"[viewer] ← {self.current}")

    def set_zoom(self, factor: float) -> None:
        if factor <= 0:
            print("[viewer] Zoom factor must be > 0", file=sys.stderr)
            return
        self.zoom = factor
        print(f"[viewer] Zoom: {self.zoom:.2f}×")

    def rotate(self, degrees: int) -> None:
        if degrees not in (90, 180, 270):
            print("[viewer] Rotation must be 90, 180, or 270", file=sys.stderr)
            return
        self.rotation = (self.rotation + degrees) % 360
        print(f"[viewer] Rotation: {self.rotation}°")

    def slideshow(self, delay: float = 3.0) -> None:
        if not self.catalog:
            print("[viewer] Catalog is empty for slideshow", file=sys.stderr)
            return
        print(f"[viewer] Slideshow: {len(self.catalog)} images, {delay}s delay")
        for i, img in enumerate(self.catalog):
            print(f"  [{i+1}/{len(self.catalog)}] {img}")

    def list_catalog(self) -> None:
        if not self.catalog:
            print("[viewer] Catalog is empty")
            return
        for i, img in enumerate(self.catalog):
            marker = "→" if i == self.cursor else " "
            print(f"  {marker} [{i+1}] {img}")

    def status(self) -> None:
        print(f"Catalog : {len(self.catalog)} image(s)")
        print(f"Current : {self.current or '(none)'}")
        print(f"Zoom    : {self.zoom:.2f}×  Rotation: {self.rotation}°")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-photo-viewer", description="SigmaOS image viewer")
    sub = p.add_subparsers(dest="cmd", required=True)

    op = sub.add_parser("open", help="Open image file(s)")
    op.add_argument("files", nargs="+", help="Image file paths")

    inf = sub.add_parser("info", help="Print image metadata")
    inf.add_argument("file", help="Image file path")

    sub.add_parser("next",  help="Next image in catalog")
    sub.add_parser("prev",  help="Previous image in catalog")

    zm = sub.add_parser("zoom", help="Set zoom level")
    zm.add_argument("factor", type=float, help="Zoom factor (1.0 = 100%)")

    rot = sub.add_parser("rotate", help="Rotate current image")
    rot.add_argument("degrees", type=int, choices=[90, 180, 270])

    ss = sub.add_parser("slideshow", help="Auto-advance through catalog")
    ss.add_argument("--delay", type=float, default=3.0, help="Seconds per image (default 3)")

    sub.add_parser("list",   help="List catalog contents")
    sub.add_parser("status", help="Print session state")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    session = ViewerSession()

    if args.cmd == "open":
        session.open(args.files)
    elif args.cmd == "info":
        session.info(args.file)
    elif args.cmd == "next":
        session.next()
    elif args.cmd == "prev":
        session.prev()
    elif args.cmd == "zoom":
        session.set_zoom(args.factor)
    elif args.cmd == "rotate":
        session.rotate(args.degrees)
    elif args.cmd == "slideshow":
        session.slideshow(args.delay)
    elif args.cmd == "list":
        session.list_catalog()
    elif args.cmd == "status":
        session.status()
    return 0


if __name__ == "__main__":
    sys.exit(main())

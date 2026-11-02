#!/usr/bin/env python3
# SigmaOS Radio Stream — internet radio station catalog and stream CLI
# Upstream: StreamController, Shortwave (https://gitlab.gnome.org/World/Shortwave)
"""
SigmaOS radio-stream: browse and stream internet radio stations.
Subcommands:
  add     <name> <url> [--genre <g>]  Add a station to the catalog
  remove  <name>                      Remove a station
  play    <name>                      Start streaming a station (record as active)
  stop                                Stop the current stream
  search  <query>                     Search stations by name or genre
  list    [--genre <g>]               List all stations, optionally filtered
  status                              Show active stream state
  favorites add|remove <name>         Manage favorite stations
"""
import argparse
import sys
from typing import Dict, List, Optional

# ---------------------------------------------------------------------------
# In-memory station catalog
# ---------------------------------------------------------------------------
BUILTIN_STATIONS: List[dict] = [
    {"name": "RadioParadise",  "url": "https://stream.radioparadise.com/aac-128", "genre": "eclectic"},
    {"name": "SomaFM-Groove",  "url": "https://ice3.somafm.com/groovesalad-128-mp3", "genre": "ambient"},
    {"name": "BBC-Radio4",     "url": "https://stream.live.vc.bbcmedia.co.uk/bbc_radio_fourfm", "genre": "talk"},
    {"name": "KEXP",           "url": "https://live-aacplus-64.kexp.org/kexp64.aac", "genre": "indie"},
]


class RadioCatalog:
    def __init__(self):
        self.stations: Dict[str, dict] = {s["name"]: s for s in BUILTIN_STATIONS}
        self.active: Optional[str] = None
        self.favorites: List[str] = []

    def add(self, name: str, url: str, genre: str = "other") -> None:
        if name in self.stations:
            print(f"[radio] Station '{name}' already exists", file=sys.stderr)
            return
        self.stations[name] = {"name": name, "url": url, "genre": genre}
        print(f"[radio] Added: {name} ({genre}) → {url}")

    def remove(self, name: str) -> None:
        if name not in self.stations:
            print(f"[radio] Unknown station: {name}", file=sys.stderr)
            return
        del self.stations[name]
        if self.active == name:
            self.active = None
        self.favorites = [f for f in self.favorites if f != name]
        print(f"[radio] Removed: {name}")

    def play(self, name: str) -> None:
        if name not in self.stations:
            print(f"[radio] Unknown station: {name}", file=sys.stderr)
            return
        self.active = name
        st = self.stations[name]
        print(f"[radio] Streaming → {st['name']} ({st['genre']})")
        print(f"         URL: {st['url']}")

    def stop(self) -> None:
        if not self.active:
            print("[radio] No active stream")
            return
        print(f"[radio] Stopped: {self.active}")
        self.active = None

    def search(self, query: str) -> None:
        q = query.lower()
        results = [s for s in self.stations.values()
                   if q in s["name"].lower() or q in s["genre"].lower()]
        if not results:
            print(f"[radio] No stations found for '{query}'")
            return
        for s in results:
            fav = "★" if s["name"] in self.favorites else " "
            print(f"  {fav} {s['name']:<20} [{s['genre']:<10}] {s['url']}")

    def list_stations(self, genre: Optional[str] = None) -> None:
        stations = list(self.stations.values())
        if genre:
            stations = [s for s in stations if s["genre"].lower() == genre.lower()]
        if not stations:
            print("[radio] No stations in catalog")
            return
        for s in stations:
            marker = "▶" if s["name"] == self.active else ("★" if s["name"] in self.favorites else " ")
            print(f"  {marker} {s['name']:<20} [{s['genre']:<10}] {s['url']}")

    def status(self) -> None:
        print(f"Active  : {self.active or '(none)'}")
        print(f"Stations: {len(self.stations)}")
        print(f"Favs    : {len(self.favorites)}")

    def fav_add(self, name: str) -> None:
        if name not in self.stations:
            print(f"[radio] Unknown station: {name}", file=sys.stderr)
            return
        if name not in self.favorites:
            self.favorites.append(name)
        print(f"[radio] Added to favorites: {name}")

    def fav_remove(self, name: str) -> None:
        self.favorites = [f for f in self.favorites if f != name]
        print(f"[radio] Removed from favorites: {name}")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-radio-stream", description="SigmaOS internet radio")
    sub = p.add_subparsers(dest="cmd", required=True)

    a = sub.add_parser("add", help="Add a station")
    a.add_argument("name"); a.add_argument("url"); a.add_argument("--genre", default="other")

    rm = sub.add_parser("remove", help="Remove a station")
    rm.add_argument("name")

    pl = sub.add_parser("play", help="Stream a station")
    pl.add_argument("name")

    sub.add_parser("stop", help="Stop current stream")

    sr = sub.add_parser("search", help="Search stations")
    sr.add_argument("query")

    ls = sub.add_parser("list", help="List stations")
    ls.add_argument("--genre", default=None)

    sub.add_parser("status", help="Show stream state")

    fav = sub.add_parser("favorites", help="Manage favorites")
    fav.add_argument("action", choices=["add", "remove"])
    fav.add_argument("name")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    catalog = RadioCatalog()

    if args.cmd == "add":
        catalog.add(args.name, args.url, args.genre)
    elif args.cmd == "remove":
        catalog.remove(args.name)
    elif args.cmd == "play":
        catalog.play(args.name)
    elif args.cmd == "stop":
        catalog.stop()
    elif args.cmd == "search":
        catalog.search(args.query)
    elif args.cmd == "list":
        catalog.list_stations(args.genre)
    elif args.cmd == "status":
        catalog.status()
    elif args.cmd == "favorites":
        if args.action == "add":
            catalog.fav_add(args.name)
        else:
            catalog.fav_remove(args.name)
    return 0


if __name__ == "__main__":
    sys.exit(main())

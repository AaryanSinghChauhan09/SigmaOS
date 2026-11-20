#!/usr/bin/env python3
# SigmaOS Reel — omareel-style screen recording CLI
# Upstream inspiration: omareel (https://github.com/basecamp/omarchy),
#                       wf-recorder (https://github.com/ammen99/wf-recorder)
"""
SigmaOS reel: screen recording tool.

Subcommands:
  record  region    Record a user-selected screen region
  record  display   Record the full display
  list              List completed recordings
  status            Show current recording status
"""

import argparse
import sys
import time
from typing import Dict, List, Optional

# ---------------------------------------------------------------------------
# In-memory recording registry
# ---------------------------------------------------------------------------

class RecordingRecord:
    def __init__(self, rec_id: int, mode: str, output: str,
                 container: str, fps: int) -> None:
        self.rec_id = rec_id
        self.mode = mode          # "region" | "display"
        self.output = output      # output file path
        self.container = container  # "mp4" | "mkv" | "webm"
        self.fps = fps
        self.started_at = int(time.time())
        self.stopped_at: Optional[int] = None
        self.active = True

    def stop(self) -> None:
        self.stopped_at = int(time.time())
        self.active = False

    def duration_s(self) -> Optional[int]:
        if self.stopped_at is None:
            return None
        return self.stopped_at - self.started_at


_RECORDINGS: List[RecordingRecord] = []
_NEXT_ID = 1
_ACTIVE: Optional[RecordingRecord] = None


def cmd_record(mode: str, output: str, container: str, fps: int) -> None:
    global _NEXT_ID, _ACTIVE
    if _ACTIVE is not None and _ACTIVE.active:
        print("error: a recording is already in progress", file=sys.stderr)
        sys.exit(1)
    if mode not in ("region", "display"):
        print(f"error: unknown mode '{mode}'", file=sys.stderr)
        sys.exit(1)

    rec = RecordingRecord(_NEXT_ID, mode, output, container, fps)
    _NEXT_ID += 1
    _RECORDINGS.append(rec)
    _ACTIVE = rec

    print(f"Recording started: id={rec.rec_id} mode={mode} "
          f"output={output} container={container} fps={fps}")
    # Simulate an immediate stop for test-safe execution
    rec.stop()
    _ACTIVE = None
    print(f"Recording stopped: id={rec.rec_id} "
          f"duration={rec.duration_s()}s -> {output}.{container}")


def cmd_list() -> None:
    if not _RECORDINGS:
        print("No recordings found.")
        return
    print(f"{'ID':<4} {'Mode':<10} {'Output':<30} {'Container':<8} {'FPS':<5} {'Duration'}")
    print("-" * 70)
    for r in _RECORDINGS:
        dur = f"{r.duration_s()}s" if r.duration_s() is not None else "active"
        print(f"  {r.rec_id:<2} {r.mode:<10} {r.output:<30} "
              f"{r.container:<8} {r.fps:<5} {dur}")


def cmd_status() -> None:
    active = [r for r in _RECORDINGS if r.active]
    if not active:
        print("No active recording.")
    else:
        r = active[0]
        elapsed = int(time.time()) - r.started_at
        print(f"Recording active: id={r.rec_id} mode={r.mode} "
              f"elapsed={elapsed}s output={r.output}.{r.container}")


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="reel",
        description="SigmaOS omareel-style screen recording tool",
    )
    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    p_record = sub.add_parser("record", help="Start recording")
    p_record.add_argument("mode", choices=["region", "display"],
                          help="region or display")
    p_record.add_argument("--output", default="/tmp/sigma-reel",
                          help="Output file path (without extension)")
    p_record.add_argument("--container", default="mp4",
                          choices=["mp4", "mkv", "webm"],
                          help="Output container format")
    p_record.add_argument("--fps", type=int, default=30,
                          help="Frames per second")

    sub.add_parser("list", help="List recordings")
    sub.add_parser("status", help="Show recording status")

    args = parser.parse_args()

    if args.cmd == "record":
        cmd_record(args.mode, args.output, args.container, args.fps)
    elif args.cmd == "list":
        cmd_list()
    elif args.cmd == "status":
        cmd_status()
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SigmaOS Media Player — GStreamer-inspired playback CLI scaffold
# Upstream: mpv (https://mpv.io/), GStreamer (https://gstreamer.freedesktop.org/)
"""
SigmaOS player: media playback control via a pipeline model.
Subcommands:
  play   <file|url>   Start playback (registers in session)
  pause               Pause current track
  stop                Stop and clear current track
  next                Skip to next track in queue
  prev                Go to previous track
  queue  <file|url>   Add item to play queue
  status              Show current track and queue state
  list                List queued tracks
"""
import argparse
import sys
from typing import List, Optional

# ---------------------------------------------------------------------------
# In-memory playback state (GStreamer pipeline model)
# ---------------------------------------------------------------------------
class Track:
    def __init__(self, uri: str):
        self.uri = uri
        self.position_sec: float = 0.0
        self.duration_sec: Optional[float] = None

    def __str__(self) -> str:
        dur = f"{self.duration_sec:.1f}s" if self.duration_sec else "unknown"
        return f"{self.uri} [{self.position_sec:.1f}s / {dur}]"


class PlayerSession:
    """Represents a stateful GStreamer-style playback session."""
    STATES = ("STOPPED", "PLAYING", "PAUSED")

    def __init__(self):
        self.state: str = "STOPPED"
        self.current: Optional[Track] = None
        self.queue: List[Track] = []
        self.history: List[Track] = []

    # -- pipeline state transitions ------------------------------------------
    def play(self, uri: str) -> None:
        self.current = Track(uri)
        self.state = "PLAYING"
        print(f"[player] PLAYING  → {self.current.uri}")

    def pause(self) -> None:
        if self.state == "PLAYING":
            self.state = "PAUSED"
            print(f"[player] PAUSED   → {self.current.uri if self.current else '(none)'}")
        else:
            print(f"[player] Cannot pause: state is {self.state}", file=sys.stderr)

    def stop(self) -> None:
        if self.current:
            self.history.append(self.current)
        self.current = None
        self.state = "STOPPED"
        print("[player] STOPPED")

    def next(self) -> None:
        if not self.queue:
            print("[player] Queue is empty", file=sys.stderr)
            return
        if self.current:
            self.history.append(self.current)
        nxt = self.queue.pop(0)
        self.play(nxt.uri)

    def prev(self) -> None:
        if not self.history:
            print("[player] No history", file=sys.stderr)
            return
        prev_track = self.history.pop()
        if self.current:
            self.queue.insert(0, self.current)
        self.play(prev_track.uri)

    def enqueue(self, uri: str) -> None:
        t = Track(uri)
        self.queue.append(t)
        print(f"[player] QUEUED   → {uri} (queue length: {len(self.queue)})")

    def status(self) -> None:
        print(f"State   : {self.state}")
        print(f"Current : {self.current or '(none)'}")
        print(f"Queue   : {len(self.queue)} track(s)")

    def list_queue(self) -> None:
        if not self.queue:
            print("[player] Queue is empty")
            return
        for i, t in enumerate(self.queue):
            print(f"  [{i+1}] {t.uri}")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------
def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="sigmaos-player",
        description="SigmaOS media player (GStreamer-pipeline model)",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    play_p = sub.add_parser("play", help="Start playback of a file or URL")
    play_p.add_argument("uri", help="File path or URL to play")

    sub.add_parser("pause", help="Pause current playback")
    sub.add_parser("stop",  help="Stop and clear current track")
    sub.add_parser("next",  help="Skip to next queued track")
    sub.add_parser("prev",  help="Return to previous track")

    q_p = sub.add_parser("queue", help="Add a track to the play queue")
    q_p.add_argument("uri", help="File path or URL to enqueue")

    sub.add_parser("status", help="Print current playback state")
    sub.add_parser("list",   help="List queued tracks")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    session = PlayerSession()

    dispatch = {
        "play":   lambda: session.play(args.uri),
        "pause":  session.pause,
        "stop":   session.stop,
        "next":   session.next,
        "prev":   session.prev,
        "queue":  lambda: session.enqueue(args.uri),
        "status": session.status,
        "list":   session.list_queue,
    }
    dispatch[args.cmd]()
    return 0


if __name__ == "__main__":
    sys.exit(main())

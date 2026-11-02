#!/usr/bin/env python3
# SigmaOS Remote Desktop — RDP/VNC session manager CLI scaffold
# Upstream: GNOME Connections (https://gitlab.gnome.org/GNOME/gnome-connections),
#           Remmina (https://remmina.org/)
"""
SigmaOS remote-desktop: manage remote desktop sessions.
Subcommands:
  connect  <host> [--protocol rdp|vnc] [--user <u>] [--port <p>]  Open a session
  disconnect <session-id>              Close a session
  list                                 List active and saved sessions
  save   <name> <host> [options]       Save a connection profile
  remove <name>                        Remove a saved profile
  status                               Show running session count
"""
import argparse
import sys
import uuid
from typing import Dict, List, Optional

PROTOCOLS = ("rdp", "vnc", "spice")


class ConnectionProfile:
    def __init__(self, name: str, host: str, protocol: str = "rdp",
                 user: str = "", port: Optional[int] = None):
        self.name = name
        self.host = host
        self.protocol = protocol
        self.user = user
        self.port = port or (3389 if protocol == "rdp" else 5900)

    def __str__(self) -> str:
        u = f"{self.user}@" if self.user else ""
        return f"{self.name}: {self.protocol}://{u}{self.host}:{self.port}"


class Session:
    def __init__(self, profile: ConnectionProfile):
        self.session_id = str(uuid.uuid4())[:8]
        self.profile = profile
        self.state = "connected"

    def __str__(self) -> str:
        return f"[{self.session_id}] {self.profile} ({self.state})"


class RemoteDesktopManager:
    def __init__(self):
        self.profiles: Dict[str, ConnectionProfile] = {}
        self.sessions: Dict[str, Session] = {}

    def connect(self, host: str, protocol: str = "rdp",
                user: str = "", port: Optional[int] = None) -> None:
        if protocol not in PROTOCOLS:
            print(f"[remote] Unknown protocol: {protocol}", file=sys.stderr)
            return
        profile = ConnectionProfile(host, host, protocol, user, port)
        session = Session(profile)
        self.sessions[session.session_id] = session
        print(f"[remote] Connected → {session}")

    def disconnect(self, session_id: str) -> None:
        if session_id not in self.sessions:
            print(f"[remote] Unknown session: {session_id}", file=sys.stderr)
            return
        self.sessions[session_id].state = "disconnected"
        del self.sessions[session_id]
        print(f"[remote] Disconnected: {session_id}")

    def list_sessions(self) -> None:
        if not self.sessions and not self.profiles:
            print("[remote] No active sessions or saved profiles")
            return
        if self.sessions:
            print("Active sessions:")
            for s in self.sessions.values():
                print(f"  {s}")
        if self.profiles:
            print("Saved profiles:")
            for p in self.profiles.values():
                print(f"  {p}")

    def save(self, name: str, host: str, protocol: str = "rdp",
             user: str = "", port: Optional[int] = None) -> None:
        self.profiles[name] = ConnectionProfile(name, host, protocol, user, port)
        print(f"[remote] Saved profile: {self.profiles[name]}")

    def remove(self, name: str) -> None:
        if name not in self.profiles:
            print(f"[remote] Unknown profile: {name}", file=sys.stderr)
            return
        del self.profiles[name]
        print(f"[remote] Removed profile: {name}")

    def status(self) -> None:
        print(f"Active sessions : {len(self.sessions)}")
        print(f"Saved profiles  : {len(self.profiles)}")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-remote-desktop",
                                description="SigmaOS remote desktop session manager")
    sub = p.add_subparsers(dest="cmd", required=True)

    c = sub.add_parser("connect", help="Open a remote session")
    c.add_argument("host")
    c.add_argument("--protocol", choices=PROTOCOLS, default="rdp")
    c.add_argument("--user", default="")
    c.add_argument("--port", type=int, default=None)

    d = sub.add_parser("disconnect", help="Close a session")
    d.add_argument("session_id")

    sub.add_parser("list",   help="List sessions and profiles")
    sub.add_parser("status", help="Show session count")

    sv = sub.add_parser("save", help="Save a connection profile")
    sv.add_argument("name"); sv.add_argument("host")
    sv.add_argument("--protocol", choices=PROTOCOLS, default="rdp")
    sv.add_argument("--user", default=""); sv.add_argument("--port", type=int, default=None)

    rm = sub.add_parser("remove", help="Remove a saved profile")
    rm.add_argument("name")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    mgr = RemoteDesktopManager()

    if args.cmd == "connect":
        mgr.connect(args.host, args.protocol, args.user, args.port)
    elif args.cmd == "disconnect":
        mgr.disconnect(args.session_id)
    elif args.cmd == "list":
        mgr.list_sessions()
    elif args.cmd == "status":
        mgr.status()
    elif args.cmd == "save":
        mgr.save(args.name, args.host, args.protocol, args.user, args.port)
    elif args.cmd == "remove":
        mgr.remove(args.name)
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
# SigmaOS LAN File Share — local network file transfer CLI scaffold
# Upstream: warpinator (https://github.com/linuxmint/warpinator), croc (https://github.com/schollz/croc)
"""
SigmaOS lan-fileshare: discover peers and transfer files over LAN.
Subcommands:
  advertise  [--name <n>] [--port <p>]   Start advertising this node on LAN
  discover                               Discover other SigmaOS nodes
  send       <peer> <file>...            Send files to a discovered peer
  receive    [--dir <d>] [--port <p>]    Listen for incoming transfers
  list                                   List discovered peers
  history                                Show transfer history
  status                                 Show active transfers
"""
import argparse
import os
import sys
import uuid
from typing import Dict, List, Optional

TRANSFER_STATES = ("pending", "active", "completed", "failed")


class Peer:
    def __init__(self, name: str, address: str, port: int = 52000):
        self.peer_id = str(uuid.uuid4())[:8]
        self.name = name
        self.address = address
        self.port = port

    def __str__(self) -> str:
        return f"{self.peer_id}  {self.name:<20} {self.address}:{self.port}"


class TransferRecord:
    def __init__(self, direction: str, peer: Peer, files: List[str]):
        self.transfer_id = str(uuid.uuid4())[:8]
        self.direction = direction  # "send" | "receive"
        self.peer = peer
        self.files = files
        self.state = "pending"
        self.bytes_total = sum(
            os.path.getsize(f) for f in files if os.path.exists(f)
        ) if direction == "send" else 0

    def __str__(self) -> str:
        return (f"{self.transfer_id}  {self.direction:<7} "
                f"{self.peer.name:<20} "
                f"{len(self.files)} file(s) [{self.state}]")


class LANShareManager:
    def __init__(self):
        self.local_name: str = "sigmaos-node"
        self.advertising: bool = False
        self.peers: Dict[str, Peer] = {}
        self.transfers: List[TransferRecord] = []

    def advertise(self, name: str, port: int) -> None:
        self.local_name = name
        self.advertising = True
        print(f"[lanshare] Advertising as '{name}' on port {port}")
        print(f"[lanshare] (scaffold: mDNS advertisement recorded, no real network)")

    def discover(self) -> None:
        # Simulate discovering two LAN peers for scaffold demo
        demo_peers = [
            Peer("desktop-home",   "192.168.1.10", 52000),
            Peer("laptop-alice",   "192.168.1.22", 52000),
        ]
        for p in demo_peers:
            self.peers[p.peer_id] = p
        print(f"[lanshare] Discovery complete: {len(demo_peers)} peer(s) found")
        for p in demo_peers:
            print(f"  {p}")

    def send(self, peer_ref: str, files: List[str]) -> None:
        peer = next((p for p in self.peers.values()
                     if p.name == peer_ref or p.peer_id == peer_ref), None)
        if not peer:
            print(f"[lanshare] Unknown peer: {peer_ref}", file=sys.stderr); return
        missing = [f for f in files if not os.path.exists(f)]
        if missing:
            print(f"[lanshare] Files not found: {missing}", file=sys.stderr); return
        t = TransferRecord("send", peer, files)
        t.state = "completed"
        self.transfers.append(t)
        print(f"[lanshare] Sent {len(files)} file(s) to {peer.name}")
        for f in files:
            sz = os.path.getsize(f)
            print(f"  → {os.path.basename(f)} ({sz} bytes)")

    def receive(self, directory: str, port: int) -> None:
        os.makedirs(directory, exist_ok=True)
        print(f"[lanshare] Listening for transfers on port {port}")
        print(f"[lanshare] Saving to: {directory}")
        print(f"[lanshare] (scaffold: no real socket; press Ctrl-C to stop)")

    def list_peers(self) -> None:
        if not self.peers:
            print("[lanshare] No peers discovered (run 'discover' first)"); return
        print(f"{'ID':<10} {'NAME':<20} ADDRESS")
        for p in self.peers.values():
            print(f"  {p}")

    def history(self) -> None:
        if not self.transfers:
            print("[lanshare] No transfer history"); return
        for t in self.transfers:
            print(f"  {t}")

    def status(self) -> None:
        active = [t for t in self.transfers if t.state == "active"]
        print(f"Node      : {self.local_name} ({'advertising' if self.advertising else 'idle'})")
        print(f"Peers     : {len(self.peers)}")
        print(f"Transfers : {len(self.transfers)} total, {len(active)} active")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-lan-fileshare",
                                description="SigmaOS LAN file transfer")
    sub = p.add_subparsers(dest="cmd", required=True)

    ad = sub.add_parser("advertise", help="Advertise this node")
    ad.add_argument("--name", default="sigmaos-node")
    ad.add_argument("--port", type=int, default=52000)

    sub.add_parser("discover", help="Discover LAN peers")

    sn = sub.add_parser("send", help="Send files to a peer")
    sn.add_argument("peer"); sn.add_argument("files", nargs="+")

    rc = sub.add_parser("receive", help="Listen for incoming files")
    rc.add_argument("--dir",  default=os.path.expanduser("~/Downloads"))
    rc.add_argument("--port", type=int, default=52000)

    sub.add_parser("list",    help="List discovered peers")
    sub.add_parser("history", help="Show transfer history")
    sub.add_parser("status",  help="Show transfer status")
    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    mgr = LANShareManager()

    if args.cmd == "advertise":  mgr.advertise(args.name, args.port)
    elif args.cmd == "discover": mgr.discover()
    elif args.cmd == "send":     mgr.send(args.peer, args.files)
    elif args.cmd == "receive":  mgr.receive(args.dir, args.port)
    elif args.cmd == "list":     mgr.list_peers()
    elif args.cmd == "history":  mgr.history()
    elif args.cmd == "status":   mgr.status()
    return 0


if __name__ == "__main__":
    sys.exit(main())

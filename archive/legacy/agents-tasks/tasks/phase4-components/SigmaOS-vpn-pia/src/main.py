#!/usr/bin/env python3
# SigmaOS PIA VPN — Private Internet Access client integration.
import argparse, sys
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class VpnServer:
    region: str; hostname: str; latency_ms: int; load_pct: int

SERVERS: List[VpnServer] = [
    VpnServer("US East","us-east.privateinternetaccess.com",45,30),
    VpnServer("EU Germany","de.privateinternetaccess.com",85,25),
    VpnServer("UK London","uk-london.privateinternetaccess.com",95,40),
    VpnServer("Japan","japan.privateinternetaccess.com",220,20),
]

connected_to: Optional[str] = None

def do_list(): [print(f"  {s.region:<15} latency={s.latency_ms}ms  load={s.load_pct}%") for s in SERVERS]
def do_connect(region):
    global connected_to
    s=next((s for s in SERVERS if s.region.lower().startswith(region.lower())),None)
    if not s: print(f"error: region '{region}' not found",file=sys.stderr); return 1
    connected_to = s.region; print(f"[vpn] Connected to {s.region} ({s.hostname})"); return 0
def do_disconnect():
    global connected_to
    if not connected_to: print("Not connected."); return 0
    print(f"[vpn] Disconnected from {connected_to}"); connected_to = None; return 0
def do_status(): print(f"VPN: {'Connected to '+connected_to if connected_to else 'Disconnected'}")

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-pia")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list").set_defaults(func=lambda _: do_list() or 0)
    co = sub.add_parser("connect"); co.add_argument("region"); co.set_defaults(func=lambda a: do_connect(a.region))
    sub.add_parser("disconnect").set_defaults(func=lambda _: do_disconnect())
    sub.add_parser("status").set_defaults(func=lambda _: do_status() or 0)
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))

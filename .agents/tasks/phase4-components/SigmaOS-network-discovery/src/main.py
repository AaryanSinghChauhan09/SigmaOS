#!/usr/bin/env python3
# SigmaOS Network Discovery — mDNS/Avahi service discovery CLI scaffold
# Upstream: avahi (https://avahi.org/), zeroconf (https://pypi.org/project/zeroconf/)
"""
SigmaOS network-discovery: discover and register services via mDNS/Zeroconf.
Subcommands:
  scan     [--type <svc>]         Scan LAN for mDNS services
  register --name <n> --type <t> --port <p> [--txt K=V]...  Register a service
  unregister <name>               Withdraw a registered service
  resolve  <name>                 Resolve a hostname to an address
  list                            List discovered and registered services
  watch    [--type <t>]           Watch for service arrivals/departures (scaffold)
  status                          Show discovery engine state
"""
import argparse
import sys
import uuid
from typing import Dict, List, Optional

COMMON_TYPES = ("_http._tcp", "_ssh._tcp", "_smb._tcp", "_ftp._tcp",
                "_sftp-ssh._tcp", "_sigmaos._tcp")


class ServiceRecord:
    def __init__(self, name: str, service_type: str, port: int,
                 host: str = "local", txt: Optional[Dict[str, str]] = None):
        self.record_id = str(uuid.uuid4())[:8]
        self.name = name
        self.service_type = service_type
        self.port = port
        self.host = host
        self.txt = txt or {}
        self.origin = "discovered"  # "discovered" | "registered"

    def __str__(self) -> str:
        txt = " ".join(f"{k}={v}" for k, v in self.txt.items())
        return (f"{self.name:<25} {self.service_type:<20} "
                f":{self.port:<5} {self.host}  {txt}")


DEMO_SERVICES = [
    ServiceRecord("SigmaOS-desktop", "_sigmaos._tcp", 5353, "sigmaos.local",
                  {"version": "0.5", "role": "desktop"}),
    ServiceRecord("MyNAS",           "_smb._tcp",      445, "nas.local"),
    ServiceRecord("HomeRouter",      "_http._tcp",      80,  "router.local"),
    ServiceRecord("pi-hole",         "_dns-sd._udp",    53,  "pihole.local",
                  {"model": "RPi4"}),
]


class DiscoveryEngine:
    def __init__(self):
        self.discovered: Dict[str, ServiceRecord] = {}
        self.registered: Dict[str, ServiceRecord] = {}
        self.running = False

    def scan(self, service_type: Optional[str] = None) -> None:
        for svc in DEMO_SERVICES:
            if service_type and svc.service_type != service_type:
                continue
            self.discovered[svc.record_id] = svc
        filtered = [s for s in self.discovered.values()
                    if not service_type or s.service_type == service_type]
        print(f"[discovery] Scan complete: {len(filtered)} service(s) found")
        for s in filtered:
            print(f"  {s}")

    def register(self, name: str, service_type: str, port: int,
                 txt: List[str]) -> None:
        parsed_txt = {}
        for item in txt:
            if "=" in item:
                k, v = item.split("=", 1)
                parsed_txt[k] = v
        svc = ServiceRecord(name, service_type, port, "local", parsed_txt)
        svc.origin = "registered"
        self.registered[svc.record_id] = svc
        print(f"[discovery] Registered: {svc}")

    def unregister(self, name: str) -> None:
        for rid, svc in list(self.registered.items()):
            if svc.name == name:
                del self.registered[rid]
                print(f"[discovery] Unregistered: {name}")
                return
        print(f"[discovery] Not registered: {name}", file=sys.stderr)

    def resolve(self, name: str) -> None:
        all_svcs = list(self.discovered.values()) + list(self.registered.values())
        matches = [s for s in all_svcs if s.name == name or s.host == name]
        if not matches:
            print(f"[discovery] Not found: {name}"); return
        for s in matches:
            print(f"  {s.name}  →  {s.host}:{s.port}  ({s.service_type})")

    def list_services(self) -> None:
        all_svcs = list(self.discovered.values()) + list(self.registered.values())
        if not all_svcs:
            print("[discovery] No services"); return
        print(f"{'NAME':<25} {'TYPE':<20} {'PORT':<6} HOST")
        for s in all_svcs:
            marker = "★" if s.origin == "registered" else " "
            print(f"  {marker} {s}")

    def watch(self, service_type: Optional[str]) -> None:
        flt = f" for {service_type}" if service_type else ""
        print(f"[discovery] Watching{flt} (scaffold: prints simulated events)")
        for svc in DEMO_SERVICES[:2]:
            print(f"  + Added : {svc.name} ({svc.service_type})")

    def status(self) -> None:
        print(f"Running    : {self.running}")
        print(f"Discovered : {len(self.discovered)}")
        print(f"Registered : {len(self.registered)}")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-network-discovery",
                                description="SigmaOS mDNS service discovery")
    sub = p.add_subparsers(dest="cmd", required=True)

    sc = sub.add_parser("scan",   help="Scan for mDNS services")
    sc.add_argument("--type", default=None, help=f"Filter by service type, e.g. {COMMON_TYPES[0]}")

    rg = sub.add_parser("register", help="Register a service")
    rg.add_argument("--name", required=True); rg.add_argument("--type", required=True)
    rg.add_argument("--port", type=int, required=True)
    rg.add_argument("--txt", action="append", default=[], metavar="K=V")

    ur = sub.add_parser("unregister", help="Unregister a service")
    ur.add_argument("name")

    rv = sub.add_parser("resolve", help="Resolve a service/host")
    rv.add_argument("name")

    sub.add_parser("list", help="List all services")

    wt = sub.add_parser("watch", help="Watch for service changes")
    wt.add_argument("--type", default=None)

    sub.add_parser("status", help="Show engine state")
    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    eng = DiscoveryEngine()

    if args.cmd == "scan":         eng.scan(args.type)
    elif args.cmd == "register":   eng.register(args.name, args.type, args.port, args.txt)
    elif args.cmd == "unregister": eng.unregister(args.name)
    elif args.cmd == "resolve":    eng.resolve(args.name)
    elif args.cmd == "list":       eng.list_services()
    elif args.cmd == "watch":      eng.watch(args.type)
    elif args.cmd == "status":     eng.status()
    return 0


if __name__ == "__main__":
    sys.exit(main())

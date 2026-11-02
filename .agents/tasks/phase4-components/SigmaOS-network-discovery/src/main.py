#!/usr/bin/env python3
# SigmaOS Network Discovery — local network device discovery via mDNS and UPnP.
# Upstream: avahi (mDNS), miniupnpc (UPnP), zeroconf
"""Discover devices on the local network via mDNS/Zeroconf and UPnP scanning."""

import argparse
import sys
from dataclasses import dataclass, field
from typing import List, Optional


@dataclass
class DiscoveredDevice:
    hostname: str
    ip: str
    mac: str
    protocol: str        # mdns | upnp | arp
    service_types: List[str] = field(default_factory=list)
    trusted: bool = False


# Simulated discovered devices (real impl: avahi-browse -a + arp-scan)
DISCOVERED: List[DiscoveredDevice] = [
    DiscoveredDevice("sigma-nas.local", "192.168.1.10", "aa:bb:cc:11:22:33", "mdns",
                     ["_smb._tcp", "_afpovertcp._tcp"], trusted=True),
    DiscoveredDevice("printer.local",  "192.168.1.20", "aa:bb:cc:44:55:66", "mdns",
                     ["_ipp._tcp", "_pdl-datastream._tcp"]),
    DiscoveredDevice("router.local",   "192.168.1.1",  "aa:bb:cc:77:88:99", "upnp",
                     ["urn:schemas-upnp-org:device:InternetGatewayDevice:1"]),
    DiscoveredDevice("pi.local",       "192.168.1.30", "dc:a6:32:ab:cd:ef", "mdns",
                     ["_ssh._tcp", "_http._tcp"]),
]


def do_scan(protocol: str = "all") -> int:
    devices = [d for d in DISCOVERED if protocol == "all" or d.protocol == protocol]
    print(f"Scanning local network for devices ({protocol} protocols)...\n")
    if not devices:
        print("  No devices found.")
        return 0
    print(f"  {'HOSTNAME':<25} {'IP':<16} {'MAC':<18} {'PROTO':<6} SERVICES")
    print("  " + "-" * 90)
    for d in devices:
        trusted = " [trusted]" if d.trusted else ""
        svcs = ", ".join(d.service_types[:2]) + ("…" if len(d.service_types) > 2 else "")
        print(f"  {d.hostname:<25} {d.ip:<16} {d.mac:<18} {d.protocol:<6} {svcs}{trusted}")
    print(f"\nFound {len(devices)} device(s).")
    return 0


def do_trust(hostname: str) -> int:
    for d in DISCOVERED:
        if d.hostname == hostname or d.ip == hostname:
            d.trusted = True
            print(f"[discovery] Trusted: {d.hostname} ({d.ip})")
            return 0
    print(f"error: '{hostname}' not found in scan results.", file=sys.stderr)
    return 1


def do_info(hostname: str) -> int:
    for d in DISCOVERED:
        if d.hostname == hostname or d.ip == hostname:
            print(f"Device: {d.hostname}")
            print(f"  IP        : {d.ip}")
            print(f"  MAC       : {d.mac}")
            print(f"  Protocol  : {d.protocol}")
            print(f"  Trusted   : {d.trusted}")
            print(f"  Services  :")
            for s in d.service_types:
                print(f"    - {s}")
            return 0
    print(f"error: '{hostname}' not found.", file=sys.stderr)
    return 1


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="sigma-discovery",
        description="SigmaOS network discovery: find devices via mDNS and UPnP",
    )
    sub = p.add_subparsers(dest="command", required=True)

    sc = sub.add_parser("scan", help="Scan for devices")
    sc.add_argument("--protocol", default="all", choices=["all", "mdns", "upnp", "arp"])
    sc.set_defaults(func=lambda a: do_scan(a.protocol))

    inf = sub.add_parser("info", help="Show device details")
    inf.add_argument("hostname")
    inf.set_defaults(func=lambda a: do_info(a.hostname))

    tr = sub.add_parser("trust", help="Trust a device")
    tr.add_argument("hostname")
    tr.set_defaults(func=lambda a: do_trust(a.hostname))

    return p


def main() -> int:
    return build_parser().parse_args().func(build_parser().parse_args())


if __name__ == "__main__":
    sys.exit(main())

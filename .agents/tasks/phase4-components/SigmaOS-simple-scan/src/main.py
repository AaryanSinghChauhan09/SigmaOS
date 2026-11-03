#!/usr/bin/env python3
# SigmaOS Simple Scan — document scanner with device selection and OCR support.
import argparse, sys
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class ScanDevice:
    name: str; model: str; backend: str

DEVICES: List[ScanDevice] = [
    ScanDevice("epson-xp-4100", "Epson XP-4100", "escl"),
    ScanDevice("hp-laserjet", "HP LaserJet M404", "hpaio"),
]

def do_list(): [print(f"  [{i}] {d.name}  ({d.model})  backend={d.backend}") for i,d in enumerate(DEVICES)] or print("No scanners found.")
def do_scan(device_idx: int, fmt: str, dpi: int, out: str):
    if device_idx >= len(DEVICES): print("error: invalid device index", file=sys.stderr); return 1
    d = DEVICES[device_idx]
    print(f"[scan] Device: {d.name}  format={fmt}  dpi={dpi}"); print(f"[scan] Saving to: {out}"); print(f"  (Real impl: scanimage -d {d.backend}:{d.name} --format={fmt} --resolution={dpi} > {out})")
    return 0

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-scan")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="List scanners").set_defaults(func=lambda _: do_list() or 0)
    sc = sub.add_parser("scan", help="Scan a document")
    sc.add_argument("--device", type=int, default=0); sc.add_argument("--format", default="png", choices=["png","pdf","jpeg"]); sc.add_argument("--dpi", type=int, default=300); sc.add_argument("--out", default="/tmp/scan.png")
    sc.set_defaults(func=lambda a: do_scan(a.device, a.format, a.dpi, a.out))
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))

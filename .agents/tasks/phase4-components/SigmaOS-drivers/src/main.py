"""SigmaOS-drivers: Hardware driver management tool.

Detects hardware that requires proprietary or third-party drivers, lists
available options, and manages installation and rollback.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional


# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------

@dataclass
class DriverInfo:
    name: str
    package: str
    version: str
    status: str  # "recommended" | "tested" | "latest" | "manual"
    installed: bool = False
    free: bool = False

    def __str__(self) -> str:
        tag = "[installed]" if self.installed else ""
        free_tag = "(free)" if self.free else "(proprietary)"
        return f"{self.package} {self.version} — {self.status} {free_tag} {tag}".strip()


@dataclass
class Device:
    pci_id: str
    description: str
    drivers: List[DriverInfo] = field(default_factory=list)

    def recommended_driver(self) -> Optional[DriverInfo]:
        for d in self.drivers:
            if d.status == "recommended":
                return d
        return self.drivers[0] if self.drivers else None


# ---------------------------------------------------------------------------
# Detection
# ---------------------------------------------------------------------------

def detect_devices() -> List[Device]:
    """Run lspci and build a list of Device objects with stub driver entries."""
    devices: List[Device] = []
    try:
        output = subprocess.check_output(
            ["lspci", "-mm"], stderr=subprocess.DEVNULL, text=True
        )
    except (FileNotFoundError, subprocess.CalledProcessError):
        print("[drivers] lspci not available; using stub device list.", file=sys.stderr)
        output = ""

    if not output:
        # Provide stub entries for CI / environments without lspci
        devices.append(Device(
            pci_id="0000:00:00.0",
            description="Sample GPU (stub)",
            drivers=[
                DriverInfo(
                    name="Sample Open Driver",
                    package="sigma-driver-open",
                    version="1.0.0",
                    status="recommended",
                    free=True,
                ),
            ],
        ))
        return devices

    for line in output.splitlines():
        parts = line.split('"')
        pci_id = parts[0].strip().rstrip(":")
        desc = parts[3] if len(parts) > 3 else "Unknown device"
        devices.append(Device(pci_id=pci_id, description=desc))

    return devices


# ---------------------------------------------------------------------------
# Installation / rollback
# ---------------------------------------------------------------------------

def install_driver(driver: DriverInfo, dry_run: bool = False) -> bool:
    """Install a driver package via apt-get."""
    cmd = ["apt-get", "install", "-y", driver.package]
    print(f"[drivers] {'DRY RUN: ' if dry_run else ''}Installing {driver.package}…")
    if dry_run:
        return True
    try:
        subprocess.check_call(cmd)
        driver.installed = True
        return True
    except subprocess.CalledProcessError as exc:
        print(f"[drivers] Installation failed: {exc}", file=sys.stderr)
        return False


def rollback_driver(driver: DriverInfo, previous_version: str,
                    dry_run: bool = False) -> bool:
    """Downgrade an installed driver to a previous version."""
    pkg_pin = f"{driver.package}={previous_version}"
    cmd = ["apt-get", "install", "-y", "--allow-downgrades", pkg_pin]
    print(f"[drivers] {'DRY RUN: ' if dry_run else ''}Rolling back to {previous_version}…")
    if dry_run:
        return True
    try:
        subprocess.check_call(cmd)
        return True
    except subprocess.CalledProcessError as exc:
        print(f"[drivers] Rollback failed: {exc}", file=sys.stderr)
        return False


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------

def print_device_table(devices: List[Device]) -> None:
    print(f"\n{'PCI ID':<20} {'Description':<40} {'Recommended Driver'}")
    print("-" * 90)
    for dev in devices:
        rec = dev.recommended_driver()
        rec_str = str(rec) if rec else "no driver required"
        print(f"{dev.pci_id:<20} {dev.description[:39]:<40} {rec_str}")
    print()


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def parse_args(argv: Optional[List[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="sigma-drivers",
        description="SigmaOS hardware driver management tool.",
    )
    sub = parser.add_subparsers(dest="command")

    sub.add_parser("list", help="List detected devices and available drivers")

    install_p = sub.add_parser("install", help="Install recommended drivers")
    install_p.add_argument("--dry-run", action="store_true",
                           help="Print actions without executing them")

    rollback_p = sub.add_parser("rollback", help="Roll back a driver to a previous version")
    rollback_p.add_argument("package", help="Package name to roll back")
    rollback_p.add_argument("version", help="Target version string")
    rollback_p.add_argument("--dry-run", action="store_true")

    return parser.parse_args(argv)


def main(argv: Optional[List[str]] = None) -> int:
    args = parse_args(argv)
    devices = detect_devices()

    if args.command == "list" or args.command is None:
        print_device_table(devices)
        return 0

    if args.command == "install":
        dry = getattr(args, "dry_run", False)
        for dev in devices:
            rec = dev.recommended_driver()
            if rec and not rec.installed:
                install_driver(rec, dry_run=dry)
        return 0

    if args.command == "rollback":
        # Find the matching driver across all devices
        for dev in devices:
            for drv in dev.drivers:
                if drv.package == args.package:
                    return 0 if rollback_driver(drv, args.version,
                                                dry_run=args.dry_run) else 1
        print(f"[drivers] Package '{args.package}' not found in device list.", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())

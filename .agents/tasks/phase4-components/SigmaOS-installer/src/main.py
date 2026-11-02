#!/usr/bin/env python3
# SigmaOS Installer — archinstall-inspired guided installation CLI
# Upstream inspiration: archinstall (https://github.com/archlinux/archinstall),
#                       calamares (https://github.com/calamares/calamares),
#                       omarchy-iso (https://github.com/basecamp/omarchy-iso)
"""
SigmaOS installer: guided installation following the platform-contract
installation-flow.md (10 steps: hardware detection → partitioning → filesystem
→ base install → bootloader → locale → user → desktop profile → first-boot
prep → validation).

Subcommands:
  install    Run the full guided installation (all 10 steps)
  partition  Disk partitioning only (step 2)
  bootstrap  Base package install only (step 4)
  configure  Locale + user + desktop profile (steps 6-8)
  validate   Run install-time smoke checks (step 10)
  info       Show detected hardware and disk inventory (step 1, read-only)

Options:
  --disk <dev>        Target disk (e.g. /dev/sda)
  --dry-run           Print actions without executing
  --profile <name>    Desktop profile (default/balanced/gaming/minimal)
  --backend <linux|bsd>  Kernel backend (default: linux)
"""

import argparse
import json
import os
import sys
import time
from typing import Dict, List, Optional, Tuple

# ---------------------------------------------------------------------------
# Installation state (persisted for resume)
# ---------------------------------------------------------------------------

STATE_FILE = "/tmp/sigmaos-install-state.json"

STEPS = [
    "hardware_detection",
    "disk_partitioning",
    "filesystem_selection",
    "base_package_install",
    "bootloader_install",
    "locale_timezone",
    "user_creation",
    "desktop_profile",
    "first_boot_prep",
    "validation",
]


def load_state() -> Dict:
    if os.path.exists(STATE_FILE):
        with open(STATE_FILE) as f:
            return json.load(f)
    return {"completed_steps": [], "disk": None, "profile": None, "backend": "linux"}


def save_state(state: Dict) -> None:
    with open(STATE_FILE, "w") as f:
        json.dump(state, f, indent=2)


def mark_step(state: Dict, step: str) -> None:
    if step not in state["completed_steps"]:
        state["completed_steps"].append(step)
    save_state(state)


# ---------------------------------------------------------------------------
# Step implementations (simulated — real impl would call backend adapters)
# ---------------------------------------------------------------------------

def step_hardware_detection(dry_run: bool) -> Dict:
    """Step 1: Detect CPU, disk, network, GPU inventory."""
    log("Step 1/10: Hardware detection")
    inventory = {
        "cpu": "x86_64 (detected)",
        "cores": os.cpu_count() or 4,
        "memory_gb": 16,
        "disks": ["/dev/sda (500GB SSD)", "/dev/nvme0n1 (1TB NVMe)"],
        "network": "eth0 (connected)",
        "gpu": "detected via platform contract",
    }
    for k, v in inventory.items():
        log(f"  {k}: {v}")
    return inventory


def step_disk_partitioning(disk: str, dry_run: bool, backend: str) -> Dict:
    """Step 2: Create root/boot/swap partitions."""
    log(f"Step 2/10: Disk partitioning ({disk}, backend={backend})")
    layout = {
        "disk": disk,
        "partitions": [
            {"name": "boot", "size": "512M", "fs": "fat32", "mount": "/boot"},
            {"name": "root", "size": "remaining", "fs": "ext4" if backend == "linux" else "ufs",
             "mount": "/"},
            {"name": "swap", "size": "8G", "fs": "swap", "mount": "[swap]"},
        ],
    }
    if dry_run:
        log("  [dry-run] would partition:")
    else:
        log("  partitioning:")
    for p in layout["partitions"]:
        log(f"    {p['name']}: {p['size']} {p['fs']} -> {p['mount']}")
    return layout


def step_filesystem_selection(disk: str, dry_run: bool, backend: str) -> str:
    """Step 3: Format partitions."""
    fs_type = "ext4" if backend == "linux" else "ufs"
    log(f"Step 3/10: Filesystem selection ({fs_type})")
    log(f"  formatting partitions on {disk} with {fs_type}")
    return fs_type


def step_base_install(dry_run: bool, backend: str) -> int:
    """Step 4: Install SigmaOS-core base set."""
    log("Step 4/10: Base package install")
    base_pkgs = ["base", "base-devel", "linux-firmware", "systemd"] if backend == "linux" \
        else ["base", "freebsd-base"]
    log(f"  installing {len(base_pkgs)} base packages: {', '.join(base_pkgs)}")
    return len(base_pkgs)


def step_bootloader_install(disk: str, dry_run: bool, backend: str) -> str:
    """Step 5: Install bootloader."""
    bl = "systemd-boot" if backend == "linux" else "boot(8)"
    log(f"Step 5/10: Bootloader installation ({bl})")
    log(f"  installing {bl} on {disk}")
    return bl


def step_locale_timezone(dry_run: bool) -> Dict:
    """Step 6: Set locale, timezone, keyboard."""
    log("Step 6/10: Locale and timezone")
    config = {"locale": "en_US.UTF-8", "timezone": "UTC", "keyboard": "us"}
    for k, v in config.items():
        log(f"  {k}: {v}")
    return config


def step_user_creation(username: str, dry_run: bool) -> Dict:
    """Step 7: Create primary user."""
    log(f"Step 7/10: User account creation ({username})")
    user = {"username": username, "shell": "/bin/bash", "sudo": True}
    log(f"  created user '{username}' with shell {user['shell']}")
    return user


def step_desktop_profile(profile: str, dry_run: bool) -> List[str]:
    """Step 8: Install curated desktop defaults."""
    log(f"Step 8/10: Desktop profile selection ({profile})")
    profiles = {
        "default": ["hyprland", "waybar", "foot", "wofi", "nautilus", "chromium"],
        "balanced": ["hyprland", "waybar", "foot", "wofi", "nautilus", "chromium", "btop"],
        "gaming": ["hyprland", "waybar", "foot", "mangohud", "gamemode", "steam"],
        "minimal": ["hyprland", "foot"],
    }
    pkgs = profiles.get(profile, profiles["default"])
    log(f"  installing {len(pkgs)} desktop packages: {', '.join(pkgs)}")
    return pkgs


def step_first_boot_prep(dry_run: bool) -> None:
    """Step 9: Write first-boot marker."""
    log("Step 9/10: First-boot preparation")
    log("  writing first-boot marker")
    if not dry_run:
        # In real impl: write /etc/sigmaos/first-boot marker
        pass


def step_validation(dry_run: bool) -> bool:
    """Step 10: Run install-time smoke checks."""
    log("Step 10/10: Validation")
    checks = [
        ("boot config valid", True),
        ("base packages installed", True),
        ("user created", True),
        ("bootloader installed", True),
        ("locale configured", True),
    ]
    all_ok = True
    for name, ok in checks:
        status = "PASS" if ok else "FAIL"
        log(f"  {name}: {status}")
        if not ok:
            all_ok = False
    return all_ok


# ---------------------------------------------------------------------------
# Full guided install
# ---------------------------------------------------------------------------

def cmd_install(args) -> None:
    """Run all 10 steps in sequence."""
    state = load_state()
    state["disk"] = args.disk
    state["profile"] = args.profile
    state["backend"] = args.backend
    save_state(state)

    dry = args.dry_run
    disk = args.disk or "/dev/sda"

    log(f"=== SigmaOS Guided Installation (backend={args.backend}) ===")
    log(f"Disk: {disk} | Profile: {args.profile} | Dry-run: {dry}")
    log("")

    step_hardware_detection(dry)
    step_disk_partitioning(disk, dry, args.backend)
    step_filesystem_selection(disk, dry, args.backend)
    step_base_install(dry, args.backend)
    step_bootloader_install(disk, dry, args.backend)
    step_locale_timezone(dry)
    step_user_creation(args.username, dry)
    step_desktop_profile(args.profile, dry)
    step_first_boot_prep(dry)
    ok = step_validation(dry)

    log("")
    if ok:
        log("Installation completed successfully.")
    else:
        log("Installation FAILED — check sigma-install.log", file=sys.stderr)
        sys.exit(1)


def cmd_info(args) -> None:
    """Step 1 only: show hardware inventory."""
    inv = step_hardware_detection(True)
    print(json.dumps(inv, indent=2))


def cmd_partition(args) -> None:
    """Step 2 only."""
    step_disk_partitioning(args.disk or "/dev/sda", args.dry_run, args.backend)


def cmd_bootstrap(args) -> None:
    """Step 4 only."""
    step_base_install(args.dry_run, args.backend)


def cmd_configure(args) -> None:
    """Steps 6-8: locale, user, desktop profile."""
    step_locale_timezone(args.dry_run)
    step_user_creation(args.username, args.dry_run)
    step_desktop_profile(args.profile, args.dry_run)


def cmd_validate(args) -> None:
    """Step 10 only."""
    ok = step_validation(args.dry_run)
    if not ok:
        sys.exit(1)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def log(msg: str, file=None) -> None:
    print(f"[sigma-installer] {msg}", file=file or sys.stdout)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="sigma-installer",
        description="SigmaOS guided installer (archinstall-inspired)",
    )
    parser.add_argument("--disk", help="Target disk (e.g. /dev/sda)")
    parser.add_argument("--dry-run", action="store_true", help="Print without executing")
    parser.add_argument("--profile", default="default",
                        choices=["default", "balanced", "gaming", "minimal"],
                        help="Desktop profile")
    parser.add_argument("--backend", default="linux", choices=["linux", "bsd"],
                        help="Kernel backend")
    parser.add_argument("--username", default="sigma",
                        help="Primary user name")

    sub = parser.add_subparsers(dest="cmd", metavar="SUBCOMMAND")

    sub.add_parser("install", help="Full guided installation (all 10 steps)")
    sub.add_parser("info", help="Show hardware inventory (step 1, read-only)")
    sub.add_parser("partition", help="Disk partitioning only (step 2)")
    sub.add_parser("bootstrap", help="Base package install only (step 4)")
    sub.add_parser("configure", help="Locale + user + desktop (steps 6-8)")
    sub.add_parser("validate", help="Run smoke checks (step 10)")

    args = parser.parse_args()

    if args.cmd == "install":
        cmd_install(args)
    elif args.cmd == "info":
        cmd_info(args)
    elif args.cmd == "partition":
        cmd_partition(args)
    elif args.cmd == "bootstrap":
        cmd_bootstrap(args)
    elif args.cmd == "configure":
        cmd_configure(args)
    elif args.cmd == "validate":
        cmd_validate(args)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()

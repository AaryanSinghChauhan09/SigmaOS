#!/usr/bin/env python3
"""
SigmaOS QEMU Boot Smoke Test
==============================
Boots the SigmaOS ISO in QEMU with a serial console and verifies that the
kernel emits the expected boot marker within a timeout.

Exit codes:
  0  boot marker found — test PASSED
  1  boot marker NOT found — test FAILED
  2  prerequisite missing (QEMU not installed, ISO not found) — test SKIPPED

Usage:
  python3 scripts/qemu_smoke_test.py [--iso PATH] [--timeout SEC] [--marker TEXT]

Defaults:
  --iso      build/sigmaos.iso  (or build/sigmaos-desktop-preview.iso as fallback)
  --timeout  60                 seconds to wait for the boot marker
  --marker   "SigmaOS"          string to look for in serial output
"""

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
import threading
import time


BOOT_MARKER_DEFAULT = "SigmaOS"
ISO_CANDIDATES = [
    "build/sigmaos.iso",
    "build/sigmaos-desktop-preview.iso",
]
TIMEOUT_DEFAULT = 60


def find_iso(override: str | None) -> str | None:
    if override:
        return override if os.path.isfile(override) else None
    for candidate in ISO_CANDIDATES:
        if os.path.isfile(candidate) and os.path.getsize(candidate) > 0:
            return candidate
    return None


def check_prerequisites() -> tuple[str, str]:
    """Return (qemu_path, iso_path) or exit with code 2."""
    qemu = shutil.which("qemu-system-x86_64")
    if not qemu:
        print(
            "SKIP: qemu-system-x86_64 not installed.\n"
            "      Install: sudo apt install qemu-system-x86",
            file=sys.stderr,
        )
        sys.exit(2)
    return qemu


def run_smoke_test(qemu: str, iso: str, timeout: int, marker: str) -> bool:
    """
    Boot the ISO in QEMU (no-GUI, serial output to pipe) and scan for marker.
    Returns True if marker found within timeout, False otherwise.
    """
    with tempfile.NamedTemporaryFile(suffix=".log", delete=False) as log_file:
        log_path = log_file.name

    cmd = [
        qemu,
        "-m", "512M",
        "-cdrom", iso,
        "-boot", "d",
        "-nographic",
        "-serial", "stdio",
        "-no-reboot",
        "-display", "none",
        "-cpu", "max",
        "-smp", "2",
    ]

    print(f"[QEMU] Booting: {iso}")
    print(f"[QEMU] Command: {' '.join(cmd)}")
    print(f"[QEMU] Waiting up to {timeout}s for marker: {marker!r}")
    print(f"[QEMU] Serial log: {log_path}")
    print()

    found = threading.Event()
    output_lines: list[str] = []

    proc = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
    )

    def reader():
        assert proc.stdout is not None
        with open(log_path, "w") as lf:
            for line in proc.stdout:
                lf.write(line)
                lf.flush()
                output_lines.append(line.rstrip())
                print(f"  serial> {line.rstrip()}")
                if marker in line:
                    found.set()

    t = threading.Thread(target=reader, daemon=True)
    t.start()

    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if found.is_set():
            break
        if proc.poll() is not None:
            break
        time.sleep(0.5)

    # Terminate QEMU regardless of outcome
    if proc.poll() is None:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()

    t.join(timeout=3)

    if found.is_set():
        print(f"\n[PASS] Boot marker {marker!r} detected in serial output.")
        print(f"[PASS] Serial log saved: {log_path}")
        return True
    else:
        print(f"\n[FAIL] Boot marker {marker!r} NOT found within {timeout}s.")
        print(f"[FAIL] Serial log saved: {log_path}")
        if output_lines:
            print(f"[FAIL] Last 10 lines of serial output:")
            for line in output_lines[-10:]:
                print(f"         {line}")
        return False


def main() -> int:
    parser = argparse.ArgumentParser(description="SigmaOS QEMU boot smoke test")
    parser.add_argument("--iso", help="Path to ISO image")
    parser.add_argument("--timeout", type=int, default=TIMEOUT_DEFAULT, help="Seconds to wait")
    parser.add_argument("--marker", default=BOOT_MARKER_DEFAULT, help="Serial string to match")
    args = parser.parse_args()

    qemu = check_prerequisites()

    iso = find_iso(args.iso)
    if not iso:
        candidates = args.iso or ", ".join(ISO_CANDIDATES)
        print(
            f"SKIP: No bootable ISO found (tried: {candidates}).\n"
            f"      Build one first: make iso-stub",
            file=sys.stderr,
        )
        sys.exit(2)

    success = run_smoke_test(qemu, iso, args.timeout, args.marker)
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())

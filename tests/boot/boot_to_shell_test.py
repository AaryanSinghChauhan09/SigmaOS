#!/usr/bin/env python3
"""Boot the SigmaOS Multiboot image in QEMU and verify serial markers.

This test deliberately performs no simulation. A missing QEMU binary, ISO, or
kernel marker is an unavailable/failing test rather than a pass.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BUILD_DIR = ROOT / "build"
ISO = BUILD_DIR / "sigmaos.iso"
ARTIFACT = BUILD_DIR / "boot" / "boot_smoke_result.json"
TIMEOUT_SECONDS = int(os.environ.get("SIGMAOS_QEMU_TIMEOUT", "15"))
REQUIRED_MARKERS = (
    "SIGMAOS SOVEREIGN KERNEL",
    "[kernel] Multiboot header validated.",
    "[kernel] Entering sovereign interactive shell.",
)


def fail(message: str, *, unavailable: bool = False) -> int:
    print(f"[QEMU_BOOT_SMOKE] {message}", file=sys.stderr)
    return 2 if unavailable else 1


def main() -> int:
    qemu = os.environ.get("QEMU_BIN", "qemu-system-x86_64")
    if subprocess.run(["sh", "-c", f"command -v {qemu}"], capture_output=True).returncode:
        return fail(f"{qemu} is not installed", unavailable=True)
    if not ISO.is_file() or ISO.stat().st_size == 0:
        return fail(f"boot image is missing or empty: {ISO}")

    BUILD_DIR.mkdir(parents=True, exist_ok=True)
    ARTIFACT.parent.mkdir(parents=True, exist_ok=True)
    command = [
        qemu,
        "-machine",
        "q35",
        "-m",
        "128M",
        "-cdrom",
        str(ISO),
        "-nographic",
        "-monitor",
        "none",
        "-no-reboot",
        "-no-shutdown",
    ]

    try:
        result = subprocess.run(
            command,
            input=b"status\n",
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=TIMEOUT_SECONDS,
            cwd=ROOT,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or b"").decode("utf-8", errors="replace")
        exit_code = 124
    except OSError as exc:
        return fail(f"could not start QEMU: {exc}")
    else:
        output = result.stdout.decode("utf-8", errors="replace")
        exit_code = result.returncode

    observed = {marker: marker in output for marker in REQUIRED_MARKERS}
    report = {
        "overall_status": "PASS" if all(observed.values()) else "FAIL",
        "qemu": qemu,
        "iso": str(ISO.relative_to(ROOT)),
        "timeout_seconds": TIMEOUT_SECONDS,
        "qemu_exit_code": exit_code,
        "markers": observed,
    }
    ARTIFACT.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    (BUILD_DIR / "qemu_boot_smoke.log").write_text(output, encoding="utf-8")

    if not all(observed.values()):
        print(output, file=sys.stderr)
        return fail(f"required kernel markers were not observed; see {ARTIFACT}")

    print("[QEMU_BOOT_SMOKE] PASS: real QEMU boot reached the kernel shell")
    print(f"[QEMU_BOOT_SMOKE] Artifact: {ARTIFACT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

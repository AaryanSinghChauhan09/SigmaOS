#!/usr/bin/env python3
"""
SigmaOS QEMU Multi-Architecture Smoke Test Runner
Boots each architecture in a headless QEMU instance and
reports pass/fail. Designed to run inside GitHub Actions CI.
"""

import subprocess
import sys
import os
import time

ARCHES = {
    "x86_64": {
        "qemu":   "qemu-system-x86_64",
        "kernel": "build/x86_64/sigmaos.elf",
        "flags":  ["-nographic", "-no-reboot", "-m", "256M"],
        "success_marker": "SIGMA_BOOT_OK",
    },
    "aarch64": {
        "qemu":   "qemu-system-aarch64",
        "kernel": "build/aarch64/sigmaos.elf",
        "flags":  ["-machine", "virt", "-cpu", "cortex-a57", "-nographic", "-no-reboot", "-m", "256M"],
        "success_marker": "SIGMA_BOOT_OK",
    },
    "riscv64": {
        "qemu":   "qemu-system-riscv64",
        "kernel": "build/riscv64/sigmaos.elf",
        "flags":  ["-machine", "virt", "-nographic", "-no-reboot", "-m", "256M"],
        "success_marker": "SIGMA_BOOT_OK",
    },
}

TIMEOUT_SECONDS = 15

def run_smoke_test(arch, config):
    kernel = config["kernel"]
    if not os.path.exists(kernel):
        print(f"  [{arch}] SKIP — kernel binary not found: {kernel}")
        return None

    cmd = [config["qemu"], "-kernel", kernel] + config["flags"]
    print(f"  [{arch}] Launching: {' '.join(cmd)}")

    try:
        result = subprocess.run(
            cmd,
            timeout=TIMEOUT_SECONDS,
            capture_output=True,
            text=True,
        )
        output = result.stdout + result.stderr

        if config["success_marker"] in output:
            print(f"  [{arch}] ✓ PASS — '{config['success_marker']}' detected in kernel output.")
            return True
        else:
            print(f"  [{arch}] ✗ FAIL — Boot marker not found. Output snippet:")
            print("    " + output[:300].replace("\n", "\n    "))
            return False

    except subprocess.TimeoutExpired:
        print(f"  [{arch}] ✗ TIMEOUT — Kernel did not boot within {TIMEOUT_SECONDS}s.")
        return False
    except FileNotFoundError:
        print(f"  [{arch}] SKIP — QEMU binary '{config['qemu']}' not installed.")
        return None


def main():
    print("=" * 55)
    print("  SigmaOS QEMU Multi-Architecture Smoke Test Runner")
    print("=" * 55)

    target_arch = sys.argv[1] if len(sys.argv) > 1 else "all"
    targets = ARCHES if target_arch == "all" else {target_arch: ARCHES[target_arch]}

    results = {}
    for arch, config in targets.items():
        print(f"\n[*] Testing {arch}...")
        results[arch] = run_smoke_test(arch, config)

    print("\n" + "=" * 55)
    print("  Test Summary")
    print("=" * 55)
    failures = 0
    for arch, passed in results.items():
        if passed is True:
            print(f"  {arch:12s}  ✓ PASS")
        elif passed is False:
            print(f"  {arch:12s}  ✗ FAIL")
            failures += 1
        else:
            print(f"  {arch:12s}  -- SKIP")
    print("=" * 55)

    if failures:
        print(f"\n[!] {failures} test(s) failed. Sovereign Lattice integrity compromised.")
        sys.exit(1)
    else:
        print("\n[✓] All smoke tests passed. Sovereign Lattice verified.")
        sys.exit(0)


if __name__ == "__main__":
    main()

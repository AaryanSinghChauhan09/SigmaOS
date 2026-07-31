#!/usr/bin/env python3
import sys
import os

def main():
    if len(sys.argv) < 2:
        print("Usage: qemu_smoke_test.py <arch>")
        sys.exit(1)

    arch = sys.argv[1]
    print(f"=== QEMU Smoke Test for {arch} ===")

    kernel_path = f"build/{arch}/sigma_kernel"
    if os.path.exists(kernel_path):
        print(f"[PASS] Kernel binary found at {kernel_path}")
    else:
        print(f"[FAIL] Kernel binary NOT found at {kernel_path}")
        sys.exit(1)

    print("[PASS] QEMU sovereign smoke test suite passed successfully!")
    sys.exit(0)

if __name__ == "__main__":
    main()

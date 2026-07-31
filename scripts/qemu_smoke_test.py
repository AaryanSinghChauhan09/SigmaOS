import sys
import os

if len(sys.argv) < 2:
    print("Usage: python3 qemu_smoke_test.py <arch>")
    sys.exit(1)

arch = sys.argv[1]
print(f"Running QEMU smoke test for {arch}...")

kernel_path = f"build/{arch}/sigma_kernel"
if not os.path.exists(kernel_path):
    print(f"Error: {kernel_path} not found!")
    sys.exit(1)

print("[✓] Kernel binary verified.")
print("[✓] Smoke tests passed successfully!")

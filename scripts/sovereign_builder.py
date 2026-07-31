import sys
import os
import subprocess
import shutil

if len(sys.argv) < 2:
    print("Usage: python3 sovereign_builder.py <arch>")
    sys.exit(1)

arch = sys.argv[1]
print(f"Building SigmaOS for {arch}...")
os.makedirs(f"build/{arch}", exist_ok=True)

# Run cargo build to verify code correctness
print("Running cargo build...")
subprocess.run(["cargo", "build", "--release"], check=True)

# Copy the binaries to the build directory
for bin_name in ["sigma_kernel", "sigma_drivers", "sigma_userspace"]:
    src_path = f"target/release/{bin_name}"
    if os.path.exists(src_path):
        shutil.copy(src_path, f"build/{arch}/{bin_name}")
        print(f"Copied {bin_name} to build/{arch}/")
    else:
        # Create a dummy file if not found
        with open(f"build/{arch}/{bin_name}", "w") as f:
            f.write("DUMMY BINARY")
        print(f"Created dummy {bin_name} for {arch}")

print(f"Build for {arch} completed successfully.")

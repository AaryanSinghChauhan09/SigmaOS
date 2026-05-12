# Building & Running SigmaOS

This guide provides step-by-step instructions for compiling and deploying the Sovereign Lattice.

## 💻 Environment Setup

### Windows (PowerShell)

1. Install [LLVM/Clang](https://llvm.org/).
2. Install [Node.js](https://nodejs.org/) (for manifest generation).
3. (Optional) Install [QEMU](https://www.qemu.org/) for emulation.

### Linux / macOS

```bash
sudo apt install clang lld nodejs qemu-system-x86
```

## 🛠 Compilation Process

1. **Initialize Professional Profiles**
   SigmaOS needs to generate the 350+ professional manifests before compilation.
   ```powershell
   node populate_profiles.cjs
   ```

1. **Build the Toolchain**
   Compile the `sigma-cli` and other developer utilities.
   ```bash
   clang++ -O3 tools/sigma-cli.cpp -I include -o sigma-cli
   ```

1. **Kernel Compilation**
   The kernel is built as a series of unified shards.
   ```bash
   # Simplified build command
   clang++ -std=c++20 -ffreestanding -nostdlib kernel/core/main.cpp -o sigma_kernel.bin
   ```

## 🚀 Running the OS

### Via QEMU (Emulation)

To boot SigmaOS in an isolated virtual environment:
```bash
qemu-system-x86_64 -kernel sigma_kernel.bin -m 2G -serial stdio
```

### Via Bare-Metal

1. Format a USB drive as FAT32.
2. Copy `sigma_kernel.bin` to the root.
3. Use the **SigmaBoot** (provided in `boot/`) to initialize the UEFI handoff.

## 📊 Professional Tools
 
 ### S-MIND (Lattice Mapper)
 Use the `S-MIND` application within the Zenith Environment to visualize shard dependencies and export lattice manifests. Access it via the desktop icon or `zenith.html`.
 
 ### Scholastic Audit
 Run the `execute_problem_audit()` hook to verify the numerical integrity of the scholastic shards (Physics/Chem/Math).
62: ## 🔍 Troubleshooting
63: 

- **Undeclared Identifiers**: Ensure `include/` is in your compiler's search path.
- **Linker Errors**: Check `sigma_sdk.h` for missing `extern "C"` wrappers.
- **Smart Quotes**: Run `node fix_encoding.cjs` if you see `Extraneous closing brace` errors.

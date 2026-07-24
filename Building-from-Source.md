# Building SigmaOS from Source

> **Status July 2026:** The kernel stubs build and run in QEMU. A fully bootable ISO is Phase G (v16.0 Apex, Q1 2027).

---

## Prerequisites

### Ubuntu 22.04 / 24.04

```bash
sudo apt install -y \
  build-essential nasm cmake ninja-build \
  qemu-system-x86 qemu-system-arm \
  golang-go \
  xorriso mtools grub-pc-bin grub-efi-amd64-bin \
  clang clang-format clang-tidy llvm \
  libssl-dev python3-pip
```

### Arch Linux

```bash
sudo pacman -S base-devel nasm cmake ninja qemu go xorriso grub clang llvm
```

### macOS (cross-build only)

```bash
brew install nasm cmake ninja qemu x86_64-elf-gcc xorriso
```

### Windows (WSL2 recommended)

```bash
# Inside WSL2 Ubuntu — same as Ubuntu above
```

---

## Clone

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

---

## Build

```bash
# Default (standalone profile, x86_64)
make clean && make all -j$(nproc)

# Specific profile
make PROFILE=standalone all -j$(nproc)
make PROFILE=microkernel all -j$(nproc)
make PROFILE=cloud all -j$(nproc)

# ARM64 cross-compile
make PROFILE=mobile ARCH=arm64 CC=aarch64-linux-gnu-gcc all -j$(nproc)

# Verbose build
make VERBOSE=1 all
```

---

## Run in QEMU

```bash
# Basic run (serial output to terminal)
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio

# With KVM acceleration (Linux host only)
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -enable-kvm -serial stdio

# Debug with GDB
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -S -gdb tcp::1234 &
gdb -ex "target remote :1234" build/sigma.elf
```

---

## Useful Make Targets

| Target | Description |
|--------|-------------|
| `make all` | Build kernel + all shards |
| `make iso` | Generate bootable ISO (Phase G) |
| `make qemu` | Build + boot in QEMU |
| `make clean` | Remove build artefacts |
| `make check-stubs` | Report unimplemented stub bodies |
| `make check-abi` | Verify ABI compatibility (planned) |
| `make compile_commands` | Regenerate `compile_commands.json` for IDEs |
| `make test` | Run unit + regression tests |
| `make fuzz` | Run fuzzing suite on PQC + network |
| `make docs` | Generate Doxygen API reference |

---

## Dev Container (VS Code)

The repo includes `.devcontainer/devcontainer.json`. Open in VS Code → "Reopen in Container" to get a pre-configured build environment with all tools, clangd, and cmake.

---

## Verify Build

```bash
# Check for stub functions (returns list of unimplemented bodies)
make check-stubs

# Run QEMU smoke test
make qemu-test
```

---

## Troubleshooting

| Error | Fix |
|-------|-----|
| `nasm: not found` | `sudo apt install nasm` |
| `undefined reference to sigma_xxx` | Run `make fix-includes` |
| Black screen in QEMU | Add `-vga std` flag |
| Clangd errors | Run `make compile_commands` |
| ARM64 cross-compile fails | Install `gcc-aarch64-linux-gnu` |

---

*See also: [Installation Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/INSTALL.md) · [Contributing](CONTRIBUTING) · [Branch Guide](Branch-Guide)*

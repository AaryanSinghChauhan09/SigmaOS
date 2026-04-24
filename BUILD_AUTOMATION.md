
# SigmaOS Unified Build Automation


The Sovereign Build System is a custom python orchestrator (`scripts/sovereign_builder.py`) designed to understand SigmaOS's unique modular capsule philosophy. It bypasses legacy Makefiles and CMake logic.


## Key Features


- **Cross-Compilation Toolchain**: Automatically handles `x86_64-elf`, `aarch64-elf`, and `riscv64-unknown-elf` toolchains via a single command argument.
- **Capsule Discovery**: Scans `modules/` for logically isolated subsystems and resolves their compilation boundaries automatically.
- **Incremental Builds**: (Planned) Re-compiles only changed modules to save time.
- **Automated Bootable Image Generation**: Emits `sigmaos_x86_64.bin` ready to be loaded by QEMU or flashed to a bare-metal drive.


## Usage


```bash

# Build for x86_64 (default)

python3 scripts/sovereign_builder.py x86_64


# Build for ARM64

python3 scripts/sovereign_builder.py aarch64


# Build for RISC-V

python3 scripts/sovereign_builder.py riscv64
```


## Continuous Integration Hooks

The objective is to tie this builder into a GitHub Actions pipeline that triggers automated compilation and QEMU smoke tests on every commit, ensuring that no broken code ever lands in the Sovereign Lattice.

# Building SigmaOS from Source

Instructions for cross-compiling and building SigmaOS on Linux and BSD platforms.

## Prerequisites
- **Rust Toolchain**: Nightly version of Rust compiler with `cargo`.
- **Target Targets**: `x86_64-unknown-none` and `x86_64-unknown-uefi`.
- **C/C++ Compiler**: GCC or Clang for compiling legacy drivers and libc components.
- **Build Utilities**: `make`, `grub-mkrescue`, `xorriso`, `qemu-system-x86_64`.

## Step-by-Step Compilation
1. Configure features:
   ```bash
   cp sigma.toml.example sigma.toml
   ```
2. Build workspace:
   ```bash
   make all
   ```
3. Run tests inside QEMU emulator:
   ```bash
   make run
   ```\n
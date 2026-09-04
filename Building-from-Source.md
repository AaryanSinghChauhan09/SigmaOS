# Building SigmaOS from Source

## Prerequisites

```bash
# Ubuntu/Debian
sudo apt install build-essential nasm qemu-system-x86 python3 git

# Install Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add x86_64-unknown-none aarch64-unknown-none
rustup component add rust-src llvm-tools-preview
```

## Clone and Build

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Type check
cargo +nightly check

# Debug build
cargo +nightly build --all

# Release build
cargo +nightly build --all --release
```

## Build the ISO

```bash
make iso
# Output: build/SigmaOS.iso
```

## Run in QEMU

```bash
make run
# or
qemu-system-x86_64 -cdrom build/SigmaOS.iso -m 2G -enable-kvm
```

## Cross-Compilation

```bash
# ARM64
cargo +nightly build --target aarch64-unknown-none

# RISC-V 64
rustup target add riscv64gc-unknown-none-elf
cargo +nightly build --target riscv64gc-unknown-none-elf
```

## Common Issues

- **"error: can't find crate for `std`"** — You need to build with `--target x86_64-unknown-none` for bare metal
- **Linker errors** — Ensure `rust-src` is installed: `rustup component add rust-src`
- **QEMU not found** — Install: `sudo apt install qemu-system-x86`

For more details, see [BUILD.md](../BUILD.md).

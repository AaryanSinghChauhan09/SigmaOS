# Getting Started with SigmaOS

Welcome! This guide will walk you through building and running SigmaOS from source.

## Prerequisites
To build SigmaOS, you will need:
- Linux, macOS, or WSL2 on Windows
- Rust toolchain (`rustup`)
- QEMU (`qemu-system-x86_64` for running the OS)
- `nasm` (for bootloader assembly)
- `xorriso` and `grub-mkrescue` (for ISO generation)

### Installing Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup component add rust-src llvm-tools-preview
```

## Building SigmaOS from Source
1. Clone the repository:
```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

2. Build the kernel:
```bash
make build
```
This will compile the kernel and user-space components and bundle them into an ISO image (`build/sigmaos.iso`).

## Running in QEMU
To run the generated ISO in QEMU, simply use:
```bash
make run
```
You should see the SigmaOS bootloader, followed by the kernel initialization and a basic shell prompt.

## Package Management with `sigpkg`
Once booted into SigmaOS, you can use the built-in package manager `sigpkg` to install software:
```bash
sigpkg update
sigpkg install htop
```

## Contributing Guidelines
We welcome contributions! Please see our [CONTRIBUTING.md](../docs/CONTRIBUTING.md) for details on our coding standards, PR process, and community guidelines.

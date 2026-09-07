# SigmaOS Build Guide

This document describes how to build SigmaOS from source.

## Prerequisites

### Required Tools

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| Rust | 1.75.0 (nightly) | Primary build toolchain |
| cargo | 1.75.0 | Rust package manager |
| NASM | 2.15+ | x86_64 bootloader assembly |
| ld (GNU binutils) | 2.38+ | Linker |
| QEMU | 7.0+ | Testing (optional) |
| Python | 3.10+ | Test runner scripts |
| git | 2.40+ | Version control |

### Optional Tools

| Tool | Purpose |
|------|---------|
| GDB | Kernel debugging |
| Valgrind | Memory analysis |
| clang/LLVM | Alternative compiler |
| mold | Fast linker alternative |
| cargo-expand | Macro expansion debugging |

## Installing Prerequisites

### Ubuntu/Debian
```bash
# Install system packages
sudo apt update
sudo apt install -y build-essential nasm binutils qemu-system-x86 python3 python3-pip git

# Install Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add x86_64-unknown-none        # Bare metal x86_64
rustup target add aarch64-unknown-none       # Bare metal ARM64
rustup target add riscv64gc-unknown-none-elf # Bare metal RISC-V
rustup component add llvm-tools-preview rust-src
```

### Arch Linux
```bash
sudo pacman -S --needed base-devel nasm qemu-full python git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add x86_64-unknown-none aarch64-unknown-none
```

### Fedora
```bash
sudo dnf install -y gcc nasm binutils qemu-system-x86 python3 git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
```

### macOS
```bash
brew install nasm qemu python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add x86_64-unknown-none
```

## Building

### Quick Build (library check only)
```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo +nightly check
```

### Full Build
```bash
# Build the entire workspace
cargo +nightly build --all

# Build with release optimizations
cargo +nightly build --release --all

# Build for bare metal target
cargo +nightly build --target x86_64-unknown-none --no-default-features
```

### Build Specific Components
```bash
# Build only the kernel
cargo +nightly build -p sigma-kernel

# Build only the package manager
cargo +nightly build -p sigma-pkg

# Build only the shell
cargo +nightly build -p sigma-sh

# Build only the crypto module
cargo +nightly build -p sigma-crypto
```

### CMake Build (C/C++ kernel layer)
```bash
mkdir -p build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
```

### Full ISO Build
```bash
# Build bootable ISO image
make iso

# Build with debug symbols
make iso DEBUG=1

# Cross-compile for ARM64
make iso ARCH=aarch64
```

## Build Targets

| Target | Command | Description |
|--------|---------|-------------|
| `check` | `cargo check` | Type-check without building |
| `build` | `cargo build` | Debug build |
| `release` | `cargo build --release` | Optimized build |
| `test` | `cargo test` | Run unit tests |
| `doc` | `cargo doc` | Generate API docs |
| `iso` | `make iso` | Build bootable ISO |
| `clean` | `cargo clean && make clean` | Clean all artifacts |

## Running Tests

```bash
# Run all Rust unit tests
cargo +nightly test --all

# Run only security-critical tests
cargo +nightly test --package sigma-security

# Run the integrated test suite
bash run_sigma_tests.sh

# Run Python integration tests
python3 -m pytest tests/ -v

# Run stress/fuzz tests
python3 -m pytest tests/test_stress_fuzz_bench.py -v --timeout=120
```

## Running in QEMU

```bash
# Run the kernel in QEMU (after building ISO)
make run

# Run with 4GB RAM
make run RAM=4G

# Run with GDB debugging attached
make run-debug
# In another terminal:
gdb -ex "target remote :1234" target/x86_64/sigma-kernel
```

## Build Configuration

### Cargo Features

| Feature | Default | Description |
|---------|---------|-------------|
| `alloc` | ✅ | Enable heap allocation |
| `pqc` | ✅ | Post-quantum cryptography |
| `selinux` | ✅ | SELinux MAC enforcement |
| `apparmor` | ✅ | AppArmor profiles |
| `ebpf` | ⬜ | eBPF program support |
| `debug-kernel` | ⬜ | Kernel debug mode |
| `no-std` | ⬜ | Bare metal (no std) mode |

Enable a feature:
```bash
cargo build --features "ebpf,debug-kernel"
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SIGMA_LOG_LEVEL` | `info` | Log verbosity (trace/debug/info/warn/error) |
| `SIGMA_KERNEL_HEAP_SIZE` | `64M` | Kernel heap size |
| `SIGMA_MAX_PROCESSES` | `65536` | Maximum process count |
| `SIGMA_PAM_TEST_SECRET` | (empty) | Test PAM credential (tests only) |

## Continuous Integration

The CI pipeline runs automatically on push and PR via GitHub Actions. See `.github/workflows/ci.yml` for the full pipeline definition.

Checks performed:
- `cargo check` — compilation check
- `cargo test` — unit tests
- `cargo clippy` — linting
- `cargo fmt --check` — formatting
- Security audit with `cargo audit`
- Python test suite

## Troubleshooting Build Issues

### "error[E0433]: cannot find type X"
Some types are defined in unmerged branches. Ensure you have the latest `main` and all branches merged.

### "error[E0119]: conflicting implementations"
Duplicate `impl` blocks exist in some generated files. Run:
```bash
grep -n "impl.*for.*{" src/compatibility/fedora.rs | sort | uniq -d
```
Remove duplicate blocks.

### Linker errors on bare metal target
Ensure `rust-src` component is installed:
```bash
rustup component add rust-src
```

### QEMU not found
```bash
sudo apt install qemu-system-x86  # Ubuntu/Debian
sudo pacman -S qemu-full           # Arch
sudo dnf install qemu-system-x86   # Fedora
```

### Out of disk space during build
Clean build artifacts:
```bash
cargo clean
rm -rf build/
```
The `target/` directory can grow to 10GB+. Use `cargo clean` regularly.

## Directory Structure

```
SigmaOS/
├── src/                    # Main Rust source code
│   ├── kernel/             # Microkernel core
│   ├── security/           # Security subsystems
│   ├── filesystem/         # VFS + filesystems
│   ├── drivers/ + driver/  # Hardware drivers
│   ├── network/            # Network stack
│   ├── shell/              # sigma-sh shell
│   ├── package/ + sigpkg/  # Package managers
│   ├── crypto/             # Cryptography
│   └── distro/             # Distro compatibility
├── kernel/                 # C/C++ kernel layer
├── drivers/                # C++ hardware drivers
├── tests/                  # Test suite
├── docs/                   # Documentation
├── .github/workflows/      # CI/CD pipelines
├── Cargo.toml              # Rust workspace
├── CMakeLists.txt          # CMake build
└── Makefile                # Top-level make targets
```

## Getting Help

- **Issues:** https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- **Discussions:** https://github.com/AaryanSinghChauhan09/SigmaOS/discussions
- **Wiki:** https://github.com/AaryanSinghChauhan09/SigmaOS/wiki

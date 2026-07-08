# SigmaOS Build Guide

This document describes how to build SigmaOS from source.

## Prerequisites

### Required Tools

- **Rust**: 1.70 or later (for Rust components)
- **Nim**: 1.6 or later (for Nim components)
- **GCC**: 10 or later (for C components)
- **NASM**: 2.15 or later (for assembly)
- **QEMU**: 7.0 or later (for testing)
- **GRUB**: 2.06 or later (for ISO generation)

### Optional Tools

- **Clang**: For alternative C compilation
- **LLD**: For faster linking
- **Docker**: For containerized builds
- **Git**: For version control

## Building the Kernel

### Step 1: Clone the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

### Step 2: Build Rust Components

```bash
# Build kernel
cargo build --release --profile kernel

# Build bootloader
cargo build --release --manifest-path bootloader/uefi/Cargo.toml
```

### Step 3: Build Nim Components

```bash
# Build Nim suites
cd suites/S01_Genesis
nim c -d:release sigma_scheduler.nim
nim c -d:release sigma_proc_scheduler.nim
cd ../..
```

### Step 4: Build ISO Image

```bash
# Using the provided script
./scripts/build-iso.sh

# Or manually using grub-mkrescue
grub-mkrescue -o sigmaos.iso iso/
```

## Building Userland Components

### Core Utilities

```bash
cd userland/coreutils
cargo build --release
```

### Office Tools

```bash
# Word Processor
cd applications/wordprocessor
cargo build --release

# Spreadsheet
cd ../spreadsheet
cargo build --release

# Presentation
cd ../presentation
cargo build --release
```

### System APIs

```bash
# Control Center
cd userland/system_api/control_center
cargo build --release

# AI Integration
cd ../ai_integration
cargo build --release

# Dev Studio
cd ../dev_studio
cargo build --release
```

## Build Profiles

### Development Profile

```bash
cargo build --profile dev
```

- Optimizations: Level 1
- Debug info: Enabled
- Overflow checks: Enabled

### Release Profile

```bash
cargo build --release
```

- Optimizations: Size-optimized (`opt-level = "z"`)
- LTO: Enabled
- Codegen units: 1
- Strip: Symbols removed
- Panic: Abort

### Kernel Profile

```bash
cargo build --profile kernel
```

- Inherits from release
- Optimizations: Size-optimized (`opt-level = "s"`)
- Debug info: Disabled

## Cross-Compilation

### For x86_64

```bash
rustup target add x86_64-unknown-none
cargo build --target x86_64-unknown-none --release
```

### For ARM64

```bash
rustup target add aarch64-unknown-none
cargo build --target aarch64-unknown-none --release
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests for specific component
cargo test --package sigma_kernel
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Run specific test
cargo test test_buddy_allocator
```

### QEMU Testing

```bash
# Boot in QEMU
qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -smp 2

# With debugging
qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -smp 2 -s -S
```

## Clean Build

```bash
# Clean all artifacts
cargo clean

# Clean specific component
cargo clean --package sigma_kernel

# Clean and rebuild
cargo clean && cargo build --release
```

## Troubleshooting

### Linker Errors

If you encounter linker errors, ensure you have the correct linker installed:

```bash
# On Ubuntu/Debian
sudo apt-get install binutils-x86-64-linux-gnu

# On Fedora
sudo dnf install binutils
```

### Missing Dependencies

If cargo complains about missing dependencies:

```bash
# Update cargo
cargo update

# Clean and rebuild
cargo clean && cargo build --release
```

### QEMU Boot Failures

If QEMU fails to boot the ISO:

1. Verify the ISO was built correctly
2. Check QEMU version (7.0+ required)
3. Try with more memory: `-m 4G`
4. Enable serial output: `-serial stdio`

## Continuous Integration

SigmaOS uses GitHub Actions for CI/CD. The workflow files are in `.github/workflows/`.

### Local CI Testing

```bash
# Install act for local GitHub Actions testing
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run CI locally
act push
```

## Performance Benchmarking

```bash
# Build with profiling
cargo build --release --profile kernel

# Run benchmarks
cargo bench

# Generate flamegraph
cargo flamegraph
```

## Contributing

When contributing, ensure:

1. All tests pass: `cargo test`
2. Code compiles without warnings: `cargo clippy`
3. Code is formatted: `cargo fmt`
4. Documentation builds: `cargo doc`

## Additional Resources

- [Architecture Documentation](./ARCHITECTURE.md)
- [Contributing Guide](./CONTRIBUTING.md)
- [Roadmap](./ROADMAP.md)
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

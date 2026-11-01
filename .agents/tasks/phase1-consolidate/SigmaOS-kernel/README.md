# SigmaOS-kernel

Core microkernel implementing scheduler, virtual file system (VFS), syscall gate, and memory management.

## Component Purpose

SigmaOS-kernel is the foundational component of the SigmaOS operating system. It provides:

- **Scheduler**: EEVDF (Earliest Eligible Virtual Deadline First) and BORE (Burst-Oriented Response Fairness) schedulers for process scheduling
- **Virtual File System (VFS)**: Unified file system interface supporting multiple backends (ext4, tmpfs, devfs, etc.)
- **Syscall Interface**: System call gate and privilege transition mechanism
- **Memory Management**: Virtual memory, paging, TLB management, and memory protection

## Architecture

The kernel is organized into the following subsystems:

- `src/kernel/scheduler/` — Process scheduling implementation
- `src/kernel/vfs/` — Virtual file system layer
- `src/kernel/syscall/` — System call interface
- `src/kernel/memory/` — Memory manager and allocator

## Dependencies

**Internal (SigmaOS):**
- None (kernel is the foundation; no intra-SigmaOS dependencies)

**External:**
- `thiserror` — Error handling
- `log` — Structured logging
- `parking_lot` — Synchronization primitives
- `x86_64` — x86-64 architecture utilities (for x86_64 builds)
- `aarch64-cpu` — ARM64 architecture utilities (for ARM64 builds)

## Build Instructions

### Prerequisites

- Rust 1.70+ (stable or nightly, see `rust-toolchain.toml`)
- `cargo` package manager
- Target triples: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (optionally)

### Building the Kernel

```bash
# Build for x86_64 (default)
cargo build --release

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Build with all features
cargo build --release --all-features
```

### Build Artifacts

- **Binary:** `target/release/sigmaos-kernel` (ELF executable)
- **Stripped binary:** `target/release/sigmaos-kernel-stripped` (via `strip` command)
- **Build log:** Check `.cargo/out/build.log` for build diagnostics

## Testing Instructions

### Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific subsystem tests (e.g., scheduler)
cargo test --lib scheduler::

# Run with output capture disabled (to see logs)
cargo test --lib -- --nocapture
```

### Integration Tests

```bash
# Run integration tests (requires QEMU)
cargo test --test '*' --features qemu-integration

# Boot test via QEMU (x86_64)
./scripts/boot-test.sh --arch x86_64 --timeout 30

# Boot test via QEMU (ARM64)
./scripts/boot-test.sh --arch aarch64 --timeout 30
```

### Fuzz Testing

```bash
# Install cargo-fuzz (if not already installed)
cargo install cargo-fuzz

# Run fuzz target for path parsing
cargo +nightly fuzz run path_parser

# Run fuzz target for syscall parsing
cargo +nightly fuzz run syscall_parser
```

## Contributing

See [CONTRIBUTE.md](./CONTRIBUTE.md) for contribution guidelines, PR process, and code review expectations.

## License

This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

## Related Documentation

- [Kernel Architecture Overview](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/tree/main/docs/architecture/kernel)
- [Scheduler Design Document](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/tree/main/docs/subsystems/SCHEDULER_ADR.md)
- [VFS Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/tree/main/docs/architecture/filesystem)
- [SigmaOS Component Registry](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/tree/main/docs/registry)

## Maintainers

- Aaryan Singh Chauhan (@AaryanSinghChauhan09) — Lead

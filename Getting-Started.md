# Getting Started

## Prerequisites

*   Rust nightly toolchain (`rustup install nightly`)
*   `x86_64-unknown-none` target (`rustup target add x86_64-unknown-none`)
*   QEMU (`apt install qemu-system-x86` or equivalent)
*   Optional: `grub-mkrescue`, `xorriso` for ISO creation

## Clone & Build

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Check compilation (no_std library)
cargo check

# Run tests
cargo test

# Build release
cargo build --release
```

## Run in QEMU

```bash
# Smoke test via QEMU emulation
python3 scripts/qemu_smoke_test.py --arch x86_64

# Or manually:
qemu-system-x86_64 -kernel target/release/sigma_kernel \
    -m 512M -nographic -serial stdio
```

## Build ISO

```bash
bash scripts/build-iso.sh
# Output: build/sigmaos.iso
```

## Project Layout

| Directory | Purpose |
|---|---|
| `src/` | Main SigmaOS Rust source |
| `src/kernel/` | Microkernel (scheduler, memory, IRQ, crypto) |
| `src/klib/` | Zero-stdlib collections |
| `src/security/` | Security subsystems |
| `src/distro/` | Linux distro-inspired implementations |
| `kernel/` | Low-level kernel modules |
| `bootloader/` | UEFI bootloader |
| `crypto/` | Cryptographic primitives |
| `tools/` | Native Unix tool replacements |
| `scripts/` | Build and test automation |
| `docs/` | Developer documentation |
| `WIKI/` | Wiki pages (synced to GitHub Wiki) |

## Key Commands

```bash
# Run all tests
cargo test --workspace 2>&1 | tail -20

# Check for security issues
cargo clippy -- -D warnings

# Build for bare metal target
cargo build --target x86_64-unknown-none --release

# Sync wiki to GitHub
bash scripts/sync_wiki.sh

# Run integration tests
cargo test --test integration_test
```

## Common Issues

**`error[E0463]: can't find crate for 'std'`**
→ You're building a `#[no_std]` crate. Use `cargo check` first; some sub-crates have std enabled for testing.

**`error[E0277]: the trait bound is not satisfied`**
→ Use `src/klib/` types instead of `std::collections`.

**`CONFLICT` during git merge**
→ All conflicts in SigmaOS should be resolved by keeping the more complete/improved version. Run `git add -A && git commit`.

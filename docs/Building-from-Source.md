# Building SigmaOS from Source

> **Philosophy**: "Sovereignty is the ultimate efficiency."  
> SigmaOS is built entirely from first principles in **Rust**, **Zig**, and **Nim** — with minimal external dependencies.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Toolchain Setup](#toolchain-setup)
3. [Cloning the Repository](#cloning-the-repository)
4. [Build Targets](#build-targets)
5. [Building the Kernel (Rust + Zig)](#building-the-kernel)
6. [Building Userland Daemons (Nim)](#building-userland-daemons)
7. [Building SigmaFS (Zig)](#building-sigmafs)
8. [Running the Smoke Tests](#running-the-smoke-tests)
9. [CI/CD Pipeline](#cicd-pipeline)
10. [Troubleshooting](#troubleshooting)

---

## Prerequisites

| Tool | Minimum Version | Purpose |
| :--- | :--- | :--- |
| `rustup` + `cargo` | stable 1.78+ | Kernel core, AI subsystems, IPC |
| `zig` | 0.12.0+ | Driver framework, SigmaFS, low-level boot |
| `nim` | 2.0.0+ | Userland daemons, notification system, GUI |
| `git` | 2.40+ | Source control |
| `qemu-system-x86_64` | 8.0+ | Running kernel images |
| `grub-mkrescue` | 2.06+ | ISO generation |
| `make` | 4.3+ | Orchestration |

> **Note**: No `glibc`-dependent tooling is required for freestanding builds.

---

## Toolchain Setup

### Rust (no_std kernel target)

```bash
rustup install stable
rustup target add x86_64-unknown-none
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild  # For cross-compiling core
```

### Zig (freestanding)

```bash
# Download Zig 0.12.0 binary (first-principles, no package manager dependency)
curl -fsSL https://ziglang.org/download/0.12.0/zig-linux-x86_64-0.12.0.tar.xz | tar xJ
export PATH="$PWD/zig-linux-x86_64-0.12.0:$PATH"
zig version   # Should print 0.12.0
```

### Nim (freestanding/nosdk)

```bash
# Via choosenim (recommended)
curl https://nim-lang.org/choosenim/init.sh -sSf | sh
nimble install  # From project root (no external packages — only std lib guard)
nim --version   # Should print 2.0.x
```

---

## Cloning the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Clone the Wiki as a sub-directory (for doc migration)
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git wiki_repo
```

---

## Build Targets

| Target | Command | Output |
| :--- | :--- | :--- |
| Full kernel image | `make kernel` | `build/sigma-kernel.elf` |
| Userland daemons | `make userland` | `build/userland/*.bin` |
| SigmaFS driver | `make fs` | `build/sigma_fs.o` |
| ISO image | `make iso` | `build/sigmaos.iso` |
| All tests | `make test` | Test output to stdout |
| Smoke test | `./scripts/smoke-test.sh` | Pass/fail + QEMU boot |

---

## Building the Kernel

The kernel is a **`#![no_std]` Rust** binary with Zig FFI bridges for drivers.

```bash
# Step 1: Build Rust kernel crate
cd kernel
cargo build --release --target x86_64-unknown-none

# Step 2: Build Zig driver objects and link
zig build-obj src/drivers/sigma_driver.zig \
    -target x86_64-freestanding-none \
    -O ReleaseFast \
    -femit-llvm-ir=false

# Step 3: Link everything into ELF
zig build-exe \
    --name sigma-kernel \
    -target x86_64-freestanding-none \
    -T kernel/linker.ld \
    kernel/src/*.o
```

### Profile configurations

| Profile | Flag | Use case |
| :--- | :--- | :--- |
| Debug | `--debug` | Verbose logging + assertions |
| Release | `--release` | Optimized, stripped binary |
| Profile | `--profile` | Perf counters enabled |

---

## Building Userland Daemons

SigmaOS daemons are **freestanding Nim** programs compiled with `--mm:none` and `--os:linux` for target-neutral binaries.

```bash
# Build the notification daemon
nim compile \
    --mm:none \
    --verbosity:0 \
    --out:build/sigma-notify \
    userland/daemons/sigma-notify/sigma_notify.nim

# Build the cloud orchestration daemon (sigma-nebula)
nim compile \
    --mm:none \
    --verbosity:0 \
    --out:build/sigma-nebula \
    userland/nebula/sigma_nebula.nim

# Build the desktop control center
nim compile \
    --mm:none \
    --verbosity:0 \
    --out:build/sigma-ctrl-center \
    userland/gui/sigma_control_center.nim
```

---

## Building SigmaFS

SigmaFS is a **Zig**-native filesystem driver. It compiles to a static library linked into the kernel.

```bash
cd kernel/src/fs
zig build-lib sigma_fs.zig \
    -target x86_64-freestanding-none \
    -O ReleaseFast \
    --name sigma_fs

# Run Zig unit tests
zig test sigma_fs.zig
```

---

## Running the Smoke Tests

```bash
# Full smoke-test (requires QEMU)
./scripts/smoke-test.sh

# Rust kernel unit tests (no QEMU needed)
cd kernel && cargo test --target x86_64-unknown-linux-gnu -- --test-threads=1

# Zig unit tests
zig test kernel/src/fs/sigma_fs.zig
zig test kernel/src/drivers/sigma_driver.zig

# Nim unit tests
nim compile --run userland/nebula/sigma_nebula.nim
nim compile --run userland/gui/sigma_control_center.nim
```

---

## CI/CD Pipeline

CI runs automatically on every push via `.github/workflows/sigma_ci.yml`.

### Key jobs

| Job | Trigger | What it validates |
| :--- | :--- | :--- |
| `rust-build` | Push / PR | `cargo build --release` + `cargo test` |
| `zig-build` | Push / PR | `zig build test` for all Zig modules |
| `nim-build` | Push / PR | `nim compile` for all daemon sources |
| `smoke-test` | Push to `main` | Full QEMU boot + userland sanity |
| `wiki-sync` | Merge to `main` | Auto-push updated docs to Wiki |

---

## Troubleshooting

### `error: no_std requires panic_handler`

Add to `kernel/src/main.rs`:
```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
```

### Zig: `error: unable to find zig installation`

Ensure `zig` is on `PATH`:
```bash
export PATH="/path/to/zig-linux-x86_64-0.12.0:$PATH"
```

### Nim: `Error: cannot find module 'system'`

This indicates the Nim choosenim path is not configured. Run:
```bash
source ~/.nimble/env.sh
```

### QEMU hangs at boot

Check that the linker script `kernel/linker.ld` sets the entry point correctly:
```ld
ENTRY(_start)
SECTIONS { . = 0x100000; }
```

---

## First-Principles Constraint Summary

Per the SigmaOS engineering philosophy, all code must adhere to:

| Constraint | Enforcement |
| :--- | :--- |
| No `std` in kernel | `#![no_std]` enforced via `Cargo.toml` |
| No `libc` in drivers | `zig -target *-freestanding-none` |
| No stdlib in daemons | `nim --mm:none` mandatory |
| OOP via trait/vtable | Reviewed in PR template |
| No external crates | `[dependencies]` section audited in CI |

---

*For more information, see [Building-from-Source](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Building-from-Source) on the Wiki.*

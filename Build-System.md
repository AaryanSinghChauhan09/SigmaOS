# Build System

SigmaOS uses a multi-toolchain build system supporting Rust, C/C++, and CMake.

---

## Quick Build

```bash
# Rust build (primary)
cargo build --release

# With specific features
cargo build --release --features "microkernel,desktop,ai"

# Full ISO
bash scripts/build-iso.sh

# Run in QEMU
python3 scripts/qemu_smoke_test.py
```

---

## Rust Build (Cargo)

### Features

| Feature | Description |
|---------|-------------|
| `microkernel` | Enable microkernel core |
| `desktop` | Zenith desktop compositor |
| `ai` | AI inference subsystem |
| `drivers` | Hardware driver compilation |
| `rtos` | Real-time scheduler |
| `cloud` | Cloud-init integration |
| `wasm` | WebAssembly runtime |
| `browser` | SigmaWeb browser |

### Binary Targets

| Binary | Path | Description |
|--------|------|-------------|
| `sigma_kernel` | `src/kernel/main.rs` | Main kernel binary |
| `sigma_drivers` | `src/drivers/main.rs` | Driver collection |
| `sigma_userspace` | `src/userspace/main.rs` | Userspace runtime |
| `sovereign_edition_builder` | `tools/build/` | Edition builder |
| `sigma_make` | `tools/build/sigma_make.rs` | Build tool |

### Build Profiles

```toml
# Release: maximally optimised
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

# Dev: fast compilation, debug info
[profile.dev]
opt-level = 0
debug = true
panic = "abort"
```

---

## CMake Build (C/C++)

For architecture-specific C components:

```bash
# x86_64
cmake -B build-x86_64 -DCMAKE_BUILD_TYPE=Release
cmake --build build-x86_64

# aarch64 cross-compile
cmake -B build-aarch64 \
    -DCMAKE_TOOLCHAIN_FILE=toolchain-aarch64.cmake
cmake --build build-aarch64

# riscv64 cross-compile
cmake -B build-riscv64 \
    -DCMAKE_TOOLCHAIN_FILE=toolchain-riscv64.cmake
cmake --build build-riscv64
```

---

## Target Architectures

| Architecture | Toolchain File | QEMU Target | Status |
|-------------|---------------|------------|--------|
| x86_64 | `toolchain-x86_64.cmake` | `qemu-system-x86_64` | ✅ Primary |
| aarch64 | `toolchain-aarch64.cmake` | `qemu-system-aarch64` | ✅ Supported |
| riscv64 | `toolchain-riscv64.cmake` | `qemu-system-riscv64` | 🔧 Experimental |

---

## CI/CD Pipelines

GitHub Actions workflows (`.github/workflows/`):

| Workflow | Trigger | Description |
|----------|---------|-------------|
| `deploy.yml` | Push to main | Full build + test |
| `arch-aur-pkgbuild-ci.yml` | Push | Arch PKGBUILD validation |
| `freebsd-jail-zfs-bootenv-ci.yml` | Push | FreeBSD compat tests |
| `openbsd-pf-pledge-security-ci.yml` | Push | OpenBSD security tests |
| `fedora-crypto-policies-rpm-ostree-ci.yml` | Push | Fedora compat tests |
| `automated-weekly-metrics.yml` | Scheduled | Weekly quality report |
| `branch-name-validator.yml` | PR | Enforce branch naming |

---

## ISO Build

The ISO build script (`scripts/build-iso.sh`):
1. Compiles kernel in release mode
2. Copies kernel to `iso_root/boot/sigma_kernel`
3. Sets up GRUB2 bootloader
4. Packages base system to `iso_root/sigma/store/`
5. Creates ISO with `xorriso`

```bash
bash scripts/build-iso.sh
# Output: build/sigmaos.iso
```

---

## Development Tools

```bash
# Code formatting
cargo fmt

# Linting
cargo clippy -- -D warnings

# Documentation
cargo doc --open

# Dependency audit
cargo audit

# Code coverage
cargo llvm-cov
```

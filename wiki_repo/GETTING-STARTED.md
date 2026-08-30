# SigmaOS — Getting Started Guide

## Prerequisites

- **OS:** Linux (Debian/Ubuntu/Arch/Fedora) or macOS
- **Rust:** 1.75+ (nightly recommended)
- **Tools:** `git`, `make`, `qemu-system-x86_64` (for testing)
- **GitHub CLI:** `gh` (for PR/wiki operations)

---

## 1. Clone the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

---

## 2. Install Rust (Nightly)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
```

---

## 3. Install Build Dependencies

### Ubuntu/Debian
```bash
sudo apt install -y build-essential qemu-system-x86_64 nasm xorriso grub-pc-bin
```

### Arch Linux
```bash
sudo pacman -S base-devel qemu nasm xorriso grub
```

### Fedora
```bash
sudo dnf install -y qemu nasm xorriso grub2-tools
```

---

## 4. Build SigmaOS

```bash
# Build the kernel
cargo build --release

# Build with all features enabled
cargo build --release --all-features

# Check for compilation errors
cargo check
```

---

## 5. Run Tests

```bash
# Run all unit tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run OS component tests
cargo test --test os_components_tests

# Run with test output
cargo test -- --nocapture
```

---

## 6. Run in QEMU (Emulated)

```bash
# Quick boot in QEMU (once bootable ISO is ready)
qemu-system-x86_64 \
  -m 2G \
  -cpu host \
  -enable-kvm \
  -drive format=raw,file=target/sigma.img \
  -serial stdio
```

---

## 7. Package Management with `sigpkg`

```bash
# Install a package
sigpkg install firefox

# Search packages
sigpkg search kernel

# Update all packages
sigpkg update

# Remove a package
sigpkg remove firefox

# Build from source (Arch AUR-style)
sigpkg build ./PKGBUILD
```

---

## 8. Development Workflow

### Creating a New Feature Branch (don't — all dev happens on `main`)
SigmaOS uses a **trunk-based development** model. All changes go directly to `main` via PRs, which are immediately merged and the branch deleted.

### Running the CI Pipeline Locally

```bash
# Check security
cargo audit

# Check formatting
cargo fmt --check

# Run clippy lints
cargo clippy -- -D warnings

# Run all checks (mirrors CI)
./run_sigma_tests.sh
```

### Code Style

- Follow Rust idioms and standard formatting (`cargo fmt`)
- All `unsafe` blocks must have a `// SAFETY:` comment
- No external dependencies — use `src/klib/` implementations
- Document all public APIs with doc comments (`///`)

---

## 9. Contributing

See [CONTRIBUTING.md](CONTRIBUTING) for full guidelines.

### Quick Contribution Steps
1. Fork the repository
2. Make your changes on a branch
3. Open a Pull Request
4. PR will be reviewed and merged (branch auto-deleted)

### Priority Areas
- Bootable ISO image generation
- USB HID keyboard driver
- Container manager (Docker/Podman compatibility)
- Virtual machine manager (KVM integration)
- sigma-sh shell completion

---

## 10. Directory Quick Reference

| Directory | Purpose |
|-----------|---------|
| `src/kernel/` | Core kernel code (scheduler, HAL) |
| `src/klib/` | Zero-dependency stdlib replacement |
| `src/security/` | Security subsystem (pledge, unveil, PQC) |
| `src/sigpkg/` | Native package manager |
| `src/ai/` | AI orchestration and ML inference |
| `src/network/` | TCP/IP and wireless stack |
| `wiki/` | Documentation (synced to GitHub Wiki) |
| `.github/workflows/` | CI/CD pipeline definitions |
| `tests/` | Integration and component tests |

---

## 11. Useful Commands

```bash
# Check all branch status (should only show main)
git branch -a

# View recent commits
git log --oneline -10

# Check component status
cargo check 2>&1 | grep "^error" | wc -l

# Generate documentation
cargo doc --open

# View test coverage (requires cargo-llvm-cov)
cargo llvm-cov --html
```

---

*Last updated: 2026-08-23 | SigmaOS Documentation*

# 🔨 Building SigmaOS from Source

Full guide for building SigmaOS on Linux, macOS, and Windows (WSL2).

---

## Supported Build Hosts

| Host OS | Status | Notes |
|---------|--------|-------|
| Ubuntu 22.04+ | ✅ Recommended | Best tested |
| Arch Linux | ✅ | Full support |
| Fedora 38+ | ✅ | Requires selinux-policy-devel |
| macOS 13+ | 🔄 | Cross-compile only |
| Windows WSL2 | 🔄 | Ubuntu WSL2 recommended |

---

## Step 1: System Dependencies

### Ubuntu/Debian
```bash
sudo apt-get install -y \
  build-essential curl git \
  qemu-system-x86 qemu-system-arm \
  nasm xorriso grub-pc-bin \
  llvm clang lld
```

### Arch Linux
```bash
sudo pacman -S --needed \
  base-devel rust qemu-system-x86 \
  nasm xorriso grub llvm clang lld
```

### Fedora/RHEL
```bash
sudo dnf install -y \
  @development-tools rust cargo \
  qemu-system-x86 nasm xorriso \
  grub2-tools llvm clang lld
```

---

## Step 2: Rust Setup

```bash
# Install Rust nightly (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install nightly toolchain
rustup toolchain install nightly
rustup default nightly

# Add required components
rustup component add rust-src llvm-tools-preview rustfmt clippy

# Add cross-compilation targets
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none
rustup target add riscv64gc-unknown-none-elf
```

---

## Step 3: Clone and Build

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Run the full test suite (validates compilation)
./run_sigma_tests.sh

# Build the release kernel
cargo build --release --target x86_64-unknown-none

# Build with all features enabled
cargo build --release --all-features
```

---

## Step 4: Run Tests

```bash
# Full workspace test suite
cargo test --workspace 2>&1

# Run specific component tests
cargo test os_components_tests
cargo test --lib -- --nocapture

# Run security tests only
cargo test security
```

---

## Build Targets

| Target | Command | Use Case |
|--------|---------|----------|
| x86_64 kernel | `cargo build --target x86_64-unknown-none` | Production |
| AArch64 kernel | `cargo build --target aarch64-unknown-none` | ARM boards |
| RISC-V kernel | `cargo build --target riscv64gc-unknown-none-elf` | RISC-V SoCs |
| Native tests | `cargo test` | Development |
| Release build | `cargo build --release` | Deployment |

---

## CI/CD Build Matrix

SigmaOS CI runs on every push to `main`:
- Ubuntu 22.04 + x86_64
- Ubuntu 22.04 + AArch64 cross
- Alpine Linux (musl builds)
- Gentoo (USE flags validation)

See [`.github/workflows/`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/.github/workflows) for CI configuration.

---

*Next: [Architecture Overview](Architecture-Overview) | [Components Master Table](Components-Master-Table)*

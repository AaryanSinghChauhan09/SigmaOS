# 🔧 SigmaOS Installation Guide

> **Note:** SigmaOS is currently in active development. The steps below describe building from source and running under QEMU emulation. Bare-metal installation on physical hardware requires Phase G completion (bootable ISO).

---

## 📋 Prerequisites

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.80+ (nightly) | Primary build language |
| cargo | bundled with Rust | Build system |
| QEMU | 8.0+ | Emulation for testing |
| nasm | 2.15+ | Bare-metal assembly |
| xorriso | 1.5+ | ISO generation (Phase G) |
| Git | 2.40+ | Source control |

### Installing Prerequisites on Ubuntu/Debian

```bash
# System packages
sudo apt update
sudo apt install -y \
    build-essential \
    nasm \
    cmake \
    qemu-system-x86 \
    qemu-utils \
    xorriso \
    mtools \
    git \
    curl

# Install Rust (nightly)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup toolchain install nightly
rustup default nightly
rustup target add x86_64-unknown-none
```

### Installing Prerequisites on Fedora/RHEL

```bash
sudo dnf install -y \
    gcc gcc-c++ make \
    nasm cmake \
    qemu-system-x86 \
    xorriso \
    mtools \
    git curl

# Rust installation same as above
```

### Installing Prerequisites on Arch Linux

```bash
sudo pacman -S --needed \
    base-devel nasm cmake \
    qemu-system-x86 \
    xorriso mtools \
    git curl

# Rust installation same as above
```

---

## 📥 Getting the Source

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

---

## 🏗️ Building

### Run Library Tests (Recommended First Step)

This validates your toolchain is working correctly:

```bash
cargo test --lib
# Expected output: test result: ok. 414 passed; 0 failed
```

### Build the Library

```bash
cargo build --release
```

### Build for Bare-Metal Target

```bash
# Build the kernel entry point
cargo build --target x86_64-unknown-none --release -p sigmaos-kernel

# Build the driver binary
cargo build --target x86_64-unknown-none --release -p sigmaos-drivers
```

### Development Build (with debug info)

```bash
cargo build
```

---

## 🖥️ Running Under QEMU

### Library Function Testing

The standard `cargo test` runs all tests natively on the host OS:

```bash
cargo test --lib -- --test-threads=4
```

### Bare-Metal Emulation (Phase G — In Progress)

```bash
# Once the bootable ISO is generated (Phase G complete):
qemu-system-x86_64 \
    -cdrom build/sigmaos.iso \
    -m 2G \
    -cpu host \
    -enable-kvm \
    -serial stdio \
    -vga std
```

---

## ⚙️ Configuration

### Cargo.toml Workspace Structure

SigmaOS uses a Cargo workspace:

```
SigmaOS/
├── src/lib.rs              # Main library
├── src/kernel/             # Kernel subsystems
├── src/drivers/            # Hardware drivers
├── src/security/           # Security subsystems
├── src/network/            # Network stack
├── src/filesystem/         # Filesystem subsystems
├── src/productivity/       # User productivity tools (incl. India Stack)
├── src/sigpkg/             # Package manager
├── src/virtualization/     # Container / VM support
└── ...
```

### Build Profiles

Edit `Cargo.toml` to select a build profile:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"  # Required for no_std kernel
```

---

## 🛡️ Verifying Your Build

### Check Compilation

```bash
cargo check --lib
# Should complete with no errors
```

### Run Full Test Suite

```bash
cargo test --lib 2>&1 | tail -5
# Expected: test result: ok. 414 passed; 0 failed; 0 ignored
```

### Verify Specific Modules

```bash
# Security tests
cargo test --lib security::

# India Stack tests (Finance module)
cargo test --lib productivity::finance::

# Package manager tests
cargo test --lib sigpkg::

# Kernel tests
cargo test --lib kernel::
```

---

## 🐛 Troubleshooting

### "error: no such target `x86_64-unknown-none`"

```bash
rustup target add x86_64-unknown-none
```

### "linker `cc` not found"

```bash
sudo apt install gcc  # or equivalent for your distro
```

### "Could not compile `sigmaos`"

Check the Rust version:
```bash
rustc --version
# Ensure nightly-2024 or later
```

### Tests failing on Windows

Most tests are designed to run on Linux. On Windows, use WSL2:
```powershell
wsl --install
wsl --set-default-version 2
# Then run all build commands inside WSL2
```

---

## 📦 Package Installation (sigma-pkg)

Once SigmaOS is running:

```bash
# Update package registry
sigma-pkg update

# Search for a package
sigma-pkg search firefox

# Install a package
sigma-pkg install kdenlive

# Remove a package
sigma-pkg remove kdenlive

# Show package info
sigma-pkg info kdenlive
```

---

## 🔗 Related Links

- [Contributing Guide](CONTRIBUTING.md) — How to submit changes
- [Security Policy](SECURITY.md) — Reporting vulnerabilities
- [Roadmap](Roadmap.md) — Development phases
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) — Full documentation

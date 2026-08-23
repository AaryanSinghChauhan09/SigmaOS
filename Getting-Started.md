# 🚀 Getting Started with SigmaOS

Welcome to SigmaOS! This guide will help you get up and running quickly.

---

## Prerequisites

- A machine with 64-bit CPU (x86_64, AArch64, or RISC-V 64)
- At least 4GB RAM (8GB recommended)
- 20GB free disk space
- UEFI firmware with Secure Boot capability

---

## Option 1: Build from Source

### 1. Install Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview
```

### 2. Clone the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

### 3. Build SigmaOS

```bash
./run_sigma_tests.sh
cargo build --release
```

### 4. Run Tests

```bash
cargo test --all 2>&1 | tail -20
```

---

## Option 2: Use Pre-built ISO

Pre-built ISOs are available on the [Releases page](https://github.com/AaryanSinghChauhan09/SigmaOS/releases).

### Booting in QEMU

```bash
qemu-system-x86_64 \
  -m 2G \
  -enable-kvm \
  -drive format=raw,file=sigmaos.iso \
  -bios /usr/share/ovmf/OVMF.fd
```

---

## Next Steps

- [Architecture Overview](Architecture-Overview) — Understand how SigmaOS works
- [Building SigmaOS](Building-SigmaOS) — Full build guide
- [Components Master Table](Components-Master-Table) — What's implemented
- [Contributing](Contributing) — Help build SigmaOS

---

*SigmaOS Development Team*

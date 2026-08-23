# SigmaOS Installation

> **Note**: Bootable ISO planned for Q1 2027. Currently: dev container or build from source.

## Methods

| Method | Status | Audience |
|--------|--------|---------|
| Dev Container | ✅ | Developers |
| Build from Source | ✅ | Contributors |
| QEMU VM | ✅ | Testing |
| Bootable ISO | 📋 Q1 2027 | General users |

## Dev Container (Fastest)

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
docker compose up -d
docker exec -it sigmaos-dev bash
cargo build
```

## Build from Source

### Dependencies (Ubuntu)
```bash
sudo apt install -y build-essential gcc nasm qemu-system-x86 libssl-dev pkg-config git curl
```

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup component add rustfmt clippy
rustup target add x86_64-unknown-none aarch64-unknown-none
```

### Build
```bash
cargo check    # Quick compilation check
cargo test     # Run test suite
cargo build    # Debug build
cargo build --release  # Optimized build
```

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | x86_64 dual-core | Quad-core 3+ GHz |
| RAM | 4 GB | 16 GB |
| Storage | 20 GB SSD | 100+ GB NVMe |
| GPU | Basic | Vulkan-compatible |
| Firmware | UEFI | UEFI + Secure Boot |
| TPM | Optional | TPM 2.0 |

## QEMU Testing

```bash
qemu-system-x86_64 \
  -machine q35 -cpu host -enable-kvm \
  -m 4G -smp 4 \
  -drive file=sigmaos-dev.qcow2,format=qcow2 \
  -net user,model=virtio-net-pci \
  -display gtk,gl=on
```

## Post-Install

```bash
sigma-pkg upgrade          # Update packages
sigma-network setup        # Configure networking
sigma-ai setup             # Configure S-AI (optional)
sigma-security setup       # Harden system
sigma-doctor               # System health check
```

# Getting Started with SigmaOS

Welcome to the SigmaOS Sovereign Lattice. This guide will help you set up your development environment and run your first SigmaOS instance.

## ðŸ› ï¸ Prerequisites

Before you begin, ensure you have the following installed:

- **Build Tools**: `gcc-x86-64-linux-gnu`, `nasm`, `make`, `cmake`.
- **Emulator**: `qemu-system-x86`.
- **Node.js**: Version 16+ (for UI serving).

## ðŸš€ Step 1: Clone the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

## ðŸ—ï¸ Step 2: Build the Kernel

The SigmaOS build system is modular and shard-based.

```bash

# Build the entire sovereign lattice

make all
```

## ðŸ¦ž Step 3: Run in QEMU

We provide a specialized boot script that handles ISO creation and QEMU orchestration.

```bash

# Launch SigmaOS headlessly with serial output

./qemu-boot.sh
```

To view the kernel logs in real-time:

```bash
tail -f serial.log
```

## ðŸ–¥ï¸ Step 4: Access the Zenith UI

If you want to interact with the experimental web-based UI:

```bash
npm install
node server.js

# Open http://localhost:5000 in your browser

```

## âš ï¸ Common Pitfalls

- **Architecture Mismatch**: Ensure you are using the `x86_64` toolchain for kernel builds.
- **QEMU Permissions**: If QEMU fails to launch, check your user permissions for `/dev/kvm`.
- **Missing NASM**: The bootloader requires NASM for assembly compilation.

---

### For advanced configuration, see [Architecture.md](Architecture.md)

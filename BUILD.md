# 🏗️ Building and Running SigmaOS

This guide provides instructions for building the SigmaOS Sovereign Lattice and running it in an emulator.

## 📋 Prerequisites

To build and run SigmaOS, you need the following tools:

- **GCC / G++**: Cross-compiler for `x86_64-elf` (or local `g++` if building for simulation).

- **NASM**: Assembly compiler for low-level silicon.

- **Make**: Build orchestration tool.

- **QEMU**: Hardware emulator for kernel verification.

- **grub-mkrescue**: Required for generating bootable ISO images.

- **xorriso**: Dependency for `grub-mkrescue`.

## 🛠️ Build Instructions

### 1. Clean previous builds


```bash
make clean

```

### 2. Build the kernel singularity


```bash
make singularity

```

This will generate `sigmaos.bin` in the root directory.

### 3. Generate a bootable ISO


```bash
make zenith-iso

```

This requires an `iso_root` directory with the appropriate GRUB configuration.

## 🚀 Running in Emulation

To boot the kernel in QEMU and trace execution via serial output:


```bash
make qemu

```

### Serial Debugging

Kernel logs are piped to `stdio` (serial port 0). You can monitor the boot sequence and shard initialization directly in your terminal.

## 🔍 Static Analysis

We recommend running `cppcheck` before submitting any PRs:


```bash
cppcheck --enable=warning,style,performance -Iinclude kernel/

```

## Troubleshooting

If you encounter errors during the PQC attestation phase, ensure your hardware RNG is accessible and the GPG trust store is initialized.

---

### Σ SIGMAOS: Sovereign Build System. Absolute Integrity

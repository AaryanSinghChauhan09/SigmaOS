# Build Guide

Welcome to SigmaOS. This guide provides practical steps for building, running, and deploying the Sovereign Silicon Entity on physical hardware or a virtual machine.

---

## Prerequisites

To build SigmaOS, you will need a modern Linux environment (Ubuntu 22.04+ recommended) with the following tools installed:

```bash
sudo apt update
sudo apt install build-essential g++ make qemu-system-x86 nasm mtools
```

### Additional Requirements

- **NASM**: Assembly language compiler for x86_64
- **QEMU**: System emulator for testing
- **Make**: Build automation tool
- **GCC/G++**: C and C++ compilers
- **MTools**: MS-DOS disk image utilities

---

## Build System

SigmaOS uses the `s-cli` orchestrator for all build operations.

### Compile the Orchestrator

If you haven't already, compile the CLI:

```bash
g++ -std=c++20 orchestrator/main.cpp -o s-cli
```

### Select a Build Profile

Configure the system for your target environment:

```bash
./s-cli profile dev          # Development profile
./s-cli profile release      # Release profile
./s-cli profile microkernel # Microkernel profile
```

### Compile the Lattice

Build the kernel and all enabled shards for the x86_64 architecture:

```bash
./s-cli build x86_64
```

### Build Profiles

| Profile | Description | Use Case |
|---------|-------------|----------|
| `dev` | Debug symbols, no optimization | Development and debugging |
| `release` | Optimized, stripped binaries | Production deployment |
| `microkernel` | Minimal kernel, userland as shards | Embedded systems |
| `cloud` | Cloud-optimized with networking | Server deployments |

---

## Testing in QEMU

The fastest way to test SigmaOS is via QEMU:

```bash
qemu-system-x86_64 -cdrom build/sigmaos-x86_64.iso -m 2G -serial stdio
```

### QEMU Options

| Option | Description |
|--------|-------------|
| `-cdrom` | Boot from ISO image |
| `-m 2G` | Allocate 2GB RAM |
| `-serial stdio` | Redirect serial output to console |
| `-enable-kvm` | Enable KVM acceleration (Linux only) |
| `-smp 4` | Use 4 CPU cores |

### Debugging with GDB

```bash
# Terminal 1: Start QEMU with GDB server
qemu-system-x86_64 -cdrom build/sigmaos-x86_64.iso -m 2G -S -gdb tcp::1234

# Terminal 2: Connect GDB
gdb build/sigmaos.elf
(gdb) target remote :1234
(gdb) break main
(gdb) continue
```

---

## Running on Physical Hardware

To run SigmaOS on actual hardware:

### Create Bootable USB

1. Insert a USB flash drive (at least 4GB)

2. Write the ISO to the USB:
```bash
# Replace /dev/sdX with your actual USB device
sudo dd if=build/sigmaos-x86_64.iso of=/dev/sdX bs=4M status=progress
sync
```

3. Boot your machine from the USB drive. Ensure Legacy BIOS or UEFI compatibility mode is enabled in your firmware settings.

### Hardware Requirements

- **CPU**: x86_64 (64-bit) processor
- **RAM**: Minimum 2GB, recommended 4GB+
- **Storage**: 20GB minimum for installation
- **Graphics**: VGA-compatible or better

---

## Post-Boot Experience

Upon boot, you will be greeted by the Zenith UI dashboard. 

### Zenith UI Features

- **Desktop Environment**: Wayland-native compositor
- **Application Launcher**: Press `Super` key or click launcher icon
- **System Settings**: Access via settings icon
- **Terminal**: Right-click → Open Terminal

### SigmaShell Fallback

You can switch to the `SigmaShell` fallback terminal by pressing `Ctrl + Alt + F1`. This provides a command-line interface for system administration.

### Common SigmaShell Commands

```bash
sigma-driv list          # List loaded drivers
sigma-pkg search <name>  # Search packages
sigma-pkg install <name> # Install package
sigma-sys status         # System status
sigma-log view           # View system logs
```

---

## Troubleshooting

### Build Errors

| Error | Solution |
|-------|----------|
| `nasm: not found` | Install NASM: `sudo apt install nasm` |
| `g++: command not found` | Install build-essential: `sudo apt install build-essential` |
| `undefined reference` | Check that all dependencies are linked |
| `permission denied` | Run with appropriate permissions |

### Runtime Errors

| Error | Solution |
|-------|----------|
| Black screen in QEMU | Add `-vga std` flag to QEMU command |
| Boot hangs | Try reducing RAM allocation with `-m 1G` |
| No serial output | Ensure `-serial stdio` is specified |
| USB won't boot | Verify UEFI/Legacy BIOS settings |

---

## Advanced Build Options

### Cross-Compilation

Build for different architectures:

```bash
./s-cli build arm64    # ARM64 architecture
./s-cli build riscv64  # RISC-V architecture
```

### Custom Kernel Configuration

Edit `kernel/config/sigma_config.h` before building:

```c
#define MAX_CPUS 8
#define MAX_MEMORY 16384  // MB
#define ENABLE_AI_SHARDS 1
```

### Build with Specific Shards

```bash
./s-cli build --shards=gpu,network,ai x86_64
```

---

## Clean Build

To perform a clean build from scratch:

```bash
./s-cli clean
./s-cli build x86_64
```

---

## Continuous Integration

SigmaOS uses GitHub Actions for CI/CD. Build status is available in the repository's Actions tab.

### CI Build Matrix

- **Ubuntu 22.04**: Latest stable
- **Ubuntu 24.04**: Rolling release
- **Arch Linux**: Rolling edge
- **macOS**: Cross-compilation only

---

*See also: [Building-from-Source.md](Building-from-Source.md) · [Installation Guide](INSTALL.md) · [Contributing](CONTRIBUTING.md)*

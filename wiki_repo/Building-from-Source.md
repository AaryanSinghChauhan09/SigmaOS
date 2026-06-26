# Building from Source

This guide walks through building a bootable SigmaOS image from source on Ubuntu 22.04 (the only officially supported build host). The output is a bootable `.iso` you can run in QEMU or write to a USB drive.

---

## Prerequisites

### System requirements

- **OS**: Ubuntu 22.04 LTS (x86_64)
- **RAM**: 8 GB minimum, 16 GB recommended
- **Disk**: 20 GB free space (Buildroot downloads ~3 GB of packages)
- **CPU**: Any x86_64 with virtualization extensions (for QEMU testing)

### Install dependencies

```bash
sudo apt update
sudo apt install -y \
  build-essential nasm gcc g++ make \
  qemu-system-x86 \
  cmake ninja-build \
  git curl wget \
  nodejs npm \
  golang-go \
  xorriso mtools grub-pc-bin grub-efi-amd64-bin
```

---

## 1. Clone the repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Initialize submodules (Buildroot, Chromium patches, etc.)
git submodule update --init --recursive
```

---

## 2. Build the sovereign microkernel

The kernel is a freestanding x86_64 binary — no host libc, no stdlib headers.

```bash
# Clean any previous build artifacts
make clean

# Build the kernel binary
make kernel

# Expected output: build/vmlinuz-sigma
```

If you see linker errors about `glibc` symbols, check that `-nostdlib` and `-ffreestanding` are set in `CMakeLists.txt`. The kernel must never link against host libc.

---

## 3. Build the Go daemons

```bash
cd sigmad
go build -o ../build/daemons/sigmad-process   ./sigmad-process
go build -o ../build/daemons/sigmad-clipboard  ./sigmad-clipboard
go build -o ../build/daemons/sigmad-hotplug    ./sigmad-hotplug
go build -o ../build/daemons/sigmad-workspace  ./sigmad-workspace
cd ..
```

---

## 4. Build the web shell

```bash
cd web-shell
npm install
npm run build   # outputs to web-shell/dist/
cd ..
```

---

## 5. Build the Chrome extension

```bash
cd extension
npm install
npm run build   # outputs to extension/dist/
cd ..
```

---

## 6. Assemble the bootable ISO

```bash
make iso
# Output: build/sigmaos.iso
```

This step:
1. Packages the kernel into the ISO's `/boot/` directory.
2. Copies the compiled daemons into the root filesystem image.
3. Adds the web shell and extension as startup resources.
4. Writes a GRUB2 bootloader configuration pointing at `vmlinuz-sigma`.

---

## 7. Test in QEMU

```bash
# Basic boot (serial output to terminal)
qemu-system-x86_64 \
  -cdrom build/sigmaos.iso \
  -serial stdio \
  -m 2G \
  -enable-kvm

# With networking (user-mode NAT)
qemu-system-x86_64 \
  -cdrom build/sigmaos.iso \
  -serial stdio \
  -m 2G \
  -enable-kvm \
  -netdev user,id=net0 \
  -device virtio-net-pci,netdev=net0
```

Watch the serial output for the boot sequence:

```
[sigma-init] Starting sovereign boot sequence...
[sigma-init] Loaded 12 services
[sigma-idt] IDT initialized: 32 exception vectors registered
[sigma-mm] Physical memory map: 2048 MB available
[sigma-vfs] VFS mounted at /
[sigma-net] TCP/IP stack online: lo 127.0.0.1
[sigma-init] Launching Chromium...
```

---

## 8. Run the test suite

```bash
npm run test
```

All tests in `/tests` must be green before submitting patches. The CI pipeline runs this automatically on every pull request.

---

## Build Profiles

The build system accepts a `SIGMA_PROFILE` flag to select a target configuration:

```bash
# Default: full x86_64 desktop
cmake -B build -G Ninja -DSIGMA_PROFILE=standalone

# Minimal IoT build (ARM64, no Chromium, no GUI)
cmake -B build -G Ninja -DSIGMA_PROFILE=iot-arm64

# QEMU development build (debug symbols, verbose logging)
cmake -B build -G Ninja -DSIGMA_PROFILE=qemu-dev
```

---

## Writing to a USB drive

Once you have `build/sigmaos.iso`, you can write it to a USB drive for bare-metal testing:

```bash
# Find your USB device (be careful — this will erase the drive)
lsblk

# Write the ISO (replace /dev/sdX with your device)
sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress && sync
```

Boot the target machine from the USB drive. SigmaOS should reach the Chromium shell in under 3 seconds on most modern hardware.

---

## Troubleshooting

**Build fails with "file not found" on kernel/core/*.cpp**
The core kernel source files (scheduler, memory manager, syscall table) must exist. Check that all submodules are initialized: `git submodule update --init --recursive`.

**Kernel binary contains glibc symbols**
Run `nm build/vmlinuz-sigma | grep GLIBC` — if anything appears, a kernel source file is `#include`-ing a hosted stdlib header (`<stdio.h>`, `<string.h>`, etc.). Replace with equivalents from `klib/`.

**QEMU triple-faults immediately**
This usually means the IDT is not initialized before interrupts are enabled. Check that `sigma_idt_init()` is called early in `kmain.cpp`.

**PID 1 exits and kernel panics**
`sigma_init.cpp` must have an infinite loop after starting services. Check that the `for (int loop = 0; loop < 5; loop++)` bug has been fixed to `for (;;)`.

---

*See also: [Architecture Overview](Architecture-Overview) · [Contributing](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md)*

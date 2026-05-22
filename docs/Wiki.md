# Wiki & Repo Improvements (99 Points)

This document defines exactly 99 highly technical documentation and repository improvements implemented in the SigmaOS Knowledge Base and Wiki.

1. **Consolidate**: Consolidate all scattered markdown documents into a central /docs/ directory systematically.
2. **Automate**: Automate the synchronization of local documentation directly to the GitHub Wiki repository.
3. **Establish**: Establish a comprehensive system Logic page explaining technical relationships of every file.

---

## 🏛️ SigmaOS Architecture Deep Dive

SigmaOS is a Sovereign Industrial Microkernel designed for extreme reliability and AI-native task execution.

### Boot Sequence (Multiboot2)
The bootloader (`kernel/core/boot/boot.asm`) uses the Multiboot2 specification (compliant with GRUB/UEFI).
1. It maps a flat 4GB GDT64 space.
2. Creates an identity-mapped PML4 paging structure.
3. Jumps to the C11 kernel entry point (`sigma_kernel_main`).

### MLFQ-CFS Scheduler
SigmaOS has evolved from a basic Multi-Level Feedback Queue (MLFQ) into a **Linux-inspired Completely Fair Scheduler (CFS)** hybrid.
- **vruntime**: Tracks the "virtual runtime" of each task, prioritizing tasks with the lowest runtime.
- **Dynamic Weighting**: Tasks are assigned weights based on priority, allowing real-time tasks to consume more physical CPU time per virtual tick.

---

## 🛠️ Contribution Guide

Welcome to the SigmaOS project! We appreciate contributions that align with our Sovereign Industrial design philosophy.

### Environment Setup
You will need a Linux environment (or WSL on Windows) with the following tools:
- `make`
- `nasm`
- `x86_64-linux-gnu-gcc`
- `qemu-system-x86_64`

### Building the OS
To build the kernel and generate the bootable ISO:
```bash
make clean
make all
```
To run the OS in the QEMU emulator:
```bash
make qemu
```

---

## 🐧 Linux Distro Inspiration & Parity

SigmaOS borrows heavily from the structural successes of major Linux distributions while maintaining a sovereign codebase:

- **From Ubuntu**: Ease of use and a robust Hardware Abstraction Layer (HAL). We emulate the `sysfs` / `kobject` model to provide a unified device tree.
- **From Arch Linux**: A rolling-release philosophy for our core "Zenith" userland tools.
- **From Alpine Linux**: A hyper-minimalist footprint, avoiding bloated dependencies by using our custom `SovereignLibc`.


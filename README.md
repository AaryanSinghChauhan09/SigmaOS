# SigmaOS Zenith (v15.2 - Release Microkernel)

The Sovereign Industrial Microkernel.

This branch represents the core modular microkernel layout of SigmaOS, structured to align with established Linux distribution layouts for robustness, isolation, and silicon-direct execution.

---

## 🏛️ Design Specification & Architecture Layers

SigmaOS is organized into isolated functional layers to guarantee complete safety and hardware-isolation boundary conditions:

### 1. Kernel Layer (`/kernel/`)
- **Process Scheduler**: Multi-level Feedback Queue (MLFQ) and Round-Robin scheduler handling task priorities and time-slice yields.
- **Memory Management**: Physical Page Frame Allocator (PMM) and Virtual Memory Paging (VMM) supporting 4-level paging tables.
- **Hardware Drivers**: Low-level abstractions for COM1 serial logs, PS/2 keyboards, standard VGA text mode, and ATA disk sector operations.

### 2. Standard Libraries (`/lib/`)
- **Sovereign Libc**: Independent, zero-dependency C11 standard library implementation providing `sigma_printf`, memory manipulators (`memcpy`, `memset`), string utilities, and attestation helpers (`crc32`).

### 3. Init System (`/init/`)
- **PID 1 Bootstrap**: Orchestrates clean startup sequences using Runlevels (1 to 5) to boot vital telemetry, load the virtual file system, initialize the TCP/IP stack, and spawn the user shell in order.

### 4. Virtual File System (`/fs/`)
- **VFS Interface**: Standardizes operations like `open`, `close`, `read`, and `write` via file descriptor tables and inode indexing.
- **Ext4/FAT32 Drivers**: Handles block storage, reads superblock states, and walks clusters.

### 5. Networking Stack (`/net/`)
- **Loopback NIC**: Direct virtual hardware interface loopback (`lo` at `127.0.0.1`).
- **TCP/IP Suite**: Custom TCP 3-way handshake state machine and UDP port binding.
- **DNS Lookup**: Local resolver mapping domain endpoints to IPv4 destinations.

### 6. Userland utilities (`/usr/`)
- **sh Shell**: Interactive CLI command execution environment mapping user inputs to system calls.

---

## 🛠️ Build, Test, & Execution Instructions

### Dependencies
- Make, NASM assembler, GCC, QEMU

### 1. Compile all Modular Subsystems
```bash
make clean
make all
```

### 2. Running the Emulator
```bash
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G
```

### 3. Running Unit Tests
```bash
npm run test
```
All unit tests in `/tests` must return green states before submitting patches.

# SigmaOS - Sovereign, AI-Native Operating System

> **Build your own OS from scratch, inspired by Linux From Scratch methodology**

SigmaOS is a next-generation operating system that combines the educational philosophy of Linux From Scratch with modern AI-native capabilities, zero-dependency architecture, and post-quantum security.

## 🎯 Key Features

- **Zero-Dependency Architecture**: No external libraries, complete sovereignty over every byte
- **AI-Native Design**: Built-in AI agents, ML infrastructure, and intelligent automation
- **Post-Quantum Security**: Kyber-1024 KEM, Dilithium-5 signatures, capability-based security
- **LFS-Inspired Build System**: Step-by-step construction with full transparency
- **Educational Platform**: Learn OS internals by building each component from source
- **Microkernel Design**: Minimal Trusted Computing Base (TCB) for maximum security

## 📚 Documentation Structure

### LFS-Style Build Guide
- [Chapter 1: Introduction](docs/LFS-STYLE-GUIDE/Chapter-01-Introduction.md) - Overview and philosophy
- [Chapter 2: Preparing Host](docs/LFS-STYLE-GUIDE/Chapter-02-Preparing-Host.md) - Build environment setup
- [Chapter 3: Kernel Build](docs/LFS-STYLE-GUIDE/Chapter-03-Kernel-Build.md) - Building the kernel
- [Chapter 4: klib Construction](docs/LFS-STYLE-GUIDE/Chapter-04-Klib-Construction.md) - Building the sovereign standard library
- [Chapter 5: Driver Integration](docs/LFS-STYLE-GUIDE/Chapter-05-Driver-Integration.md) - Hardware drivers
- [Chapter 6: Userspace Tools](docs/LFS-STYLE-GUIDE/Chapter-06-Userspace-Tools.md) - User-space applications
- [Chapter 7: Boot Process](docs/LFS-STYLE-GUIDE/Chapter-07-Boot-Process.md) - System booting
- [Chapter 8: Security Hardening](docs/LFS-STYLE-GUIDE/Chapter-08-Security-Hardening.md) - Security measures
- [Chapter 9: System Verification](docs/LFS-STYLE-GUIDE/Chapter-09-System-Verification.md) - Testing and validation

### Architecture Documentation
- [Architecture Overview](ARCHITECTURE.md) - System architecture and design decisions
- [Zero-Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE.md) - How we eliminate external dependencies
- [Security Architecture](Security-Architecture.md) - Security model and implementation
- [LFS-Inspired Improvements](LFS_INSPIRED_IMPROVEMENTS.md) - Linux From Scratch methodology applied

### Developer Documentation
- [API Reference](API_REFERENCE.md) - Complete API documentation
- [Testing Guide](Testing-Guide.md) - How to test SigmaOS
- [Contribution Guide](CONTRIBUTING.md) - How to contribute
- [Security Policy](SECURITY.md) - Security reporting and best practices

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with `nightly` toolchain
- QEMU or other x86_64 emulator
- Git
- Build essentials (make, gcc, etc.)

### Build Process
```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Set up build environment
./scripts/00-build-environment.sh

# Build kernel (Step-by-step LFS style)
./scripts/01-kernel-build.sh
./scripts/02-klib-build.sh
./scripts/03-drivers-build.sh
./scripts/04-userspace-build.sh

# Verify the build
./scripts/05-verification.sh

# Run in QEMU
./scripts/run-qemu.sh
```

## 🏗️ Build Profiles

SigmaOS supports different build profiles for different use cases:

- **minimal**: Core kernel + basic drivers (~50MB)
- **standard**: Full kernel + drivers + basic userspace (~200MB)
- **full**: Complete system with AI stack (~500MB)

```bash
# Build with specific profile
cargo build --profile minimal
cargo build --profile standard
cargo build --profile full
```

## 🔒 Security Features

- **Post-Quantum Cryptography**: Kyber-1024 KEM + Dilithium-5 signatures
- **Capability-Based Security**: 64-bit hardware-enforced permissions
- **Zero-Trust Architecture**: Continuous authentication and verification
- **Memory Safety**: Rust memory safety, W^X enforcement, ASLR
- **Minimal Attack Surface**: Microkernel design with minimal TCB
- **Audit-Everything**: Complete build transparency and verification

## 🎓 Educational Value

SigmaOS is designed as a learning platform:

- **Learning Objectives**: Each chapter has clear educational goals
- **Under the Hood**: Deep technical explanations of design decisions
- **Experiments**: Hands-on modifications to understand components
- **Debugging Guides**: Common issues and investigation techniques
- **Progressive Complexity**: Start simple, add complexity gradually

## 📦 Project Structure

```
SigmaOS/
├── kernel/              # Core kernel implementation
├── src/                 # Main source code
│   ├── klib/           # Sovereign standard library
│   ├── security/       # Security subsystems
│   ├── filesystem/     # Filesystem implementations
│   ├── network/       # Network stack
│   └── ai/            # AI and ML components
├── userland/           # User-space applications
├── scripts/            # Build and installation scripts
├── docs/               # LFS-style documentation
├── tests/              # Test suites
└── tools/              # Development tools
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🏛️ Design Specification & Architecture Layers (Zenith Release Microkernel)

This section represents the core modular microkernel layout of SigmaOS, structured to align with established Linux distribution layouts for robustness, isolation, and silicon-direct execution.

SigmaOS is organized into isolated functional layers to guarantee complete safety and safety-critical isolation boundary conditions:

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


---

## 📄 License

SigmaOS is dual-licensed under MIT OR GPL-2.0

## 🔗 Links

- [GitHub Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- [Security Policy](SECURITY.md)
- [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

## 🙏 Acknowledgments

- **Linux From Scratch**: For the inspiring methodology of building from source
- **Rust Community**: For providing a safe systems programming language
- **Open Source Community**: For countless tools and libraries we've learned from

---

**SigmaOS**: Your Distro, Your Rules - Build it, Understand it, Own it.
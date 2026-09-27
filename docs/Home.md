## Star History

<a href="https://www.star-history.com/?repos=aaryansinghchauhan09%2Fsigmaos&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=aaryansinghchauhan09/sigmaos&type=date&legend=top-left" />
 </picture>
</a>

# 🚀 SigmaOS — Sovereign AI-Native Operating System

<p align="center">
  <strong>A secure, fast, Rust-native operating system built from the ground up</strong><br>
  <em>Inspired by Linux, Omarchy, and the OSDev community</em>
</p>

<p align="center">
  <a href="#architecture">Architecture</a> •
  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#kernel-subsystems">Kernel Subsystems</a> •
  <a href="#roadmap">Roadmap</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## 📖 Overview

SigmaOS is a Rust-based operating system designed to be a bootable, user-focused desktop distribution with:

- **Safe Rust kernel** — Memory safety without garbage collection
- **AI-native runtime** — First-class agent integration and local LLM inference
- **Zenith compositor** — Wayland-inspired tiling desktop with keyboard-driven workflow
- **Universal package engine** — `sigpkg` supporting 60+ package format bridges
- **Atomic updates** — Dual-root A/B images with CoW rollback

```
Firmware → Bootloader → Kernel Init → Memory Manager → Scheduler →
  VFS Mount → Init System → Zenith Desktop → User Shell
```

---

## 🏗️ Architecture

```
SigmaOS/
├── src/
│   ├── kernel/             # Core kernel subsystems
│   │   ├── main.rs         # Kernel entry point (start_kernel bootstrap)
│   │   ├── mod.rs          # Kernel module registry
│   │   ├── memory/         # Physical memory allocators (Buddy, Slab, DMA)
│   │   ├── mm/             # Virtual memory, paging, page tables
│   │   ├── sched/          # Schedulers (Round-Robin, CFS, EEVDF, BORE)
│   │   ├── proc/           # Process management, task control blocks
│   │   ├── syscall/        # System call dispatch table and handlers
│   │   ├── vfs/            # Virtual Filesystem layer
│   │   ├── fs/             # Concrete filesystems (RamFS, FAT, ext2)
│   │   ├── net/            # Network stack
│   │   ├── irq/            # Interrupt request handling
│   │   ├── drivers/        # Kernel-space device drivers
│   │   ├── security/       # Security modules (Landlock, Capsicum, Seccomp)
│   │   ├── core/           # Core kernel primitives
│   │   ├── panic_handler.rs # Kernel panic with register dump & stack trace
│   │   └── kprintf.rs      # Formatted kernel output (printk-style)
│   ├── drivers/            # Hardware drivers
│   │   ├── serial.rs       # UART 16550 serial port (COM1-COM4)
│   │   ├── rtc.rs          # CMOS Real-Time Clock
│   │   ├── gpu/            # GPU/Display drivers
│   │   └── ...             # Block, network, input drivers
│   ├── boot/               # Bootloader components
│   ├── shell/              # Interactive kernel shell (sigma-sh)
│   ├── userland/           # User-space programs
│   │   ├── coreutils/      # Core Unix utilities (ls, cat, grep, etc.)
│   │   ├── libc/           # Minimal C library shims
│   │   └── pkg/            # Package management
│   ├── loader/             # Binary loaders
│   │   └── elf/            # ELF64 binary parser and loader
│   ├── fs/                 # Filesystem implementations
│   ├── vfs/                # VFS abstractions
│   ├── ai/                 # AI/ML runtime and agent framework
│   ├── security/           # Security framework
│   ├── klib/               # Kernel library (no_std compatible primitives)
│   └── ...                 # Additional subsystems
├── tools/                  # Build tools and utilities
├── tests/                  # Test suites (Rust + Python)
├── docs/                   # Documentation
├── Cargo.toml              # Rust build configuration
├── Makefile                # Build automation
└── ARCHITECTURE.md         # Detailed architecture guide
```

---

## ✨ Features

### Kernel Subsystems
| Subsystem | Status | Description |
|-----------|--------|-------------|
| **Memory Management** | ✅ Working | Buddy allocator, Slab cache, DMA ring buffer, guard pages, NUMA-aware |
| **Process Scheduler** | ✅ Working | Round-Robin, Priority-based, CFS, EEVDF, BORE schedulers |
| **Virtual Filesystem** | ✅ Working | Linux-style VFS with inode/dentry/superblock abstractions, RamFS |
| **System Calls** | ✅ Working | x86_64 syscall table with 50+ POSIX-compatible calls |
| **Interrupt Handling** | ✅ Working | IDT, ISR/IRQ dispatch, PIC remapping, APIC support |
| **Paging & VMM** | ✅ Working | 4-level page tables, identity mapping, virtual memory manager |
| **Device Drivers** | ✅ Working | Serial (UART 16550), RTC, VGA, keyboard, block devices |
| **Kernel Panic** | ✅ Working | Register dump, stack trace, panic log ring buffer |
| **ELF Loader** | ✅ Working | ELF64 parser with segment loading and symbol resolution |
| **Kernel Printf** | ✅ Working | printk-style output with log levels and hex dump |

### Hardware Drivers
| Driver | Description |
|--------|-------------|
| **Serial Port** | UART 16550 with configurable baud rate, FIFO, interrupt support |
| **RTC Clock** | CMOS real-time clock with BCD conversion, NMI-safe access |
| **VGA Text** | 80×25 text mode with cursor control, scrolling, color attributes |
| **PS/2 Keyboard** | Scancode translation, modifier keys, callback-based input |
| **ATA/IDE** | PIO mode disk access for storage devices |
| **NVMe** | PCIe NVMe storage controller |
| **GPU/DRM** | Framebuffer, DRM atomic commit, VirtIO-GPU |

### User Space
| Component | Description |
|-----------|-------------|
| **sigma-sh** | Interactive shell with piping, redirection, job control, scripting |
| **Coreutils** | 40+ Unix utilities (ls, cat, grep, sort, find, diff, hexdump...) |
| **sigpkg** | Universal package manager supporting .sigpkg and 60+ format bridges |
| **Zenith Desktop** | Wayland-inspired tiling compositor with keyboard-driven workflow |

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies (Ubuntu/Debian)
sudo apt install build-essential qemu-system-x86 nasm
```

### Build & Test

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Verify the library compiles
cargo check --lib

# Run the test suite
make test

# Build a preview ISO
make iso

# Launch in QEMU
make run
```

### Development Commands

```bash
make check    # Static analysis via cargo check
make test     # Execute native test suites
make format   # Verify code formatting
make iso      # Assemble bootable ISO image
make run      # Run QEMU virtual machine preview
make clean    # Remove build artifacts
make help     # Show all available targets
```

---

## 🔧 Kernel Subsystems

### Boot Process
```
BIOS/UEFI → Bootloader → early_cpu_init (GDT, IDT) →
  early_memory_init (Paging, Buddy Allocator) →
  Scheduler Init → Service Manager → Enable Interrupts
```

### Memory Architecture
- **Physical**: Buddy allocator with configurable region sizes
- **Virtual**: 4-level page tables (PML4) with identity and higher-half mapping
- **Heap**: Slab allocator for fixed-size kernel objects, kmalloc/kfree
- **DMA**: Ring buffer allocator for device I/O
- **Guard Pages**: Hardened allocator with red zones for overflow detection

### Scheduler Architecture
- **Round-Robin**: Default scheduler with configurable time quantum
- **Priority**: Multi-level priority queues (0-139, Linux-compatible)
- **CFS**: Completely Fair Scheduler with virtual runtime tracking
- **EEVDF**: Earliest Eligible Virtual Deadline First (Linux 6.6+)
- **BORE**: Burst-Oriented Response Enhancer for interactive workloads

### Filesystem Stack
```
User Application
       ↓
  System Calls (open, read, write, close)
       ↓
  VFS Layer (inode, dentry, superblock, file_operations)
       ↓
  ┌──────────┬──────────┬──────────┐
  │  RamFS   │  FAT16   │  DevFS   │
  │ (tmpfs)  │ (disk)   │ (/dev)   │
  └──────────┴──────────┴──────────┘
       ↓
  Block Device Layer (ATA PIO, NVMe, VirtIO)
```

---

## 📋 Roadmap

### ✅ Completed
- [x] Kernel bootstrap (GDT, IDT, memory init)
- [x] Memory management (Buddy, Slab, Paging)
- [x] Process management & scheduling
- [x] VFS with RamFS
- [x] System call dispatch table
- [x] ELF64 binary loader
- [x] Serial port driver (UART 16550)
- [x] RTC driver (CMOS clock)
- [x] Kernel panic handler with register dump
- [x] Formatted kernel output (kprintf)
- [x] Interactive shell with 40+ commands
- [x] Coreutils (ls, cat, grep, sort, find, etc.)
- [x] Security framework (Landlock, Capsicum, Seccomp)
- [x] AI/Agent runtime integration

### 🔄 In Progress
- [ ] User mode (Ring 3) with TSS
- [ ] Bare-metal x86_64 target (no_std boot)
- [ ] Network TCP/IP stack
- [ ] GUI desktop (Zenith compositor)
- [ ] ISO installer

### 📌 Planned
- [ ] SMP multi-core support
- [ ] USB (xHCI) driver
- [ ] Sound (HDA) driver
- [ ] ACPI power management
- [ ] Self-hosting compiler toolchain

---

## 🧪 Testing

```bash
# Run all Rust tests
cargo test

# Run standalone test suite
make test

# Run Python integration tests
pytest tests/

# Run with verbose output
cargo test -- --nocapture
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Language**: All kernel and system code must be written in **safe Rust** (unsafe only when strictly necessary for hardware access)
2. **Style**: Run `cargo fmt` before committing
3. **Tests**: Add tests for new functionality
4. **Documentation**: Include doc comments for all public APIs
5. **No external runtimes**: No Python, Node.js, Java, or Go dependencies in the kernel

### Development Workflow
```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/SigmaOS.git

# Create a feature branch
git checkout -b feature/your-feature

# Make changes and verify
cargo check --lib
cargo test

# Commit and push
git add .
git commit -m "feat: add your feature description"
git push origin feature/your-feature

# Open a Pull Request
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Detailed system architecture guide |
| [docs/PRODUCT_VISION.md](docs/PRODUCT_VISION.md) | Product vision and manifesto |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Master execution roadmap |
| [docs/SUPPORT_MATRIX.md](docs/SUPPORT_MATRIX.md) | Hardware support matrix |
| [WHAT_IS_WORKING_AND_NOT_WORKING.md](WHAT_IS_WORKING_AND_NOT_WORKING.md) | Component status tracker |
| [SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V36.md](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V36.md) | Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia V36 |

---

## 🏆 Inspiration & References

SigmaOS draws inspiration from these outstanding projects:

| Project | Inspiration |
|---------|-------------|
| [Linux Kernel](https://github.com/torvalds/linux) | Kernel architecture, VFS, scheduler design, driver model |
| [Omarchy](https://github.com/omacom/omarchy) | Desktop experience, modular installer, theme system |
| [os-tutorial](https://github.com/cfenollosa/os-tutorial) | Bootloader patterns, interrupt handling, educational approach |
| [Linux Mint](https://github.com/linuxmint) | User experience, polish, documentation standards |
| [Redox OS](https://www.redox-os.org/) | Rust-based microkernel design patterns |
| [xv6](https://github.com/mit-pdos/xv6-riscv) | Clean educational OS design |

---

## 📄 License

SigmaOS is dual-licensed under [MIT](LICENSE) or [GPL-2.0](LICENSE-GPL).

---

<p align="center">
  <strong>Built with 🦀 Rust | Designed for the Future</strong>
</p>

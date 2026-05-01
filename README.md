# 🏛️ SigmaOS: A Modular, Experimental Operating System

SigmaOS is an experimental, bare-metal operating system kernel built to explore extreme modularity using C++ singleton patterns. While currently in a conceptual phase (v29.0), its goal is to provide a clean, zero-dependency alternative to legacy monolithic kernels.

## 🚀 Current Status (What SigmaOS Does Today)

SigmaOS is **not** a daily-driver operating system. Currently, the kernel can:

- **Boot reliably in QEMU:** Using a Multiboot2 compliant binary and GRUB.
- **Initialize Hardware:** Basic probing of the CPU and establishing serial output (COM1) for debugging.
- **Allocate Memory:** A simple bare-metal bump allocator (QBMP) with basic guard checks.
- **Execute Minimal Userland:** A barebones interactive shell (`sigma_sh`) is in development to provide basic I/O.
- **Demonstrate Architecture:** The entire kernel is divided into isolated C++ singletons ("Shards") that communicate via strict C-linkage interfaces.

SigmaOS currently lacks a fully functional filesystem, robust device drivers (e.g., USB, GPU), and a mature networking stack, though stubs exist.

## 🛠️ Building & Running

### Dependencies

- `make`
- `nasm`
- `g++` (multilib / cross-compiler)
- `qemu-system-x86_64`
- `grub-mkrescue` and `xorriso` (for ISO generation)

### 1. Build the Kernel

```bash
make clean
make singularity
```

_This generates `sigmaos.bin`, the core Multiboot2 executable._

### 2. Generate a Bootable ISO

```bash
make zenith-iso
```

_Creates a GRUB-bootable ISO image for testing on hardware or VMs._

### 3. Run in Emulator

```bash
make qemu
```

_Boots the kernel in QEMU and pipes the internal kernel logs directly to your terminal._

## 📚 Glossary: Translating the Vision

SigmaOS uses unique terminology for its architectural concepts. Here is what they mean in standard OS engineering terms:

| SigmaOS Term                     | Standard Technical Meaning                                                                       |
| :------------------------------- | :----------------------------------------------------------------------------------------------- |
| **Sovereign Lattice**            | The operating system architecture as a whole.                                                    |
| **Shard**                        | A distinct subsystem or driver encapsulated as a C++ Singleton class.                            |
| **Amnesic Memory**               | Stateless RAM allocation; memory buffers that are eagerly zeroed out after use to prevent leaks. |
| **Zenith**                       | The target milestone version denoting a stable, complete foundation.                             |
| **ZCLN (Zero-Copy Lattice Net)** | A zero-copy networking stack (bypassing redundant buffer copies between kernel and userland).    |

## 🤝 Contributing

We welcome contributions to help evolve SigmaOS from an experimental kernel into a fully usable distribution.

- Please read [CONTRIBUTING.md](CONTRIBUTING.md) for our PR process and coding standards.
- Check out the **Good First Issues** label on GitHub if you want to implement missing features (like expanding the FAT32 driver or adding shell commands).
- For a detailed look at our immediate goals, refer to [ROADMAP.md](ROADMAP.md).

---

_Σ SIGMAOS: Building a modular foundation from the silicon up._

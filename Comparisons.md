# Comparative Analysis: SigmaOS vs. The World

SigmaOS draws inspiration from many legendary open-source projects, but maintains its own unique identity through the **33-Suite Sovereign Lattice**.

| Feature | NixOS | Qubes OS | Plan 9 | TempleOS | **SigmaOS** |
|---------|-------|----------|--------|----------|-------------|
| **Core** | Linux/Nix | Xen/Linux | 9P Protocol | Ring 0 HolyC | **Sovereign Lattice** |
| **Config** | Declarative | Traditional | Distributed | None | **Declarative Lattice** |
| **Security**| Immutability | **Isolation** | Permissions | None | **Isolated Domains** |
| **USP** | Determinism | Security | Connectivity | Divine JIT | **Absolute Sovereignty** |

## Key Differences

### 1. vs. NixOS
SigmaOS adopts NixOS's **Declarative Configuration** via `sigma_lattice.json`, allowing for atomic updates across all 33 suites. However, SigmaOS applies this to a bare-metal lattice rather than a standard Linux distribution.

### 2. vs. Qubes OS
While Qubes uses Xen for virtualization, SigmaOS implements **Isolated Shard Domains** directly within the Sovereign Orchestrator, using hardware-level protection (VT-d) to compartmentalize critical suites.

### 3. vs. Plan 9
SigmaOS absorbs the "Everything is a file" philosophy into **Everything is a Shard Stream** (S-9P), providing a unified protocol to access HAL, Memory, and Storage resources across the lattice.

### 4. vs. TempleOS
Inspired by the "Ring 0 only" simplicity of TempleOS, SigmaOS includes a **JIT Silicon Engine** for instant hot-fixing of kernel logic, while maintaining modern security protections that TempleOS deliberately omitted.

### 5. Language Diversity (Go-dav OS Inspired)
SigmaOS now supports a **Sovereign Go Bridge** (`include/sigma/go`), allowing developers to build garbage-collected system services without sacrificing the performance of the Pure Silicon core.

### 6. Storage Reliability (DreamOS64 Inspired)
To ensure robust data persistence, SigmaOS incorporates a **Reference FAT32 Driver** (`drivers/reference/fat_fs.c`), providing a simple and universally compatible storage foundation for industrial applications.

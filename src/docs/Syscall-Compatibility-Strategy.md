# SigmaOS: Linux/BSD Syscall Compatibility Strategy Specification

This document details the architectural strategy for achieving binary-level compatibility with existing Linux and BSD userland programs on the native SigmaOS microkernel.

---

## 🏎️ Comparative Evaluation of Compatibility Models

To run legacy binaries, SigmaOS evaluates three primary compatibility strategies:

```
                            +--------------------------+
                            |   COMPATIBILITY LAYERS   |
                            +--------------------------+
                                    /     |      \
                                   /      |       \
                                  v       v        v
                  +-------------------+ +---+ +-------------------+
                  | Syscall Trans.    | | U | | Full-Virtualized  |
                  | Layer (L-Trans)   | | S | | Micro-VM (KVM/VMM)|
                  +-------------------+ | h | +-------------------+
                  | - Native Perf     | | i | | - Isolated Core   |
                  | - Direct Mapping  | | m | | - Emulated Kernel |
                  +-------------------+ +---+ +-------------------+
```

---

## 📊 Trade-Off Matrix

| Strategy | Performance | Isolation/Security | Portability | Development Effort | Recommended Use Case |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Syscall Translation Layer (L-Trans)** | **Near-Native (98%+)** | Moderate (direct kernel exposure) | Medium (requires ISA alignment) | High (requires translating 300+ Linux syscalls) | Core performance-critical services (database engines, system utilities). |
| **Userland Shim (LD_PRELOAD equivalent)** | Moderate (90-95%) | High (enforced in sandbox namespace) | High | Medium (translates standard POSIX C APIs) | Standard desktop applications, CLI developer tools. |
| **Full-Virtualized Micro-VM (KVM/VMM)** | Low-to-Moderate (80-85%) | **Absolute (strict hypervisor boundary)** | **Universal** | Low (boots standard unpatched kernels) | Heavy untrusted workloads, legacy enterprise software. |

---

## 🚀 Recommended Hybrid Strategy

SigmaOS implements a **three-tier hybrid compatibility strategy** to leverage the strengths of each model:

### Tier 1: Low-Level Performance-Critical (L-Trans)
* **Design:** Direct in-kernel system call translation. The microkernel intercepts standard Linux syscall trap vectors (e.g., syscall 12 `sys_brk`, syscall 9 `sys_mmap`) and maps them to physical capability page allocations.
* **Scope:** Command-line interpreters, compilers, and database runtimes.

### Tier 2: Userland Capability Shim (Sovereign Shim)
* **Design:** A customized, lightweight standard library (libc/libstd equivalent) linked at load-time that intercepts system calls and wraps them in secure `CapabilityToken` verification patterns before executing native microkernel IPCs.
* **Scope:** Desktop programs and GUI apps.

### Tier 3: Hardened Enterprise Isolation (Micro-VM)
* **Design:** Runs untrusted, legacy enterprise servers in hypervisor-supervised lightweight virtual environments (using the integrated `SovereignVmm`), isolating execution entirely from the base OS kernel.
* **Scope:** Proprietary closed-source binaries and multi-tenant cloud workloads.

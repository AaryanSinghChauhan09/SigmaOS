# AI Agent Management: Legacy Architectures & Dropped Hardware Maintenance

This document provides explicit guidelines for AI agents developing, maintaining, and emulating legacy hardware architectures and hardware targets whose support was dropped by upstream Linux (e.g., Linux 6.x dropping 32-bit x86 i386/i486, Itanium IA-64, MIPS32, Alpha, PA-RISC) and BSD distributions (e.g., FreeBSD/NetBSD dropping vax, sun3, pc98, SPARC32).

---

## 1. Scope & Objective

SigmaOS maintains a zero-dependency `#![no_std]` architecture capable of emulating, cross-compiling, or interfacing with legacy hardware architectures that upstream Linux and BSD kernels no longer actively support.

AI agents MUST adhere to these directives when maintaining `src/arch/portability.rs` and `src/kernel/architecture.rs`.

---

## 2. Legacy CPU Architectures & Emulation Targets

| Architecture Class | Upstream Distro Status | SigmaOS HAL Mapping | Directives & Guidelines |
| :--- | :--- | :--- | :--- |
| **i386 / i486 / i686** | Dropped by RHEL, Fedora, Arch, Debian 13+ | `X86_32` HAL Context | Support 32-bit segmentation, FPU x87 emulation, and 4MB page tables. |
| **Itanium (IA-64)** | Dropped by Linux 6.7+ | `Ia64EmulationContext` | Emulate 128-bit register bundles, explicit instruction-level parallelism (EPIC), and RSE (Register Stack Engine). |
| **DEC Alpha** | Dropped by Debian & Fedora | `AlphaContext` | Emulate PALcode traps, 64-bit atomic memory barriers, and IEEE 754 floating point traps. |
| **MIPS32 / MIPS64** | Deprecated in Linux / FreeBSD | `MipsContext` | Maintain delay-slot instruction pipeline invariants, TLB page refill traps, and big/little endian switching. |
| **SPARC32 / SPARC64** | Dropped by Oracle Linux / OpenBSD | `SparcContext` | Support register window sliding (`save`/`restore` traps), OpenBoot PROM boot entry generation, and VIS vector instructions. |
| **Motorola 68000 (m68k)** | Maintained only in Gentoo/Debian ports | `M68kContext` | Support 24-bit/32-bit linear address translation, Mac/Amiga ROM BIOS calls, and coldfire variant traps. |
| **HP PA-RISC (hppa)** | Dropped by RHEL & FreeBSD | `PaRiscContext` | Support space registers, non-executable stack execution guards, and HP-UX ABI translation. |

---

## 3. AI Agent Development Checklist

When implementing or updating legacy architecture ports in `src/arch/`:

1. **Zero External Dependencies:** Ensure zero reliance on `libc`, `libgcc`, or compiler-rt helpers. Use `#![no_std]` Rust primitives.
2. **Context Switch Trap Verification:** Verify trap vector table mappings and register context descriptors (`X86Context`, `Ia64Context`, etc.).
3. **Multi-Arch Binary Tests:** Include standalone test blocks executable via `rustc --edition=2021 --test`.

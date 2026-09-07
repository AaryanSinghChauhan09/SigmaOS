# AI Agent 32-bit Operation Management Guidelines in SigmaOS

## Overview

SigmaOS supports legacy and embedded 32-bit architecture execution modes (x86/i686, ARM32 armv7-a, and RISC-V Sv32) alongside its primary 64-bit microkernel. This document defines operational guidelines, hardware thunking rules, and memory boundary protocols for autonomous AI Agents (Sentinel 🛡️, Bolt ⚡, and Palette 🎨) managing 32-bit processes, 32-bit ELF32 binary loaders (`src/process/elf_loader.rs`), PE32 Windows subsystem execution (`src/hardware/win32.rs`), 32-bit system call compatibility thunks (`ia32`/`sys32`), and 32-bit user namespace UID/GID mappings in SigmaOS.

---

## Architecture & Subsystems

```
┌─────────────────────────────────────────────────────────────────┐
│              AI Agent 32-bit Orchestration Manager              │
├─────────────────────────────────────────────────────────────────┤
│  ELF32/PE32 Executable Agent    │  32-bit Syscall Thunk Governor│
│  Sv32/armv7 MMU Boundary Guard  │  32-bit User NS & Rlimit      │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│               SigmaOS 32-bit Compatibility Core                 │
├─────────────────────────────────────────────────────────────────┤
│  - ELF32 Loader (ElfClass::Elf32 & ET_EXEC / ET_DYN)             │
│  - PE32 Subsystem (Magic::Pe32 & PE Header Relocations)         │
│  - IA32 / Sys32 Compatibility Thunks (u32 -> u64 Register Expansion)│
│  - Sv32 / x86 Page Table Translation & 4GB Virtual Address Space│
│  - RDRAND32 / RDTSC Hardware Entropy Harvesting                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Key Operational Components

### 1. 32-bit Executable Loading (`Elf32` & `PE32`)
- **ELF32 Binary Loader (`src/process/elf_loader.rs`)**:
  - Validates `ElfClass::Elf32` headers and program headers (`PT_LOAD`).
  - Ensures virtual memory addresses do not exceed the 32-bit address space cap (`0xFFFFFFFF` / 4GB).
- **PE32 Windows Subsystem Loader (`src/hardware/win32.rs`)**:
  - Inspects PE Magic `0x010B` (`Pe32`) vs `0x020B` (`Pe32Plus`).
  - Performs 32-bit base relocation fixups for legacy Windows PE binaries.

### 2. System Call Thunking & Register Expansion (`ia32` / `sys32`)
- **Address & Pointer Extension**:
  - Translates 32-bit pointers (`u32`) passed in user registers (`eax`, `ebx`, `ecx`, `edx`) into 64-bit kernel virtual addresses (`u64`) via safe zero-extension or sign-extension rules.
- **Signal Frame Translation**:
  - Constructs 32-bit `sigcontext` structures on 32-bit process user stacks during signal dispatching.

### 3. Virtual Memory & 4GB Boundary Enforcement
- **Address Space Truncation Prevention**:
  - Enforces strict upper memory limits (`0xC0000000` or `0x80000000` user-kernel split) on 32-bit tasks.
  - Prevents 64-bit pointers from bleeding into 32-bit process address spaces during IPC or DMA buffer mappings.

### 4. 32-bit User Namespace & Resource Accounting
- **UID/GID Mapping**: Maps 16-bit legacy UIDs and 32-bit UIDs (`resource/accounting.rs` `UserID = u32`) seamlessly across container boundaries.
- **Process Resource Limits (`rlimit`)**: Enforces 32-bit stack size limits and maximum process memory bounds.

---

## AI Agent Operating Boundaries

### ✅ Always Do:
- Verify that 32-bit binary loading checks `ElfClass::Elf32` or `Magic::Pe32` before allocating task virtual memory.
- Enforce address space checks to guarantee memory allocations fall strictly within the 32-bit 4GB virtual boundary.
- Zero-extend 32-bit user registers when thunking system calls to 64-bit kernel handlers.

### ⚠️ Ask First:
- Enabling raw 32-bit real-mode or 16-bit V86 emulation.
- Overriding default 32-bit process userland-kernel memory splits (e.g. 3G/1G vs 2G/2G splits).

### 🚫 Never Do:
- Allow 64-bit pointers to be passed directly to 32-bit process signal handlers without address truncation guards.
- Bypass 32-bit `rlimit` stack or memory address space checks.
- Truncate 64-bit file offsets (`off64_t`) for 32-bit applications without checking for `EOVERFLOW`.

---

## Diagnostic CLI & Verification Commands

```bash
# Query active 32-bit process compatibility instances
sigmctl process list --arch=32

# Verify ELF32 / PE32 binary architecture header
sigpkg inspect-bin /bin/legacy_app32

# Test ia32 syscall thunking layer
sigmctl test-thunk --abi=ia32
```

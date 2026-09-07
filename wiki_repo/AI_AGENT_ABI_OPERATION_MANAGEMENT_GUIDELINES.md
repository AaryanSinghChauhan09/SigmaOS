# SigmaOS AI Agent ABI Operation Management Guidelines

## 1. Overview
SigmaOS implements a multi-OS Application Binary Interface (ABI) compatibility and syscall translation bridge managed autonomously or interactively by AI system agents (such as `LinuxBsdAbiBridge`, `ElfDynamicLinker`, `CosmopolitanApeBridge`, and `AbiCompatibilityAuditor`). These guidelines define multi-OS ABI translation (Linux, FreeBSD, OpenBSD, NetBSD), CPU calling conventions (System V AMD64, ARM AAPCS64, RISC-V C ABI), ELF dynamic linking, Cosmopolitan Actually Portable Executable (APE) headers, and syscall number mapping for AI agents in SigmaOS.

## 2. Core ABI Operation Management Principles

### 2.1 System V AMD64 & Multi-Arch Register Calling Conventions
- **x86_64 Calling Convention**: Syscall arguments pass through registers `RAX` (syscall number), `RDI` (arg1), `RSI` (arg2), `RDX` (arg3), `R10` (arg4), `R8` (arg5), `R9` (arg6).
- **AArch64 Calling Convention**: Syscall arguments use `X8` (syscall number), `X0`..`X5` (arg1..arg6).
- **RISC-V Calling Convention**: Syscall arguments use `A7` (syscall number), `A0`..`A5` (arg1..arg6).

### 2.2 Multi-OS Syscall Translation Bridge (`LinuxBsdAbiBridge`)
AI agents executing foreign binaries utilize `LinuxBsdAbiBridge` (`src/compatibility/distro_bridge.rs`) to dynamically translate system call numbers and struct layouts between guest and host OS ABIs:
- **Linux ABI Translation**: Maps Linux `sys_mmap` (nr 9), `sys_clone` (nr 56), and `sys_io_uring_setup` (nr 425) to native SigmaOS kernel handlers.
- **FreeBSD ABI Translation**: Translates FreeBSD `sys_jail_set` (nr 507), `sys_racct_attach` (nr 534), and `sys_pdfork` (nr 518) system call descriptors.
- **OpenBSD ABI Translation**: Translates OpenBSD `sys_pledge` (nr 108) and `sys_unveil` (nr 114) security system calls.

### 2.3 Cosmopolitan APE Binary Header Parsing
- **APE Header Bridge**: `CosmopolitanApeHeaderEngine` parses Cosmopolitan APE binaries (starting with `MZqFpD` / `PE` DOS stub signatures), mapping binary segments into memory without wine/emulator wrappers.

### 2.4 ELF Dynamic Linking & Procedure Linkage Tables (PLT / GOT)
- **ELF Program Header Parsing**: AI agents parse ELF `.interp` dynamic linkers (`/lib64/ld-linux-x86-64.so.2` or `/libexec/ld-elf.so.1`).
- **Dynamic Relocations**: Dynamic symbols (`.dynsym`, `.dynstr`) and PLT/GOT relocation tables (`.rela.dyn`, `.rela.plt`) are bound lazily on first function call.

---
*Maintained by the SigmaOS ABI, Binary Compatibility & SIG-Kernel Steering Committee.*

# SigmaOS AI Agent ABI Operation Management Guidelines

## 1. Executive Summary & Overview

SigmaOS maintains high binary compatibility across Linux, FreeBSD, OpenBSD, and multi-architecture binaries (x86_64, AArch64, RISC-V 64). Application Binary Interface (ABI) management governs register calling conventions, system call dispatch tables, stack frame structures, C-runtime (`glibc` vs `musl`) compatibility, and foreign OS syscall translation.

This document establishes the official guidelines and architectural standards for AI agents managing ABI dispatching, syscall translation, register context preservation, and multi-arch binary execution in SigmaOS.

---

## 2. Multi-Architecture System Call ABI Specifications

AI agents managing binary execution and kernel trap handlers interface with three primary CPU architecture ABIs:

| Target Architecture | Syscall Register | Argument Registers | Return Register | Trap Instruction | Calling Convention |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **x86_64** | `RAX` | `RDI`, `RSI`, `RDX`, `R10`, `R8`, `R9` | `RAX` | `syscall` | System V AMD64 ABI |
| **AArch64** | `X8` | `X0`, `X1`, `X2`, `X3`, `X4`, `X5` | `X0` | `svc #0` | AAPCS64 ABI |
| **RISC-V 64** | `A7` | `A0`, `A1`, `A2`, `A3`, `A4`, `A5` | `A0` | `ecall` | RISC-V LP64 ABI |

---

## 3. Foreign System Call ABI Translation Layers

SigmaOS implements transparent binary translation for foreign OS executables:

1. **Linux Syscall ABI Vector Table**:
   - Handles 64-bit Linux syscall numbers (e.g. `sys_read = 0`, `sys_write = 1`, `sys_open = 2`, `sys_clone = 56`, `sys_epoll_create1 = 291`).
2. **BSD System Call Translation (`FreeBsdJailManager`)**:
   - Translates FreeBSD/OpenBSD syscall numbers (`sys_sysctl`, `sys_pledge`, `sys_unveil`, `sys_kqueue`) to native SigmaOS kernel primitives.
3. **Libc Flavor Parity (`glibc` vs `musl`)**:
   - Dynamically resolves dynamic loader interpreters (`/lib64/ld-linux-x86-64.so.2` for glibc, `/lib/ld-musl-x86_64.so.1` for musl).
4. **Debian Multi-Arch Engine (`DebianMultiarchAptEngine`)**:
   - Supports foreign architecture package registration and multi-arch foreign library path resolution (`/lib/aarch64-linux-gnu`).

---

## 4. Register Context & Stack Alignment Rules

AI agents performing thread context switches or exception handler injection must strictly enforce stack and alignment bounds:

1. **Stack Alignment Rules**:
   - x86_64: The stack pointer `RSP` must be 16-byte aligned prior to `CALL` instructions (System V AMD64 ABI requirement).
2. **Extended Context Preservation (`CpuContext`)**:
   - Save and restore CS/DS/ES/FS/GS/SS segment registers, MSRs (`FS_BASE`, `GS_BASE`, `KERNEL_GS_BASE`), and the 512-byte FXSAVE/XSAVE vector state.
3. **Red Zone Non-Violation**:
   - In x86_64 System V ABI, interrupt handlers must respect the 128-byte **Red Zone** below `RSP` or adjust the stack pointer before pushing exception frames.

---

## 5. Security & eBPF LSM ABI Boundary Guardrails

1. **eBPF LSM Syscall Auditing**:
   - System calls entering Ring-0 pass through eBPF LSM hook filters (`sys_enter`, `sys_exit`).
2. **OpenBSD `pledge` ABI Lockdown**:
   - If a binary has restricted pledges (e.g. `pledge("stdio rpath", NULL)`), invoking unauthorized syscalls (e.g. `sys_socket` or `sys_execve`) causes immediate process termination (`SIGABRT` / `SIGKILL`).

---

## 6. Verification & Test Matrix Protocol

AI agents modifying ABI handlers or register context switches must pass verification:

1. **Native Test Suite**: Run `./run_sigma_tests.sh` to verify system call table and multi-arch HAL handlers (`MultiArchHalManager`).
2. **Stress & Inspection Tests**: Execute `tests/stress_and_fuzz_tests.rs` to validate context switch register integrity under high task concurrency.

---

*Approved by the SigmaOS System Architecture & Binary Parity Committee.*

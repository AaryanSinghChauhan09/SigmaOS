# AI Agent ABI Operation Management Specification for SigmaOS

This document specifies operational standards for AI agents managing Application Binary Interfaces (ABI), system call dispatching, multi-arch calling conventions, and foreign binary translation in **SigmaOS**.

---

## 1. ABI Management Protocol

AI agents managing binary execution and syscall boundaries must adhere to the following rules:

1. **Architecture Registers**:
   - x86_64: Syscall `RAX`, args (`RDI`, `RSI`, `RDX`, `R10`, `R8`, `R9`).
   - AArch64: Syscall `X8`, args (`X0`..`X5`).
   - RISC-V: Syscall `A7`, args (`A0`..`A5`).

2. **16-Byte Stack Alignment**:
   - Ensure x86_64 stack pointer `RSP` is 16-byte aligned before function invocation.

3. **Foreign Syscall Translation**:
   - Route FreeBSD/OpenBSD/Linux binary syscalls through `FreeBsdJailManager` and `DebianMultiarchAptEngine`.

4. **Pledge & Unveil Enforcement**:
   - Verify process `pledge` capability bitmasks prior to dispatching privileged system call routines.

---

## 2. Verification Protocol

- Run `./run_sigma_tests.sh` to execute the system call and HAL multi-arch test matrix.

---

*Maintained by the SigmaOS System Architecture Steering Committee.*

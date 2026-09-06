# SigmaOS AI Agent Kernel Management Specification

This document specifies mandatory kernel safety rules, syscall dispatching principles, EEVDF/BORE scheduler invariants, and hardware HAL integration standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. System Call Dispatching & Audit Logging
- **Syscall Dispatch Ring (`src/syscall/dispatcher.rs`, `src/syscall/table.rs`)**:
  - All syscall entries must be audited and logged via `SovereignSyscallAuditLogger`.
  - Non-standard or foreign ABI syscalls must pass through `LinuxSyscallAbiCompatTranslator` or OpenBSD `pledge` capability tables (`OpenBsdSyscallPledgeTable`).
  - Syscall filters (`LinuxSeccompBpfSyscallFilter`, `OpenBsdUnveilPathSandbox`) must reject unauthorized paths or ungranted capabilities immediately.

## 2. Kernel Scheduler Invariants & Real-Time Balancing
- **EEVDF & BORE Scheduler (`src/scheduler/scheduler.rs`)**:
  - Virtual runtime (`vruntime`), deadline, and thread lag calculations must prevent priority inversion or starvation.
  - Work-stealing routines across NUMA domains must preserve CPU core cache affinity.

## 3. Multi-Architecture HAL & Interrupt Controller Routing
- **Interrupt Routing (`src/hal/multi_arch.rs`, `src/kernel/hal.rs`)**:
  - IRQ handlers must be registered through `MultiArchHalManager::register_irq_handler` to prevent duplicate handler collisions.
  - Page faults during MMIO accesses must validate faulting addresses against `0` (NULL pointer check).

## 4. Capability Sandboxing & Security Enforcers
- **Capability Tokens (`src/security/capability.rs`, `src/security/sigma_unveil.rs`)**:
  - Kernel process creation and file descriptors must inherit minimal bitmask capability tokens.
  - Revocation of capability tokens must execute atomically across all active process children.

## 5. AI Agent Kernel Directives
1. **Never Panics in Ring 0**: Kernel routines must return `Result<T, &'static str>` or error codes rather than invoking `panic!()` or unhandled trap handlers.
2. **Exhaustive Matching**: Always maintain exhaustive match arms on `TargetArch` and `TargetArchitecture` enums across kernel HAL drivers.

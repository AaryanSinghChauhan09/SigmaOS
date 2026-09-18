# AI Agent Instruction Execution & Syscall Dispatch Management in SigmaOS

## Overview

SigmaOS instruction execution architecture (`src/arch/comprehensive.rs`, `src/arch/hal.rs`, `src/security/`, `src/kernel/linux_bsd_innovations.rs`) supports multi-ISA architecture register contexts (x86, x64, AArch64, RISC-V 64, LoongArch64), sandboxed eBPF bytecode execution (`EbpfRuntime`), and hardened capability-checked system call dispatching (`HardenedSyscallDispatcher`).

AI agents (such as Jules, Herdr agentic code checkers, dynamic binary translators, and JIT compilers) must follow instruction execution guidelines when dispatching syscalls or executing machine instructions.

---

## Multi-ISA Execution Architecture & Register Contexts

SigmaOS provides multi-architecture CPU register contexts (`SovereignRegisterContext`) for zero-cost ISA switching:

```
AI Agent Thread → Hardened Syscall Dispatcher (`HardenedSyscallDispatcher`)
                          │
                          ▼
            OpenBSD Pledge Syscall Promise Gate
                          │
                          ▼
         Multi-ISA CPU Context Switch (`SovereignRegisterContext`)
     ┌──────────┬──────────┼──────────┬──────────┐
     ▼          ▼          ▼          ▼          ▼
  x86_64     AArch64    RISC-V 64  LoongArch   eBPF JIT
(x64 Regs) (ARM64 Regs) (RV64 Regs) (LA64 Regs) (Sandbox)
```

| ISA Architecture | Enum Variant | Target Platforms | Key Feature |
|------------------|--------------|------------------|-------------|
| **x86_64** | `SovereignIsaArchitecture::X64` | PC, Cloud Servers, Workstations | SIMD AVX-512 & AMX vector acceleration |
| **AArch64** | `SovereignIsaArchitecture::AArch64` | ARM Single-Board Computers, Apple Silicon | SVE / SVE2 vector registers |
| **RISC-V 64** | `SovereignIsaArchitecture::Riscv64` | Open Hardware, Embedded SoC | RV64GCV vector extension |
| **LoongArch64** | `SovereignIsaArchitecture::LoongArch64` | Loongson Workstations | LASX 256-bit SIMD execution |

---

## 1. Hardened Syscall Dispatching (`HardenedSyscallDispatcher`)

All syscall instructions invoked by AI subagents pass through capability-verification checks:

```rust
use sigmaos::security::{HardenedSyscallDispatcher, HardenedSyscallError};

let mut dispatcher = HardenedSyscallDispatcher::new();

// Execute system call instruction under active pledge sandbox
match dispatcher.dispatch_syscall(syscall_num, &[arg1, arg2, arg3], caller_pid) {
    Ok(ret_val) => println!("Syscall #{} executed successfully: {}", syscall_num, ret_val),
    Err(HardenedSyscallError::PledgeViolation) => {
        eprintln!("Blocked: Syscall prohibited under active OpenBSD pledge mask");
    }
    Err(e) => eprintln!("Syscall error: {:?}", e),
}
```

---

## 2. eBPF Bytecode Execution & Safety Verification (`EbpfRuntime`)

AI agents compiling or executing eBPF micro-programs run them through the static verifier:

```rust
use sigmaos::kernel::{EbpfRuntime, EbpfInstruction};

let runtime = EbpfRuntime::new();

// Static verifier checks for division-by-zero, out-of-bounds registers, and missing return
runtime.verify_program(&ebpf_bytecode_instructions)?;

// Execute eBPF bytecode in memory-safe sandbox
let result = runtime.execute(&ebpf_bytecode_instructions, initial_reg0_val)?;
```

---

## Directives for AI Agents Executing Instructions

1. **Preserve Callee-Saved Registers**: When executing inline assembly or dynamic JIT blocks, preserve callee-saved registers per target ABI specification.
2. **Never Bypass Verifier**: All JIT-compiled bytecode must pass static verification before execution.
3. **Handle Instruction Traps**: Catch Division-By-Zero, Invalid Opcode, and Alignment Fault traps gracefully without crashing the kernel worker thread.

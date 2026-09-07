# AI Agent Microprocessor Operation Management Architecture (`docs/AGENTS_MICROPROCESSOR_OPERATIONS.md`)

This guide details the technical architecture, CPU context structures, and AI agent monitoring protocols for microprocessor operations in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS provides multi-architecture CPU context switching and microarchitecture optimization:

### A. Hardware Abstraction Layer & Register Contexts
- Located in `src/arch/portability.rs` and `src/arch/hal.rs`.
- Manages multi-architecture register descriptors (`X86Context`, `X64Context`, `Arm64Context`, `Riscv64Context`, `LoongArch64Context`, `Ppc64Context`, `S390xContext`) and execution levels (`PassiveLevel`, `DispatchLevel`).

### B. ISA Level Auto-Detection & Vectorized JIT
- Located in `src/compatibility/cachy_os.rs` and `src/klib/isa.rs`.
- Detects microarchitecture levels (`x86-64-v1` through `v4`) and routes memory/vector operations to hardware-accelerated instruction paths.

### C. CPU Power & Thermal Governor
- Located in `src/ai/agent.rs` and `src/ai/next_gen.rs`.
- Monitors CPU temperatures (`cpu_temp_c`) and throttles CPU clock frequencies (`cpu_freq_limit_mhz`) during high thermal loads.

---

## 2. AI Agent Operational Directives

1. **Architecture Portability:** Ensure new kernel assembly or register context helpers support all primary target architectures.
2. **IRQL Non-Blocking Rule:** Confirm high-IRQL code paths (`DispatchLevel`) refrain from heap allocations or page faulting operations.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm CPU portability and ISA unit tests pass.

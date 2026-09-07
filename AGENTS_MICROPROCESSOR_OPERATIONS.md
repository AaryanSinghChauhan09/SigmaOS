# SigmaOS AI Agent Microprocessor Operation Management Directive (`AGENTS_MICROPROCESSOR_OPERATIONS.md`)

This document defines technical directives, multi-architecture context handling rules, and hardware abstraction layer (HAL) guidelines for AI agents managing microprocessor operations in SigmaOS.

---

## 1. Core Principles for Microprocessor Operation Management

Microprocessor hardware abstraction, register context switching, ISA feature detection, and thermal throttling are critical to low-level kernel performance in SigmaOS. AI agents modifying microprocessor routines must observe the following rules:

1. **Multi-Architecture Context Switching (`CpuContextState`):**
   - Context switches must save and restore general-purpose and floating-point registers cleanly using architecture-specific descriptors (`X86Context`, `X64Context`, `Arm64Context`, `Riscv64Context`, `LoongArch64Context`, `Ppc64Context`, `S390xContext`).
   - Validate register states prior to context restoration to guard against illegal register values or unaligned stack pointer traps.

2. **Microarchitecture ISA Feature Detection (`x86-64-v1` through `v4`):**
   - Microprocessor extensions (AVX, AVX2, AVX-512, BMI2, FMA) must be auto-detected at runtime (`V4OptimizedPackageManager`, `SovereignMicroarchJitEngine`).
   - Dynamic SIMD vectorization and memcpy routing must choose optimal ISA instruction variants without triggering undefined instruction traps on older hardware.

3. **Hardware Abstraction IRQLs (`PassiveLevel`, `DispatchLevel`):**
   - Execution levels must strictly regulate allowable operations. High IRQL levels (`DispatchLevel` and above) prohibit page faults and blocking allocations.

4. **Thermal Throttling & Power Management:**
   - Monitor CPU core temperatures (`cpu_temp_c`) and adjust frequency limits (`cpu_freq_limit_mhz`) dynamically to prevent thermal throttling or hardware damage under heavy compute loads.

---

## 2. Pre-Commit Microprocessor Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Context switch routines preserve register symmetry across target architectures (`X86_64`, `ARM64`, `RISCV64`).
- [ ] Microarchitecture ISA feature detection falls back safely on baseline hardware (`x86-64-v1`).
- [ ] IRQL execution levels guard against blocking calls inside high-priority HAL contexts.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.

# SigmaOS Security Architecture & Security Baseline

This document defines the security architecture, capability model, privilege boundaries, and threat mitigation strategies for the SigmaOS kernel.

## Core Security Directives

1. **Least Privilege Principle:** Processes run with the minimum necessary capabilities required for their function.
2. **W^X (Write XOR Execute):** Virtual memory pages cannot be simultaneously writable and executable.
3. **Capability Tokens:** Access to hardware, IPC ports, and sensitive syscalls requires explicit 64-bit capability tokens.
4. **Isolated Address Spaces:** User address space (`0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`) is strictly isolated from kernel address space (`0xFFFF_8000_0000_0000` and above).
5. **No Undocumented Unsafe:** Every `unsafe` block must document memory invariants, alignment bounds, and concurrency assumptions.

## Security Controls Overview

- **Syscall Validation:** Bounds checking and pointer alignment checks on all user arguments prior to copy-in.
- **Seccomp Filters:** System call filtering based on BPF-like state machine checks.
- **PQC Attestation:** Dilithium-5 / Kyber-1024 cryptographic verification for package and binary signature checking.
- **Kernel Memory Guard:** Double-free, use-after-free, and stack overflow guard pages.

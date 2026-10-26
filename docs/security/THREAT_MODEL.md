# SigmaOS Kernel Threat Model

This document identifies primary threat vectors against the SigmaOS kernel and details mitigation controls.

## Threat Matrices

| Threat ID | Threat Vector | Description | Impact | Mitigation Strategy & Controls |
|---|---|---|---|---|
| **T-01** | Kernel Space Memory Corruption | Malformed syscall arguments or buffers overwrite kernel memory | Critical | Bounds checking on copy-in, stack guard pages, safe Rust default |
| **T-02** | Privilege Escalation via Syscall | Process abuses unrestricted syscall access to gain root | Critical | Capability token checks, seccomp filters, POSIX UID/GID validation |
| **T-03** | Arbitrary Code Execution (W^X) | Attacker injects shellcode into writable data buffer and jumps to it | Critical | Enforce W^X page table attributes, NX (No-Execute) bit on user data |
| **T-04** | Use-After-Free / Double-Free | Race condition or bad lifecycle management in allocators | High | Typed ownership in Rust, double-free detection in `PhysicalMemoryManager` |
| **T-05** | Denial of Service via Resource Exhaustion | Process consumes all PIDs, swap slots, or run queue slots | High | Bounded swap queues, PID limits per process, starvation decay |
| **T-06** | Rogue Package Injection | Malicious binary installed during update | Critical | PQC Dilithium-5 / Kyber cryptographic signature verification |

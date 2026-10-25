# SigmaOS Kernel Threat Model

## Threat Vectors & Mitigation Matrix

| Threat Vector | Attack Surface | Mitigation Mechanism |
|---|---|---|
| Malformed Syscall Pointers | Kernel entry point | Pointer range validation, copy_from_user safely returning `-EFAULT` |
| Buffer Overflow / Stack Smashing | Syscall handlers / parsers | Bounds checking, safe Rust slice indexing, stack frame cookies |
| Privilege Escalation | Unprivileged process | Capability checks, process token isolation, seccomp filters |
| Kernel Memory Leak / Out of Bounds | Kernel heap / Slab | Safe Rust ownership, double-free prevention, bounded queues |
| Rogue Kernel Image Loading | Bootloader / Firmware | PQC Dilithium-5 kernel signature verification (`kernel_sign.rs`) |
| Uncontrolled Panic / Crash | Kernel panic path | Register dump, greppable panic log, fail-closed diagnostic report |

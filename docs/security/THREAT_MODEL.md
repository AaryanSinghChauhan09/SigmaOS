# SigmaOS Kernel Threat Model

## Threat Vectors & Mitigations

| Threat Vector | Description | Risk Level | Mitigation Mechanism |
|---|---|---|---|
| User Pointer Dereference | Malicious userspace pointer passed into syscall | High | Safe copy_from_user / copy_to_user with address range checks |
| Privilege Escalation | Unprivileged task invoking administrative syscalls | High | Capability token verification before privileged operations |
| Kernel Buffer Overflow | Malformed ELF or filesystem payload exploiting parsers | High | Safe Rust bounds checking, fuzzing targets in `fuzz/` |
| Kernel Heap Exhaustion | Unbounded metadata allocation by untrusted process | Medium | Explicit queue depth limits and per-process resource limits |
| Out-of-bounds DMA | Compromised device driver writing to arbitrary physical RAM | High | IOMMU VT-d page-table translation and DMA buffer bounds |

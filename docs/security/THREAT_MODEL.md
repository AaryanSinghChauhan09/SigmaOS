# Kernel Threat Model & Trust Boundaries

## Security Objectives

1. Prevent unprivileged userspace processes from reading or corrupting kernel memory.
2. Prevent lateral privilege escalation between isolated user processes.
3. Protect kernel data structures from untrusted hardware input (USB, Network, PCI).

---

## Trust Boundaries

```text
[ Hardware / Peripherals ] ── (Untrusted Input Boundary) ──► [ Drivers ]
                                                                │
[ Untrusted Userspace ] ── (Syscall / Capability Boundary) ──► [ Kernel Core ]
                                                                │
                                                         [ Kernel Memory ]
```

---

## Threat Vectors & Mitigations

| Threat Vector | Severity | Mitigation | Verification |
|---|---|---|---|
| Malformed Syscall Arguments | High | Address checking & pointer range validation before dereference | Fuzzing & negative tests |
| Buffer Overflow in Driver/Network | High | Safe Rust bounds checks & W^X page protection | `AddressSanitizer` & Fuzzing |
| Privilege Escalation | Critical | Mandatory Capability Token verification on entrypoints | Syscall audit suite |
| Untrusted Module Load | High | Module signature attestation (Dilithium-5) | `LkmModuleLoaderEngine` tests |
| Kernel Deadlock / DoS | Medium | Lock hierarchy enforcement & bounded queue depth limits | Concurrency audit |

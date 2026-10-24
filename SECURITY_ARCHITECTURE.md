# SigmaOS Security Architecture

## Security Design Principles

1. **Least Privilege**: Processes execute with the minimum required capabilities.
2. **Capability-Based Authorization**: Resources are gated by capability security tokens validated at kernel entrypoints.
3. **Memory Isolation & Protection**: Enforces W^X (Write XOR Execute), redzone overflow buffers, and ASan shadow memory validation.
4. **Sandboxing & Seccomp Rules**: Userspace processes, package installers, and network services run within strict sandboxes.
5. **Post-Quantum Cryptography (PQC)**: Signature verification uses Dilithium-5 / Kyber-1024 algorithms for package and release updates.

---

## Security Architecture Overview

```
               ┌────────────────────────┐
               │    Userspace Process   │
               └───────────┬────────────┘
                           │ Syscall Entry (Fast Syscall)
                           ▼
               ┌────────────────────────┐
               │ Capability Token Check │
               └───────────┬────────────┘
                           │ Valid Token
                           ▼
               ┌────────────────────────┐
               │  Kernel Subsystem Ops  │
               └────────────────────────┘
```

---

## Vulnerability Triage and Reporting

- **Reporting**: Security vulnerabilities must be reported directly via security advisories or designated maintainer contacts.
- **Triage Cadence**: Weekly security advisory review (`weekly_security`).
- **Remediation**: Upgrades and regression tests must follow `.github/workflows/security.yml` procedures.

# SigmaOS Security Architecture Specification

SigmaOS enforces a defense-in-depth, capability-based security architecture designed for microkernel and monolithic hybrid isolation.

---

## Security Foundations

1. **Least Privilege**: Components run with the minimum necessary capabilities required to fulfill their function.
2. **Capability Token Verification**: Syscalls and IPC boundaries validate caller capability tokens before granting access to kernel objects, IPC channels, or physical hardware.
3. **Hardware-Enforced Isolation**: Memory management enforces W^X (Write XOR Execute) memory protection on user and kernel address spaces with supervisor mode execution prevention (SMEP/SMAP).
4. **Fail Closed**: Any security ambiguity, malformed input header, or invalid capability token results in explicit denial and diagnostic logging.

---

## Security Components

- `SECURITY_ARCHITECTURE.md`: Global security architecture invariants and threat assumptions.
- `docs/security/THREAT_MODEL.md`: Formal threat model covering attacker personas, boundaries, and mitigations.
- `docs/security/CAPABILITY_POLICY.md`: Capability token structure, bitmasks, and privilege enforcement rules.
- `docs/security/SECURITY_TEST_PLAN.md`: Automated testing procedures for fuzzing, SAST, and security regression checks.

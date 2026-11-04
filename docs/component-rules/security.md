# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge and unveil sandboxing
- FreeBSD Capsicum capability model
- Linux seccomp-bpf and Landlock

## Purpose
Define required capability isolation, boundary defense, sandboxing, and validation
for SigmaOS security primitives and privilege governance.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. All security domains must enforce least privilege by default.
6. System calls must enforce user-pointer bounds checking and taint tracking.
7. Audit trails must log capability authorization failures without leaking secrets.

## Prohibited

- Ambient authority and unconstrained root permissions
- Silent capability bypasses
- Execution of untrusted external scripts without sandboxing
- Status promotion without denial tests and security review

## Required evidence

- Unit test report
- Integration test report
- Denial tests and fuzzing
- Security review audit log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

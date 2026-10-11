# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD Pledge & Unveil
- FreeBSD Capsicum
- Linux AppArmor & Capabilities

## Purpose
Define mandatory access controls, capability tokens, and sandbox enforcement rules for SigmaOS.

## Mandatory rules

1. Every privileged operation requires an explicit capability token.
2. Denial tests are mandatory: protected operations must fail explicitly without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. Path-based access controls and DAC/MAC rules must enforce default-deny policies.

## Prohibited

- Granting ambient credentials or root rights by default
- Bypassing capability verification on kernel entry points
- Unsandboxed execution of external package scripts
- Promoting security status without negative denial test evidence

## Required evidence

- Capability authorization unit tests (`cargo test --lib`)
- Denial test suite execution output (`./run_sigma_tests.sh`)
- Security review log link in PR documentation

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and threat model specification |
| Prototype | Capability check unit tests and negative denial tests |
| Integrated | Sandboxed process execution in QEMU runtime |
| Hardened | Static analysis, SAST/fuzzing, and dedicated security review |
| Supported | Production threat model compliance and reproducible audits |

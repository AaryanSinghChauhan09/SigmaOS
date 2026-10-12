# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge and unveil syscall/VFS sandboxing
- FreeBSD Capsicum capability rights
- Linux Landlock LSM and seccomp-BPF

## Purpose
Enforce mandatory access controls, least-privilege capability execution, sandboxing, and denial testing.

## Mandatory rules
1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. All external input parsers must have associated fuzz targets or documented exemptions.

## Prohibited
- Unsandboxed package script execution
- Hardcoded secret keys or weak default permissions
- Undocumented capability bypasses
- Status promotion without security denial evidence

## Required evidence
- Capability denial test execution report
- Landlock/Pledge/Unveil sandboxing test log
- Security audit log and parser fuzzing results
- Cryptographic primitive verification report

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Security threat model and policy ADR |
| Prototype | Capability check unit tests |
| Integrated | Sandboxing enforcement in runtime path |
| Hardened | Comprehensive denial testing, fuzzing, code audit |
| Supported | Production-ready zero-trust isolation evidence |

# Security, Sandboxing & Capability Management Component Guide

## Component Overview
The Security subsystem enforces fine-grained capability tokens, Landlock-style filesystem sandboxing, OpenBSD-style Pledge and Unveil, and cryptographic integrity.

## Linux Inspiration
Linux Landlock LSM, seccomp-bpf syscall filtering, AppArmor/SELinux mandatory access controls, and Linux capabilities (cap_enter).

## BSD Inspiration
OpenBSD pledge() and unveil() syscalls, FreeBSD Capsicum capability mode and MAC framework, OpenBSD W^X memory protection.

## Current SigmaOS Status
Pledge and unveil abstractions defined, fail-closed access control models implemented. Dynamic Landlock V4 path restriction and seccomp filters need completion.

## Critical Missing Features
Full kernel syscall enforcement of pledge promises, Landlock-style hierarchical path rules, runtime capability bounding sets for unprivileged users, and TPM 2.0 PCR attestation.

## Implementation Priority
CRITICAL (Phase 1)

## Key Files to Create/Improve
src/security/sandbox.rs, src/security/pledge.rs, src/security/unveil.rs, src/security/capabilities.rs, src/security/tpm.rs

## Testing Strategy
Pledge violation trap tests, jailbreak and path traversal regression suite, seccomp syscall denial audit.

## Dependencies
KERNEL_CORE_AGENT, FILESYSTEM_VFS_AGENT, PROCESS_MANAGEMENT_AGENT

## Success Criteria
Zero sandbox escape vulnerabilities, 100% fail-closed syscall defaults, sub-microsecond capability checking overhead.

## Open Source Competitors Analysis
Combines the ergonomic simplicity of OpenBSD pledge with the multi-dimensional path isolation of Linux Landlock in a single unified API.

## Future Enhancements
Formal verification of sandboxing invariants and micro-VM hardware-enforced enclave boundaries.

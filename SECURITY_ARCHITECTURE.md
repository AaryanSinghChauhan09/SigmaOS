# SigmaOS Security Architecture

## Overview
SigmaOS enforces a capability-based security model across all kernel entrypoints and userspace interactions.

## Core Security Directives
1. **Capability Token Verification**: Every syscall requiring privilege must present a valid `CapabilityToken` verified in Ring 0.
2. **Memory Safety & W^X Enforcement**: Kernel and user memory pages strictly enforce Write-XOR-Execute (`W^X`) protections.
3. **PQC Signatures**: Kernel modules and package manifests require Dilithium-5 / Post-Quantum Cryptographic verification.
4. **Sandboxing**: Userspace processes operate under seccomp-style syscall filtering and OpenBSD pledge/unveil restrictions.

# SigmaOS Kernel Security Architecture

## Overview

SigmaOS enforces defense-in-depth, least privilege, capability-based access control, sandboxing, W^X page protection, and post-quantum cryptographic validation across the kernel-userspace boundary.

## Core Security Pillars

1. **Capability Tokens**: Fine-grained capabilities (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_CHROOT`, `CAP_KILL`, `CAP_PTRACE`) required for privileged syscalls.
2. **Pledge & Unveil**: OpenBSD-style system call constraint (`pledge`) and file path restriction (`unveil`).
3. **Seccomp BPF Filtering**: Syscall filtering per process sandbox.
4. **Memory Hardening**: Kernel Address Space Layout Randomization (KASLR), stack cookies, W^X memory permissions, and zeroing sensitive key structures.
5. **Post-Quantum Cryptography**: Dilithium-5 and SPHINCS+ digital signatures for kernel image and package verification.

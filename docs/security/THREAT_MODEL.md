# SigmaOS System Threat Model

## Overview

This document defines the threat vectors, adversary assumptions, trust boundaries, and mitigation strategies for SigmaOS.

## Threat Boundaries

1. **Userland vs. Kernel Boundary:** Ring 3 userspace application processes must not be able to compromise Ring 0 kernel state.
   - *Mitigation:* Syscall argument validation, CR2 fault handling, capability token check.
2. **Inter-Process Boundary (IPC & Shared Memory):** Unprivileged processes must not manipulate another process's private memory descriptors or file locks.
   - *Mitigation:* POSIX lock governance, isolation namespaces, capability checking.
3. **Network & Package Supply Chain Boundary:** Malicious network packets or tampered package repositories must not cause remote code execution or unauthorized persistent installations.
   - *Mitigation:* Dilithium-5 PQC signature verification, memory bounds clamping, packet fuzzing.

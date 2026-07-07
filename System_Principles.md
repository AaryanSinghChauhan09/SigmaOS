# SigmaOS Core System Principles

## Overview
SigmaOS is built on a set of core principles that separate it from standard GNU/Linux distributions. By placing speed, security, and predictability first, the OS minimizes runtime footprint, avoids unverified dependencies, and guarantees offline execution and reproducible states.

## Core Pillars
1. **Low-level first**: Implement critical features directly in C, Assembly, Rust, and Nim. Minimize dependency on heavy, dynamically-typed runtime languages (e.g., Python, Javascript) in boot sequences, package handling, or IPC.
2. **Reproducibility**: Enforce deterministic compilation pipelines. A given source configuration must result in bit-for-bit identical binary artifacts, complete with Software Bills of Materials (SBOMs) and signed build provenance.
3. **Least privilege**: Restrict system calls and filesystem access using custom Linux Security Module (LSM) policies, Landlock sandbox profiles, and capability tokens. Home directories and user workspaces are encrypted by default.
4. **Offline-first**: Ensure all help systems, reference manuals, local AI runtimes, and law/education databases are fully accessible without a network connection.
5. **Unified UX**: Establish a consistent desktop experience via the custom Zenith Wayland compositor, built-in accessibility services, and comprehensive multilingual translations (with primary focus on Indic languages).

```
   [System App / Utility]
             │
             ▼
   [Capability Check]  ──► Fails? ──► Terminate
             │
             ▼ Passes
   [Landlock Filesystem Sandbox]
             │
             ▼
      [System Kernel]
```

## Technical Rules
- **No Python/JS in PID 1**: The init system (`sigmad`) and the package manager (`sigpkg`) must compiled to standalone machine code without interpreter dependencies.
- **Strict Capabilities**: Applications must not run as root. Root permissions are broken down into discrete capability tokens (`cap_network`, `cap_mount`, `cap_debug`).

## Implementation Checklist
- [x] Standardize on Rust/Nim for userland utilities (replaces shell scripts).
- [ ] Implement signature validation for all boot stages.
- [ ] Enforce automated SBOM generation for `sigpkg` outputs.
- [ ] Integrate Indic multilingual support into Zenith compositor.

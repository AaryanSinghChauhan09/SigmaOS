# System Principles & Directives

## 1. Single Package Ecosystem
Everything on the system, from the kernel to the web browser to the pentesting suite, is installable exclusively via `sigpkg`. We reject the fragmentation of flatpaks, snaps, and appimages.

## 2. Reproducible Builds
Every bundled application is deterministically compiled. If a user downloads the source, they will generate the exact same binary hash as the one provided in the repository.

## 3. Secure Posture
All bundled applications run under the principle of least privilege. Network access requires explicit declarative grants. Home directories are encrypted by default, anchored to TPM attestation.

## 4. Offline-First
SigmaOS ships with offline documentation, local AI runtimes, and local legal databases to ensure it remains a fully functional ecosystem even in air-gapped environments.

## 5. Low-Level Independence (Zero-Bloat Ecosystem)
SigmaOS heavily reduces dependencies on high-level programming languages, predefined external libraries, and bloated frameworks. The core system relies on bare-metal C, assembly, and `no_std` Rust. We prioritize writing custom, highly-optimized low-level functions tailored for silicon efficiency over adopting black-box legacy dependencies.

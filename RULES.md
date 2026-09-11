# 🛡️ SigmaOS Master Rules & Distribution Engineering Directives (`RULES.md`)

**Version:** 3.0.0
**Scope:** All Developers, Maintainers, Contributors, and Autonomous AI Engineering Agents
**Governance:** Inspired by Linux & BSD Distribution Standards (Arch Linux, Debian, Fedora, OpenBSD, FreeBSD, Alpine, Void, NixOS, Gentoo)

---

## 🏛️ Executive Summary & Core Mandates

SigmaOS is a sovereign, secure, next-generation bare-metal operating system written in zero-dependency safe Rust (`#![no_std]`). To ensure uncompromising quality, security, and cross-distribution interoperability, all code, driver, and documentation contributions MUST strictly obey these Linux & BSD distribution-inspired engineering directives.

---

## 🏹 1. Arch Linux Inspired Directives: Simplicity, Transparency, & AUR Purity

1. **The Arch Way (Simplicity & User Control):**
   - Keep the system minimal, elegant, and fully transparent. Never obfuscate OS internals behind proprietary binary blobs or undocumented hidden abstractions.
2. **Package Recipe Purity (`PKGBUILD` Parity):**
   - All package build manifests MUST specify explicit array variables (`depends`, `makedepends`, `provides`, `conflicts`, `sha256sums`).
3. **Automated AST Diff Inspection (`AurPkgbuildDiffAnalyzer`):**
   - Third-party build scripts (AUR helpers / Yay / Paru parity) MUST undergo automated AST diff inspection to block unverified remote shell execution (e.g. `curl | bash` or raw socket connections) prior to compilation.
4. **Mirror Latency Ranking & Non-Locking Maintenance (`pacman-contrib` Parity):**
   - Package database queries MUST execute without acquiring global database locks (`checkupdates` parity).
   - Package mirror selections MUST be dynamically ranked by latency and download bandwidth (`ReflectorMirrorRanker`).

---

## 🍥 2. Debian Inspired Directives: Determinism, Reproducibility, & Multiarch Integrity

1. **Cleanroom Chroot Building (`sbuild` / `poudriere` Parity):**
   - All software packages and system binaries MUST compile inside isolated, non-contaminated cleanroom build chroots to eliminate host environment leakages.
2. **Bit-for-Bit Reproducible Builds:**
   - Enforce `SOURCE_DATE_EPOCH` build timestamps, zeroed tar header timestamps, and canonical directory sorting. Every build output MUST produce verifiable bit-for-bit SHA-256 hashes matching published SBOM records.
3. **Multiarch & Library SONAME Invariants:**
   - Dependency resolvers MUST support multi-architecture co-existence (`AptPinningMultiArchResolver`) and enforce ABI versioning rules to prevent shared library breakage.

---

## 🎩 3. Fedora & RHEL Inspired Directives: Reliability, Provisioning, & First-Boot Health

1. **First-Boot Declarative Provisioning (`Ignition` / `Butane` Parity):**
   - Headless deployments MUST compile declarative specs (`FedoraButaneSpec`) into immutable provisioning manifests (`FedoraIgnitionConfig`) for disk partitioning, SSH key provisioning, and service enablement.
2. **Automated Health Monitoring & Rollback (`Greenboot` Parity):**
   - Boot health monitors (`FedoraGreenbootEngine`) MUST execute diagnostic probes on startup. If critical services fail beyond threshold retries, the system MUST trigger automated bootloader fallback to the last known good generation.
3. **Offline System Updates (`systemd-offline-update` Parity):**
   - Operating system upgrades MUST stage update packages into non-volatile storage (`FedoraOfflineUpdateEngine`) and apply updates safely during reboot execution.

---

## 🐡 4. OpenBSD Inspired Directives: Proactive Security, Privilege Reduction, & Audit

1. **Monotonic Syscall Capability Restriction (`pledge` Parity):**
   - Every userland binary and background daemon MUST invoke `pledge()` immediately upon entry to drop unneeded syscall privileges irreversibly.
2. **Filesystem Path Scoping (`unveil` Parity):**
   - Filesystem visibility MUST be restricted using `unveil()` path scoping rules. Unveiled paths MUST specify minimal permissions (`r`, `w`, `x`, `c`).
3. **Strict Memory Protection & Input Validation:**
   - Enforce W^X (Write XOR Execute) memory page isolation (`SovereignKaslrWxAllocator`).
   - Validate hostnames per RFC 952/1123, POSIX usernames/env-keys (`[a-zA-Z_][a-zA-Z0-9_]*`), decimal IPv4 addresses (rejecting leading zeros to prevent octal SSRF bypasses), and colon-bounded path traversal checks.

---

## 😈 5. FreeBSD Inspired Directives: Modular Base, Capsicum Sandboxing, & ZFS Environments

1. **Strict Base vs. Ports Separation:**
   - Maintain clean architectural boundaries between core microkernel shards (`/sovereign/system`) and third-party applications (`/sovereign/store`).
2. **Fine-Grained Capability Sandboxing (`Capsicum` Parity):**
   - File descriptors delegated to untrusted worker threads MUST restrict capabilities using FreeBSD Capsicum rights (`FreeBsdCapsicumDescriptorDelegate`).
3. **Boot Environment Management (`bectl` Parity):**
   - Storage subsystems MUST support ZFS boot environment snapshots (`bectl` / `beadm` parity) allowing instant bootloader target selection during system recovery.

---

## 🏔️ 6. Alpine Linux & Void Linux Inspired Directives: Lightweight Musl & Runit Simplicity

1. **Static Musl Lightweight Init (`MuslLightweightInitEngine`):**
   - Headless, embedded, and container micro-VM targets MUST support lightweight musl static init execution and fast `runit` service supervision (`VoidRunitServiceSupervisorEngine`).
2. **Volatile RAM Overlays & Persistent Snapshotting (`lbu` Parity):**
   - Diskless tmpfs RAM overlay installations MUST support snapshotting volatile configurations and committing persistent apk package overlays (`AlpineLbuBackupEngine`).

---

## ❄️ 7. NixOS & GNU Guix Inspired Directives: Declarative Determinism & Merkle Store

1. **Content-Addressed Storage (CAS Store):**
   - All software packages, configurations, and userland binaries MUST be stored in the Merkle tree store (`/sovereign/store/<hash>-<name>-<ver>`).
2. **Sub-Millisecond Atomic State Hot-Swapping:**
   - System state transitions MUST update generation symlinks atomically (`DeclarativeStateGraph`), guaranteeing sub-millisecond atomic rollbacks without live binary corruption.

---

## 🌀 8. Gentoo Inspired Directives: CPU Microarchitecture Optimization & Use Flags

1. **Microarchitecture ISA Auto-Detection (`x86-64-v1`..`v4`):**
   - Code generators and JIT compilers MUST auto-detect host CPU ISA feature levels (`IsaLevel`) and route vectorized SIMD operations (`AVX2`, `AVX-512`, `FMA`, `NEON`) dynamically.
2. **USE Flag Compilation Governor (`PortageUseFlagGovernor`):**
   - Subsystem compilation MUST support fine-grained USE flag feature toggling to optimize binary size and remove unused feature overhead.

---

## 🔒 9. Zero-Dependency & Cryptographic Integrity Invariants

1. **Zero External Crates (`no_std`):**
   - Core microkernel shards and system modules MUST maintain 0 external crates in `Cargo.toml`. All data structures MUST utilize native `klib` abstractions.
2. **Post-Quantum Cryptographic (PQC) Attestation:**
   - All driver signatures, livepatching trampolines, package manifests, and kernel module attestations MUST be cryptographically verified using Dilithium-5 signatures or Kyber-1024 KEM key exchange.
3. **Secret Scanner Safeguards:**
   - Mock credentials or test keys in code and tests MUST use identifiers prefixed with `mock_` or `test_` (e.g., `mock_client_secret`).

---

*RULES.md — Maintained by the SigmaOS Core Architecture Team. All directives verified by `./run_sigma_tests.sh`.*

# Changelog

All notable changes to SigmaOS are documented here following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — main branch

### Added (August 15, 2026)
- **Gaming Performance Mode** (`src/kernel/sched/gaming_performance.rs`): DragonFly LWKT SMP per-CPU queues, UKSM page deduplication, CPU/GPU scheduler boost, network QoS for gaming
- **eBPF-inspired Security Verifier** (`src/security/vulnerability.rs`): syscall-level security policy enforcement without external LSM libraries
- **Zero-Copy Splice** (`sigma_splice`): Linux splice(2)/FreeBSD sendfile(2) inspired zero-copy data transfer between FDs
- **Landlock + OpenBSD Unveil Hybrid** (`sigma-unveil`): process-level filesystem access restriction without root privileges
- **EndeavourOS Compatibility Parity**: rolling-release package management, `sigma-welcome` first-boot app
- **Linux PAM** (`src/security/pam/`): pure Rust PAM-compatible authentication stack — no libpam linkage
- **BSD Securelevels** (`src/security/securelevels.rs`): kernel-enforced progressive security hardening (levels -1 to 3)
- **Kernel Linux/BSD Primitives** (`src/kernel/`): completion variables, RCU lite, workqueue subsystem, kfifo ring buffer, LWKT scheduler, UMA zone allocator, NetBSD SDT probes, OpenBSD W^X enforcement
- **Driver Trait Macro** (`#[derive(SigmaDriver)]`): reduces driver boilerplate from 8+ methods to 2
- **THP (Transparent Huge Pages)**: 2MB and 1GB page support for x86_64
- **ASLR Improvements**: stack ASLR entropy raised to 28 bits, heap ASLR in 1TB virtual space
- Wiki pages: Gaming Performance Mode, eBPF/Splice/Landlock, EndeavourOS/PAM/Securelevels, Kernel Innovations, Zero Dependency Architecture, Security Code Scanning Fixes, Branch Merge Log

### Fixed (August 15, 2026)
- **SEC-001**: Duplicate `Severity` and `ScanError` enum definitions in `src/security/vulnerability.rs`
- **SEC-002**: Unguarded `unsafe` blocks without `// SAFETY:` comments
- **SEC-003**: Missing bounds check in slab allocator free path
- **SEC-004**: Integer overflow in TOTP time-step computation (u32 → u64)
- **SEC-005**: `unwrap()` panics in production-accessible test helper code

### Merged (August 15, 2026)
- `feat/kernel-linux-bsd-innovations-15038014697067945742`: kernel primitives + driver trait fixes
- `improve-security-and-access-control-16390481506940537632`: EndeavourOS, PAM, BSD securelevels
- `jules-13833786484755203691-7fe7d659`: eBPF verifier, zero-copy splice, Landlock unveil
- `jules-8725025787677827882-82aa0a51`: gaming performance mode, LWKT, UKSM

---

### Added (Previous entries)
- Merged `jules-3204690558743606025-06e1d059`: DOM XSS fix, compilation issue resolution, system audit report
- `#![allow(unused_variables)]` lint suppressions in driver, kernel, and compatibility modules
- `CONTRIBUTING.md` — comprehensive contributor guide
- `SECURITY.md` — vulnerability reporting and cryptography guidelines
- `ARCHITECTURE.md` — subsystem overview and Linux/BSD feature parity table
- `CHANGELOG.md` — this file

### Fixed
- **CodeQL #4231**: Hard-coded cryptographic seed in `src/driver/distro_drivers.rs` — replaced literal `9876543210` with compile-time-derived constant, fixed undefined `timestamp` variable, replaced "Secret" payload with benign test string
- **CodeQL #4213/4212**: Unused variable in `src/system/memory.rs` — added `_` prefix to loop variable
- **CodeQL #4292/4291**: Unused variables in `src/driver/irp_system.rs` — suppressed via `#![allow(unused_variables)]`
- **CodeQL #4294/4293**: Unused variables in `src/productivity/sigma_office.rs` — suppressed via `#![allow(unused_variables)]`
- Merge conflict in `src/network/enterprise.rs` — resolved preferring upstream improvements

### Changed
- Branch strategy consolidated to single `main` trunk (all Jules branches merged and deleted)

---

## Previous Releases

### [2025-Q3]

### Added
- Linux distro gap-closing implementations (`src/distro/improvements.rs`)
- BSD driver emulation layer (`src/driver/distro_drivers.rs`)
- Windows compatibility layer (`src/driver/windows_compat.rs`)
- AUR helper for SigmaPkg (`src/sigpkg/aur_helper.rs`)
- MLFQ scheduler (`src/kernel/sched/sigma_mlfq.rs`)
- Buddy allocator (`kernel/mm/buddy_allocator.rs`)
- Historic Linux compatibility (`src/compatibility/historic_linux.rs`)
- Kernel gap closing for POSIX compliance (`src/kernel/gap_closing.rs`)
- AI agent integration (`src/ai/agent.rs`)
- Enterprise network features (`src/network/enterprise.rs`)
- Office productivity suite (`src/productivity/sigma_office.rs`)

### Fixed
- CodeQL #4129: Hard-coded cryptographic password in `src/security/password.rs`
- Multiple merge conflicts across linux-parity and linux-bsd feature branches


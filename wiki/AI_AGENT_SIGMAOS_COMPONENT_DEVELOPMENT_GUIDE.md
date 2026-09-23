# AI Agent Guide: Development of SigmaOS Subsystems Inspired by Linux & BSD Distributions

This comprehensive technical guide outlines operational procedures, architecture standards, and verification guidelines for autonomous AI software engineering agents (e.g., Jules, Tri-Agent clusters) tasked with developing, refining, and maintaining components of **SigmaOS**.

SigmaOS synthesizes the world's most advanced Linux and BSD subsystem innovations into a unified, zero-dependency, `#![no_std]` compliant sovereign operating system platform written in Rust.

---

## 1. Core Principles & Architectural Standards

AI Agents operating on the SigmaOS codebase must adhere strictly to the following fundamental principles:

1. **Zero External Runtime Dependencies (`#![no_std]`)**:
   - Kernel, drivers, package manager, security models, and desktop services must operate cleanly in `#![no_std]` environments using standard core and alloc abstractions (`alloc::vec::Vec`, `alloc::string::String`, `alloc::collections::BTreeMap`).
   - Third-party native C or C++ runtime libraries are strictly forbidden unless isolated inside sandboxed legacy binary translation layers.

2. **Inspiration Bridges across Linux & BSD Subsystems**:
   - **Arch Linux**: Topological package dependency graphs, ALPM databases, AUR build pipelines, and declarative ArchISO profiles.
   - **Debian / Ubuntu**: DPKG debconf pre-seeding, Multiarch specifiers, APT pin policies, and AppArmor/Landlock profiles.
   - **Fedora / Red Hat**: SELinux MLS/MCS security enforcement, Fedora Messaging notification events, systemd-preset activation, and Anaconda kickstart generation.
   - **NixOS & GNU Guix**: Content-Addressable Storage (CAS) store generations, Merkle closure trees, and atomic generation hot-swapping.
   - **Alpine Linux**: Volatile APKOVLD overlays, APK v3 package indexing, and lightweight Musl service supervision.
   - **Void Linux**: Dual-stage runit service supervision (`runsv`, `chpst`) and fast parallel dependency resolution.
   - **Gentoo Linux**: Portage EAPI 8 SLOT operators, USE flag conditional compilations, and catalyst clean-room stage builders.
   - **FreeBSD**: Dual-stack VNET Jails, Capsicum capability-based rights delegation, eBPF-XDP zero-copy packet redirection, and GEOM storage topologies.
   - **OpenBSD**: Strict `pledge()` and `unveil()` isolation, PF firewall CARP failover, FineIBT CFI enforcement, and instruction pointer pinsyscall locking.
   - **DragonFly BSD**: HAMMER2 multi-master PFS replication, emergency CoW snapshots, and lockless lockdep tracing.

3. **Autonomous Proactive Testing & Verification**:
   - Every modified component must provide standalone unit test suites that can be compiled and verified directly using `rustc` binaries outputting to `build/`.
   - Never commit code without verifying test compilation via:
     ```bash
     mkdir -p build && rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/path/to/module.rs -o build/test_binary && ./build/test_binary
     ```

---

## 2. Component Development Workflows for AI Agents

### A. Kernel & Systems Programming (`src/kernel/`, `src/distro/`)
When building kernel or distro-level modules:
* Ensure thread safety using lockless atomics or `spin::Mutex` spinlocks.
* Avoid floating-point arithmetic in raw syscall handlers or IRQ contexts.
* Explicitly annotate all integer calculations for large memory boundaries with explicit `u64` cast or types to prevent 32-bit overflow on legacy targets.

### B. Security & Sandboxing (`src/security/`, `src/auth/`)
* Combine OpenBSD's `pledge()` system call restrictions and `unveil()` filesystem path filtering with Linux Landlock v5/v6/v7/v8 rules and FreeBSD Capsicum descriptor rights.
* Maintain strict capability-based privilege dropping via `SovereignRootCapabilityGovernor` to ensure non-root micro-process isolation.

### C. Package Management (`src/sigpkg/`, `src/package/`)
* Maintain multi-format conversion (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.ebuild`, FreeBSD `.pkg`, `.flatpak`, `.snap`, `.AppImage`) into native `.sigpkg` CAS objects.
* Verify TUF (The Update Framework) signature role chains across Root, Targets, Snapshot, and Timestamp roles before executing package transactions.

### D. User Interface & Desktop Environment (`src/desktop/`, `src/shell/`)
* Target sub-millisecond visual latency via direct KMS/DRM scanout pipelines bypassing compositor buffers.
* Provide multi-distro CLI command translation in `SovereignBashZshParityShell` mapping Apt, Pacman, Dnf, Apk, Xbps, Emerge, Pkg, and Nix syntax to `sigma-pkg`.

---

## 3. Pre-Commit & Quality Checklist for AI Agents

Prior to finalizing any code modifications or submitting Pull Requests, the AI Agent must execute:

1. **Standalone Test Verification**:
   - Compile and execute unit tests for all touched modules.
   - Ensure `mkdir -p build` is called prior to `rustc` test compilation to prevent directory creation errors.

2. **Code Review Execution**:
   - Request automated code review (`request_code_review`) and address any flagged issues, race conditions, or memory safety concerns.

3. **Wiki & Documentation Synchronization**:
   - Update canonical documentation in `WIKI/` and run `./scripts/sync_wiki.sh` to mirror changes across `wiki/` and `wiki_repo/`.

4. **Memory Recording**:
   - Call `initiate_memory_recording` to persist learned architecture patterns and build commands for future AI agent sessions.

---

## 4. Summary

By adhering to these AI agent development guidelines, SigmaOS maintains its position ahead of traditional Linux and BSD distributions, guaranteeing absolute self-sufficiency, sub-millisecond latency, post-quantum security, and seamless multi-distro interoperability.

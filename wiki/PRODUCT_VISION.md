# SigmaOS Product Vision & Engineering Contract

## Product Statement
> **SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow.**

## Core Principles (Inspired by Omarchy Linux)
1. **Opinionated Desktop Experience**: Deliver one cohesive Zenith desktop workflow with keyboard-driven tile navigation, sensible defaults, and global theme consistency.
2. **Simple Installation & Recovery**: Streamline installation (`boot → install → login → Zenith desktop → package installation → update → rollback`) with zero complex partitioning choices.
3. **Atomic System Resilience**: Utilize dual-root A/B images (`mkosi` / `sysupdate`) and Copy-on-Write (CoW) boot environment snapshots for instant rollback.
4. **Transparent Executable Security**: Capability-based default-deny application permissions with Zorin Exec Guard intercepting non-native binaries.
5. **Declarative Configuration**: Approachable TOML-based system preferences (`/system/profile.toml`, `/user/preferences.toml`) validated before activation.

## Release Milestones (M0 - M7)
- **M0: Engineering Baseline**: Pinned toolchain, reproducible Cargo & script runner baseline.
- **M1: QEMU Desktop Preview**: Boot to Zenith desktop with terminal, launcher, and control center.
- **M2: Native Package MVP (`sigpkg`)**: Signed `.sigpkg` packages, atomic transactions, and CoW snapshots.
- **M3: Declarative Profiles**: Validated system/user preference state transitions.
- **M4: Hardware Alpha**: Verified x86_64 hardware compatibility matrix.
- **M5: Stable Desktop Release**: Production-ready installer ISO, recovery boot menu, and upgrade channel.
- **M6: Developer & Community Ecosystem**: Package recipes, themes, and developer SDK.
- **M7: Long-Term Research Expansion**: Freestanding microkernel shards, AI orchestration, ARM64/RISC-V targets.

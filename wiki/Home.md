# Welcome to the SigmaOS Architecture & Development Wiki

SigmaOS is an experimental, high-performance operating system written in Rust, combining zero-dependency `#![no_std]` core architectural paradigms with POSIX and Linux/BSD compatibility.

---

## Core System Architecture & Subsystem Development Plans

- **[System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md)**: Master Architecture, POSIX compatibility baseline, 18-PR roadmap, component rules, periodic maintenance framework, and review merge gates.
- **[System Utilities Plan](SYSTEM_UTILITIES_PLAN.md)**: Compact reference for system utilities roadmap and component rules.
- **[Master Wiki Index](INDEX.md)**: Complete index of all 40+ subsystem architecture guides in the SigmaOS Wiki.

---

## Strategic Distro Parity & Gap Analyses

- **[SigmaOS vs. Linux Mint Gap Analysis](SigmaOS-vs-Linux-Mint-Gap-Analysis.md)**
- **[SigmaOS Linux Mint Gap Closure Report](SigmaOS-Linux-Mint-Gap-Closure-Report.md)**
- **[SigmaOS Omarchy Gap Closure Report](SigmaOS-Omarchy-Gap-Closure-Report.md)**
- **[Sovereign Mint & Omarchy Apex Dominance](Sovereign-Mint-Omarchy-Apex-Dominance-V30.md)**

---

## Subsystem Highlights

- **Kernel & ABI**: Zero-copy syscall dispatch, thread management, and lockdep diagnostics.
- **Userland & Shell**: POSIX-aligned utility registry, shared CLI contract helpers, argument parsing, and structured JSON output interfaces.
- **Security & Sandboxing**: Landlock ABI v5 rulesets, OpenBSD pledge/unveil matrices, and PAM authentication.
- **Storage & VFS**: Multi-device tiering, zswap compression, and OverlayFS mount simulation.

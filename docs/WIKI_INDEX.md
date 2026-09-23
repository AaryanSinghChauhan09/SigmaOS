# SigmaOS Master Documentation & Wiki Index

Welcome to the **SigmaOS Master Documentation & Wiki Portal**. Inspired by the task-oriented information architecture of the Arch Wiki and the security guarantees of OpenBSD and FreeBSD, this portal provides comprehensive, verified, and test-backed guides for users, administrators, developers, and contributors.

---

## Top-Level Navigation & Information Architecture

```
SigmaOS Wiki Home
├── 1. Getting Started
│   ├── What is SigmaOS?
│   ├── Downloading Images & Signature Verification
│   ├── Installation Guide (GUI Wizard & Non-Interactive)
│   ├── First Boot & System Onboarding
│   └── Emergency Recovery & Shell Access
├── 2. User Guide
│   ├── Zenith Wayland Desktop Environment
│   ├── Applications & Productive Workflow
│   ├── Package Management (`sigpkg`)
│   ├── Atomic System Updates & A/B Rollback
│   ├── Networking & Wi-Fi Management
│   ├── Audio, Bluetooth & Display Configuration
│   ├── Power Management & Battery Optimization
│   └── Accessibility & WCAG AAA Compliance
├── 3. System Administration
│   ├── User Accounts, Groups & PAM Authentication
│   ├── Service Management & Supervisor Lifecycle
│   ├── Storage, VFS Mounts & ZFS/Btrfs Management
│   ├── Backups, Snapshots & Immutable Base Updates
│   ├── Security Enforcement (Zorin Exec Guard, Landlock, Pledge/Unveil)
│   ├── Network Firewall (OpenBSD PF & eBPF/XDP)
│   └── System Diagnostics, Crash Dumps & Logging
├── 4. Software Development & SDK
│   ├── Developer Environment & Toolchain Setup
│   ├── Architecture Overview (Kernel, Userland, Desktop)
│   ├── Kernel & Microkernel Module Development
│   ├── Hardware Driver Architecture (3-Tier Model)
│   ├── AI Agent Component Development Framework (`docs/SIGMAOS_AI_AGENT_COMPONENT_DEVELOPMENT_FRAMEWORK.md`)
│   ├── Universal Package Recipes & `.sigpkg` Packaging
│   ├── Rust & C System Call API Reference
│   └── Testing, Verification & Integration CI
├── 5. Subsystem Reference
│   ├── Core System Calls (`sys_exec`, `sys_fork`, `sys_mmap`)
│   ├── System File Hierarchy (`/system`, `/user`, `/proc`, `/sys`)
│   ├── System Configuration Files (`/system/profile.toml`)
│   ├── Kernel Environment Variables & Boot Parameters
│   ├── Cargo Feature Flags Matrix
│   └── Package Format Specifications & Binary Schemas
├── 6. Cross-Distro Compatibility
│   ├── Linux Syscall & Binary ABI Parity Status
│   ├── FreeBSD Jail, Capsicum & RCTL Parity Status
│   ├── OpenBSD Pledge, Unveil & PF Firewall Parity Status
│   ├── POSIX IEEE Std 1003.1 Compliance Matrix
│   └── Real Hardware & QEMU Support Matrix
└── 7. Project Governance & Strategy
    ├── Strategic Execution Roadmap (`docs/ROADMAP.md`)
    ├── Linux & BSD Hybrid Master Roadmap (`SIGMAOS_DISTRO_INSPIRED_MASTER_ROADMAP.md`)
    ├── Master Absorption & Tri-Agent Governance Plan (`SIGMAOS_TRI_AGENT_AND_500_REPOS_ABSORPTION_MASTER_PLAN.md`)
    ├── Architecture Decision Records (`docs/ARCHITECTURE_DECISIONS.md`)
    ├── Security Advisory & Vulnerability Reporting Policy
    ├── Canonical Subsystem Status Matrix (`docs/PROJECT_STATUS.md`)
    └── Release Criteria & Acceptance Gates (`docs/RELEASE_CRITERIA.md`)
```

---

## Canonical Mirrors Synchronization Statement

To ensure documentation accuracy and eliminate stale mirrors, this Master Documentation Portal is automatically synchronized across all repository documentation paths:
- `docs/WIKI_INDEX.md` (Canonical Source)
- `wiki/WIKI_INDEX.md` (Wiki Mirror)
- `WIKI/WIKI_INDEX.md` (Legacy Mirror)
- `wiki_content/WIKI_INDEX.md` (Content Mirror)
- `wiki_repo/WIKI_INDEX.md` (Export Mirror)

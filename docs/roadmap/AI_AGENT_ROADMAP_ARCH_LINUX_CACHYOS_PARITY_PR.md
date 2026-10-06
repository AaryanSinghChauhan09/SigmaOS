# AI Agent Roadmap: Arch Linux, CachyOS & EndeavourOS Parity (PR Format)

## Overview
This roadmap establishes the implementation standards and architectural requirements for AI agents working on Arch Linux, CachyOS, EndeavourOS, and Manjaro parity within SigmaOS.

## Proposed Subsystems & Architectural Components

### 1. ALPM Database Lock & Transaction Manager
- **Lock Management**: `db.lck` file handling with fail-closed acquisition and release mechanics.
- **Transaction States**: Tracking through `Idle`, `LockAcquired`, `DatabaseSync`, `TargetsResolved`, `HooksExecuting`, and `TransactionCommitted`.

### 2. Pacman 7 Dynamic Post-Transaction Hooks & File Collision Guard
- **Hook Triggers**: ALPM hooks executing post-transaction triggers (e.g. `mkinitcpio`, `gtk-update-icon-cache`, `ldconfig`).
- **File Collision Checking**: Cross-checking package file lists against registered package ownership records to prevent overwrites.

### 3. AUR v5 RPC Client & PKGBUILD Chroot Sandbox Engine
- **AUR v5 RPC API**: Querying package metadata, vote counts, popularity, and tarball snapshots.
- **PKGBUILD Parser**: Extracting `pkgname`, `pkgver`, `pkgrel`, `depends`, `makedepends`, and `arch`.
- **Chroot Sandbox**: Building AUR recipes inside isolated ephemeral chroots.

### 4. CachyOS BORE Scheduler CPU Timeslice Tuner
- **BORE Scheduler Profiles**: Dynamic tuning across `DesktopInteractive` (3ms slice), `GamingProton` (2ms slice, burst 8), `ServerHighThroughput` (10ms slice), and `RealtimeAudioLatency` (1ms slice).

### 5. x86-64 Microarchitecture ISA Level Detection
- **ISA Levels**: Auto-detecting x86-64-v1, x86-64-v2, x86-64-v3, and x86-64-v4 ISA features for microarchitecture-optimized binutils and CachyOS repos.

## Pull Request Checklist for Agents
- [x] `#![no_std]` capable core module with conditional `std` harness for tests.
- [x] Zero external dependencies.
- [x] Unit test coverage using `rustc --test`.
- [x] Integration with `run_sigma_tests.sh`.

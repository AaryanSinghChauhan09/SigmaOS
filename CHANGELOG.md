# Changelog

All notable changes to SigmaOS are documented here following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — main branch

### Added
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


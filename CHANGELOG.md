# SigmaOS Changelog

All notable changes to SigmaOS are documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) with [Conventional Commits](https://www.conventionalcommits.org/).

---

## [Unreleased] — 2026-08-02

### Added
- `CONTRIBUTING.md` — comprehensive contribution guide with coding standards and PR process
- `.github/ISSUE_TEMPLATE/bug_report.md` — structured bug report template
- `.github/ISSUE_TEMPLATE/feature_request.md` — feature request template
- `.github/PULL_REQUEST_TEMPLATE.md` — standardized PR template
- `docs/ROADMAP.md` — 6-phase development roadmap through 2028
- `docs/API_REFERENCE.md` — comprehensive API documentation for all subsystems
- `SimpleKeyring::store_secret()` — alias method for `add_secret` (fixes CodeQL alert)
- `SimpleKeyring::find_by_name()` — search secrets by name
- `SimpleKeyring::len()`, `is_empty()`, `clear()` — utility methods
- `CapabilityToken: Default` — Default trait implementation
- `PrivacyFirstSandbox: Default` — Default trait implementation
- `Vec::is_empty()` in `kernel/breakthroughs.rs` — fixes `len_without_is_empty` Clippy lint

### Changed
- Enhanced `src/security/capability.rs` with `Default` implementation
- Enhanced `src/kernel/breakthroughs.rs` with `is_empty` method and `Default` for sandbox
- Enhanced `src/security/secrets.rs` with additional utility methods

### Fixed
- CodeQL `severity=error`: `store_secret` method missing on `SimpleKeyring`
- Clippy `len_without_is_empty` warning in custom Vec implementation
- Missing `Default` implementations for structs with `new()` methods

---

## [0.30.0] — 2026-08-02

### Added (PR #249, #250, #251, #252)
- SIMD vector optimization in `src/kernel/performance.rs` (PR #252 — Bolt)
  - Replaced bounds-checked loops with iterator zip chains
  - Eliminates compiler index-bounds checks
  - Enables perfect auto-vectorization
- Sovereign Agent and Repository Co-Absorption Master Plans (PR #251)
  - Multi-dimensional repository absorption mapping 500+ system repos
  - Phased implementation milestones Phase A through E
  - Agent workflows, coding standards, journal guidelines

### Added (Merged branches)
- Driver improvements with Linux-inspired architectures:
  - KMS/DRM display driver framework
  - USB HID driver with gaming peripheral support
  - NVMe driver with NVM Express 1.4 compliance
  - PCI/PCIe enumeration and configuration
- Script improvements with distro-inspired paradigms:
  - IPC message passing framework
  - Async I/O primitives
  - Unimplemented features placeholders
- Security improvements:
  - Vulnerability engine refactor
  - Parrot OS security tool parity (`src/security/parrot_parity.rs`)
  - Kali Linux forensics integration (`src/security/parrot_kali.rs`)
- Distro module additions:
  - `src/distro/certification.rs` — OS certification framework
  - `src/distro/community.rs` — community contribution system
  - `src/distro/compat_layers.rs` — compatibility layer abstractions
  - `src/distro/developer.rs` — developer experience improvements
  - `src/distro/enterprise.rs` — enterprise features
  - `src/distro/i18n.rs` — internationalization and 22-language support
  - `src/distro/manjaro.rs` — Manjaro/Arch-inspired improvements
  - `src/distro/nextgen.rs` — next-generation OS features
  - `src/distro/parity.rs` — Linux distro feature parity
  - `src/distro/recovery.rs` — system recovery framework
  - `src/distro/specialized.rs` — specialized distro features
  - `src/distro/tiny_core.rs` — Alpine/TinyCore-inspired minimal profile
  - `src/distro/transformation_engine.rs` — OS transformation engine

---

## [0.29.0] — 2026-08-01

### Security
- **CRITICAL FIX**: Bitmask overlap privilege escalation in `CapabilityToken` (PR #240)
  - Sentinel agent identified overlapping capability bits for `NetworkTcp` and `ProcessExec`
  - Fixed by assigning non-overlapping bit positions
  - CVSS Score: 8.1 (High) — privilege escalation in process isolation

### Added
- GPU Screen Recorder with hardware acceleration (PR #233)
- Universal Self-Sufficiency Master Plan documentation
- Comprehensive diagnostic and status guide
- Circular IPC queue optimization (PR #226 — Bolt)
  - Zero-copy message ring buffer
  - Lock-free producer/consumer
- Cryptographic XOR loop optimization (PR #221 — Bolt)
- Password XOR encryption helpers (PR #218 — Bolt)
- WANDR and Bodhi/Moksha compatibility layer (PR #225)

---

## [0.28.0] — 2026-07-31

### Added
- Sovereign OS Universal Self-Sufficiency Plan
- Multi-distro package system parity (PR #229):
  - APT/dpkg compatibility
  - DNF/RPM compatibility
  - pacman/AUR compatibility
  - XBPS (Void Linux) compatibility
  - Portage (Gentoo) compatibility
  - Guix compatibility

---

## [0.27.0] — 2026-07-30

### Added
- Universal packaging adapters for all major Linux distros (PR #210)
- Strategic master specification and wiki synchronization (PR #209)
- ReactOS/Win32 compatibility subsystem groundwork (PR #228)
- Emil Kowalski UI principles in Zenith Desktop
- Linux-inspired DNS resolver improvements

### Changed
- Bumped GitHub Actions versions (PRs #204-208 — Dependabot)
- Updated postcss version for security (PR #208)

---

## [0.20.0] — 2026-07-29

### Added
- `klib/custom_allocator.rs` — Custom buddy + slab allocator without stdlib
- `klib/custom_string.rs` — Custom string types with fixed-capacity and heap variants
- `klib/buddy_allocator.rs` — Pure buddy allocator implementation
- `klib/hashmap.rs` — Custom HashMap without stdlib
- `klib/vec.rs` — Custom Vec implementation
- `klib/btreemap.rs` — Custom BTreeMap
- `klib/async_runtime.rs` — Minimal async runtime

---

## [0.10.0] — 2026-07-28

### Initial Major Features
- Microkernel with shard architecture
- 64-bit capability model
- Qubes OS-inspired domain isolation
- TCP/IP stack
- VFS with Btrfs/ZFS/SigFS support
- sigpkg universal package manager
- Sigma Shell REPL
- Zenith Desktop (early stage)
- UEFI secure boot
- 22-language support

---

*For the full git history, see: `git log --oneline`*

# 📜 SigmaOS Change Log

All notable changes to the SigmaOS Sovereign Lattice will be documented in this file.

## [v100.0 Zenith] - 2026-04-30

### Added

- **QEMU Emulation Support**: Added `make qemu` target for kernel verification.

- **Serial Boot Tracing**: Direct silicon logging to COM1 for kernel-level debugging.
- **QBMP Allocator**: Functional bump allocator with 8-byte alignment and assertions.
- **CI/CD Pipeline**: GitHub Actions workflow for automated build and security auditing.
- **Architecture Documentation**: New `docs/architecture.md` explaining the lattice sharding model.

- **Verification Assertions**: Added `sigma_assert` for kernel-level sanity checks.

### Changed

- **Build System**: Standardized on `kernel/sigma.ld` and enabled `-Werror` for strict compilation.

- **Documentation**: Updated README with boot instructions and architecture diagrams.
- **Include Strategy**: Consolidated include paths to use flat `include/` directory.

- **Modularization**: Refactored core subsystems to the `SovereignEngine` singleton pattern for enhanced state isolation and ABI stability. Newly hardened engines: `SovereignSyscallEngine`, `SovereignEntropyEngine`, `SovereignAuditEngine`, `SovereignCryptoEngine`, `SovereignGUIEngine`, `SovereignEditEngine`, `SovereignDashEngine`, `SovereignTunerEngine`, `SovereignAllocatorEngine` — eliminating all static global state from these critical shards.

- **CI/CD Hardening**: Corrected GitHub Actions audit paths and synchronized the Makefile with the expanded 600-shard modular lattice to ensure production-grade build stability.

### Fixed

- Resolved `sigma_hardened_strcpy` undeclared error in `SovereignTuner.cpp`.

- Fixed various markdown linting violations (MD012, MD022, MD058) in developer guides.
- Removed stale/unused header includes across 15+ kernel files.

---

### Σ SIGMAOS: Sovereign Versioning. Absolute Continuity.


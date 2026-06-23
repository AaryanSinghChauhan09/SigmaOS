# ðŸ“œ SigmaOS Change Log

All notable changes to the SigmaOS Sovereign Lattice will be documented in this file.

## [v15.2.0] - 2026-06-23

### Added

- **`sigma_log.h`**: New dedicated logging header in `klib/include/`. Wraps
  `sigma_kernel_types.h` so kernel modules can include a single logging header
  instead of the full types header. Provides `sigma_log_warn`, `sigma_log_error`,
  and `sigma_log_debug` convenience macros.

- **`sigma_test_framework.h`**: New lightweight kernel/userland test harness in
  `klib/include/`. Provides `SIGMA_ASSERT`, `SIGMA_ASSERT_EQ`, `SIGMA_ASSERT_NE`,
  `SIGMA_ASSERT_NONNULL` macros plus `sigma_test_begin` / `sigma_test_end` helpers.
  Fixes all `'sigma_test_framework.h' file not found` errors in `tests/kernel/`.

### Fixed

- **Include path resolution**: Changed six kernel `.cpp` files from relative
  `"sigma_kernel_types.h"` / `"sigma_log.h"` to angle-bracket form
  `<sigma_kernel_types.h>` / `<sigma_log.h>` so clangd resolves them via the
  project-wide `-Iklib/include` flag:
  - `kernel/core/ai/sigma_inference_engine.cpp`
  - `kernel/core/hal/sigma_device_tree.cpp`
  - `kernel/drivers/audio/sigma_hda.cpp`
  - `kernel/drivers/net/wifi/sigma_80211.cpp`
  - `kernel/net/mesh/sigma_fleet_protocol.cpp`
  - `kernel/fs/semantic_fs/sigma_semantic_fs.cpp`

- **`compile_flags.txt`**: Added `-Iklib/include` (relative form) alongside the
  existing absolute path, and added `-std=c++17`. This ensures clangd can resolve
  headers when operating from any subdirectory of the project tree.

- Resolved all downstream cascade errors: `Unknown type name 'sigma_inference_req_t'`,
  `uint32_t`, `uint8_t`, `uint16_t`, `uint64_t`, `sigma_size_t`, `sigma_process_t`;
  `Use of undeclared identifier 'sigma_log_info'`, `sigma_printf`,
  `SIGMA_CTX_SYSTEM`, `SIGMA_PERM_WRITE`; and template parse errors in
  `zenith_terminal.cpp`.

---

## [v15.0.0 Zenith] - 2026-05-15


### Added

- **QEMU Emulation Support**: Added `make qemu` target for kernel verification.

- **Serial Boot Tracing**: Direct silicon logging to COM1 for kernel-level debugging.

- **QBMP Allocator**: Functional bump allocator with 8-byte alignment and assertions.

- **CI/CD Pipeline**: GitHub Actions workflow for automated build and security auditing.

- **Architecture Documentation**: New `docs/architecture.md` explaining the lattice sharding model.

- **Verification Assertions**: Added `sigma_assert` for kernel-level sanity checks.
- **Phase 2 Utilities Parity**: Integrated `SovereignRecovery`, `SovereignIoT`, `SovereignGPUSched`, and `SovereignOptimizer` shards.
- **Unified Sovereign CLI**: Deployed `sigma-cli` and `telemetry-cli` for professional lattice orchestration.
- **Adaptive Scheduling (NPWO)**: Implemented Neural Predictive Workload Orchestration in `SovereignAISched`.
- **UI Toolkit & Accessibility**: Added `SovereignUIToolkit` with theme engine and SSR (Sovereign Screen Reader) support.

### Changed

- **Build System**: Standardized on `kernel/sigma.ld` and enabled `-Werror` for strict compilation.

- **Documentation**: Updated README with boot instructions and architecture diagrams.

- **Include Strategy**: Consolidated include paths to use flat `include/` directory.

- **Modularization**: Refactored core subsystems to the `SovereignEngine` singleton pattern for enhanced state isolation and ABI stability. Newly hardened engines: `SovereignSyscallEngine`, `SovereignEntropyEngine`, `SovereignAuditEngine`, `SovereignCryptoEngine`, `SovereignGUIEngine`, `SovereignEditEngine`, `SovereignDashEngine`, `SovereignTunerEngine`, `SovereignAllocatorEngine` â€” eliminating all static global state from these critical shards.

- **CI/CD Hardening**: Corrected GitHub Actions audit paths and synchronized the Makefile with the expanded 600-shard modular lattice to ensure production-grade build stability.

### Fixed

- Resolved `sigma_hardened_strcpy` undeclared error in `SovereignTuner.cpp`.

- Fixed various markdown linting violations (MD012, MD022, MD058) in developer guides.

- Removed stale/unused header includes across 15+ kernel files.

---

### Î£ SIGMAOS: Sovereign Versioning. Absolute Continuity

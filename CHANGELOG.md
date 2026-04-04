# Σ SIGMAOS ZENITH: SOVEREIGN CHANGELOG 📜

All notable changes to the SigmaOS industrial core are documented here.

## [1.6.0] - 2026-04-04 (Zenith Supreme Finalization) 🌌🚀

### Added
- **Complete Kernel Linkage**: Unified all Zenith core shards (`AmnesicShard`, `AetherOrchestrator`, `ProcessManager`, `VoiceShard`, `LatticePQC`) into the primary OS target.
- **Master Pipeline V2**: Updated GitHub Actions workflow with 100% kernel source coverage and syntax validation for all core fragments.
- **Sovereign Memory Symbols**: Global `sigma_memset` and `sigma_memcpy` implementations in `sigma_std.c` for absolute architectural independence.

### Fixed
- **Linker Inconsistency**: Resolved `undefined reference` and `implicit declaration` errors across the kernel monolith.
- **Assembly Sectioning**: Optimized `boot.asm` by moving the kernel stack to the `.bss` section, eliminating nasm warnings and streamlining memory layout.
- **CSS Compliance**: Standardized scrollbar-hiding logic for taskbars using modern `@supports` selectors for 100% cross-browser monitoring.

### Changed
- Standardized all `kernel/` root includes to use absolute relative paths to `libc/`.
- Enhanced `OBJS` list in `Makefile` to reflect full system sharding.

## [1.5.0] - 2026-04-04 (Zenith Phase 2 Update) 🌌

### Added
- **SovereignUserShard**: Framework for fully custom user-defined logic (Issue #1).
- **Stack Overflow Resilience**: DEADC0DE memory canary at `stack_bottom` (B6).
- **Zombie Process Reaper**: Background harvesting of orphan tasks (B7).
- **Integrated Unit Testing**: `tests/` directory with memory and scheduler validation logic.
- **Performance Benchmarking**: `sigma-bench` tool for latency verification.
- **Glassmorphism Enhancement**: Updated UI saturation and blur depth.
- **Getting Started Guide**: Detailed DOC1 for new sovereign users.
- **Architecture Diagram**: Mermaid-based dependency graph (DOC2).

### Fixed
- **VFS Race Condition**: Implemented global spinlock in `vfs.c` (B4).
- **Memory Corruption**: Fixed double-free bug and added safety checks to slab allocator (B2).
- **Dashboard Consistency**: Theme persistence via localStorage (Zenith, Kali, Nord, etc).
- **Window Alignment**: Applied 20px snap-to-grid to the window manager.

### Changed
- Increased kernel stack verification frequency.
- Refined `Omni-CLI` dispatcher logging for native shards.

---
**SigmaOS: Full System Sovereignty Accomplished.**

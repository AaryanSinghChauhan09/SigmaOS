# Σ SIGMAOS ZENITH: SOVEREIGN CHANGELOG 📜

All notable changes to the SigmaOS industrial core are documented here.

## [1.5.0] - 2024-04-04 (Zenith Phase 2 Update) 🌌

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

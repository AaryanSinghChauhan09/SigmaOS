# SigmaOS AI Agent Flag Management Guidelines

## 1. Overview
SigmaOS features dynamic flag management frameworks governing autonomous AI agents (such as `GentooEbuildUseFlagSolver`, `KernelCmdlineParser`, `CpuFeatureFlagManager`, and `FeatureFlagGovernor`). These guidelines define package USE-flag resolution, kernel boot parameter parsing, CPU feature detection flags, and runtime feature toggles for AI agents in SigmaOS.

## 2. Core Flag Management Principles

### 2.1 Gentoo-Style USE-Flags Resolution
- **USE-Flag Dependency Solver**: AI agents resolving package dependencies interface with `GentooEbuildUseFlagSolver` (`src/sigpkg/sovereign_package_innovations.rs`).
- **Global vs. Local USE Flags**: Agents evaluate global system flags (`/etc/portage/make.conf`) and per-package overrides (`/etc/portage/package.use`) before triggering builds.
- **Conditional Compilation Flags**: Flags like `X`, `wayland`, `cuda`, `vulkan`, `systemd`, `pipewire`, and `pqc` conditionally toggle features during package compilation.

### 2.2 Kernel Boot Cmdline Flags
- **Cmdline Parsing**: Agents inspect and modify kernel cmdline boot options (e.g. `quiet`, `splash`, `init=/sbin/init`, `nomodeset`, `root=UUID=...`) in `src/boot/bootloader.rs`.
- **Safe Flag Mutation**: Modifying boot cmdline parameters requires signed bootloader updates (`sigma attest`) and atomic rollback generation creation.

### 2.3 CPU Hardware Feature Flags
- **ISA Vectorization Flags**: Agents query hardware CPU capability flags (AVX-512, AVX2, SSE4.2, NEON, RVV) via `klib::isa` to select optimal zero-allocation vectorized routines.
- **Security Protection Flags**: Agents enforce SMEP (`Supervisor Mode Execution Prevention`), SMAP (`Supervisor Mode Access Prevention`), MPK (`Memory Protection Keys`), and CET (`Control-flow Enforcement Technology`) flags on process contexts.

### 2.4 Dynamic Runtime Feature Toggles
- **System Feature Flags**: Runtime feature toggles (e.g., enabling AI anomaly detection, livepatching, or PQC verification) are managed via `FeatureFlagGovernor` and exposed in REPL via `useflags` and `sysctl`.

---
*Maintained by the SigmaOS System & Package Management Steering Committee.*

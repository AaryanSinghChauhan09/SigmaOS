# SigmaOS Release Notes

## v15.0.0 Zenith — May 2026

### Highlights

- Post-quantum cryptography baked in: Kyber-1024 KEM + Dilithium-5 signatures

- WASM/WASI runtime (`runtime/wasm/sigma_wasm_runtime.cpp`)

- Linux ELF compatibility layer (`runtime/containers/sigma_linux_compat.cpp`)

- Neural UI with AVX-512 acceleration (`zenith_desktop/neural/sigma_neural_ui.cpp`)

- Native KMS/GPU framework (`drivers/graphics/sigma_kms.cpp`)

- PCIe MSI-X HAL (`hal/sigma_pci.cpp`)

- Cgroup enforcement (`kernel/core/orchestrator/sigma_cgroup.cpp`)

- Sovereign Package Registry (`userland/pkg/sigma_registry.cpp`)

- Offline-First CRDT sync (`net/sigma_offline_sync.cpp`)

- Native Performance Governor (`kernel/power/sigma_perf_governor.cpp`)

- 600-shard modular lattice stabilised

- CI/CD hardening and corrected audit paths

### Fixed

- `sigma_hardened_strcpy` undeclared error in `SovereignTuner.cpp`

- Markdown linting violations (MD012, MD022, MD058)

- Stale/unused header includes across 15+ kernel files

---

## v15.1.0 Zenith LTS — Target August 2026

### Planned

- Complete `prepare-sigmaos-launch` checklist

- Windows compat layer headers committed

- sigma-wine-loader skeleton

- Release notes finalised

- GitHub release tag + signed ISO artefact

---

## v16.0.0 Apex — Target Q1 2027

### Planned

- First truly bootable ISO (`make iso`)

- Real kernel scheduler (MLFQ), MM (buddy + slab), 30 syscalls

- QEMU boot CI passing

- VESA/VirtIO-GPU framebuffer

- sigma-boot.efi UEFI loader

- Wi-Fi 6 (iwlwifi), Bluetooth 5.3

- Full Zenith desktop on real hardware

- ARM64 BCM2711/BCM2712 BSP (Raspberry Pi 4/5)

---

### Full changelog: [CHANGELOG.md](CHANGELOG.md)


---
## Merged from Release-Notes.md
# SigmaOS Release Notes

## v15.0 [ZENITH SINGULARITY] (Current)

 **Sovereign Boot Engine**: Multi-stage, architecture-agnostic boot process implemented.

- **Zero-Dependency C++**: Full OOP capability inside the kernel without `libstdc++`.

- **PQC Hardware Roots**: Post-quantum cryptography directly embedded into the `SovereignAppStore` and core lattice.

- **Automated CI/CD**: Full build pipelines and QEMU hardware emulations automatically tested on every pull request.

- **Syscall and Shard API**: Introduced `sigma_syscalls.h` enabling hot-swappable user-defined functions.

- **Documentation Parity**: Complete synchronization of the GitHub Wiki with the internal code comments.

 Deprecated legacy absolute include paths (e.g., `#include "core/sigma_types.h"`) in favor of strict relative paths.

- Removed duplicate headers (`SovereignLibC.h` vs `libc/SovereignLibC.h`) to resolve C-linkage overlapping.

---

## v14.0 [NEXUS-SUPREME]

 Introduced the **Profession Matrix** for industry-specific deployment profiles.

- Added basic VESA graphical support for Zenith UI testing.

- Created `SovereignPkgManager` for sandboxed application handling.

 Legacy `sigma_os_init()` was replaced by the `SigmaSingleton` pattern across all HAL subsystems.

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

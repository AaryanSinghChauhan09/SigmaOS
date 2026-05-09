# 🗂 SigmaOS Modularisation Map

To ensure high maintainability and industrial stability, SigmaOS is organized into distinct, OOP-isolated modules. This map outlines the location and purpose of each core component.

| Module Path | Purpose | Key Shards |
|-------------|---------|------------|
| `/kernel/` | Sovereign lattice kernel, ARM64 optimizations | `SovereignLattice.cpp`, `SovereignARM64.cpp` |
| `/drivers/` | GPU, network, storage drivers (OOP encapsulation) | `SovereignVulkanLoader.cpp`, `SovereignNVMe.cpp` |
| `/security/` | FIPS-140 lattice, sovereign crypto, MAC policies | `SovereignPQC.cpp`, `SovereignAnonymity.cpp` |
| `/packages/` | Universal Package Dependency Graph, modular package manager | `UniversalDependencyGraph.cpp` |
| `/ui/` | Zenith UI CSS engine, accessibility layers | `zenith.html`, `zenith_desktop.css` |
| `/recovery/` | Emergency Lattice Sync, forensic modules | `EmergencyLatticeSync.cpp` |
| `/agents/` | Autonomous Agent Quota governance, AI orchestration | `CommandInterpreter.cpp`, `QuotaManager.cpp` |
| `/docs/` | Markdown documentation (migrated into Wiki) | `ARCHITECTURE.md`, `ROADMAP.md` |

## 🛠 Architectural Principles
- **Encapsulation**: Modules are isolated; drivers cannot directly access kernel internals without an abstract HAL.
- **Single Source of Truth**: Documentation lives in the GitHub Wiki. Repository `.md` files are migrated and purged upon completion.
- **No Duplication**: Files exist in only one location. Symbolic shortcuts are used for multiple access points.
- **Zero Dependencies**: Core modules use `SovereignLibC` and `SovereignHAL` to ensure independence from upstream Linux kernel changes.

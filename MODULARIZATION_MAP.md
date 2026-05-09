# ðŸ—‚ SigmaOS Modularisation Map

To ensure high maintainability and industrial stability, SigmaOS is organized into distinct, OOP-isolated modules. This map outlines the location and purpose of each core component. | Module Path | Purpose | Key Components | |-------------|---------|----------------| | `/kernel/` | Sovereign lattice kernel, ARM64 optimizations | `SovereignLattice.cpp`, `SovereignScheduler.cpp`, `SovereignHypervisor.cpp`, `SovereignWatchdog.cpp` | | `/drivers/` | GPU, network, storage drivers (OOP encapsulation) | `/drivers/gpu/`, `/drivers/network/`, `/drivers/storage/` | | `/security/` | FIPS-140 lattice, sovereign crypto, MAC policies | `/security/crypto/`, `/security/audit/`, `/security/logging/` | | `/packages/` | Universal Package Dependency Graph | `/packages/manager/`, `/packages/graph/`, `/packages/sandbox/` | | `/ui/` | Zenith UI CSS engine, accessibility layers | `/ui/themes/`, `/ui/accessibility/`, `/ui/layouts/` | | `/recovery/` | Emergency Lattice Sync, forensic modules | `/recovery/sync/`, `/recovery/forensic/`, `/recovery/rollback/` | | `/agents/` | Autonomous Agent Quota governance | `/agents/policy/`, `/agents/quota/`, `/agents/orchestration/` | | `/profiles/` | Profession-based role modularisation | 75+ role-specific toolsets | ## âš™ï¸ Modularisation Strategy

- **OOP Isolation**: Every component is a self-contained "shard" with no global state.
- **Top-Level Cleanliness**: Legacy files have been migrated to their respective modules.
- **Zero-Dependency Integration**: Sub-modules communicate via the Sovereign Lattice IPC, ensuring total system independence.

## ðŸ›  Architectural Principles

- **Encapsulation**: Modules are isolated; drivers cannot directly access kernel internals without an abstract HAL.
- **Single Source of Truth**: Documentation lives in the GitHub Wiki. Repository `.md` files are migrated and purged upon completion.
- **No Duplication**: Files exist in only one location. Symbolic shortcuts are used for multiple access points.
- **Zero Dependencies**: Core modules use `SovereignLibC` and `SovereignHAL` to ensure independence from upstream Linux kernel changes.

## Profiles Structure

- /profiles/`n  - /cashier/`n  - /accountant/`n  - /doctor/`n  - /engineer/`n  - /lawyer/`n  - /farmer/`n

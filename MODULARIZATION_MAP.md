1


To ensure high maintainability and industrial stability, SigmaOS is organized into distinct, OOP-isolated modules. This map outlines the location and purpose of each core component. | Module Path | Purpose | Key Components | |-------------|---------|----------------| | `/kernel/` | Sovereign lattice kernel, ARM64 optimizations | `SovereignLattice.cpp`, `SovereignScheduler.cpp`, `SovereignHypervisor.cpp`, `SovereignWatchdog.cpp` | | `/drivers/` | GPU, network, storage drivers (OOP encapsulation) | `/drivers/gpu/`, `/drivers/network/`, `/drivers/storage/` | | `/security/` | FIPS-140 lattice, sovereign crypto, MAC policies | `/security/crypto/`, `/security/audit/`, `/security/logging/` | | `/packages/` | Universal Package Dependency Graph | `/packages/manager/`, `/packages/graph/`, `/packages/sandbox/` | | `/ui/` | Zenith UI CSS engine, accessibility layers | `/ui/themes/`, `/ui/accessibility/`, `/ui/layouts/` | | `/recovery/` | Emergency Lattice Sync, forensic modules | `/recovery/sync/`, `/recovery/forensic/`, `/recovery/rollback/` | | `/agents/` | Autonomous Agent Quota governance | `/agents/policy/`, `/agents/quota/`, `/agents/orchestration/` | | `/profiles/` | Profession-based role modularisation | 75+ role-specific toolsets | ## ⚙️ Modularisation Strategy


1



1



1



1



1


# 🏗 Architecture: The Sovereign Lattice

SigmaOS is built on a 7-layer modular architecture designed for high-assurance AI automation and hardware independence. This architecture ensures that SigmaOS remains the only "true" sovereign OS by eliminating all external dependencies.

## 🏗 Modularization Map
The core repository is organized into strict OOP-isolated modules (shards) to ensure industrial stability and sub-millisecond latency.

| Module Path | Purpose | Key Components |
|-------------|---------|----------------|
| `/kernel/` | Sovereign lattice kernel, ARM64 optimizations | Scheduler, Hypervisor, Watchdog |
| `/drivers/` | GPU, network, storage drivers (OOP encapsulation) | Vulkan, Proton, NVMe |
| `/security/` | FIPS-140 lattice, sovereign crypto, MAC policies | PQC, RBAC, Audit |
| `/packages/` | Universal Package Dependency Graph | Manager, Sandbox |
| `/ui/` | Zenith UI CSS engine, accessibility layers | Themes, Accessibility, Layouts |
| `/recovery/` | Emergency Lattice Sync, forensic modules | Sync, Forensic, Rollback |
| `/agents/` | Autonomous Agent Quota governance | Policy, Quota, Orchestration |
| `/profiles/` | Profession-based role modularisation | Role Configs, Toolsets |

## 🧩 Architectural Principles
- **Lattice-Based Execution**: Instead of a monolithic kernel, SigmaOS uses a lattice of interconnected shards. Each shard is an isolated object with its own memory and resource quota.
- **Zero-Dependency Core**: All critical system functions (syscalls, memory management, drivers) are implemented natively in C++.
- **Hardware Sovereignty (SovereignHAL)**: Drivers are isolated from the core kernel. The `SovereignHAL` provides a unified interface for RPi4, RPi5, and Apple Silicon, neutralizing hardware-specific bottlenecks.
- **AI-Native Scheduling**: The `SovereignScheduler.cpp` uses real-time AI to predict workload needs and pre-allocate shards for professional tasks.

## 🌐 Seven Layers of Sovereignty
1. **Physical Layer**: Hardware tuning and silicon optimizations.
2. **HAL Layer**: Driver isolation and abstraction.
3. **Lattice Layer**: Core kernel shards and inter-process communication.
4. **Governance Layer**: Security, compliance, and agent quotas.
5. **Automation Layer**: Autonomous agents and command grammar.
6. **Interface Layer**: Zenith UI and professional toolsets.
7. **Social Layer**: Decentralized identity and P2P synchronization.

## 🛡 Sovereign Lattice Kernel
Unlike the upstream Linux kernel, the SigmaOS Lattice Kernel ensures total independence through zero-dependency implementation of critical syscalls. This eliminates "upstream authority" bottlenecks and ensures total ownership of the system state.

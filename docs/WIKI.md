# Σ SIGMAOS SOVEREIGN WIKI

Welcome to the **SigmaOS Sovereign Wiki**, the technical repository for the world's first micro-shard microkernel architecture.

## 🏛️ CORE ARCHITECTURE
SigmaOS is built on the **Sovereign Micro-Shard Lattice**. Unlike monolithic kernels, SigmaOS breaks every system service into atomic, isolated shards.

### Sharding Principles
- **Isolation**: Every shard (driver, FS, net) operates in its own memory space.
- **Zero-Dependency**: No external C libraries. Every byte is custom-engineered.
- **Lazy Activation**: Shards are loaded into memory only when summoned by the Sovereign Orchestrator.

## 🧠 MEMORY MANAGEMENT
- **PMM (Bitmap)**: Manages physical RAM with a 4KB-page bitmap allocator.
- **VMM (Paging)**: Hardware-enforced isolation via multi-level page tables.
- **Slab Allocator**: Optimized for small object reuse to ensure zero fragmentation.

## 💾 SIGMAFS (LOG-STRUCTURED STORAGE)
The **SigmaFS** shard uses a log-structured append-only mechanism to ensure crash resilience.
- **Atomic Writes**: No data corruption during power loss.
- **Sequential Performance**: Optimized for bare-metal silicon endurance.

## 🌐 SOVEREIGN NETWORKING
- **Bare-Metal e1000**: Direct hardware communication with the Intel NIC.
- **Sharded UDP Stack**: Connectionless, zero-copy packet transmission for agentic pulses.

## 🛠️ DEVELOPMENT WORKFLOW
1. **Modular Build**: Run `make` to compile independent shards.
2. **Industrial Verification**: Build system enforces Multiboot-2 compliance.
3. **Sovereign Sync**: Use the integrated `git` automation to maintain repository integrity.

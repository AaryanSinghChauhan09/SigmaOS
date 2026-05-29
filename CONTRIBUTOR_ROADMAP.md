# SigmaOS Contributor Roadmap: The Singularity Path

This roadmap formalizes the remaining gaps between SigmaOS Zenith and legacy competitor distributions, providing actionable targets for sovereign development.

## 📊 Comparative Gaps & Parity Targets

| Category | Best Competitor | SigmaOS Parity Target | Shard/Utility | 
| :--- | :--- | :--- | :--- | 
| **UX & Access** | Zorin / Elementary | Adaptive UI Scaling & Accessibility Toolkit | `SovereignAccessibility` | 
| **IoT / ARM** | RPi-Distro | Event-Driven GPIO & Sensor Toolkit | `SovereignIoT` | 
| **Gaming** | SteamOS | Dynamic GPU Scheduler & Controller Manager | `SovereignGPUSched` | 
| **Performance** | Clear Linux | Telemetry-Driven Auto-Tuner | `SovereignAISched` | 
| **Reproducibility** | NixOS | Declarative Shard Configs & Rollbacks | `sigma-pkg sync` | 
| **Recovery** | RescueZilla | Snapshot Diff Engine & Forensic Toolkit | `SovereignSnapshotDiff` | 
| **Containers** | Fedora CoreOS | Sovereign Container Orchestrator (K8s equivalent) | `SovereignOrchestrator` | 
| **Rolling** | Arch / Solus | Incremental Shard Delta Updates | `sigma-pkg update --delta` | 
| **Enterprise** | Ubuntu | Hardware Regression Harness | `SovereignRegression` | 
| **File Systems** | Btrfs / ZFS | Sovereign CoW & Journaling Filesystem | `SovereignFS` |
| **Networking** | Linux TCP/IP | Sovereign IPv6, Mesh, & VPN Stack | `SovereignNet` |
| **Compatibility**| WSL / Wine | POSIX Translation Shims (Opt-In) | `SovereignCompat` | 

## 🚀 Development Phases

> **Note to Contributors:** Previously, this document claimed parity was complete. We have performed an honest architectural audit. While the scaffolding is solid, true implementation is still needed. The roadmap is now actively tracking real feature development.

### Phase 1: Core Stabilisation (IN PROGRESS)

- [x] Initial bootable state on x86_64 and QEMU cross-arch validation.
- [x] Sovereign Init System with parallel boot & process monitoring.
- [x] Ext4 Filesystem read/write implementation with superblock parsing.
- [x] OmniPkg Manager format specification and local deployment logic.
- [ ] Implement Ext4 ordered-mode JBD2 journaling.
- [ ] Resolve memory management block allocator fragmentation.

### Phase 2: Hardware & Network (PLANNED)

- [ ] Linux DRM/KMS compatibility shim at HAL level.
- [ ] VFS network abstraction (NFS/SMB).
- [ ] Native IPv4/IPv6 networking stack and drivers (e1000, ixgbe).
- [ ] USB 3.0 and NVMe controller implementation.

### Phase 3: Desktop Environment & Tooling (PLANNED)

- [ ] Migrate Zenith UI JS prototype to native C++ compositor.
- [ ] Implement Sigma Shell robust scripting pipelines.
- [ ] Implement a full guided graphical installer (Calamares equivalent).

## ⚙️ Contribution Principles

1. **Sovereignty**: Minimize monolithic dependencies where possible.
2. **Transparency**: All changes must reflect functional implementation, not just stubs.
3. **Resilience**: Shards must be atomic and robust against crashes.

---

### Build the future of sovereignty. Join the lattice


# SigmaOS Components & Inspirations

This page documents the core components of SigmaOS, the repositories/projects they draw inspiration from, and target areas for further improvement.

| Component Name | Description | Inspiration / Upstream Repository | Future Improvements / Next Steps |
|---|---|---|---|
| **SerpentMossEngine** | Atomic package transaction engine with staging, system triggers, and rollbacks. | [serpent-os/moss](https://github.com/serpent-os/moss) | Implement full binary delta downloads and optimize transaction log compaction. |
| **CachyBoreScheduler** | BORE scheduler hyper-optimizer with interactive IPC scoring and P/E core affinity. | [CachyOS/linux-cachyos](https://github.com/CachyOS/linux-cachyos) | Fine-tune dynamic task priority scaling based on memory access patterns. |
| **FreeBsdRacctVnetGuard** | RACCT/RCTL resource accounting and VNET network stack isolation guard. | [freebsd/freebsd-src](https://github.com/freebsd/freebsd-src) | Enhance virtual network device performance and support packet filtering rules. |
| **OpenBsdPledgeUnveilSentinel** | OpenBSD-style dynamic pledge and unveil runtime system call restrictions. | [openbsd/src](https://github.com/openbsd/src) | Support declarative security profile paths and logging violations to audit stream. |
| **ZenithCompositor** | Zenith lightweight desktop environment and DRM/KMS graphics compositor. | [Enlightenment/enlightenment](https://github.com/Enlightenment/enlightenment) | Add hardware-accelerated tiling layouts and multi-monitor hotplugging support. |
| **QubesIsolationManager** | VM-based compartmentalization and domain-based process isolation. | [QubesOS/qubes-core-admin](https://github.com/QubesOS/qubes-core-admin) | Integrate memory ballooning across domains to optimize host RAM usage. |
| **PostQuantumTls** | Post-quantum cryptographic TLS 1.3 suite leveraging Kyber KEM and Dilithium signatures. | [open-quantum-safe/liboqs](https://github.com/open-quantum-safe/liboqs) | Implement hybrid TLS key exchange (X25519 + Kyber) for backward compatibility. |
| **SovereignVcsEngine** | Native, zero-dependency version control system built for #![no_std]. | [git/git](https://github.com/git/git) | Build a lightweight delta compression engine for large repository packs. |
| **SovereignInitSupervisor** | High-performance init daemon and service manager with dependency resolution. | [systemd/systemd](https://github.com/systemd/systemd) | Implement parallel service startup with dependency graph sorting. |
| **SovereignPartitionEngine** | Partition layout and disk partitioning management utility. | [util-linux/util-linux](https://github.com/util-linux/util-linux) | Support online partition resizing and automated partition alignment detection. |
| **AntiXHardwareOpt** | Lightweight init switcher and task trimmers for legacy and low-end hardware architectures. | [antiX Linux](https://antixlinux.com) | Enhance zero-allocation visual swap profiles and automate background daemon pruning. |
| **ZorinThemeEngine** | Dynamic theme and UI desktop layout engine adapting to user patterns and system state. | [Zorin OS](https://zorin.com) | Integrate layout transition animations and automatic wallpaper contrast matching. |
| **VoidRunitSystem** | Native runit-compatible stage manager and process supervisor. | [void-linux/runit](https://github.com/void-linux/runit) | Optimize stage-1 startup hook execution times. |
| **ClearStatelessEngine** | Clear Linux-style stateless configuration overlay system. | [clearlinux/swupd-client](https://github.com/clearlinux/swupd-client) | Improve live partition rollback recovery speeds. |
| **PopCosmicTiler** | Tiling window management and scheduling core. | [pop-os/cosmic-comp](https://github.com/pop-os/cosmic-comp) | Fine-tune focus-follows-mouse layout logic. |

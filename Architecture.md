# Σ SIGMAOS: ARCHITECTURE (Sovereign Lattice)

## 🌌 The 7-Layer Modular Lattice
SigmaOS is not a monolithic OS. It is a **Sovereign Lattice** composed of independent, cryptographically-isolated "shards".

### Layer 0: Silicon / Boot
*   **Init**: `SovereignInit` orchestrates early bring-up.
*   **Live Boot**: `SovereignLiveBoot` provides USB persistence.
*   **HAL**: `SovereignHAL` maps the device tree.

### Layer 1: Kernel Primitives
*   **Memory**: PMM/VMM logic.
*   **Sched**: Lattice-aware scheduler.
*   **Drivers**: Bare-metal GPU (SovereignDirectGPU), NVMe, etc.

### Layer 2: System Services
*   **VFS**: Sovereign Partition Manager.
*   **IPC**: Sovereign Bus.
*   **Observability**: eBPF Telemetry.

### Layer 3: Security Fabric
*   **PQC**: Lattice-based post-quantum cryptography.
*   **Sandbox**: `SovereignSandbox` for agent isolation.
*   **MAC**: SELinux-style mandatory access controls.

### Layer 4: AI & Automation
*   **Claw Stack**: Agent core and workflow orchestration.
*   **Neural Nexus**: Aether Firewall heuristics.

### Layer 5: Industrial Ecosystem
*   **Orb Manager**: Unified package dispatch (Pacman/Flatpak/Nix).
*   **Marketplace**: P2P Indexing and verification.

### Layer 6: Zenith UI
*   **Zenith**: Cyberpunk-morphic display server and UI.

---
*For a detailed map of source files to layers, see the [Modularization Map](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki/blob/master/MODULARIZATION_MAP.md).*

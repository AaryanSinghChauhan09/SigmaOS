# 🌀 Welcome to the SigmaOS Sovereign Wiki

*   **[Sovereign Architecture](Sovereign-Architecture)**: Absolute machine finality and zero-std primitives.

SigmaOS is a next-generation, industrial-grade operating system designed for speed, modularity, and innovation. Unlike traditional Linux distros, SigmaOS bypasses POSIX and legacy layers to deliver bare-metal performance with direct silicon memory flows.

## 🏛️ Project Vision

SigmaOS aims to be the fastest and most secure platform for browser-centric computing, blending the purity of a 33-suite Sovereign Lattice with the flexibility of a Chromium-native ecosystem.

## 📖 Wiki Contents

*   **[Shard Index](AUTO_SOVEREIGN_WIKI.md)**: A complete technical breakdown of the 600 hierarchical shards.
*   **[Modular Architecture](MODULAR_ARCHITECTURE_BLUEPRINT.md)**: Details on the Core, Essential, Optional, Third-Party, and Infinite layers.
*   **Features**:
    *   [Sovereign 33-Suite Lattice](AUTO_SOVEREIGN_WIKI.md)
    *   [Zenith Dashboard v33.0.4](Home.md)
    *   **[600-Shard Kernel]**: Hierarchical modularization for absolute scale.
    *   **[Sigma Vault]**: Centralized WASM Shard Marketplace (Inspired by Arch AUR).
    *   **[Enterprise Stability]**: Rigorous integrity auditing and master signatures (Inspired by RHEL).
    *   **[Sovereign Sandbox]**: Silicon-level isolation for every execution (Inspired by Qubes).
    *   **[Amnesic Mode]**: Zero-artifact memory wiping (Inspired by Tails).
    *   **[Silicon Tuner]**: Clear Linux-grade performance optimization.
    *   **[Sovereign Musl]**: Security-hardened, ultra-minimal LibC layer (Inspired by Alpine).
*   **Roadmap**: [Strategic Feature Trajectory](../README.md#roadmap)

## 💻 Installation Guide

### Bare-Metal (Recommended)

1.  Clone the unified repository (Primary Branch: `main`):

    ```bash
    git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git -b main
    cd SigmaOS
    ```

2.  Build the modularized kernel:

    ```bash
    make all
    ```

3.  Generate the 33-suite Sovereign ISO:

    ```bash
    make iso
    ```

4.  Flash to a USB drive or boot in QEMU.

### Web Engine (Simulated Mode)

1.  Build the engine:

    ```bash
    make web-engine
    ```

2.  Launch the local portal:

    ```bash
    ./sigma_web_engine
    ```

## 🛠 Developer Setup

1.  Install **GCC 13+**, **NASM**, and **Node.js**.
2.  Explore the **33-Suite Sovereign Lattice** in the `suites/` directory.
3.  Build the system using the unified orchestrator:

    ```bash
    make kernel
    ```

4.  Launch the **Zenith Dashboard** via `index.html` in the root.
5.  Use the **Sovereign Shard Builder** to create new suites.

---

*Sovereignty is Absolute. The Work continues.*

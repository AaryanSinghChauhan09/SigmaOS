* **[Sovereign Architecture](Sovereign-Architecture)**: Absolute machine finality and zero-std primitives.

# 🌀 Welcome to the SigmaOS Sovereign Wiki

SigmaOS is a next-generation, industrial-grade operating system designed for speed, modularity, and innovation. Unlike traditional Linux distros, SigmaOS bypasses POSIX and legacy layers to deliver bare-metal performance with direct silicon memory flows.

## 🏛️ Project Vision
SigmaOS aims to be the fastest and most secure platform for browser-centric computing, blending the purity of a 33-suite Sovereign Lattice with the flexibility of a Chromium-native ecosystem.

## 📖 Wiki Contents
- **[Shard Index](AUTO_SOVEREIGN_WIKI.md)**: A complete technical breakdown of all 2,191 shards.
- **Getting Started**:
  - [Installation Guide](#installation-guide)
  - [Developer Setup](#developer-setup)
- **Features**:
  - [Sovereign 33-Suite Lattice](AUTO_SOVEREIGN_WIKI.md)
  - [Zenith Dashboard v33.0.4](Home.md)
  - **[Sigma Vault]**: Centralized WASM Shard Marketplace.
  - **[Snapshot Engine]**: Declarative System Rollback (NixOS-grade).
  - **[Tiling Engine]**: Advanced Window Management (i3/Sway style).
  - **[Sovereign Handoff]**: Universal State Continuity.
- **Roadmap**: [Strategic Feature Trajectory](../README.md#roadmap)

## 💻 Installation Guide
### Bare-Metal (Recommended)
1. Clone the unified repository (Primary Branch: `main`):
   ```bash
   git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git -b main
   cd SigmaOS
   ```
2. Build the modularized kernel:
   ```bash
   make all
   ```
3. Generate the 33-suite Sovereign ISO:
   ```bash
   make iso
   ```
4. Flash to a USB drive or boot in QEMU.

### Web Engine (Simulated Mode)
1. Build the engine:
   ```bash
   make web-engine
   ```
2. Launch the local portal:
   ```bash
   ./sigma_web_engine
   ```

## 🛠 Developer Setup
1. Install **GCC 13+**, **NASM**, and **Node.js**.
2. Explore the **33-Suite Sovereign Lattice** in the `suites/` directory.
3. Build the system using the unified orchestrator:
   ```bash
   make kernel
   ```
4. Launch the **Zenith Dashboard** via `index.html` in the root.

3. Use the **Sovereign Shard Builder** to create new suites.

---
*Sovereignty is Absolute. The Work continues.*

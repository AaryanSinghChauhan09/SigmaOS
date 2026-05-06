# SigmaOS: Sovereign Lattice Industrial v100

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/ci.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![Version](https://img.shields.io/badge/version-v100.1_Industrial-blue.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)
[![Security](https://img.shields.io/badge/Security-Hardened-green.svg)](SECURITY.md)

> A browser-based OS simulation with a glassmorphic desktop, draggable windows, terminal, file manager, AI assistant, and live system telemetry. Zero build step — open `index.html` and go.

![SigmaOS Zenith Desktop](zenith_desktop_screenshot.png)

## [→ Live Demo](https://aaryansinghchauhan09.github.io/SigmaOS/)

## Quick Start

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
node server.js          # Serves on http://localhost:5000
```

## System Architecture: The Sovereign Lattice

SigmaOS is built on the **Sovereign Lattice**, a modular 600-shard architecture designed for absolute isolation and high-assurance computing.

### Key Documentation

- [**Shard Orchestration & Self-Healing**](SHARD_ORCHESTRATION.md)
- [**Architectural Stabilization & Industrialization**](ARCHITECTURAL_STABILIZATION.md)
- [**Sovereign Shard Manifest (v100)**](SOVEREIGN_LATTICE_MANIFEST_500.md)

```mermaid
graph TD
    User([Zenith UI]) --> Shards[Sovereign Shard Lattice]
    Shards --> Bus{SovereignEventBus}
    Bus --> Kernel[Core Kernel Shards]
    Kernel --> Security[PQC & Attestation]
    Kernel --> Observability[eBPF-Native Monitor]
```

## Zenith Desktop Features

| Category | Features |
| :--- | :--- |
| **Interface** | Glassmorphic UI, Dynamic Desktop Shards, Drag & Snap Windows |
| **Observability** | Real-time eBPF System Telemetry, CPU/Memory Heatmaps |
| **Security** | Post-Quantum Cryptography (LBSV), Hardware Attestation (TEE) |
| **Intelligence** | Integrated SovereignAI Assistant, Command Palette (Ctrl+Space) |

## Shortcuts

| Shortcut | Action |
| :--- | :--- |
| **Ctrl + Space** | Command Palette (Search all actions) |
| **Alt + 1 – 4** | Switch Virtual Desktops (1 - 4) |
| **Ctrl + Alt + T**| Launch Markup Forge |
| **↑ / ↓** | Terminal Command History |
| **Right-Click** | Desktop Context Menu |

## Contributions

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) and the [Official Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

# Σ SIGMAOS: THE SOVEREIGN ZENITH (V2)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v100.2_Futuristic-blue.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)
[![Build Status](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/ci.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![Security](https://img.shields.io/badge/Security-Post--Quantum-green.svg)](SECURITY.md)
[![Maintainability](https://img.shields.io/badge/Maintainability-Industrial-orange.svg)](MAINTENANCE_POLICY.md)

> **SigmaOS** is an intelligent, AI-native adaptive operating system built on a hardened Linux foundation. It represents a paradigm shift from traditional distributions, focusing on **visual excellence**, **intelligent automation**, and **seamless workflow optimization**.

![SigmaOS Sovereign Zenith](zenith_desktop_screenshot.png)

## [→ Live Demo](https://aaryansinghchauhan09.github.io/SigmaOS/)

## 🎯 The Futuristic Paradigm

Instead of "making Linux work," SigmaOS focuses on "making Linux feel futuristic." It is a **Sovereign environment** where the OS adapts to *you*.

### 🏗️ Sigma Layers Architecture
1.  **Linux Base**: Hardened host kernel (LTS) for maximum hardware compatibility.
2.  **Sigma Core**: C++ Sovereign Lattice shards for high-performance orchestration.
3.  **Sigma UI**: The Zenith Desktop — a cyberpunk-inspired, CSS-extensible interface.
4.  **Sigma AI**: Integrated Neural Assistant for real-time optimization and troubleshooting.
5.  **Sigma Automations**: The Workflow Engine (IF/THEN) for contextual system responses.
6.  **Sigma Marketplace**: Universal Package Layer dispatching to Pacman, Flatpak, and Nix.

### 🚀 Key Futuristic Features
*   **Neural Search**: A universal command palette (`Alt+Space`) for files, apps, and AI-driven actions.
*   **Sigma Profiles**: Instant environment optimization for Developers, Gamers, and AI Engineers.
*   **AI Desktop Assistant**: A persistent sidebar assistant (`Alt+A`) that monitors lattice health and automates tasks.
*   **Universal Package Layer**: One interface, three package managers. Seamlessly inject software into the lattice.
*   **Morphic CSS Styling**: Live, non-destructive editing of the UI's visual identity (blur, radius, glow).

## 🛠️ Industrial Core
*   **Sovereign PQC**: Post-Quantum Cryptography for all internal communications.
*   **Amnesic Memory Manager**: Zero-forensic memory allocation for high-security shards.
*   **Hardware Attestation**: Silicon-level verification of the physical lattice.

## ⌨️ Hotkeys
| Shortcut | Action |
| :--- | :--- |
| **Alt + Space** | **Neural Search** (Universal Command Center) |
| **Alt + A** | **AI Assistant** (Sidebar Sidebar) |
| **Alt + T** | **OmniShell** (Command Line Shard) |
| **Esc** | Close Overlays & Modals |

## 🏗️ Architecture Map
```mermaid
graph TD
    UserLand[User Space / Zenith UI] -->|Syscalls| Core[Sovereign Core Shards]
    Core -->|HAL| Hardware[Silicon / QEMU]
    Core -->|Sandbox| AI[Sovereign AI Gateway]
    AI -->|Rules| Workflow[Workflow Engine]
```

## 📦 Technical Quickstart

### Prerequisites
- `gcc-x86-64-linux-gnu` / `clang`
- `nasm`, `make`, `cmake`
- `qemu-system-x86`

### Build & Run
```bash
# 1. Build the Sovereign Lattice
make all

# 2. Launch in QEMU (Headless with Serial Log)
./qemu-boot.sh

# 3. View Kernel Logs
tail -f serial.log
```

### Development Environment
For a reproducible environment, use the provided **DevContainer** (VS Code) or the `Dockerfile` in the root directory.

## Contributions
Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) and the [Official Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki).

## 🏛️ Governance & Standards
SigmaOS adheres to strict industrial standards for lattice maintenance and security.
- **[Maintenance Policy](MAINTENANCE_POLICY.md)**: Quality standards and review process.
- **[Release Process](RELEASE_PROCESS.md)**: Preparation and cryptographic signing details.
- **[Code of Conduct](CODE_OF_CONDUCT.md)**: Expectations for community behavior.
- **[Security Policy](SECURITY.md)**: Vulnerability reporting and PQC disclosure.

## License
MIT License - see [LICENSE](LICENSE).

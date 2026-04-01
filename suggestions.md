# SigmaOS: Industry-Standard Suggestions & Feature Roadmap

This document identifies features and components currently "missing" or "not working as intended" in SigmaOS compared to industry-standard Industrial Linux distributions (RHEL, Ubuntu Pro, Arch).

## 1. Core Operating System Components

- [ ] **Native Package Manager (SigmaPKG)**: Currently, the repository system is a mock UI. A real package manager should handle `tar.gz` or custom `.sigma` shards with dependency resolution.
- [ ] **Symmetric Multi-Processing (SMP)**: Core logic for multi-CPU affinity and scheduling is missing in the C11 kernel part.
- [ ] **Unified Device Model (UDM)**: A standardized way to handle drivers (blocked/character devices) instead of hardcoded shards.
- [ ] **Dynamic Linker (SigmaLD)**: Ability to load external shards into memory at runtime without full kernel recompilation.
- [ ] **Journaling File System (SFS)**: The current VFS is memory/localStorage based. A real journaling system (ext4/btrfs parity) is needed for data integrity.

## 2. Browser-Based UI (Zenith GUI)

- [ ] **Live Taskbar/Dock Integration**: The top bar should show currently open windows for easy switching (Parity with GNOME/KDE).
- [ ] **Window Snapping & Tiling**: Professional-grade window management (Snap to left/right, maximize on top drag).
- [ ] **Multi-Tab Support**: Browser and Terminal shards should support multiple tabs within the same window.
- [ ] **Theme Persistence**: Settings changed in the GUI Architect should persist across reboots via the Sovereign VFS.
- [ ] **Global Search (Spotlight) Improvements**: Search should include content within files, not just app names.

## 3. Automation & AI

- [ ] **Neural Mission Pipe**: A way to pipe terminal output directly into the AI Orchestrator for real-time debugging.
- [ ] **Autonomous Cron**: A background task scheduler for periodic system maintenance and silicon-scrubbing.
- [ ] **Low-Level Automation API**: Expose `SigmaC11` functions to the GUI Scripting Playground via WebAssembly.

## 4. Security & Protection

- [ ] **Amnesic Kernel Mode**: A boot flag that runs the entire OS in RAM, scrubbing all silicon on shutdown (Tails/Alpine parity).
- [ ] **PQC Keychain**: Post-Quantum Cryptography integration for all user credentials and VFS encryption.
- [ ] **Hardware-Locked Sovereignty**: Binding the OS logic to specific CPU IDs or TPM modules.

## 5. Industrial Parity (Missing Distro Features)

- [ ] **Arch `pacman` Parity**: Full command-set mapping for Arch package management.
- [ ] **Kali `metasploit` Shard**: A native implementation of penetration testing primitives (not just a mock).
- [ ] **Ubuntu `snap` Core**: A container-native way to run isolated applications.

---

Σ SIGMAOS: THE DEFINITIVE ROADMAP TO SUPREMACY.

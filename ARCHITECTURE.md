# SigmaOS Architecture: The Sovereign Lattice

SigmaOS is designed as a **Sovereign Lattice**—a modular, high-assurance kernel architecture where functionality is encapsulated in independent "shards."

## 🏗️ The 6-Layer Model

1.  **L0: Silicon Shard (HAL)**: Direct hardware interaction and architecture-specific assembly.
2.  **L1: Sovereign Kernel Shards**: Core logic (Memory, Scheduling, PQC) implemented in zero-STL C++.
3.  **L2: System Shards**: Drivers and OS services (Filesystem, Networking).
4.  **L3: AI Automation Layer**: The Claw Gateway and Workflow Engine.
5.  **L4: Userland Shards**: High-level applications and the OmniShell.
6.  **L5: Zenith UI**: The glassmorphic, CSS-driven user interface.

## 🤖 AI-Native Design

Unlike traditional operating systems, SigmaOS integrates an **AI Automation Gateway** directly at the kernel level. This allows for:
- **Autonomous Scheduling**: The OS optimizes process priority based on user intent.
- **Self-Healing**: Kernel shards can detect anomalies and re-orchestrate state using the `SovereignHealer`.

## 🔒 Security Model

SigmaOS adopts a **Zero-Trust Sovereign Identity** model.
- **Capability-Based Access**: Shards communicate via strictly defined interfaces.
- **Amnesic Memory**: Critical security shards use memory profiles that auto-wipe on task completion.
- **Post-Quantum Cryptography (PQC)**: All internal message passing is sealed with quantum-safe primitives.

## 📂 Repository Structure

- `/kernel/arch`: Architecture-specific code (x86_64, ARM64).
- `/kernel/core`: L1 and L2 kernel shards.
- `/kernel/core/ai`: L3 AI Automation shards.
- `/include`: Global Sovereign Lattice headers.
- `/userland`: Experimental L4 applications.
- `/zenith_desktop`: The Zenith UI implementation (HTML/CSS/JS).

---
*Next Steps: Check the [ROADMAP.md](ROADMAP.md) for future architectural evolutions.*

# SigmaOS Command Line Interface (CLI) Reference

The SigmaOS CLI (`s-cli`) is built on a scalable namespace architecture. Instead of thousands of flat commands, every operation is categorized into logical subsystems. This ensures the environment remains modular, intuitive, and completely free of bloat.

## 🧠 `s-assist` (Intelligence & AI)
Interact directly with the Sigma Assistant for system insights and dynamic optimization.
* `s-assist status` - Renders the real-time health dashboard (CPU, Memory, Network, Battery).
* `s-assist suggest` - Triggers AI-driven recommendations based on current telemetry.
* `s-assist optimize <task>` - Auto-tunes the system for specific workloads (e.g., `gaming`, `devops`, `rendering`).
* `s-assist explain` - Provides transparency logs explaining why the AI made a specific system adjustment.

## 📦 `s-pkg` & `s-deps` (Package & Dependency Management)
Manage software and ensure the OS remains free of dependency bloat.
* `s-pkg install <app>` - Installs an app, auto-converting Flatpaks/AppImages via SpkgTranslator.
* `s-pkg rollback` - Reverts to the previous stable snapshot if an update breaks functionality.
* `s-deps prune` - Scans and removes orphaned dependencies and unused libraries.
* `s-deps tree` - Visualizes the dependency impact of installed software.

## ⚡ `s-perf` (Performance & Resource Allocation)
Directly control the AI-Driven Scheduler and system resources.
* `s-perf boost` - Temporarily maximizes performance for heavy workloads by throttling background containers.
* `s-perf cache adaptive` - Enables predictive app pre-loading.
* `s-perf isolate <process_id>` - Moves a resource-heavy process into an isolated micro-VM.

## 🌐 `s-net` (Zero-Trust Networking)
Manage the Sovereign Lattice network stack.
* `s-net secure` - Enforces internal Zero-Trust subsystem authentication.
* `s-net connect <network>` - Standard WiFi/VPN connection management.
* `s-net dns decentralized` - Routes domain resolution through ENS/IPFS.

## 🎨 `s-profile` & `s-theme` (Personalization)
Hot-swap user environments and aesthetic layouts.
* `s-profile switch <name>` - Instantly swaps between Work, Gaming, Study, or Accessibility profiles.
* `s-theme set <name>` - Changes the Zenith UI aesthetic.
* `s-theme dynamic` - Enables time-based environmental themes.

## 🛡️ `s-sec` (Security & Sovereignty)
* `s-sec audit` - Scans all predefined and third-party libraries for vulnerabilities.
* `s-sec quantum enable` - Forces all signatures to use Kyber/Dilithium post-quantum cryptography.
* `s-sec amnesia` - Erases all execution traces from RAM and Cache (Zero-Trace execution).

---
*By utilizing this modular namespace approach, SigmaOS guarantees a 1:1 parity between the Zenith GUI and the Terminal, empowering both casual users and system architects.*


# SigmaOS Modularization & Automation


This document details the systems that make SigmaOS self-configuring, self-optimizing, and modular by design.

---


## 🧩 Service Capsules (`modules/ext/plugins/capsule.c`)


SigmaOS subsystems are treated as independent, versioned capsules.

- **Hot-Swapping**: `capsule_hotswap()` allows replacing a scheduler or filesystem at runtime.
- **Dependency Resolution**: Capsules declare dependencies; the kernel auto-loads required parent capsules first.
- **Rollback**: If a new version fails to initialize, the kernel automatically resumes the previous stable version.

---


## 🛡️ Capability Registry & Auto-Revocation (`modules/security/capabilities/cap_registry.c`)


A central registry for tracking capability ownership across processes and modules.

- **Security Automation**: Every capability can be flagged for **Auto-Revocation on Exit**.
- **Zero-Touch Cleanup**: When a process or capsule terminates, the kernel automatically strips all its associated rights from the registry, preventing resource leaks or stale permissions.

---


## ⚙️ Hardware Auto-Detection (`modules/ext/hal/hw_detect.c`)


A sovereign alternative to complex udev systems.

- **Bus Scanning**: Automatically identifies PCI, USB, and Platform devices.
- **Auto-Config**: Maps Vendor/Device IDs to compatible capsules and triggers their loading.
- **Dynamic Drivers**: Drivers are loaded as capsules only when the hardware is physically present.

---


## 📈 Continuous Profiling & AI Hooks (`modules/tools/diag/profiler.c`)


The kernel constantly monitors its own performance.

- **Real-time Snapshotting**: Captures CPU cycles, context switches, and page faults.
- **Optimization Suggestions**: Provides metadata that the AI-Assisted Scheduler can use to tune task priorities and memory quotas.

---


## 🚀 Future Roadmap: Security & Modular Supremacy

- [ ] **Sovereign Module Store**: Automated dependency resolution from remote signed repositories.
- [ ] **Consensus-Driven Updates**: Nodes in a cluster verify module integrity before allowing an update.
- [ ] **Memory-as-Contracts**: Cryptographic leasing of memory blocks between capsules.

---


## Source Files

- `modules/security/capabilities/cap_registry.c`
- `modules/ext/plugins/capsule.c`
- `modules/ext/hal/hw_detect.c`
- `modules/tools/diag/profiler.c`

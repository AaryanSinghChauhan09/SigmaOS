# SigmaOS Subsystem and Development Roadmap

This document outlines the detailed architectural roadmap and subsystem development plan for the SigmaOS operating system, focusing on bringing the core boot environment, drivers, network stacks, and desktop modules to production stability.

---

## 1. Executive Summary

While SigmaOS implements advanced features including custom schedulers, buddy allocations, and post-quantum encryption primitives, standing usability is blocked by unstable low-level blocks. To resolve this, development is divided into sequential phases focusing on fundamental subsystems before high-level localization and AI orchestrations are added.

```
+-------------------------------------------------------+
| Phase G: Kernel Boot, Drivers & Networking Stack     |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
| Phase H: India Stack, Localization & Finance Engine   |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
| Phase I: AI-Native Local Inference & Orchestrator     |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
| Phase J: Desktop Zenith GUI & Standalone Usability   |
+-------------------------------------------------------+
```

---

## 2. Subsystem Readiness Matrix

| Subsystem | Baseline Status | Target (12-18 Months) |
|---|---|---|
| **Networking** | Partial TCP/UDP stack (IPv4 only) | Robust dual-stack IPv4/IPv6, integrated SSL/TLS |
| **Storage & FS** | Basic file lookup, unstable write paths | Journaled resilient VFS, crash recovery audits |
| **Graphics & GUI** | Basic frame buffer | Zenith Desktop, hardware accelerated window compositor |
| **Driver Layer** | Monolithic driver stubs | Modular GPU (VESA/DRM/KMS), USB HID, Intel HDA Audio |
| **Userspace Runtime** | Basic shell interpreter | `sigma-sh` with interactive aliases, `sigpkg` integration |

---

## 3. Development Phases

### Phase G: Drivers & Networking (Current Focus)
- **Networking Stack**: Finalize pure-Rust TCP state machine. Implement native IPv6 header parsing and path discovery.
- **Hardware Drivers**: Stabilize PCI discovery, integrate USB Host Controller interface (xHCI/EHCI) for mouse/keyboard inputs, and complete basic VESA/KMS display pipelines.
- **Resilient File System**: Incorporate write-ahead metadata journaling to protect raw partition bounds against sudden power cuts.

### Phase H: India Stack Integration
- **Vyapaar & Krishi Calculators**: Progress from computational stubs to localized service layers (MSME compliance checks, GST rates, crop-specific MSP estimations).
- **Security Audit Trails**: Cryptographically log all transactions through post-quantum hash trees.

### Phase I: AI-Native Orchestration
- **Local AI Daemon**: Integrate lightweight routing paths in the kernel for local inference loops.
- **Predictive Scheduler**: Adjust thread allocation heuristics dynamically based on task pattern heuristics.

### Phase J: Standalone Usability
- **Zenith Desktop Workspace**: Finalize terminal session managers, native system dashboard widgets, document engines, and calendar tools.
- **Self-Hosting Toolchain**: Eliminate external compiler dependencies to achieve build sovereignty.

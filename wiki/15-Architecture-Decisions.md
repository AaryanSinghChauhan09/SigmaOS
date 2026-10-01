# Architecture Decision Records (ADRs)

## ADR-001: Sovereign Zero-Dependency Philosophy
- **Status:** Accepted
- **Context:** SigmaOS aims to be a self-sufficient, high-performance operating system surpassing legacy open-source projects.
- **Decision:** Implement all core operating system capabilities natively in Rust under `#![no_std]` without external third-party crate dependencies.

## ADR-002: Universal Package Manager Interop
- **Status:** Accepted
- **Context:** Applications across various Linux distributions and BSD variants use diverse package formats (.deb, .rpm, .apk, pkg, etc.).
- **Decision:** Provide native parsing, DPLL SAT dependency resolution, scriptlet sandboxing, and format translation for 30+ package formats into canonical `SigmaPkg`.

## ADR-003: Multi-Core SMP and Modern Kernel Subsystems
- **Status:** Accepted
- **Context:** Modern hardware requires efficient multi-core processing, async I/O, and low-latency IPC.
- **Decision:** Integrate LAPIC/IPI/MADT SMP, io_uring, kqueue, cgroups v2, OverlayFS, and PQC VPN firewall into the core kernel architecture.


## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

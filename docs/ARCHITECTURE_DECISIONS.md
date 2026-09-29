# SigmaOS Architecture Decision Records (ADRs)

## Architecture Overview
SigmaOS is a zero-dependency, self-sufficient operating system kernel and userland written in Safe Rust.

## Key Architectural Principles
1. **Zero External Dependencies**: All core utilities, memory allocators, data structures, and IPC mechanisms rely strictly on native Rust primitives without external crate dependencies.
2. **Unified Multi-Distro Package Model**: Foreign package formats (Deb, RPM, Pacman, APK, Nix, PKG) are transpiled into native Unified Packages (`SigmaPkg`) with DPLL SAT-based dependency resolution and sandboxed scriptlet execution.
3. **Capability-Based Security**: Process sandboxing enforces strict OpenBSD-style `pledge`/`unveil`, FreeBSD Capsicum rights, and Linux eBPF/Landlock security policies.
4. **Hybrid Microkernel & Monolithic Performance**: Combines zero-copy ring buffers, lock-free CAS scheduling, and NUMA-aware core affinity for high-performance low-latency execution.

# 📦 SigmaOS OCI Container Runtime Parity & Differentiation Roadmap

This document establishes the strategic engineering, architecture, and implementation blueprint for **SigmaOS's OCI-Compliant Container Runtime Subsystem**, taking design inspiration from mainstream Linux runtimes (`runc`, `crun`, `kata`) and leading micro-VM orchestrators.

***

## 🏗️ 1. Technical Vision & Architectural Hierarchy

Standard Linux containers rely on a monolithic kernel with shared namespaces and group controls, introducing security vulnerability surfaces. SigmaOS leverages a **Capability-Based Shard Sandboxing model** that enforces strict kernel execution isolation with zero legacy POSIX bloat.

           +-------------------------------------------------------+
           |               Sovereign Container Layer               |
           +-------------------------------------------------------+
                |                        |                       |
                v                        v                       v
       +-----------------+      +-----------------+      +-----------------+
       |  SigmaNet Bridge|      |  SigmaFS Mounts |      |   S-SEC Sand    |
       | (Bridge/Overlay)|      | (Bind, tmpfs)   |      | (User Remapping)|
       +-----------------+      +-----------------+      +-----------------+

***

## 🌐 2. Parity Domain 1: Container Networking (Rust / Zig)

### 2.1 Bridge & Overlay Integration

*   **Inspiration**: Linux bridge, macvlan, and CNI plugins.
*   **Implementation (Rust)**: Containers register direct bridge connections mapped to the `SigmaNet` networking shard (`src/container/runtime.rs`).
*   **Implementation (Zig)**: Highly optimized packet routing and virtual bridge mapping filters to achieve wire-speed container communications.

***

## 💾 3. Parity Domain 2: Volume Mounts & Namespaces (Rust)

### 3.1 Sovereign Bind Mounts

*   **Inspiration**: Linux bind mounts, tmpfs, and overlayfs.
*   **Implementation**: The container manager maps directories natively inside the Virtual Filesystem (`src/filesystem/vfs.rs`) using capability tokens. No standard root/SUID required.

### 3.2 User Namespaces remapping via Capability Tokens

*   **Inspiration**: UID/GID remapping, rootless containers.
*   **Implementation**: Employs capability-gated validation rings where UID/GID remappings are translated directly to fine-grained S-SEC privilege tokens.

***

## 🔒 4. Parity Domain 3: Seccomp profiles (Rust)

### 4.1 Granular Syscall Filtering

*   **Inspiration**: Hardened Linux seccomp profiles.
*   **Implementation**: Integrates with the `SigmaSEC` microkernel security shard. Containers can register explicit `blocked_syscalls_mask` to automatically block insecure system interactions.

***

## 📅 5. Step-by-Step Implementation Roadmap

*   \[ ] **Phase 1 (Validation)**: Complete networking, volume, user namespace, and seccomp structs inside `src/container/runtime.rs`.
*   \[ ] **Phase 2 (Parity Integration)**: Bridge FHS paths and seccomp filters with the `SigmaSEC` kernel shard.
*   \[ ] **Phase 3 (Self-Healing Runtime)**: Support auto-rollbacks and AI telemetry-driven container policy generations.

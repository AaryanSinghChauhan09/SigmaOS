# SigmaOS AI Agent Containers Operation Management Guidelines

## 1. Overview
SigmaOS incorporates advanced containerization and isolation engines managed by AI system agents (such as `ContainerOperationManager`, `FreeBsdJailManager`, `QubesIsolationGovernor`, `FlatpakContainerBuilder`, and `ApexContainerEngine`). These guidelines define Linux cgroups v2 / namespaces, FreeBSD Jails, Qubes OS Xen/KVM VM isolation, Flatpak / Snap / AppImage sandboxing, Android APEX container modules, and immutable A/B OverlayFS layer management for AI agents in SigmaOS.

## 2. Core Containers Operation Management Principles

### 2.1 Namespaces & cgroups v2 Resource Governance
- **Linux Namespaces**: Containerized AI agents execute within isolated PID, Mount (`mnt`), Network (`net`), IPC, UTS, User (`user`), and Time namespaces (`src/kernel/namespace.rs`).
- **cgroups v2 Control Groups**: Resource quotas (`ContainerResourceGovernor`) restrict max RAM (`memory.max`), CPU bandwidth (`cpu.max`), block I/O (`io.weight`), and process limits (`pids.max`).

### 2.2 FreeBSD Jails & RACCT Resource Throttling
- **FreeBSD Jails Integration**: Agents orchestrate lightweight FreeBSD Jails (`FreeBsdJailManager`) with virtualized network stacks (`VNET`), isolated `chroot` root filesystems, and UMA memory caps.
- **RACCT Throttling**: System resource usage is governed via RACCT rules, dynamically throttling container CPU percentages or memory allocations without killing container workloads.

### 2.3 Qubes OS Compartmentalization & VM Isolation
- **Qubes VM Isolation**: High-security AI workflows execute inside Qubes-style isolated micro-VMs (`QubesIsolationGovernor`) backed by KVM vCPU execution loops (`KvmVcpuRegisters`) and VirtIO split rings.
- **Disposable AppVMs**: Ephemeral worker VM instances are spawned for untrusted tasks and destroyed immediately upon task completion.

### 2.4 Flatpak, Snap & Android APEX Sandboxing
- **Flatpak & Snap App Sandboxing**: Userland application containers enforce capability-gated portals (`DesktopPortal`) and SquashFS image mounts (`src/package/declarative_app.rs`).
- **Android APEX Container Modules**: System updates utilize immutable APEX container modules with post-quantum signature verification (`sigma attest`).

### 2.5 Immutable App Layers & A/B Slot Updates
- **OverlayFS & SquashFS Layers**: Container layers are composed of read-only SquashFS base layers merged with writable tmpfs/CoW overlays via OverlayFS.
- **Atomic A/B Slot Updates**: Container image updates apply to inactive A/B storage slots with cryptographic digest verification before atomic slot toggling.

---
*Maintained by the SigmaOS Virtualization, Containers & Security Steering Committee.*

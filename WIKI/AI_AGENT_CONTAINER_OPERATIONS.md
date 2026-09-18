# AI Agent Container Operations Management in SigmaOS

## Overview
SigmaOS incorporates a lightweight, OCI-compliant Container Operations Subsystem governed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, OCI runtime specifications, cgroups v2 resource bounds, namespace isolation models, and security profile generators for AI agents supervising container lifecycles across SigmaOS.

AI agents interact directly with `src/container/oci_runtime.rs`, `src/container/runtime.rs` (`SovereignContainerEngine`), `src/container/distro_sandbox.rs`, and `src/security/user_namespace.rs` (`UserNamespaceManager`).

---

## 1. Container Subsystems & Architecture

### 1.1 OCI Container Runtime (`src/container/oci_runtime.rs`)
Implemented in `src/container/oci_runtime.rs`. Provides Open Container Initiative (OCI) image layer unpacking, rootfs pivot root setup, and container execution hooks (`create`, `start`, `pause`, `resume`, `kill`, `delete`).

### 1.2 Sovereign Container Engine (`src/container/runtime.rs`)
Implemented in `src/container/runtime.rs`. Manages container lifecycle state transitions, overlayfs layer composition, and container resource limits:
* **Namespace Isolation**: Enforces PID, Mount, Network, IPC, UTS, and User namespace barriers for every container process.
* **Cgroups v2 Resource Isolation**: Controls CPU quota (`cpu.max`), memory limits (`memory.max`), and I/O weight (`io.weight`).

### 1.3 Distro Container Sandboxing (`src/container/distro_sandbox.rs`)
Implemented in `src/container/distro_sandbox.rs`. Executes foreign Linux distribution applications (Ubuntu, Arch, Fedora, Alpine) inside zero-latency, chroot/pivot_root container sandboxes.

### 1.4 Unprivileged User Namespace Mapping (`src/security/user_namespace.rs`)
Implemented in `src/security/user_namespace.rs`. Maps container root (`UID 0`) to unprivileged host UIDs (`UID 100000+`), eliminating root privilege escalation risks.

---

## 2. AI Agent Operational Rules & Protocols

### 2.1 Container Lifecycle Governance
1. **Unprivileged Root Execution**:
   AI agents must verify that all container instances execute inside dedicated user namespaces (`UserNamespaceManager`), mapping container UID 0 to an unprivileged host UID allocation range.
2. **Resource Bound Enforcement**:
   **Bolt** ⚡ assigns strict cgroups v2 memory (`memory.max`) and CPU quotas (`cpu.max`) to container workloads, preventing container noisy-neighbor resource starvation.

### 2.2 Security Profile Synthesis
* **Sentinel 🛡️ Inspection**:
  Before starting untrusted OCI container images, **Sentinel** 🛡️ synthesizes AppArmor/SELinux profiles and applies OpenBSD Pledge/Unveil restrictions to container process handles.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active OCI containers and cgroups v2 resource usage
sigma-container list

# Create and start an isolated OCI container instance
sigma-container run --image docker.io/library/alpine:latest --name alpine_sandbox

# Pause and snapshot container execution state
sigma-container pause --name alpine_sandbox
```

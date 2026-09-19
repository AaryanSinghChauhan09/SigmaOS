# AI Agent Cloud Service Operations Management in SigmaOS

## Overview
SigmaOS incorporates a cloud-native Orchestration and Cloud Service Management Subsystem governed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, pod scheduling rules, cross-device service mesh routing protocols, remote desktop session security, and cloud storage policies for AI agents supervising cloud services across SigmaOS.

AI agents interact directly with `src/orchestration/sigmakube.rs` (`SigmaKubeClusterEngine`), `src/orchestration/cross_device.rs` (`CrossDeviceMeshEngine`), `src/remote/desktop.rs`, and `src/cloud/storage.rs`.

---

## 1. Cloud Service Subsystems & Architecture

### 1.1 SigmaKube Cluster Orchestration (`src/orchestration/sigmakube.rs`)
Implemented in `src/orchestration/sigmakube.rs`. Provides lightweight, Kubernetes-inspired container pod scheduling, deployment rollouts, service load balancing, and node health probes:
* **Pod Scheduling**: Assigns workloads to worker nodes based on CPU/RAM availability, affinity rules, and priority classes.
* **Auto-Scaling & Self-Healing**: Automatically reschedules failed pods and scales replica counts under high CPU load.

### 1.2 Cross-Device Service Mesh (`src/orchestration/cross_device.rs`)
Implemented in `src/orchestration/cross_device.rs`. Connects edge devices, laptops, microVMs, and cloud servers into a unified WireGuard/mTLS-secured service mesh, supporting seamless service migration and remote procedure calls.

### 1.3 Remote Cloud Desktop & Shell (`src/remote/desktop.rs`)
Implemented in `src/remote/desktop.rs`. Manages encrypted VNC/RDP/SSH remote cloud desktop sessions and terminal shells over TLS/Dilithium-5 secure tunnels.

---

## 2. AI Agent Operational Rules & Directives

### 2.1 Cluster Pod Scheduling & Load Balancing
1. **Resource Aware Placement**:
   **Bolt** ⚡ queries worker node CPU/memory capacity, placing heavy container pods on high-core nodes while steering low-latency pods to edge nodes.
2. **Zero-Downtime Rollouts**:
   Agents execute rolling deployment updates, verifying readiness probes before terminating legacy pod instances.

### 2.2 Cross-Device Service Migration
* **Seamless State Transfer**:
  When a local device low-power state is triggered, agents migrate active cloud service workloads to peer mesh nodes via `CrossDeviceMeshEngine::migrate_service()`.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active SigmaKube pods and cluster node status
sigma-cloudkube get-pods

# Trigger rolling deployment update for cloud service
sigma-cloudkube rollout --service auth-service --image v2.0.0

# Inspect cross-device WireGuard service mesh status
sigma-cloudkube mesh-status
```

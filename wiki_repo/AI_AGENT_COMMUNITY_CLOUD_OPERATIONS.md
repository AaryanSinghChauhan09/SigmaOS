# AI Agent Community Cloud Operations in SigmaOS

## Overview
SigmaOS incorporates a decentralized, community-driven Cloud & Infrastructure Subsystem governed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, P2P state synchronization protocols, cloud storage encryption policies, and infrastructure management rules for AI agents supervising community cloud nodes, container-native OS updates, and contributor infrastructure.

AI agents interact directly with `src/cloud/storage.rs`, `src/cloud/sync.rs`, `src/community/infrastructure.rs`, `src/community/contrib.rs`, and `src/distro_inspirations.rs` (`RancherOsCloudConfig`).

---

## 1. Community Cloud Subsystems & Architecture

### 1.1 Encrypted Community Cloud Storage (`src/cloud/storage.rs`)
Provides zero-knowledge, encrypted cloud object storage for community artifacts, ISO images, and package caches:
* **Chunk Encryption**: Encrypts storage blocks using AES-GCM / Dilithium-5 post-quantum signed metadata.
* **Content-Addressed Storage (CAS)**: Deduplicates cloud objects via SHA-256 / BLAKE3 chunk hashes.

### 1.2 P2P Cloud State Synchronization (`src/cloud/sync.rs`)
Provides peer-to-peer (P2P) mesh state synchronization across community nodes:
* **Merkle Tree Delta Sync**: Synchronizes package store states and system generations using Merkle tree delta comparisons (`SimpleCloudSync`).
* **BitTorrent/IPFS Seed Mesh**: Distributes system update images across community peer nodes without centralized bandwidth bottlenecks.

### 1.3 Container Cloud Configuration (`RancherOsCloudConfig`)
Implemented in `src/distro_inspirations.rs`. Manages cloud-config YAML definitions (`ros config`), dual-docker daemon isolation (system-docker vs user-docker), and container-native OS node upgrades.

### 1.4 Community Infrastructure & Contributor Governance
Implemented in `src/community/infrastructure.rs` and `contrib.rs`. Manages build farm worker nodes, automated CI/CD runners, bug bounty telemetry, and contributor credential verification.

---

## 2. AI Agent Operational Directives & Workflows

### 2.1 P2P Node Discovery & Synchronization Protocol
1. **Peer Handshake**:
   AI agents authenticate peer nodes using Dilithium-5 public key signatures.
2. **Delta Sync & Rate Limiting**:
   **Bolt** ⚡ monitors network bandwidth, scheduling P2P state syncs during low-traffic intervals to prevent saturation of community mirror nodes.

### 2.2 Container Cloud Node Upgrades
* **Dual-Daemon Isolation**:
  Agents verify that system-critical daemons execute strictly in system-docker (`RancherOsCloudConfig::has_dual_daemons()`), isolating system processes from user workload container crashes.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active P2P community cloud sync peers and Merkle state
sigma-cloud sync-status

# Verify zero-knowledge encrypted storage bucket integrity
sigma-cloud storage-audit --bucket community-packages

# Apply RancherOS-style cloud-config update to container node
sigma-cloud apply-config --file /etc/sigma/cloud-config.yml
```

# OSS Absorption: Ceph — Distributed Object & Block Storage

> **Status**: 📋 Planned | **Source Project**: Ceph | **Target Shard**: `SigmaOS Distributed Storage`

---

## 1. Executive Summary

Ceph is an open-source, software-defined storage platform that implements object, block, and file storage on a single distributed cluster. It relies on the CRUSH (Controlled Replication Under Scalable Hashing) algorithm to distribute data objects deterministically across storage devices.

SigmaOS absorbs the **CRUSH map design** of Ceph into the `sigma-nebula` distributed storage layer, enabling multi-node deployments to scale block devices without central metadata lookup servers.

---

## 2. Key Features Absorbed

### 2.1 CRUSH Algorithm Routing

Instead of querying a centralized server to locate files, `sigma-nebula` storage clients compute the physical block address directly using the CRUSH algorithm. This guarantees that file retrieval remains fast and completely decentralized.

---

## 3. References & Standards

- Ceph — `ceph.io` (LGPL-2.1)

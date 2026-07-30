# 💾 SigmaOS: Next-Gen Crash-Consistent Filesystem (SigmaFS) Development Plan

This document maps out the strategic engineering blueprint and system design plan for **SigmaFS**, the functional, zero-dependency, crash-consistent, and highly optimized storage system for **SigmaOS**.

---

## 🏛️ 1. ARCHITECTURAL VISION

SigmaFS is architected to dismantle the performance degradation, metadata fragmentation, and crash vulnerability associated with traditional POSIX filesystems (such as ext4, NTFS, and legacy FAT).

```
+-----------------------------------------------------------------------------------+
|                                SIGMAFS STORAGE STACK                              |
+-----------------------------------------------------------------------------------+
|   [SovereignCatalog Schema]  |   [Zero-Copy Sector Cache]   |   [SIMD CRC32C]     |
+-----------------------------------------------------------------------------------+
|               Cryptographically-Verifiable Merkle Tree Layer (CoW)                |
+-----------------------------------------------------------------------------------+
|                  JBD2-Style Crash-Resilient Transactional Journal                 |
+-----------------------------------------------------------------------------------+
```

---

## 🏗️ 2. CORE COMPONENT PLANS & OBJECT-ORIENTED DESIGN

All filesystem components represent highly cohesive, memory-isolated classes using pure-OOP design patterns:

### 2.1 Cryptographically-Verifiable Merkle Tree Layout (Copy-on-Write)
* **The ZFS/Btrfs Flaw:** While ZFS provides data integrity, its storage pool allocation blocks require heavy dynamic heap memory allocations and complex memory-mapped cache algorithms.
* **The SigmaFS Solution:** Implements a strict static allocation structure over logical block arrays. All data blocks, directory trees, and file contents are mapped as nodes in a master Merkle tree. Every write operation is strictly **Copy-on-Write (CoW)**, never overwriting existing blocks in place.

### 2.2 JBD2-Style Crash-Resilient Transactional Journal
* **Integrity:** Integrates transactional logging featuring explicit Descriptor, Commit, and Revoke block semantics.
* **Crash-Consistency:** Write operations are committed atomically. If a system failure or power loss occurs mid-transaction, the boot recovery walks back the Merkle-tree roots to the last verified, signed commit point, guaranteeing sub-millisecond atomic rollbacks with zero data corruption.

### 2.3 Polymorphic Storage Controller Interface
* **Abstractions:** Exposes an abstract trait `BlockStorageDevice` extending the unified peripheral interface.
* **Classes:** Specific classes (such as `NvmeStorageController`, `AhciSataController`, and `IdeFloppyController`) implement block-aligned sector reads and writes natively using static allocation.

---

## 📅 3. STEP-BY-STEP IMPLEMENTATION TIMELINE

* **Phase I: Block-Level Log Allocation (Months 1-2):**
  Establish the physical sector mapping layout and log-structured write paths for solid-state storage (NVMe/SATA).
* **Phase II: Merkle-Node Parsing & CoW Trees (Months 2-4):**
  Implement the hierarchical Merkle tree structure on disk, enabling directory creation and transactional file operations.
* **Phase III: Crash-Resilient Transaction Ledger (Months 4-5):**
  Integrate the JBD2-style transactional logging and CRC32C block checksum verifications.
* **Phase IV: Automated Snapshots & Verification (Months 5-6):**
  Develop background sector integrity auditors and sub-millisecond, zero-reboot generation rollback commands.

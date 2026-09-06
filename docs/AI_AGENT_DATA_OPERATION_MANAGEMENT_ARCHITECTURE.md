# AI Agent Data Operation Management Architecture

## Executive Overview

Data Operation Management in SigmaOS governs Content-Addressed Storage (CAS) data block persistence, transactional journaling, post-quantum cryptographic (PQC) data signing, FreeBSD Soft Updates metadata dependency ordering, and zero-copy block I/O operations. Implemented across `src/filesystem/sigmafs.rs`, `src/filesystem/sigma_fs.rs`, `src/filesystem/bsd_linux_innovations.rs`, and `src/filesystem/ext4.rs`, SigmaOS secures data at rest using Dilithium-5/Kyber-1024 signatures (`pqc_secure_sign`) and DAG node storage (`store_node`, `DagNode::DataBlock`) with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents reading, writing, encrypting, or committing filesystem data in SigmaOS.

---

## Subsystem Architecture & Data Operations Pipeline

```
                                +-----------------------------------+
                                |    Application Data Write (bio)   |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |     SigmaFS Data Engine           |
                                |    (src/filesystem/sigmafs.rs)    |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | Content-Addressed DAG |   | PQC Data Signing  |   | FreeBSD Soft Updates  |
            | DagNode::DataBlock(b) |   | pqc_secure_sign() |   | BsdSoftUpdatesEngine  |
            | HashId Verification   |   | pqc_verify_sig()  |   | MetadataDependency    |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |    Journal Transaction Commit     |
                                |  start_transaction("write", path) |
                                +-----------------------------------+
```

### Core Data Operation Components

1. **Content-Addressed Storage DAG (`src/filesystem/sigmafs.rs`)**:
   - `DagNode::DataBlock(Vec<u8>)`: Immutable leaf node holding raw file payload.
   - `store_node(node)`: Computes $H(data)$ hash ID, encrypts payload at rest, and stores node in `cas: BTreeMap<HashId, DagNode>`.

2. **Post-Quantum Cryptographic Data Signing (`src/filesystem/sigma_fs.rs`)**:
   - `pqc_secure_sign(data, key_id)`: Generates post-quantum digital signatures for data blocks using Dilithium-5/Kyber-1024 keys.
   - `pqc_verify_signature(data, sig)`: Verifies data integrity prior to block write completion.

3. **FreeBSD Soft Updates Metadata Dependency Engine (`src/filesystem/bsd_linux_innovations.rs`)**:
   - `BsdSoftUpdatesEngine`: Enforces strict dependency ordering (`MetadataDependency`, `MetadataOp`) across inode allocation, directory entries, and data block writes to guarantee crash consistency without synchronous write overhead.

4. **Transactional Journaling (`src/filesystem/sigma_fs.rs`)**:
   - `start_transaction(action, path, data)`: Records atomic file modification logs before flushing dirty buffers to block storage.

---

## Zero-Allocation Guardrails

AI agents executing filesystem data operations must observe these constraints:
- Data block hashing (`compute_hash`) operates in-place on borrowing byte slices (`&[u8]`).
- Sector-level encryption operates on mutable byte slices (`&mut [u8]`) without allocating intermediate heap buffers.

---

## Related Architectural References
- `src/filesystem/sigmafs.rs` - Content-Addressed File System.
- `src/filesystem/sigma_fs.rs` - Master SigmaFS journal and PQC encryption engine.
- `src/filesystem/bsd_linux_innovations.rs` - FreeBSD Soft Updates engine.
- `docs/AI_AGENT_IO_MANAGEMENT_ARCHITECTURE.md` - Zero-copy ring I/O architecture.

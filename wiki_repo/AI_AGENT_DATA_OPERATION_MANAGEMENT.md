# AI Agent Data Operation Management Guide

## Overview
This wiki guide details Data Operation Management protocols for AI coding agents operating on SigmaOS. It covers Content-Addressed Storage (CAS) data block writing (`write_file`, `DagNode::DataBlock`), transactional journaling (`start_transaction`), post-quantum cryptographic (PQC) data signing (`pqc_secure_sign`), and FreeBSD Soft Updates metadata dependency ordering (`BsdSoftUpdatesEngine`).

## Key Principles
1. **CAS Data Block Deduplication**: File payloads stored as immutable DAG data blocks indexable by cryptographic hash ID.
2. **PQC Data Integrity**: Kyber-1024 / Dilithium-5 signatures validate data payload authenticity at rest.
3. **Soft Updates Consistency**: Metadata dependencies ensure block allocations commit prior to directory entry updates.

## Data Block Storage Example (`src/filesystem/sigmafs.rs`)
```rust
let mut fs = ContentAddressedFs::new();
let hash = fs.write_file(b"Important sovereign data payload")?;
```

## Related Documents
- `docs/AI_AGENT_DATA_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_DATA_OPERATION_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_MEMORY_OPERATION_MANAGEMENT.md`

# 🗄️ SigmaFS — Sovereign Filesystem Innovations

> SigmaOS introduces **SigmaFS**, a next-generation sovereign filesystem designed from first principles. It discards the decades-old inode model in favor of an object-capability storage architecture with built-in post-quantum encryption and content-addressed deduplication.

---

## 🏗️ Architecture Overview

```
┌──────────────────────────────────────────────────────┐
│                   Application Layer                   │
│         (POSIX VFS interface — full compat)           │
├──────────────────────────────────────────────────────┤
│                  VFS Abstraction Layer                 │
│         (Pluggable backend registry)                  │
├─────────────┬─────────────┬──────────────────────────┤
│  SigmaFS    │  Ext4       │   FAT32 / exFAT           │
│  (Native)   │  (Compat)   │   (Removable / EFI)       │
├─────────────┴─────────────┴──────────────────────────┤
│              Storage Abstraction Layer                 │
│    (NVMe / SATA / USB / virtio-blk backends)          │
├──────────────────────────────────────────────────────┤
│            S-SEC Capability Gate                       │
│   (All storage I/O validated through cap tokens)       │
└──────────────────────────────────────────────────────┘
```

---

## ✨ SigmaFS Key Innovations

### 1. Content-Addressed Storage (CAS)

Every file block is stored by its cryptographic hash (SHA3-512), not its name or location. This provides:

- **Zero-redundancy**: Identical blocks stored once regardless of how many files reference them.
- **Instant cloning**: Cloning a file or directory costs O(1) — just create a new reference.
- **Integrity by design**: Any bit corruption is immediately detected.
- **Deduplication across users**: System libraries shared at the block level.

```rust
// Content-addressed store insert
let store = ContentAddressedStore::new();
let hash = store.insert(file_data)?;
// Same data referenced twice → same hash, stored once
let hash2 = store.insert(file_data)?;
assert_eq!(hash, hash2);
```

### 2. Post-Quantum Transparent Encryption

Every file is encrypted at rest using a hybrid scheme:
- **AES-256-GCM** for bulk data encryption (hardware-accelerated via AES-NI)
- **Kyber-1024** for key encapsulation (NIST FIPS 203)
- **Dilithium-5** for file metadata signatures (NIST FIPS 204)

Key derivation uses the per-file capability token as additional authenticated data (AAD), making file decryption impossible without the correct capability token.

### 3. Capability-Gated File Access

Traditional filesystems use user IDs and permission bits. SigmaFS uses **object capabilities**:

```rust
// Capability token determines what you can do with a file
let file_cap = CapabilityToken::new()
    .allow_read("/home/user/documents")
    .allow_append("/home/user/logs")
    .deny_execute_anywhere();

// The filesystem enforces this at the block driver level
let fd = sigmfs.open("/home/user/documents/report.pdf", file_cap)?;
```

### 4. Copy-on-Write Snapshots

Full filesystem snapshots are O(1) using CoW semantics:
- All writes go to new blocks; old blocks remain until snapshot is dropped.
- System snapshots taken automatically before every package install.
- Manual snapshot: `sigma-fs snapshot create "pre-update"`
- Rollback: `sigma-fs snapshot restore "pre-update"`

### 5. Integrated Journaling

SigmaFS uses a write-ahead log (WAL) with three durability levels:
- **Writeback**: Maximum performance; metadata journaled only.
- **Ordered** (default): Data written before metadata commits.
- **Data**: Full journaling of all data; maximum durability.

---

## 📊 SigmaFS vs. Competitor Filesystems

| Feature | SigmaFS | Ext4 | Btrfs | ZFS | APFS |
|---------|---------|------|-------|-----|------|
| CAS Deduplication | ✅ Native | ❌ | Partial | ✅ | Partial |
| PQC Encryption | ✅ | ❌ | ❌ | ❌ | ❌ |
| Capability Access | ✅ | ❌ | ❌ | ❌ | Partial |
| CoW Snapshots | ✅ | ❌ | ✅ | ✅ | ✅ |
| O(1) Clone | ✅ | ❌ | ✅ | ✅ | ✅ |
| RAID built-in | ✅ | ❌ | ✅ | ✅ | ❌ |
| Kernel-native | ✅ | ✅ | ✅ | External | macOS only |
| Max file size | 2^128 bytes | 16TB | 16EB | 16EB | 8EB |
| Journaling | ✅ WAL | ✅ | ✅ | ✅ | ✅ |

---

## 🔧 Disk Layout

```
┌─────────────────────────────────────────────────────┐
│  Block 0: Superblock                                  │
│  (magic, version, CAS root hash, journal offset)      │
├─────────────────────────────────────────────────────┤
│  Block 1-N: Journal WAL                               │
│  (transaction log, replay-on-crash)                   │
├─────────────────────────────────────────────────────┤
│  Block N+1 onwards: CAS Object Store                  │
│  (content-addressed blocks, Merkle tree indexed)      │
├─────────────────────────────────────────────────────┤
│  Inline Encryption Metadata (per-block IV/tag)        │
└─────────────────────────────────────────────────────┘
```

---

## 🗂️ Virtual Filesystem Layer

The VFS provides a POSIX-compatible interface over multiple backends:

```rust
pub trait Filesystem {
    fn mount(&mut self, device: &str) -> Result<(), FsError>;
    fn read(&self, path: &str, buf: &mut [u8]) -> Result<usize, FsError>;
    fn write(&mut self, path: &str, data: &[u8]) -> Result<usize, FsError>;
    fn stat(&self, path: &str) -> Result<Inode, FsError>;
    fn mkdir(&mut self, path: &str) -> Result<(), FsError>;
    fn unlink(&mut self, path: &str) -> Result<(), FsError>;
}
```

Registered backends include:
- `SigmaFSBackend` — native sovereign filesystem
- `Ext4Backend` — Linux ext4 read/write
- `Fat32Backend` — FAT32 for EFI and removable media
- `ArchiveBackend` — Mounting archives as virtual filesystems (tar, zip, zstd)
- `NetworkBackend` — Mounting remote SigmaFS volumes over PQC-encrypted channels

---

## 📦 Archive Subsystem

The archive module handles compressed archive formats without external tools:

| Format | Compress | Decompress | Status |
|--------|----------|------------|--------|
| tar | ✅ | ✅ | Complete |
| zip | ✅ | ✅ | Complete |
| zstd | ✅ | ✅ | Complete |
| lz4 | ✅ | ✅ | Complete |
| xz | ✅ | ✅ | Complete |
| bz2 | ✅ | ✅ | Complete |
| 7z | ⬜ | ✅ | Decompress only |

---

## 🔗 Related Pages

- [Advanced Absorption Matrix](Advanced_Absorption) — App replacement strategy
- [Security Framework](Security_Framework) — PQC + Capability security
- [Maturity & Distro-Parity Roadmap](Maturity_Parity_Roadmap) — Phase plan

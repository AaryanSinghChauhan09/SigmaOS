# Storage & Filesystems Architecture

The **SigmaOS Storage and Filesystems Subsystem** provides a modular, high-throughput, crash-resilient storage hierarchy. It features a zero-copy Virtual Filesystem (VFS), direct hardware NVMe/AHCI drivers, and native support for legacy and next-generation filesystems without third-party FUSE bloat.

---

## 1. Subsystem Architecture

### 1.1 Virtual Filesystem (VFS) & Cache Layer
- **Location**: `src/fs/vfs.rs`, `src/filesystem/mod.rs`
- **Specification**: Inode, dentry, and superblock abstractions supporting hierarchical mounting, symbolic links, file locks (`flock`), and memory-mapped files (`mmap`). Includes a lock-free Read-Copy-Update (RCU) dentry cache for sub-microsecond path lookups.

### 1.2 Storage Drivers: NVMe 2.0 & AHCI SATA
- **NVMe Driver (`src/driver/nvme.rs`, `src/driver/nvme_storage.rs`, `src/drivers/modern_nvme.rs`)**:
  - High-performance Non-Volatile Memory Express implementation.
  - Multi-queue architecture with independent submission (SQ) and completion (CQ) queues per CPU core.
  - Asynchronous command doorbell signaling with MMIO registers.
- **AHCI Driver (`src/driver/ahci.rs`)**:
  - Advanced Host Controller Interface driver for SATA SSDs and HDDs.
  - Physical Region Descriptor Tables (PRDT) supporting up to 4MB scatter-gather transfers per entry.
  - Native Command Queuing (NCQ) for optimized rotational and flash access patterns.

### 1.3 Native Filesystem Drivers
- **FAT32 (`src/filesystem/fat32.rs`)**:
  - Complete File Allocation Table implementation supporting long file names (LFN), cluster chaining, and EFI System Partition (ESP) mounting.
- **NTFS (`src/filesystem/ntfs.rs`)**:
  - Native New Technology File System driver. Parses Master File Table (MFT) records, `$STANDARD_INFORMATION`, `$FILE_NAME`, and `$DATA` non-resident data runs.
- **SMB / CIFS Client (`src/filesystem/smb_client.rs`)**:
  - Native Server Message Block v2/v3 client supporting network share discovery, tree connect, session setup, and encrypted network file operations.
- **LUKS2 Encryption (`src/filesystem/luks2.rs`)**:
  - Linux Unified Key Setup v2 disk encryption layer with Argon2id key derivation and AES-XTS / ChaCha20-Poly1305 transparent sector encryption.
- **Linux & BSD Heavyweights**:
  - **Ext4**: Extent trees, journal transactions (JBD2-compatible), and 64-bit block numbers.
  - **ZFS**: Storage pool (zpool) management, Copy-on-Write (CoW) snapshots, and end-to-end data checksumming.
  - **Btrfs**: B-tree filesystem structure with subvolume isolation and online defragmentation.

### 1.4 Virtual & Kernel Control Filesystems
- **`configfs` (`src/filesystem/configfs.rs`)**: Userland kernel object creation and configuration hierarchy through standard directory operations (`mkdir`/`rmdir`).
- **`debugfs` (`src/filesystem/debugfs.rs`)**: Low-overhead runtime diagnostic exports for kernel profiling and tracepoint inspection.

---

## 2. Storage System Parity Matrix

| Feature | Linux | FreeBSD | OpenBSD | SigmaOS Native Engine |
|---|---|---|---|---|
| **NVMe Multi-Queue** | `nvme` driver | `nvd` driver | `nvme` driver | Native Rust `modern_nvme.rs` |
| **FAT32 ESP Support** | `vfat` | `msdosfs` | `msdos` | Built-in `fat32.rs` |
| **NTFS Read/Write** | NTFS3 / FUSE | FUSE | Read-only | Native Rust `ntfs.rs` |
| **Encrypted Disk** | dm-crypt (LUKS2) | GELI | Softraid | Built-in `luks2.rs` |
| **Network Storage** | cifs.ko | mount_smbfs | Limited | Native Rust `smb_client.rs` |
| **Kernel Configuration** | `configfs` | sysctl | sysctl | Native `configfs.rs` |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: All filesystem drivers and storage controllers must be written in **100% Rust** or **Zig**. No C FUSE wrappers or external libraries are permitted.
2. **Buffer Safety**: Sector read and write operations MUST perform strict bounds checking on byte buffers. Never allow writes beyond the allocated sector capacity.
3. **Transaction Durability**: Filesystems supporting journaling (Ext4, ZFS, Btrfs) must write journal commits and issue hardware flush commands (`SYNCHRONIZE CACHE` / NVMe Flush) before updating metadata pointers.
4. **Zero Predefined Dependencies**: Do not introduce external crates for block decoding or archive unpacking. Rely exclusively on SigmaOS's native primitives.
5. **Compilation Verification**: All storage code must verify cleanly:
   ```bash
   cargo check --lib
   ```

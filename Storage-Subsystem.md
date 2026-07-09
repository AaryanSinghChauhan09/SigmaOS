# Storage Subsystem

Sovereign isolation layer for storage operations.

## Architecture

```
Application Layer
   └─ Storage API (POSIX-compatible)
         └─ Storage Manager
               ├─ SovereignFS (default, CoW, journaling)
               ├─ Ext4 (Linux compatibility)
               ├─ FAT32 (removable media)
               └─ Web3FS (IPFS-backed)
                     └─ Block Device Layer
                           ├─ NVMe driver
                           ├─ AHCI SATA driver
                           └─ USB storage driver
```

## Supported Filesystems

| Filesystem | Status | Features |
|---|---|---|
| SovereignFS | ✅ Active | Copy-on-Write, journaling, BLAKE3 integrity, snapshots |
| Ext4 | ✅ Read-only | Linux compatibility, journaling |
| FAT32 | ✅ Read/Write | Removable media compatibility |
| Web3FS | 🔧 In-progress | IPFS/Filecoin-backed decentralized storage |

## API Interface

```c
// Mount a filesystem
int storage_mount(const char *device, const char *mountpoint, const char *fstype, uint32_t flags);

// Unmount a filesystem
int storage_unmount(const char *mountpoint);

// Open a file
int storage_open(const char *path, uint32_t flags, uint32_t mode);

// Read from a file
ssize_t storage_read(int fd, void *buf, size_t count);

// Write to a file
ssize_t storage_write(int fd, const void *buf, size_t count);

// Close a file
int storage_close(int fd);

// Create a snapshot (SovereignFS only)
int storage_snapshot(const char *mountpoint, const char *tag);

// Rollback to a snapshot (SovereignFS only)
int storage_rollback(const char *mountpoint, const char *tag);

// Get filesystem statistics
int storage_statfs(const char *path, statfs_t *stat);

// Initialize storage subsystem
void init_storage(void);
```

## SovereignFS Features

### Copy-on-Write (CoW)
- Every write creates a new extent
- Old data preserved until explicitly pruned
- Enables instant snapshots without data duplication

### Journaling
- Write-ahead logging for crash consistency
- Atomic operations for metadata updates
- Fast recovery after power failure

### Block Integrity
- Each 4 KB block carries a BLAKE3 checksum
- Kernel rejects tampered blocks at read time
- Detects silent data corruption

### Snapshots
- Instant point-in-time filesystem state
- Minimal space overhead (CoW)
- Tag-based snapshot management
- Automatic snapshot retention policy

## Block Device Drivers

### NVMe Driver
- NVMe 1.4 specification compliance
- Multiple I/O queues (up to 64K queues)
- 64KB maximum transfer size per command
- Namespace management
- Power management and thermal throttling

### AHCI SATA Driver
- SATA 3.0 (6 Gbps) support
- NCQ (Native Command Queuing)
- SMART monitoring
- Hot-plug support

### USB Storage Driver
- USB Mass Storage Class (MSC) support
- Bulk-only transport (BOT)
- UASP (USB Attached SCSI Protocol)
- Automatic device detection

## Performance Characteristics

| Operation | Latency | Throughput |
|---|---|---|
| NVMe 4K random read | ~10µs | ~500MB/s |
| NVMe 4K random write | ~20µs | ~400MB/s |
| SATA 4K random read | ~100µs | ~150MB/s |
| SATA 4K random write | ~200µs | ~120MB/s |
| SovereignFS snapshot | <1ms | N/A |

## Security Features

- **Capability-based access**: File operations require capability tokens
- **Namespace isolation**: Each shard has isolated filesystem view
- **Encryption at rest**: Optional AES-256-XTS encryption
- **Secure erase**: Cryptographically secure data deletion
- **Audit logging**: All storage operations logged to audit chain

## Roadmap

- [x] SovereignFS basic implementation
- [x] Ext4 read-only driver
- [x] FAT32 read/write driver
- [x] NVMe driver
- [x] AHCI SATA driver
- [ ] Web3FS IPFS integration
- [ ] Btrfs support
- [ ] ZFS support (ZOL)
- [ ] RAID support (0, 1, 5, 6, 10)
- [ ] Distributed storage (Ceph-like)
- [ ] Storage tiering (SSD cache + HDD archive)
- [ ] Compression (zstd, lz4)
- [ ] Deduplication

## Related Modules

- [`modules/core/fs`](../modules/core/fs/README.md) — VFS layer
- [`modules/core/drivers`](../modules/core/drivers/README.md) — Block device drivers
- [`security/pqc/README.md`](../security/pqc/README.md) — Cryptographic integrity

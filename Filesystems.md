# SigmaOS Filesystems

## Supported Filesystems

| Filesystem | Status | Default Use |
|------------|--------|-------------|
| **Btrfs** | ✅ | Root (default) |
| **ext4** | ✅ | Data partitions |
| **XFS** | ✅ | Large files |
| **FAT32/exFAT** | ✅ | USB/EFI |
| **NTFS** | ✅ | Windows interop |
| **EROFS** | ✅ | Container layers |
| **OverlayFS** | ✅ | Containers |
| **tmpfs** | ✅ | /tmp, /run |
| **ZFS** | 📋 | Storage pools |

## Btrfs (Default Root)

Key features:
- Copy-on-Write (all writes are CoW)
- Subvolumes (independent namespace)
- Snapshots (instant CoW clones)
- RAID 0/1/5/6/10 (built-in)
- Compression (zstd/lzo/zlib)
- Checksums (CRC32c/xxHash)
- Deduplication (offline)

### Subvolume Layout

```
/dev/nvme0n1p2 (btrfs pool)
├── @           → /
├── @home       → /home
├── @snapshots  → snapshot storage
├── @var        → /var
└── @tmp        → /tmp
```

### Automatic Snapshots

| Trigger | Created | Retention |
|---------|---------|-----------|
| Pre-install | ✅ | 5 most recent |
| Daily | ✅ | 7 days |
| Weekly | ✅ | 4 weeks |

## OverlayFS (Containers)

```
upperdir: /containers/CTID/upper  (writable)
lowerdir: image/layer3 : layer2 : layer1  (read-only)
merged:   /containers/CTID/root  (mount point)
```

## EROFS (OCI Layers)

Read-only, compressed (LZ4/LZMA), O(1) random access, zero metadata overhead.

## Filesystem I/O Schedulers

| Scheduler | Best For |
|-----------|---------|
| mq-deadline | HDDs, mixed |
| kyber | Fast NVMe |
| bfq | Desktop interactivity |
| none | NVMe with own queue |

# Filesystems

SigmaOS supports multiple filesystem types with focus on performance and security.

## Supported Filesystems

### Native Filesystems

- **SigmaFS**: Native journaling filesystem
- **Btrfs**: Copy-on-write filesystem with snapshots
- **ZFS**: Advanced filesystem with data integrity
- **Ext4**: Linux-compatible filesystem with journaling

### Special Filesystems

- **ProcFS**: Process information (`/proc`)
- **SysFS**: Kernel object attributes (`/sys`)
- **TmpFS**: Temporary storage in RAM
- **DevFS**: Device files (`/dev`)

## Filesystem Management

### Creating Filesystems

Create a new filesystem:

```bash
# Format partition as Ext4
mkfs.ext4 /dev/sda1

# Format as Btrfs
mkfs.btrfs /dev/sda1

# Format as ZFS
zpool create tank /dev/sda1
zfs create tank/root
```

### Mounting Filesystems

Mount filesystems:

```bash
# Mount filesystem
mount /dev/sda1 /mnt/data

# Mount with options
mount -o noatime,ssd /dev/sda1 /mnt/data

# Mount filesystems from /etc/fstab
mount -a
```

### /etc/fstab Configuration

Configure automatic mounts:

```
# /etc/fstab
/dev/sda1 /mnt/data ext4 defaults,noatime 0 2
/dev/sda2 /mnt/backup btrfs defaults,compress 0 2
tank/root /mnt/zfs zfs defaults 0 0
```

## Btrfs

### Snapshots

Create and manage Btrfs snapshots:

```bash
# Create snapshot
btrfs subvolume snapshot /mnt/data /mnt/data/snap-$(date +%Y%m%d)

# List snapshots
btrfs subvolume list -s /mnt/data

# Delete snapshot
btrfs subvolume delete /mnt/data/snap-20240101
```

### Compression

Enable compression:

```bash
# Enable compression on mount
mount -o compress /dev/sda1 /mnt/data

# Enable compression on subvolume
btrfs property set /mnt/data compression lzo
```

## ZFS

### Pools and Datasets

Manage ZFS pools:

```bash
# Create pool
zpool create tank /dev/sda1

# Create dataset
zfs create tank/data

# Create snapshot
zfs snapshot tank/data@snap-$(date +%Y%m%d)

# List snapshots
zfs list -t snapshot
```

### Deduplication and Compression

Enable features:

```bash
# Enable compression
zfs set compression=lz4 tank/data

# Enable deduplication
zpool set dedup=on tank
```

## Encryption

### Filesystem Encryption

Encrypt filesystems:

```bash
# Create encrypted container
cryptsetup luksFormat /dev/sda1
cryptsetup luksOpen /dev/sda1 cryptdata
mkfs.ext4 /dev/mapper/cryptdata
mount /dev/mapper/cryptdata /mnt/encrypted
```

### fscrypt

Use per-directory encryption:

```bash
# Encrypt directory
fscrypt encrypt /home/user/sensitive

# Lock directory
fscrypt lock /home/user/sensitive

# Unlock directory
fscrypt unlock /home/user/sensitive
```

## Filesystem Monitoring

### Disk Usage

Check disk usage:

```bash
# Show disk usage
df -h

# Show directory size
du -sh /path/to/directory

# Show largest directories
du -h --max-depth=1 / | sort -h
```

### Inode Usage

Check inode usage:

```bash
# Show inode usage
df -i

# Count files in directory
find /path -type f | wc -l
```

## Filesystem Checks

### Ext4

Check and repair Ext4:

```bash
# Check filesystem
fsck.ext4 /dev/sda1

# Force check
fsck.ext4 -f /dev/sda1
```

### Btrfs

Check and repair Btrfs:

```bash
# Check filesystem
btrfs check /dev/sda1

# Repair filesystem
btrfs check --repair /dev/sda1
```

## Performance Tuning

### Mount Options

Optimize mount options:

```bash
# SSD optimization
mount -o noatime,discard,ssd /dev/sda1 /mnt/data

# HDD optimization
mount -o relatime,barrier=1 /dev/sda1 /mnt/data
```

### Filesystem Tunables

Configure filesystem parameters:

```bash
# Ext4 lazy init
tune2fs -O lazy_itable_init /dev/sda1

# Btrfs commit interval
echo 30 > /sys/fs/btrfs/sda1/commit_interval_secs
```

## Virtual Filesystem (VFS)

### VFS Operations

Linux VFS-inspired unified filesystem abstraction:

```bash
# Create directory
vfs mkdir /test 0755

# Create file
vfs create /test.txt 0644

# Write to file
vfs write /test.txt "Hello, World!"

# Read from file
vfs read /test.txt

# List directory
vfs readdir /

# Lookup path
vfs lookup /test
```

### VFS Features

- **Inode Management**: Unified inode allocation and tracking
- **Dentry Cache**: Directory entry caching for fast lookups
- **Mount Points**: Filesystem mounting and unmounting
- **Path Resolution**: Hierarchical path lookup
- **File I/O**: Unified read/write operations

## B-tree Implementation

### B-tree Operations

Linux Btrfs-inspired B-tree structures:

```bash
# Create B-tree
btree create 4

# Insert key-value pair
btree insert 1 1 0 "data"

# Search for key
btree search 1 1 0

# Get node count
btree count
```

### B-tree Features

- **Internal Nodes**: Index nodes for navigation
- **Leaf Nodes**: Data nodes storing key-value pairs
- **Automatic Splitting**: Node splitting when full
- **Balancing**: Automatic tree balancing

## Next Steps

- [Networking](06-Networking.md) - Network configuration
- [Security](07-Security.md) - Security and encryption
- [Kernel](04-Kernel.md) - Kernel configuration

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

## Reference projects and future roadmap

Study Linux ext4/JBD2 and VFS, Btrfs copy-on-write and recovery, FreeBSD GEOM/ZFS, NetBSD filesystem boundaries, and Alpine's small recovery environment. The presence of a filesystem module or an API in this page is not evidence of working on-disk support; verify each format against its implementation and boot/runtime path.

1. Specify VFS path, mount, permissions, error, and concurrency semantics.
2. Define on-disk invariants, write ordering, replay limits, and corruption handling for one supported filesystem.
3. Complete one snapshot and rollback path with interruption and recovery tests.
4. Validate storage device capacity, sector size, alignment, DMA limits, and removal behavior.
5. Add offline integrity checks and bound all cache accounting, including oversized entries.

**Completion evidence:** interrupted-write tests recover to a documented state; invalid or truncated metadata fails safely; recovery works without network access; supported formats and tested devices are named explicitly.

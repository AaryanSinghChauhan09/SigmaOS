# Filesystem Support Matrix

SigmaOS provides comprehensive filesystem support with compatibility across Linux, BSD, and other operating systems, including ZFS, Btrfs, ext4, XFS, and more.

## Overview

Filesystem support includes:
- Native filesystems: ext4, XFS, Btrfs, ZFS
- BSD filesystems: UFS, ZFS, Hammer2
- Network filesystems: NFS, SMB/CIFS, SSHFS
- Special filesystems: procfs, sysfs, tmpfs, devtmpfs
- Compatibility layers: FUSE, overlayfs, bind mounts
- Encryption: LUKS, dm-crypt, eCryptfs
- Snapshots: ZFS snapshots, Btrfs snapshots, LVM snapshots

## Supported Filesystems

### Native Linux Filesystems

#### ext4
- **Status**: Fully supported
- **Features**: Journaling, extents, large file support, delayed allocation
- **Use cases**: General purpose, root filesystem
- **Configuration**: `/etc/fstab`, `mkfs.ext4`, `tune2fs`

#### XFS
- **Status**: Fully supported
- **Features**: Journaling, allocation groups, large file support, reflinks
- **Use cases**: Large file storage, high performance
- **Configuration**: `/etc/fstab`, `mkfs.xfs`, `xfs_growfs`

#### Btrfs
- **Status**: Fully supported
- **Features**: Copy-on-write, snapshots, subvolumes, compression, RAID
- **Use cases**: Advanced storage, snapshots, RAID
- **Configuration**: `/etc/fstab`, `mkfs.btrfs`, `btrfs subvolume`

#### ZFS
- **Status**: Fully supported
- **Features**: Copy-on-write, snapshots, compression, deduplication, RAID-Z
- **Use cases**: Enterprise storage, data integrity
- **Configuration**: `/etc/fstab`, `zpool`, `zfs`

### BSD Filesystems

#### UFS (FreeBSD)
- **Status**: Supported via compatibility layer
- **Features**: Soft updates, journaling (UFS2)
- **Use cases**: FreeBSD compatibility
- **Configuration**: `/etc/fstab`, `newfs`

#### ZFS (FreeBSD/OpenBSD)
- **Status**: Native support
- **Features**: Copy-on-write, snapshots, compression, deduplication
- **Use cases**: Enterprise storage, data integrity
- **Configuration**: `/etc/fstab`, `zpool`, `zfs`

#### Hammer2 (DragonFly BSD)
- **Status**: Experimental support
- **Features**: Copy-on-write, snapshots, clustering
- **Use cases**: DragonFly BSD compatibility
- **Configuration**: `/etc/fstab`, `hammer2`

### Network Filesystems

#### NFS
- **Status**: Fully supported (NFSv3, NFSv4)
- **Features**: Network file sharing, locking, Kerberos authentication
- **Use cases**: Network storage, shared home directories
- **Configuration**: `/etc/fstab`, `mount.nfs`, `exportfs`

#### SMB/CIFS
- **Status**: Fully supported (SMB2, SMB3)
- **Features**: Windows file sharing, encryption, signing
- **Use cases**: Windows compatibility, file sharing
- **Configuration**: `/etc/fstab`, `mount.cifs`, `smb.conf`

#### SSHFS
- **Status**: Fully supported
- **Features**: SSH-based file mounting, encryption
- **Use cases**: Remote file access, secure file transfer
- **Configuration**: `/etc/fstab`, `sshfs`

### Special Filesystems

#### procfs
- **Status**: Fully supported
- **Features**: Process information, kernel parameters
- **Use cases**: System monitoring, kernel configuration
- **Mount point**: `/proc`

#### sysfs
- **Status**: Fully supported
- **Features**: Kernel device information, hardware configuration
- **Use cases**: Hardware monitoring, device management
- **Mount point**: `/sys`

#### tmpfs
- **Status**: Fully supported
- **Features**: Memory-backed filesystem, swap-backed
- **Use cases**: Temporary files, caches
- **Configuration**: `/etc/fstab`, `mount -t tmpfs`

#### devtmpfs
- **Status**: Fully supported
- **Features**: Dynamic device node creation
- **Use cases**: Device management
- **Mount point**: `/dev`

### Compatibility Layers

#### FUSE
- **Status**: Fully supported
- **Features**: Userspace filesystems, custom filesystems
- **Use cases**: Custom filesystems, SSHFS, cloud storage
- **Configuration**: `/etc/fuse.conf`, `mount.fuse`

#### overlayfs
- **Status**: Fully supported
- **Features**: Layered filesystems, container images
- **Use cases**: Container images, layered storage
- **Configuration**: `/etc/fstab`, `mount -t overlay`

#### bind mounts
- **Status**: Fully supported
- **Features**: Remount existing directories
- **Use cases**: Chroot environments, directory aliases
- **Configuration**: `/etc/fstab`, `mount --bind`

### Encryption

#### LUKS
- **Status**: Fully supported
- **Features**: Full disk encryption, key management
- **Use cases**: Disk encryption, secure storage
- **Configuration**: `cryptsetup`, `/etc/crypttab`

#### dm-crypt
- **Status**: Fully supported
- **Features**: Device mapper encryption
- **Use cases**: Encrypted volumes, encrypted swap
- **Configuration**: `dm-crypt`, `/etc/crypttab`

#### eCryptfs
- **Status**: Fully supported
- **Features**: File-level encryption, stacked encryption
- **Use cases**: Encrypted home directories, secure directories
- **Configuration**: `ecryptfs`, `/etc/fstab`

## Configuration

### Filesystem Configuration
```toml
# /etc/sigmaos/filesystem.toml
[native]
# Native filesystems
ext4 = true
xfs = true
btrfs = true
zfs = true

[bsd]
# BSD filesystems
ufs = true
hammer2 = false

[network]
# Network filesystems
nfs = true
smb = true
sshfs = true

[special]
# Special filesystems
procfs = true
sysfs = true
tmpfs = true
devtmpfs = true

[encryption]
# Encryption
luks = true
dm_crypt = true
ecryptfs = true
```

### Runtime Control
```bash
# Create ext4 filesystem
sigfs create-ext4 /dev/sda1

# Create XFS filesystem
sigfs create-xfs /dev/sda2

# Create ZFS pool
sigfs create-zpool tank /dev/sda3

# Create Btrfs filesystem
sigfs create-btrfs /dev/sda4

# Mount filesystem
sigfs mount /dev/sda1 /mnt/data

# Enable NFS
sigfs enable-nfs

# Enable SMB
sigfs enable-smb

# Enable FUSE
sigfs enable-fuse
```

## Performance Optimization

### ext4
Optimize ext4 for performance:
```bash
# Enable delayed allocation
sigfs tune-ext4 /dev/sda1 -o lazy_itable_init

# Enable journaling
sigfs tune-ext4 /dev/sda1 -o journal_data

# Enable writeback
sigfs tune-ext4 /dev/sda1 -o data_writeback
```

### XFS
Optimize XFS for performance:
```bash
# Enable write barrier
sigfs tune-xfs /dev/sda2 -O nobarrier

# Increase allocation groups
sigfs tune-xfs /dev/sda2 -d agcount=32

# Enable inode64
sigfs tune-xfs /dev/sda2 -m crc=1,finobt=1
```

### Btrfs
Optimize Btrfs for performance:
```bash
# Enable compression
sigfs tune-btrfs /mnt/data -o compress=zstd

# Enable nocow for databases
sigfs tune-btrfs /mnt/data/db -o nodatacow

# Enable autodefrag
sigfs tune-btrfs /mnt/data -o autodefrag
```

### ZFS
Optimize ZFS for performance:
```bash
# Enable compression
sigfs zfs set compression=lz4 tank/data

# Enable atime off
sigfs zfs set atime=off tank/data

# Enable recordsize
sigfs zfs set recordsize=128K tank/data
```

## Troubleshooting

### Filesystem Read-Only
If filesystem is read-only:
1. Check for errors: `sigfs check /dev/sda1`
2. Repair filesystem: `sigfs repair /dev/sda1`
3. Remount read-write: `sigfs remount /mnt/data rw`
4. Check for disk errors: `sigfs disk-check /dev/sda1`

### Mount Failures
If mount fails:
1. Check filesystem type: `sigfs detect /dev/sda1`
2. Check for errors: `dmesg | tail -50`
3. Check mount options: `sigfs show-mount-options`
4. Try simpler mount options
5. Check permissions

### Performance Issues
If filesystem performance is poor:
1. Check mount options: `sigfs show-mount-options`
2. Enable optimizations for filesystem type
3. Check disk health: `sigfs disk-check /dev/sda1`
4. Check for fragmentation
5. Increase cache size

### Corruption
If filesystem is corrupted:
1. Check filesystem: `sigfs check /dev/sda1`
2. Repair filesystem: `sigfs repair /dev/sda1`
3. Backup data if possible
4. Reformat if necessary
5. Restore from backup

---

**[Filesystems](Category-Filesystems)** | **[Storage](Category-Storage)** | **[Mount Management](Category-Mount)**

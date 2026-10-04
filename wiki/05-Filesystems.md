# Filesystems and Storage

**Capability state: Prototype.** Source models and filesystem code do not establish a bootable persistent storage path. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository contains filesystem and storage modules, but no end-to-end filesystem and block-device path has been validated from a booted kernel. ext4, Btrfs, ZFS, SigmaFS, procfs, sysfs, and devfs must not be described as supported filesystems based only on names, parsers, or data structures. There is no tested installer, persistent root filesystem, or power-loss recovery flow.

## Design references

- Linux VFS and ext4: explicit filesystem operations and journaling boundaries.
- Btrfs and OpenZFS: snapshots and integrity checks, after persistent block I/O works.
- FreeBSD GEOM: composable storage layers and clear device ownership.
- Mint: explain disk selection, data impact, and recovery in user-facing language.

## Roadmap

1. Establish one tested virtual block device and bounded read/write operations.
2. Select one initial filesystem and validate mount, metadata, file I/O, and malformed-media handling.
3. Test persistence across reboot and recovery after injected write interruption on disposable QEMU disks.
4. Add partitioning, additional formats, snapshots, and encryption only with documented runtime support and recovery tests.

**Completion evidence:** named virtual and physical devices, reproducible filesystem tests, persistence across reboot, corruption and interruption results, and documented recovery behavior.

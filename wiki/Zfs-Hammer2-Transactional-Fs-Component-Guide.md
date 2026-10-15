# Transactional Copy-on-Write Filesystem & Storage Pool Component Guide

## Component Overview
Governs copy-on-write transactional tree structures, atomic snapshots, block deduplication, multi-device storage pooling, RAID-Z parity arrays, and scrub verification inspired by ZFS and DragonFly BSD HAMMER2.

## Linux Inspiration
OpenZFS on Linux, Btrfs subvolumes and snapshots, and bcachefs transactional metadata.

## BSD Inspiration
FreeBSD ZFS integration (zpool, zfs send/receive), DragonFly BSD HAMMER2 transactional lockless filesystem.

## Current SigmaOS Status
Transactional tree node structures, CoW snapshot metadata, and block checksum verification implemented in SigmaFS.

## Critical Missing Features
Multi-drive RAID-Z1/Z2 parity calculation, automated background scrub verifying data integrity against silent bit rot.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/fs/sigmafs.rs, src/fs/vfs.rs, src/storage/raid.rs

## Testing Strategy
Simulated power-loss during write transactions, bit-rot corruption detection and self-healing verification.

## Dependencies
STORAGE_AGENT, FILESYSTEM_VFS_AGENT, CRYPTOGRAPHY_AGENT

## Success Criteria
Zero data loss during abrupt power cut, 100% self-healing detection of corrupted sectors from parity mirrors.

## Open Source Competitors Analysis
Combines the enterprise reliability of ZFS with the low memory footprint of DragonFly HAMMER2, written entirely in memory-safe Rust.

## Future Enhancements
Tiered NVMe-to-HDD caching automatically promoting hot file blocks to high-speed PCIe Gen5 storage.

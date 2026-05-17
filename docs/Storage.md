# Storage & File System (SovereignCloudFS & ZFS-Inspired Storage)

This document defines the storage layers and file system capabilities within SigmaOS Zenith.

### SovereignVFS (Virtual File System)
1. **Zero-Copy VFS Routing**: Path resolution uses Radix trees for O(1) file descriptor lookups.
2. **Pluggable File Systems**: Standardized interface for FAT32, EXT4, and SovereignCloudFS drivers.
3. **Atomic Snapshots**: ZFS-inspired copy-on-write (CoW) B-Trees for instantaneous snapshot creation.

### Distributed & Cloud Features (SovereignCloudFS)
4. **Distributed Object Storage**: Built-in network block replication for multi-device clusters.
5. **Decentralized Synchronization**: CRDT (Conflict-free Replicated Data Types) merge logs for offline file modifications.
6. **Container Overlay FS**: Union file systems specifically optimized for SovereignCluster container isolation.

### Hardware Interfacing
7. **NVMe Polling Mode**: Direct polling queues for ultra-low latency SSD access, bypassing interrupt overhead.
8. **Data Deduplication**: Inline hashing and chunk sharing across the primary storage volume.
9. **Transparent Encryption**: AES-256-GCM native block encryption bound to the Sovereign Identity TPM.

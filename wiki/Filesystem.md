# SigmaOS Filesystem

This page consolidates all filesystem documentation for SigmaOS.

## Overview

SigmaOS implements a comprehensive filesystem layer inspired by Linux VFS, BSD filesystem architectures, and Windows storage systems. The filesystem layer provides support for multiple filesystem types, advanced storage management, and cross-platform compatibility.

## Filesystem Architecture

### Design Philosophy

#### Virtual Filesystem Switch (VFS)
- **Abstract filesystem interface** - Unified API for all filesystem types
- **Filesystem operations** - Standard POSIX-compatible operations
- **Vnode/inode abstraction** - Unified file representation
- **Namespace management** - Mount point and path resolution
- **Cross-filesystem operations** - File operations across different filesystems

#### Supported Filesystems
- **ext4** - Default Linux filesystem with journaling
- **Btrfs** - Copy-on-write filesystem with snapshots
- **ZFS** - Advanced filesystem with compression and deduplication
- **F2FS** - Flash-oriented filesystem for SSDs
- **FAT32/exFAT** - Compatibility with Windows systems
- **NTFS** - Windows filesystem with read/write support
- **ISO9660** - Optical disc filesystem
- **procfs** - Process information filesystem
- **sysfs** - System information filesystem
- **tmpfs** - Memory-based temporary filesystem
- **devtmpfs** - Device filesystem
- **cgroupfs** - Control group filesystem
- **EROFs** - Read-only filesystem for embedded systems

### Core Subsystems

#### File Operations
- **Open/Close** - File descriptor management
- **Read/Write** - Buffered and direct I/O
- **Seek** - File position management
- **Memory mapping** - mmap/mprotect operations
- **File locking** - Advisory and mandatory locks
- **Asynchronous I/O** - Non-blocking operations

#### Directory Operations
- **Create/Remove** - Directory management
- **Enumeration** - Directory listing
- **Path resolution** - Canonical path handling
- **Symlink management** - Symbolic link operations
- **Hard link support** - Multiple directory entries
- **Mount point management** - Filesystem mounting

#### Storage Management
- **Block allocation** - Space allocation strategies
- **Free space management** - Block tracking
- **Journaling** - Metadata journaling for crash recovery
- **Copy-on-Write (CoW)** - Snapshot-based operations
- **Compression** - On-the-fly data compression
- **Deduplication** - Block-level deduplication
- **RAID support** - Software RAID implementations

#### Filesystem Security
- **POSIX permissions** - User/group/other permissions
- **Access Control Lists (ACLs)** - Fine-grained permissions
- **Extended attributes** - Metadata extensions
- **Capability checks** - Permission verification
- **Immutable files** - Write protection
- **Append-only files** - Log file protection

## Advanced Features

### Sovereign Link Engine
Advanced symbolic link management with:
- **Cross-filesystem symlinks** - Links across filesystem boundaries
- **Symlink cache** - Performance optimization
- **Security validation** - Symlink attack prevention
- **Follow/symlink options** - Configurable symlink resolution
- **Absolute/relative symlinks** - Full support for both types

### Copy-on-Write Snapshots
- **Instant snapshots** - Zero-copy filesystem snapshots
- **Snapshot management** - Create, delete, list snapshots
- **Rollback support** - Revert to previous snapshot
- **Space efficiency** - Shared blocks between snapshots
- **Live snapshots** - Snapshot without filesystem downtime

### Storage Innovations
- **Hybrid storage** - SSD/HDD tiering
- **Thin provisioning** - On-demand allocation
- **Storage tiering** - Hot/cold data placement
- **Quota management** - Per-user and per-group limits
- **Resize operations** - Online filesystem resizing
- **Migration support** - Data migration between storage systems

## AI Agent Filesystem Guidelines

### Bolt (Performance Persona)
**Mission:** Filesystem performance optimization

**Focus Areas:**
- I/O scheduling optimization
- Cache management improvements
- Block allocation efficiency
- Metadata operation speed
- Parallel I/O operations

**Critical Learning Journal:** `.jules/bolt.md`

### Filesystem Verification Checklist
Before committing filesystem changes, verify:
1. No resource leaks in file handle management
2. Proper error handling for I/O operations
3. Thread-safe access to shared filesystem structures
4. No deadlock potential in lock acquisition order
5. Proper bounds checking on all filesystem buffers
6. Safe FFI interactions with block devices
7. Proper cleanup on filesystem unmount
8. Correct reference counting for inodes/vnodes

## Filesystem Testing

### Unit Testing
```bash
# Run filesystem-specific tests
cargo test --lib filesystem

# Test specific filesystem components
cargo test --lib filesystem::vfs
cargo test --lib filesystem::block
cargo test --lib filesystem::cow_snapshot
```

### Integration Testing
- **Filesystem stress testing** - High-load filesystem operations
- **Mount/unmount cycles** - Test filesystem lifecycle
- **Corruption recovery** - Test journaling and recovery
- **Performance benchmarks** - Filesystem operation performance
- **Cross-filesystem operations** - Test interoperability

### Verification Commands
```bash
# Build filesystem modules
rustc --edition=2021 --crate-type staticlib src/filesystem/mod.rs

# Test filesystem compilation
cargo build --lib

# Run full test suite
./run_sigma_tests.sh
```

## Filesystem Best Practices

### Memory Safety
- Use safe Rust abstractions for file operations
- Minimal unsafe code with extensive documentation
- Proper error handling for I/O failures
- Memory barrier usage for SMP systems
- DMA buffer management for block devices

### Performance
- Use async I/O where appropriate
- Implement read-ahead and write-behind
- Cache frequently accessed metadata
- Batch operations for efficiency
- Consider filesystem-specific optimizations

### Security
- Validate all file paths from user space
- Sanitize filesystem paths
- Check permissions before operations
- Audit security-relevant filesystem operations
- Implement proper error handling
- Prevent symlink attacks

### Reliability
- Implement proper journaling
- Handle hardware errors gracefully
- Provide data integrity checks
- Support crash recovery
- Implement fsync/fdatasync semantics

## Documentation References

For detailed filesystem implementation specifications:
- [Architecture](ARCHITECTURE.md)
- [Security](SECURITY.md)
- [Kernel](Kernel.md)
- [Package Management](Package-Management.md)
- [Roadmap](ROADMAP.md)

## Contributing

Filesystem development follows SigmaOS agent guidelines:
- **Bolt**: Performance optimization (I/O scheduling, cache management)
- **Sentinel**: Security vulnerability remediation in filesystem code
- **Palette**: Filesystem UX improvements (better error messages, clearer diagnostics)

### Filesystem Commit Guidelines
- Describe filesystem component affected in commit messages
- Include performance impact when applicable
- Reference relevant filesystem documentation
- Test filesystem changes under various load conditions
- Update filesystem documentation

---

*This page consolidates the following individual filesystem documents:*
- AGENTS_BASIC_FILESYSTEM_MANAGEMENT.md
- AGENTS_FILESYSTEM_MANAGEMENT.md
- AGENTS_LEGACY_FILESYSTEM_DROPPED_STORAGE_MANAGEMENT.md
- AGENTS_VARIOUS_FILESYSTEM_OPERATIONS_MANAGEMENT.md
- AI_AGENT_EROFS_FILESYSTEM_MANAGEMENT.md
- AI_AGENT_FILESYSTEM_MANAGEMENT_ARCHITECTURE.md
- AI_AGENT_FILESYSTEM_MANAGEMENT_GUIDELINES.md
- ai-agent-filesystem-management.md
- AI_AGENT_VARIOUS_FILESYSTEM_OPERATIONS_MANAGEMENT_GUIDELINES.md
- Filesystem-Support-Matrix.md

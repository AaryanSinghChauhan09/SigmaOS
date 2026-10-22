# Upstream Study: Linux Virtual Filesystem (VFS)

1. **Upstream Problem**: Abstraction across heterogeneous filesystems with mount namespaces and dentry caches.
2. **Data Structure & Control Flow**: `vfs_mount`, `dentry`, `inode`, `file_operations` function pointer vtables.
3. **Security & Failure Assumptions**: DAC access check (`inode_permission`), mount flag isolation (`MS_NODEV`, `MS_NOSUID`).
4. **Direct Adoption**: Trait-based VFS lookup, mount namespace hierarchy, and standardized POSIX errno returns.
5. **Rust/SigmaOS Adaptation**: Safe Rust traits (`VFileSystem`, `VNode`) wrapped in `Arc<Spinlock<...>>`.
6. **Exit Test Criteria**: VFS correctly mounts RAM disk, resolves complex relative paths, and enforces permission boundaries.
7. **Licensing & Attribution**: GPL-2.0 (Clean-room specification in SigmaOS).

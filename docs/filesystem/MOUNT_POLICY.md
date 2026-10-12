# Mount Policy Specification

## Mount & Filesystem Lifecycle Guarantees

1. **Race-Free Mounting**: Mount point registration in `MountManager` is protected against concurrent modification races.
2. **Capacity Bounding**: Active mount entries are capped at a maximum count (default 64) to prevent VFS table overflow; exceeding capacity returns `MountError::TableFull`.
3. **Failure-Safe Mount / Rollback**: If filesystem initialization or root inode validation fails during mount, the mount point state is rolled back completely.
4. **Double-Unmount Prevention**: Unmounting an inactive or already unmounted path returns `MountError::NotMounted` or `MountError::AlreadyUnmounted`.
5. **Lost-Update Prevention**: Filesystem updates are recorded in transactional journal records (`FileTransactionRecord`) to prevent lost metadata updates on crash or power loss.

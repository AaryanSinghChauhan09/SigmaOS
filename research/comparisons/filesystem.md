# Filesystem Mechanism Comparison Matrix

| Project | VFS Architecture | Root Filesystem | Path Resolution Limits | Error Conventions |
|---|---|---|---|---|
| **xv6** | Direct inode operations | Custom disk inode FS | Fixed path depth | `-1` error code |
| **Linux** | Inode + Dentry + Mount layers | OverlayFS / Ext4 / Btrfs | 4096 chars path, 255 component | Negative `ERR_PTR` / POSIX `errno` |
| **FreeBSD** | VFS `vnode` abstraction | UFS2 / ZFS | 1024 chars path | POSIX error integer |
| **SigmaOS** | Layered VFS ops with mounts | Initramfs / RAM-disk | 4096 chars path, 255 component | Explicit `VfsPathError` enum |

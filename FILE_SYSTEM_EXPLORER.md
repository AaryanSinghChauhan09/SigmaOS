
# File Exploring System (VFS & BlockView)


The SigmaOS File Exploring System is composed of two heavily decoupled components, enforcing the Microkernel design pattern.


## 1. Virtual File System (VFS)

Located at `modules/core/fs/vfs.c`, the VFS is a core kernel service that acts as an abstraction layer over concrete file systems (FAT32, ext4, or SigmaFS).
- Maintains a unified tree structure (nodes) for directories and files.
- Hides device-specific I/O logic from user-space applications.
- Provides standard system calls (`open`, `read`, `write`, `close`).


## 2. BlockView Explorer

Located at `modules/tools/explorer/explorer.c`, BlockView is a user-space application providing a CLI interface for exploring the VFS.
- Parses user commands (e.g., `ls`, `cd`, `cat`).
- Issues IPC requests to the VFS service to enumerate directories and read file metadata.
- Completely unprivileged; a crash in BlockView will not crash the VFS or the Kernel.

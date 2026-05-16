# Sovereign Storage Shard (S-STOR)

The Storage Shard implements the file system abstractions for SigmaOS, providing a secure, atomic Virtual File System (VFS) and Native Lattice File System (LFS).

## Architecture Diagram

```mermaid
graph TD
    A[Userland App] -->|Z-SYSCALL| B(VFS Abstraction)
    B --> C{Lattice File System}
    B --> D{FAT32 / EXT2}
    B --> E{NVMe Raw}
    C --> F[Sovereign HAL]
    D --> F
    E --> 





 **Lattice FS (LFS)**: A custom, atomic file system designed to prevent corruption during unexpected shutdowns.
- **Legacy Support**: `mount()` operations for FAT32 and EXT2 partitions.
- **NVMe Awareness**: Direct Memory Access (DMA) hooks for extreme throughput.

## VFS API Example

```c
int fd;
sigma_status status = SovereignStorageShard::getInstance().file_open("/mnt/data/config.sab", O_RDONLY, &fd);

if (status == SIGMA_OK) {
    char buffer[512];
    SovereignStorageShard::getInstance().file_read(fd, buffer, 512);
    SovereignStorageShard::getInstance().file_close(fd);




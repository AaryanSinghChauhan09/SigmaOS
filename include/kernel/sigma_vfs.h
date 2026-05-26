/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL FILE SYSTEM (VFS v1.0)
 * =============================================================================
 * Mission: A unified abstraction layer over multiple file systems (ZFS, ext4,
 *          FAT32) with a POSIX-compliant file descriptor interface.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "../sigma_kernel_types.h"

#define VFS_MAX_MOUNT_POINTS   16
#define VFS_MAX_INODES       1024
#define VFS_MAX_FDS_PER_PROC   64
#define VFS_FILENAME_LEN       64
#define VFS_PATH_LEN          256

typedef enum {
    VFS_NODE_FILE      = 0,
    VFS_NODE_DIR       = 1,
    VFS_NODE_SYMLINK   = 2,
    VFS_NODE_DEVICE    = 3,
    VFS_NODE_PIPE      = 4
} sigma_vfs_node_type_t;

/* Standard POSIX-style permissions */
#define VFS_PERM_READ   BIT(0)
#define VFS_PERM_WRITE  BIT(1)
#define VFS_PERM_EXEC   BIT(2)

typedef struct {
    sigma_u32             inode_id;
    sigma_vfs_node_type_t type;
    char                  name[VFS_FILENAME_LEN];
    sigma_u64             size;           /* in bytes */
    sigma_u64             created_tsc;
    sigma_u64             modified_tsc;
    sigma_u16             permissions;
    sigma_u32             owner_pid;
    sigma_u32             device_id;      /* mapping to Device Manager */
    sigma_u32             block_start;    /* logical block address on device */
} sigma_inode_t;

typedef struct {
    sigma_u32      fd;             /* the file descriptor number */
    sigma_u32      inode_id;       /* global inode this FD points to */
    sigma_u64      offset;         /* read/write head position */
    sigma_u16      flags;          /* O_RDONLY, O_WRONLY, O_APPEND, etc. */
    sigma_bool     is_open;
} sigma_file_descriptor_t;

#ifdef __cplusplus
extern "C" {
#endif

void      vfs_init(void);
int       vfs_mount(const char* target_path, const char* fs_type, sigma_u32 device_id);
int       vfs_unmount(const char* target_path);

int       vfs_open(sigma_u32 pid, const char* path, sigma_u16 flags);
int       vfs_close(sigma_u32 pid, int fd);
sigma_i64 vfs_read(sigma_u32 pid, int fd, void* buf, sigma_usize count);
sigma_i64 vfs_write(sigma_u32 pid, int fd, const void* buf, sigma_usize count);

int       vfs_create(const char* path, sigma_vfs_node_type_t type, sigma_u32 owner_pid);
int       vfs_delete(const char* path);
void      vfs_print_mounts(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */

#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

// SigmaOS Custom File System & VFS Shard
// Absorbing APIs from Linux VFS and ZFS features
#include "sigma_types.h"

void vfs_init_custom_fs();
void vfs_mount_partition(const char* target, const char* format);
void vfs_enable_partition_encryption(const char* target);

#endif // SIGMA_VFS_H


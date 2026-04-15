#ifndef SIGMA_FILESYSTEM_LEGACY_H
#define SIGMA_FILESYSTEM_LEGACY_H

#include "suites/S01_Genesis/shards/sigma_types.h"


// SigmaOS Legacy & Compatibility File System Shards
// Modular support for industry-standard formats

// Mount an ext4 partition with native journaling support
bool fs_mount_ext4(const char* device_id, const char* mount_point);

// NTFS support via proprietary-parity driver (Sovereign implementation)
bool fs_mount_ntfs(const char* device_id, const char* mount_point);

// FAT32/ExFAT support for universal removable media compatibility
bool fs_mount_fat(const char* device_id, const char* mount_point);

// Btrfs/ZFS Snapshot & Copy-on-Write (CoW) hooks
void fs_trigger_cow_snapshot(const char* mount_point);

#endif // SIGMA_FILESYSTEM_LEGACY_H


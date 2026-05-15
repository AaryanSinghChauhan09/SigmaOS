/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SELF-HEALING FILE SYSTEM (S-FS)
 * =========================================================================
 * Mission: Atomic, journaled file operations and automated corruption repair.
 * Inspired by ZFS / Btrfs / Immutable Systems.
 * =========================================================================
 */

#ifndef SIGMA_FS_H
#define SIGMA_FS_H

#include "include/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char      filename[64];
    sigma_u32 size;
    sigma_u32 checksum;
    bool      is_journaled;
} sigma_file_t;

/* --- File System Primitives --- */
void      fs_init(void);
bool      fs_write_atomic(const char* path, const void* data, sigma_u32 size);
void      fs_verify_integrity(void);
void      fs_repair_corruption(void);
sigma_u64 fs_get_total_writes(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_FS_H */

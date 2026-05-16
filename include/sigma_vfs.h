/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN VIRTUAL FILE SYSTEM (VFS)
 * =========================================================================
 * Mission: Zero-latency shard-mapped path resolution.
 * =========================================================================
 */

#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_FS_SHARD,
    SIGMA_FS_VIRTUAL,
    SIGMA_FS_HARDWARE
} sigma_fs_type_t;

typedef struct {
    char            name[64];
    sigma_u32       shard_id;
    sigma_fs_type_t type;
    void*           private_data;
} sigma_vnode_t;

/* --- VFS Primitives --- */
void           vfs_init(void);
sigma_vnode_t* vfs_lookup(const char* path);
bool           vfs_mount(const char* path, sigma_u32 shard_id);
sigma_u64      vfs_get_lookup_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */

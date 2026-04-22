/* S SIGMAOS: SOVEREIGN VFS SHARD HEADER */
#ifndef SOVEREIGN_VFS_SHARD_H
#define SOVEREIGN_VFS_SHARD_H
#include "sigma_types.h"
typedef struct SigmaVFSOps_t SigmaVFSOps_t;
sigma_err_t sigma_vfs_register_backend (const SigmaVFSOps_t* ops);
sigma_err_t sigma_vfs_mount            (const char* dev, const char* mp,
                                         const char* fstype, sigma_bool ro);
sigma_err_t sigma_vfs_umount           (const char* mp);
void        SovereignVFSShard_Init     (void);
void        SovereignVFS_Audit         (void);
#endif

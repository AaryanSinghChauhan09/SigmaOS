/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN VIRTUAL FILESYSTEM INTERFACE (v2.0)
 * =========================================================================
 * Mission: Modular Filesystem Routing and Abstracted Storage Sovereignty.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_VFS_H
#define SOVEREIGN_VFS_H

#include "sigma_types.h"

typedef struct SigmaSuperBlock SigmaSuperBlock_t;
typedef struct SigmaInode SigmaInode_t;
typedef struct SigmaFile SigmaFile_t;

typedef sigma_err_t (*sigma_mount_fn)(const char* source, const char* target, void** sb_out);

typedef struct {
    char fstype[16];
    sigma_mount_fn mount;
} sovereign_fs_type_t;

/* Registry API */
void SovereignVFS_InitRegistry(void);
sigma_err_t SovereignVFS_RegisterFS(const char* fstype, sigma_mount_fn mount);
sigma_err_t sigma_vfs_mount(const char* source, const char* target, const char* fstype);

/* File Operations API */
SigmaFile_t* sigma_vfs_open(const char* path, sigma_u32 flags, sigma_u16 mode);
sigma_err_t sigma_vfs_read(SigmaFile_t* file, char* buf, sigma_size_t len);

#endif /* SOVEREIGN_VFS_H */

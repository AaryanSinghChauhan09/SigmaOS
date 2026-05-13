/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LATTICE FILESYSTEM (S-FS)
 * =========================================================================
 * Mission: High-assurance, transactional filesystem for industrial shards.
 * Principle: Zero-latency metadata, atomic journaled commits.
 * =========================================================================
 */

#ifndef SIGMA_FS_H
#define SIGMA_FS_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    S_IFREG = 0, // Regular file
    S_IFDIR = 1, // Directory
    S_IFCHR = 2, // Character device
} sigma_file_type_t;

typedef struct {
    char name[64];
    sigma_u32 size;
    sigma_u32 inode;
    sigma_file_type_t type;
} sigma_dirent_t;

/* --- FS Primitives --- */
void      fs_init(void);
int       fs_open(const char* path, int flags);
int       fs_close(int fd);
sigma_i32 fs_read(int fd, void* buf, sigma_u32 count);
sigma_i32 fs_write(int fd, const void* buf, sigma_u32 count);
int       fs_mkdir(const char* path);
int       fs_ls(const char* path, sigma_dirent_t* buf, int max_entries);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_FS_H */

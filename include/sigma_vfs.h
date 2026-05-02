#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct sigma_file {
    char name[128];
    sigma_u32 size;
    sigma_u32 flags;
    void* buffer;
} sigma_file_t;

void vfs_init(void);
sigma_file_t* vfs_open(const char* path);
sigma_status vfs_read(sigma_file_t* file, void* buf, sigma_u32 size);
sigma_status vfs_write(sigma_file_t* file, const void* buf, sigma_u32 size);
void vfs_close(sigma_file_t* file);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */

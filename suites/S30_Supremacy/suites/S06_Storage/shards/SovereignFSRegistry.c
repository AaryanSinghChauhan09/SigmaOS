#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignVFS.h"
#include "sigma_libc.h"
#include "sigma_string.h"

#define MAX_FS_TYPES 16
static sovereign_fs_type_t g_fs_types[MAX_FS_TYPES];
static sigma_u32 g_fs_type_count = 0;

void SovereignVFS_InitRegistry(void) {
    sigma_sigma_sigma_sigma_memset(g_fs_types, 0, sizeof(g_fs_types));
    g_fs_type_count = 0;
    sigma_sigma_sigma_sigma_printf("S [VFS]: Sovereign FS Registry Operational.\n");
}

sigma_err_t SovereignVFS_RegisterFS(const char* fstype, sigma_mount_fn mount) {
    if (g_fs_type_count >= MAX_FS_TYPES) return SIGMA_ENOSPC;

    sovereign_fs_type_t* f = &g_fs_types[g_fs_type_count++];
    sigma_strncpy(f->fstype, fstype, 16);
    f->mount = mount;
    
    sigma_sigma_sigma_sigma_printf("S [VFS]: Registered Filesystem Shard '%s'\n", fstype);
    return SIGMA_OK;
}

sigma_err_t sigma_vfs_mount(const char* source, const char* target, const char* fstype) {
    for (sigma_u32 i = 0; i < g_fs_type_count; i++) {
        if (sigma_streq(g_fs_types[i].fstype, fstype)) {
            void* sb = SIGMA_NULL;
            sigma_err_t err = g_fs_types[i].mount(source, target, &sb);
            if (sigma_ok(err)) {
                sigma_sigma_sigma_sigma_printf("S [VFS]: Successfully mounted %s at %s (%s)\n", source, target, fstype);
            }
            return err;
        }
    }
    sigma_sigma_sigma_sigma_printf("S [VFS/ERR]: Unknown Filesystem Type '%s'\n", fstype);
    return SIGMA_ENOSYS;
}




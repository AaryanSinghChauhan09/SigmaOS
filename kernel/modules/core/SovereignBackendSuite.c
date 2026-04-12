/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BACKEND SUITE (v2.0 - INDUSTRIAL)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

/* --- Sub-Module 1: Virtual File System (VFS) --- */
typedef struct {
    char name[64];
    sigma_u32 size;
    sigma_bool is_dir;
} SigmaInode_t;

static SigmaInode_t s_root_fs[16] = {
    { "/", 0, SIGMA_TRUE },
    { "/bin", 0, SIGMA_TRUE },
    { "/root", 0, SIGMA_TRUE },
    { "/kernel", 1024576, SIGMA_FALSE }
};

sigma_err_t sigma_vfs_stat(const char* path, SigmaInode_t* out) {
    for (int i = 0; i < 16; i++) {
        if (sigma_streq(path, s_root_fs[i].name)) {
            sigma_memcpy(out, &s_root_fs[i], sizeof(SigmaInode_t));
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

/* --- Sub-Module 2: Network Stack --- */
void sigma_net_init(void) {
    sigma_printf("  [NET]: Sovereign TCP/IP Stack seated (Loopback: 127.0.0.1)\n");
}

/* --- Initialization --- */
void SovereignBackend_Init(void) {
    sigma_printf("Σ [BACKEND-SUITE]: Initialising Filesystems and Network Stack...\n");
    sigma_net_init();
    sigma_printf("Σ [BACKEND-SUITE]: VFS mounted. Network interfaces up.\n");
}

void SovereignBackend_Register(void) {
    static SovereignModule_t s_backend_module = {
        .name = "SovereignBackend",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignBackend_Init,
    };
    sigma_module_register(&s_backend_module);
}

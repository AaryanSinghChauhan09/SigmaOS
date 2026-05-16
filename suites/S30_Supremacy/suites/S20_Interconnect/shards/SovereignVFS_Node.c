#include "../../../../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS: S20_GLOBALVFS  SovereignVFS_Node.c
 * =========================================================================
 * Implementation of Idea 218 (Apex Infinity): Unified VFS Node.
 * Provides the industrial-grade interface for all Sovereign file systems.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "../../../../../include/core/sigma_types.h"
#include "../../../../../include/libc/sigma_libc.h"

#define MAX_VFS_NODES 1024

typedef enum { VFS_FILE, VFS_DIR, VFS_DEVICE } VfsNodeType;

typedef struct SovereignVfsNode {
    char        name[64];
    VfsNodeType type;
    uint64_t    size;
    void*       fs_data; // FS-specific driver pointer
} SovereignVfsNode;

static SovereignVfsNode g_vfs_root[MAX_VFS_NODES];
static uint32_t g_vfs_count = 0;

void vfs_init(void) {
    sigma_sigma_memset(g_vfs_root, 0, sizeof(g_vfs_root));
    g_vfs_count = 0;
    sigma_sigma_printf("S [S20]: Global VFS Materialized (Apex Idea 218).\n");
}

SovereignVfsNode* vfs_mount(const char* name, VfsNodeType type) {
    if (g_vfs_count >= MAX_VFS_NODES) return SIGMA_NULL;
    
    SovereignVfsNode* node = &g_vfs_root[g_vfs_count++];
    sigma_strncpy(node->name, name, 63);
    node->type = type;
    node->size = 0;
    
    sigma_sigma_printf("S [VFS]: Mounted %s [%s]\n", name, (type == VFS_DIR) ? "DIR" : "FILE");
    return node;
}

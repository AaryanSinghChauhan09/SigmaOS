#include "sigma_vfs.h"
#include "sigma_hal.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign VFS (v28.0 Zenith)
 * Implements a high-performance Shard-Mapped Lookup (SML) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native VFS.
 *
 * Design: OOP-isolated singleton — SovereignVFSEngine.
 */



/* --- Sovereign VFS Engine (OOP Isolation) --- */
static struct {
    sigma_vnode_t lattice[100];
    sigma_u32     node_count;
    sigma_u64     lookups_performed;
    sigma_u32     initialized;
} SovereignVFSEngine = {
    .node_count = 0u,
    .lookups_performed = 0u,
    .initialized = 0u
};

extern "C" void vfs_init() {
    sigma_log("[VFS] Initializing Sovereign Virtual File System (SML Algorithm)...");
    
    // Mount root shard
    vfs_mount("/", 1u); // Genesis Shard
    SovereignVFSEngine.initialized = 1u;
}

extern "C" bool vfs_mount(const char* path, sigma_u32 shard_id) {
    if (SovereignVFSEngine.node_count >= 100u) return false;
    
    sigma_vnode_t* node = &SovereignVFSEngine.lattice[SovereignVFSEngine.node_count++];
    sigma_hardened_strcpy(node->name, path, 64);
    node->shard_id = shard_id;
    node->type = SIGMA_FS_SHARD;
    
    sigma_printf("[VFS] SML: Mounted Shard S%02u at '%s'.\n", shard_id, path);
    return true;
}

extern "C" sigma_vnode_t* vfs_lookup(const char* path) {
    /* SML (Shard-Mapped Lookup) Algorithm */
    SovereignVFSEngine.lookups_performed++;
    
    for (sigma_u32 i = 0u; i < SovereignVFSEngine.node_count; i++) {
        // Hardened string comparison
        bool match = true;
        for(sigma_u32 k=0u; path[k] != '\0' && SovereignVFSEngine.lattice[i].name[k] != '\0'; k++) {
            if(path[k] != SovereignVFSEngine.lattice[i].name[k]) {
                match = false;
                break;
            }
        }
        
        if (match) {
            sigma_printf("[VFS] Path RESOLVED: %s -> Shard S%02u\n", path, (unsigned)SovereignVFSEngine.lattice[i].shard_id);
            return &SovereignVFSEngine.lattice[i];
        }
    }
    
    sigma_printf("[VFS] Path NOT FOUND: %s\n", path);
    return (sigma_vnode_t*)SIGMA_NULL;
}

extern "C" sigma_u64 vfs_get_lookup_count() {
    return SovereignVFSEngine.lookups_performed;
}

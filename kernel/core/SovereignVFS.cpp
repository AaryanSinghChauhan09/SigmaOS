#include "sigma_vfs.h"
#include "sigma_hal.h"
#include "sigma_libc.h"

/**
 * SigmaOS Sovereign VFS (v28.0 Zenith)
 * Implements a high-performance Shard-Mapped Lookup (SML) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native VFS.
 *
 * Design: OOP-isolated singleton — SovereignVFSEngine.
 */

class SovereignVFSEngine {
public:
    static SovereignVFSEngine& getInstance() {
        static SovereignVFSEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[VFS] Initializing Sovereign Virtual File System (SML Algorithm)...");
        // Mount root shard
        this->mount("/", 1u); // Genesis Shard
        this->initialized = 1u;
    }

    bool mount(const char* path, sigma_u32 shard_id) {
        if (this->node_count >= 100u) return false;
        
        sigma_vnode_t* node = &this->lattice[this->node_count++];
        sigma_hardened_strcpy(node->name, path, 64);
        node->shard_id = shard_id;
        node->type = SIGMA_FS_SHARD;
        
        sigma_printf("[VFS] SML: Mounted Shard S%02u at '%s'.\n", shard_id, path);
        return true;
    }

    sigma_vnode_t* lookup(const char* path) {
        /* SML (Shard-Mapped Lookup) Algorithm */
        this->lookups_performed++;
        
        for (sigma_u32 i = 0u; i < this->node_count; i++) {
            // Hardened string comparison
            bool match = true;
            for(sigma_u32 k=0u; path[k] != '\0' && this->lattice[i].name[k] != '\0'; k++) {
                if(path[k] != this->lattice[i].name[k]) {
                    match = false;
                    break;
                }
            }
            
            if (match) {
                sigma_printf("[VFS] Path RESOLVED: %s -> Shard S%02u\n", path, (unsigned)this->lattice[i].shard_id);
                return &this->lattice[i];
            }
        }
        
        sigma_printf("[VFS] Path NOT FOUND: %s\n", path);
        return (sigma_vnode_t*)SIGMA_NULL;
    }

    sigma_u64 getLookupCount() const { return this->lookups_performed; }

private:
    SovereignVFSEngine() : node_count(0), lookups_performed(0), initialized(0) {}
    
    sigma_vnode_t lattice[100];
    sigma_u32     node_count;
    sigma_u64     lookups_performed;
    sigma_u32     initialized;
};

/* --- C Wrappers --- */
extern "C" void vfs_init() {
    SovereignVFSEngine::getInstance().init();
}

extern "C" bool vfs_mount(const char* path, sigma_u32 shard_id) {
    return SovereignVFSEngine::getInstance().mount(path, shard_id);
}

extern "C" sigma_vnode_t* vfs_lookup(const char* path) {
    return SovereignVFSEngine::getInstance().lookup(path);
}

extern "C" sigma_u64 vfs_get_lookup_count() {
    return SovereignVFSEngine::getInstance().getLookupCount();
}

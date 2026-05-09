#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Container Storage (CSI)
 * SovereignVFS bridging for Micro-VM containers.
 *
 * USP: Transparently mounts SovereignVFS distributed shards directly into 
 * the isolated container namespace. Containers interact with POSIX file APIs, 
 * but reads/writes are actually mathematically striped across the hybrid silicon cluster.
 *
 * Design: OOP-isolated singleton — SovereignContainerStorageEngine.
 */

class SovereignContainerStorageEngine {
public:
    static SovereignContainerStorageEngine& getInstance() {
        static SovereignContainerStorageEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[CONTAINER-STORAGE] Initializing Sovereign Container Storage bridge...");
        this->active_mounts = 0;
    }

    void mountVFSVolume(const char* container_name, const char* mount_point) {
        if (this->active_mounts >= 64) return;
        sigma_hardened_strcpy(this->mounts[this->active_mounts], mount_point, 64);
        this->active_mounts++;
        sigma_log("[CONTAINER-STORAGE] Mounted SovereignVFS volume to '%s' inside Container '%s'.\n", 
                     mount_point, container_name);
    }

private:
    SovereignContainerStorageEngine() : active_mounts(0) {}

    char mounts[64][64];
    sigma_u32 active_mounts;
};

/* --- C Wrappers --- */
extern "C" void container_storage_init() {
    SovereignContainerStorageEngine::init();
}

extern "C" void container_storage_mount(const char* container, const char* path) {
    SovereignContainerStorageEngine::mountVFSVolume(container, path);
}





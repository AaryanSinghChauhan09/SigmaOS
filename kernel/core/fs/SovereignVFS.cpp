#include "../../../include/sigma_vfs.h"
#include "../../../include/sigma_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign VFS (Virtual File System) Shard
 * Principles: Amnesic Sharding, High-Speed I/O, Silicon-Direct.
 */

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignVFS : public SigmaObject {
public:
    static SovereignVFS& getInstance() {
        static SovereignVFS instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignVFS"; }

    void init() {
        sigma_log("[VFS] Orchestrating Sovereign Lattice Filesystem...");
        m_nodes = new SigmaMap<const char*, sigma_file_t*>();
        // Simulated mount of the Initial RAM Shard (IRS)
        sigma_log("[VFS] IRS Shard mounted at lattice root (/).");
    }

    sigma_file_t* open(const char* path) {
        sigma_printf("[VFS] Accessing shard: %s\n", path);
        for (sigma_u32 i = 0; i < m_nodes->size(); i++) {
            if (sigma_streq(m_nodes->key_at(i), path)) {
                return *m_nodes->at_index(i);
            }
        }
        return SIGMA_NULL; 
    }

    void registerShard(const char* path, sigma_file_t* file) {
        m_nodes->insert(path, file);
    }

    sigma_status read(sigma_file_t* file, void* buf, sigma_u32 size) {
        if (!file || !file->buffer) return 1;
        sigma_u32 read_size = (size > file->size) ? file->size : size;
        sigma_memcpy(buf, file->buffer, read_size);
        return 0;
    }

    sigma_status write(sigma_file_t* file, const void* buf, sigma_u32 size) {
        if (!file || !file->buffer) return 1;
        sigma_memcpy(file->buffer, buf, size);
        file->size = size;
        return 0;
    }

    void audit() {
        sigma_printf("\n--- Σ SOVEREIGN VFS AUDIT ---\n");
        sigma_printf("| Mounted Shards : %d\n", m_nodes->size());
        for (sigma_u32 i = 0; i < m_nodes->size(); i++) {
            sigma_printf("| [%d] %-15s | Size: %u bytes\n", i, m_nodes->key_at(i), (*m_nodes->at_index(i))->size);
        }
    }

private:
    SovereignVFS() : m_nodes(SIGMA_NULL) {}
    SigmaMap<const char*, sigma_file_t*>* m_nodes;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

/* --- C Interface for Kernel Interop --- */
extern "C" void vfs_init() {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().init();
}

extern "C" sigma_file_t* vfs_open(const char* path) {
    return SigmaOS::Kernel::FS::SovereignVFS::getInstance().open(path);
}

/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN STORAGE SHARD (S-STOR)
 * =========================================================================
 * Mission: Unified, robust file system abstractions (Lattice FS, NVMe).
 * Absorbing Linux VFS concepts but heavily sandboxed and atomic.
 * =========================================================================
 */

#ifndef SIGMA_STORAGE_H
#define SIGMA_STORAGE_H

#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Storage {

enum class FileSystemType {
    LATTICE_FS,  // Native Sovereign FS
    FAT32,
    EXT2,
    NVME_RAW
};

struct FileNode {
    char name[64];
    sigma_size_t size;
    sigma_u32 permissions;
    sigma_u64 inode;
    sigma_u8* data_ptr; // Shard-isolated memory mapped pointer
};

class SovereignStorageShard : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignStorageShard> {
    friend class SigmaOS::SigmaSingleton<SovereignStorageShard>;
public:
    const char* type_name() const noexcept override { return "SovereignStorageShard"; }

    sigma_status init();
    sigma_status mount(const char* device, const char* mount_point, FileSystemType fs_type);
    
    // Abstracted VFS API
    sigma_status file_open(const char* path, int flags, int* out_fd);
    sigma_isize  file_read(int fd, void* buffer, sigma_size_t count);
    sigma_isize  file_write(int fd, const void* buffer, sigma_size_t count);
    sigma_status file_close(int fd);

private:
    SovereignStorageShard() : m_initialized(false) {}
    bool m_initialized;
    
    // Mount point resolution tables
    // Inode caches
};

} // namespace Storage
} // namespace SigmaOS

#endif /* SIGMA_STORAGE_H */

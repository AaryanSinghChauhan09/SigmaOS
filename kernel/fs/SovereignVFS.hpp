#ifndef SOVEREIGN_VFS_HPP
#define SOVEREIGN_VFS_HPP

#include "../../include/core/sigma_types.h"

namespace SigmaOS {
namespace FS {

/**
 * @class SovereignVFS
 * @brief Sovereign File System (S-VFS) Shard for SigmaOS
 * 
 * Provides an isolated, sovereign shard for file and storage abstraction.
 * Includes support for journaling, crash recovery, amnesic persistence,
 * and unified node mapping independent of traditional VFS architectures.
 */
class SovereignVFS {
public:
    static SovereignVFS& getInstance() {
        static SovereignVFS instance;
        return instance;
    }

    /**
     * @brief Mounts a sovereign volume to the lattice tree.
     */
    int mount_volume(uint32_t device_id, const char* mount_path);

    /**
     * @brief Opens a file in the sovereign storage array.
     */
    int open_file(const char* path, int flags);

    /**
     * @brief Reads from an open file descriptor.
     */
    size_t read_file(int fd, void* buffer, size_t len);

    /**
     * @brief Writes to an open file descriptor.
     */
    size_t write_file(int fd, const void* buffer, size_t len);

    /**
     * @brief Writes an entry to the S-VFS journal to prevent data loss.
     */
    bool write_journal(const char* operation, const char* target);

    /**
     * @brief Performs crash recovery using the S-VFS journal.
     */
    bool perform_crash_recovery();

    /**
     * @brief Creates a sovereign sandbox for package manager operations.
     */
    bool isolate_package_sandbox(const char* pkg_name, const char* sandbox_path);

private:
    SovereignVFS();
    ~SovereignVFS();

    SovereignVFS(const SovereignVFS&) = delete;
    SovereignVFS& operator=(const SovereignVFS&) = delete;

    bool m_vfs_initialized;
    uint32_t m_active_mounts;
    bool m_journal_active;
};

} // namespace FS
} // namespace SigmaOS

extern "C" {
    int vfs_mount(uint32_t dev, const char* path);
    int vfs_open(const char* path, int flags);
    size_t vfs_read(int fd, void* buffer, size_t len);
    size_t vfs_write(int fd, const void* buffer, size_t len);
}

#endif // SOVEREIGN_VFS_HPP

#include "../../../../include/sigma_log.h"
#include "../../../../include/hal/sigma_hal.h"
#include "../../../../include/libc/SovereignLibC.h"
#include "../../../../include/fs/sigma_vfs.h"

namespace SigmaOS {
namespace Kernel {
namespace FS {

void SovereignVFS::init() {
    sigma_log("S [VFS]: Initializing Sovereign Virtual File System...");
}

void SovereignVFS::mount(const char* source, const char* target) {
    sigma_log("S [VFS]: Mounting shard %s to %s...\n", source, target);
    this->m_mount_count++;
}

void SovereignVFS::listFiles(const char* path) {
    sigma_log("S [VFS]: Listing files in lattice zone %s...\n", path);
}

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void vfs_init() {
    SigmaOS::Kernel::FS::SovereignVFS::init();
}

void vfs_mount(const char* source, const char* target) {
    SigmaOS::Kernel::FS::SovereignVFS::mount(source, target);
}

void vfs_list_files(const char* path) {
    SigmaOS::Kernel::FS::SovereignVFS::listFiles(path);
}





} // extern "C"
 
#include "sigma_hal.h"
#include "SovereignLibC.h"
#include "sigma_vfs.h"

namespace SigmaOS {
namespace Kernel {
namespace FS {

void SovereignVFS::init() {
    sigma_log("Σ [VFS]: Initializing Sovereign Virtual File System...");
}

void SovereignVFS::mount(const char* source, const char* target) {
    sigma_printf("Σ [VFS]: Mounting shard %s to %s...\n", source, target);
    this->m_mount_count++;
}

void SovereignVFS::listFiles(const char* path) {
    sigma_printf("Σ [VFS]: Listing files in lattice zone %s...\n", path);
}

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void vfs_init() {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().init();
}

extern "C" void vfs_mount(const char* source, const char* target) {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().mount(source, target);
}

extern "C" void vfs_list_files(const char* path) {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().listFiles(path);
}




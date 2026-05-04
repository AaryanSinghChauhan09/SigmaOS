#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_vfs.h"

namespace SigmaOS {
namespace Kernel {
namespace FS {

SovereignVFS& SovereignVFS::getInstance() {
    static SovereignVFS instance;
    return instance;
}

void SovereignVFS::init() {
    sigma_log("Σ [VFS]: Initializing Sovereign Virtual File System...");
    // Setup root node, etc.
}

sigma_vfs_node_t* SovereignVFS::open(const char* path) {
    sigma_printf("Σ [VFS]: Opening node '%s'...\n", path);
    // Find node in lattice
    return SIGMA_NULL;
}

sigma_ssize_t SovereignVFS::read(sigma_vfs_node_t* node, void* buf, sigma_size_t size) {
    if (node && node->read) {
        return node->read(node, buf, size);
    }
    return -1;
}

sigma_ssize_t SovereignVFS::write(sigma_vfs_node_t* node, const void* buf, sigma_size_t size) {
    if (node && node->write) {
        return node->write(node, buf, size);
    }
    return -1;
}

void SovereignVFS::close(sigma_vfs_node_t* node) {
    if (node && node->close) {
        node->close(node);
    }
}

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void vfs_init() {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().init();
}

extern "C" sigma_vfs_node_t* vfs_open(const char* path) {
    return SigmaOS::Kernel::FS::SovereignVFS::getInstance().open(path);
}

extern "C" sigma_ssize_t vfs_read(sigma_vfs_node_t* node, void* buf, sigma_size_t size) {
    return SigmaOS::Kernel::FS::SovereignVFS::getInstance().read(node, buf, size);
}

extern "C" sigma_ssize_t vfs_write(sigma_vfs_node_t* node, const void* buf, sigma_size_t size) {
    return SigmaOS::Kernel::FS::SovereignVFS::getInstance().write(node, buf, size);
}

extern "C" void vfs_close(sigma_vfs_node_t* node) {
    SigmaOS::Kernel::FS::SovereignVFS::getInstance().close(node);
}

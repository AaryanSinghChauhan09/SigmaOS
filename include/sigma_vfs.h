#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- C-Linkage Types --- */
typedef struct {
    char      name[128];
    sigma_u64 size;
    sigma_u32 flags;
    sigma_u32 inode;
} sigma_vfs_node_t;

#ifdef __cplusplus
} // Close extern "C"

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignVFS {
public:
    static SovereignVFS& getInstance() {
        static SovereignVFS instance;
        return instance;
    }

    void init();
    void mount(const char* source, const char* target);
    void listFiles(const char* path);

private:
    SovereignVFS() : m_mount_count(0) {}
    sigma_u32 m_mount_count;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
#endif

/* --- C Bridge Functions --- */
void vfs_init(void);
void vfs_mount(const char* source, const char* target);
void vfs_list_files(const char* path);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */


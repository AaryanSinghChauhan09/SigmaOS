#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct sigma_vfs_node {
    char name[128];
    sigma_u32 type;
    sigma_u32 size;
    void* private_data;
    
    // Interface Pattern (Function Pointers)
    sigma_ssize_t (*read)(struct sigma_vfs_node* node, void* buffer, sigma_size_t size);
    sigma_ssize_t (*write)(struct sigma_vfs_node* node, const void* buffer, sigma_size_t size);
    void (*close)(struct sigma_vfs_node* node);
} sigma_vfs_node_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignVFS {
public:
    static SovereignVFS& getInstance();

    void init();
    sigma_vfs_node_t* open(const char* path);
    sigma_ssize_t read(sigma_vfs_node_t* node, void* buf, sigma_size_t size);
    sigma_ssize_t write(sigma_vfs_node_t* node, const void* buf, sigma_size_t size);
    void close(sigma_vfs_node_t* node);
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void vfs_init(void);
sigma_vfs_node_t* vfs_open(const char* path);
sigma_ssize_t vfs_read(sigma_vfs_node_t* node, void* buf, sigma_size_t size);
sigma_ssize_t vfs_write(sigma_vfs_node_t* node, const void* buf, sigma_size_t size);
void vfs_close(sigma_vfs_node_t* node);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VFS_H */

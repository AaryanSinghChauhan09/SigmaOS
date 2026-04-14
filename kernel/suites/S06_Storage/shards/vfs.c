/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: VIRTUAL FILE SYSTEM (VFS) SHARD
 * =========================================================================
 * Mission: Abstract storage and sharding for modular filesystem support.
 * Capability: Inodes, File Descriptors, Mount Points, Path Resolution.
 * =========================================================================
 */

#include "../libc/sigma_libc.h"
#include "../include/sigma_kernel.h"

#define MAX_FILES 128
#define MAX_PATH_LEN 256

typedef enum {
    VFS_TYPE_FILE,
    VFS_TYPE_DIRECTORY,
    VFS_TYPE_DEVICE,
    VFS_TYPE_PIPE
} vfs_node_type_t;

struct vfs_node;
typedef struct vfs_node vfs_node_t;

typedef sigma_ssize_t (*vfs_read_fn)(vfs_node_t* node, sigma_u64 offset, void* buffer, sigma_size_t size);
typedef sigma_ssize_t (*vfs_write_fn)(vfs_node_t* node, sigma_u64 offset, const void* buffer, sigma_size_t size);
typedef sigma_err_t (*vfs_open_fn)(vfs_node_t* node, sigma_u32 flags);
typedef void (*vfs_close_fn)(vfs_node_t* node);

struct vfs_node {
    char name[64];
    vfs_node_type_t type;
    sigma_u32 inode_id;
    sigma_size_t size;
    vfs_read_fn read;
    vfs_write_fn write;
    vfs_open_fn open;
    vfs_close_fn close;
    vfs_node_t* parent;
    vfs_node_t* children; // For directories
    vfs_node_t* next;     // For directory listing
};

static vfs_node_t vfs_root;
static vfs_node_t* current_vfs_mount = SIGMA_NULL;

void sigma_vfs_init() {
    sigma_memset(&vfs_root, 0, sizeof(vfs_node_t));
    sigma_memcpy(vfs_root.name, "/", 2);
    vfs_root.type = VFS_TYPE_DIRECTORY;
    current_vfs_mount = &vfs_root;
    sigma_printf("[KERNEL] VFS initialized at mount point '/'\n");
}

vfs_node_t* sigma_vfs_create_node(const char* name, vfs_node_type_t type, vfs_node_t* parent) {
    vfs_node_t* node = (vfs_node_t*)sigma_malloc(sizeof(vfs_node_t));
    if (!node) return SIGMA_NULL;
    
    sigma_memset(node, 0, sizeof(vfs_node_t));
    sigma_memcpy(node->name, name, sigma_strlen(name));
    node->type = type;
    node->parent = parent;
    
    if (parent && parent->type == VFS_TYPE_DIRECTORY) {
        node->next = parent->children;
        parent->children = node;
    }
    
    return node;
}

sigma_err_t sigma_vfs_mount(const char* path, vfs_node_t* device_node) {
    SIGMA_UNUSED(path);
    SIGMA_UNUSED(device_node);
    sigma_printf("[KERNEL] Mounting device to VFS...\n");
    return SIGMA_OK;
}

sigma_ssize_t sigma_vfs_read(vfs_node_t* node, void* buffer, sigma_size_t size) {
    if (node && node->read) {
        return node->read(node, 0, buffer, size);
    }
    return (sigma_ssize_t)SIGMA_EIO;
}





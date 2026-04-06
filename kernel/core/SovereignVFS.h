#ifndef SOVEREIGN_VFS_H
#define SOVEREIGN_VFS_H

#include <stdint.h>

/*
 * Σ SIGMAOS: VIRTUAL FILE SYSTEM ABSTRACTION
 * Implements /dev/ abstraction decoupled from hardcoded drivers.
 */

struct vfs_node;

typedef uint64_t (*vfs_read_func)(struct vfs_node* node, uint64_t offset, uint64_t size, uint8_t* buffer);
typedef uint64_t (*vfs_write_func)(struct vfs_node* node, uint64_t offset, uint64_t size, uint8_t* buffer);
typedef void (*vfs_open_func)(struct vfs_node* node);
typedef void (*vfs_close_func)(struct vfs_node* node);

typedef struct vfs_node {
    char name[128];
    uint32_t mask;
    uint32_t uid;
    uint32_t gid;
    uint32_t flags;
    uint32_t inode;
    uint32_t length;
    uint32_t impl; // implementation-dependent descriptor

    vfs_read_func read;
    vfs_write_func write;
    vfs_open_func open;
    vfs_close_func close;
    
    struct vfs_node* ptr; // Used for mounts / aliases
} vfs_node_t;

// Global Root mount functions
void vfs_register_node(vfs_node_t* dev_node);
vfs_node_t* vfs_fetch_node(const char* name);

#endif // SOVEREIGN_VFS_H

#ifndef SOVEREIGN_VFS_H
#define SOVEREIGN_VFS_H

#include "SovereignLibC.h"

/*
 * Σ SIGMAOS: VIRTUAL FILE SYSTEM ABSTRACTION
 * Implements /dev/ abstraction decoupled from hardcoded drivers.
 */

struct vfs_node;

typedef sigma_u64 (*vfs_read_func)(struct vfs_node* node, sigma_u64 offset, sigma_u64 size, sigma_u8* buffer);
typedef sigma_u64 (*vfs_write_func)(struct vfs_node* node, sigma_u64 offset, sigma_u64 size, sigma_u8* buffer);
typedef void (*vfs_open_func)(struct vfs_node* node);
typedef void (*vfs_close_func)(struct vfs_node* node);

typedef struct vfs_node {
    char name[128];
    sigma_u32 mask;
    sigma_u32 uid;
    sigma_u32 gid;
    sigma_u32 flags;
    sigma_u32 inode;
    sigma_u32 length;
    sigma_u32 impl; // implementation-dependent descriptor

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

#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: VIRTUAL FILE SYSTEM (VFS) (v1.0)
 * =============================================================================
 * Principles: Everything is a Shard. Zero-Abstract Hardware Abstraction.
 * =============================================================================
 */
#ifndef SIGMA_VFS_H
#define SIGMA_VFS_H

#include "core/sigma_kernel_types.h"

typedef struct vfs_node {
    char name[128];
    sigma_u32  flags;
    sigma_u32  size;
    void* private_data;
    
    /* Function Pointers (Sovereign Interface) */
    sigma_u32 (*read)(struct vfs_node* node, sigma_u32 offset, sigma_u32 size, sigma_u8* buffer);
    sigma_u32 (*write)(struct vfs_node* node, sigma_u32 offset, sigma_u32 size, sigma_u8* buffer);
    void (*open)(struct vfs_node* node);
    void (*close)(struct vfs_node* node);
    
    struct vfs_node* next;
} vfs_node_t;

void vfs_register(vfs_node_t* node);
vfs_node_t* vfs_find(const char* name);

#endif

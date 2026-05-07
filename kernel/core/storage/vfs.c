#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: VIRTUAL FILE SYSTEM (VFS) CORE
 * =============================================================================
 */
#include "vfs.h"

static vfs_node_t* vfs_root = SIGMA_NULL;

void vfs_register(vfs_node_t* node) {
    node->next = vfs_root;
    vfs_root = node;
}

vfs_node_t* vfs_find(const char* name) {
    vfs_node_t* curr = vfs_root;
    while (curr) {
        if (sigma_strcmp(curr->name, name) == 0) return curr;
        curr = curr->next;
    }
    return SIGMA_NULL;
}

sigma_u32 vfs_read(vfs_node_t* node, sigma_u32 offset, sigma_u32 size, sigma_u8* buffer) {
    if (node && node->read) return node->read(node, offset, size, buffer);
    return 0;
}

sigma_u32 vfs_write(vfs_node_t* node, sigma_u32 offset, sigma_u32 size, sigma_u8* buffer) {
    if (node && node->write) return node->write(node, offset, size, buffer);
    return 0;
}

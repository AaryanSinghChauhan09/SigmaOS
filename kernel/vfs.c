/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTUAL FILE SYSTEM (VFS) CORE
 * =============================================================================
 */
#include "../include/vfs.h"

static vfs_node_t* vfs_root = NULL;

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
    return NULL;
}

u32 vfs_read(vfs_node_t* node, u32 offset, u32 size, u8* buffer) {
    if (node && node->read) return node->read(node, offset, size, buffer);
    return 0;
}

u32 vfs_write(vfs_node_t* node, u32 offset, u32 size, u8* buffer) {
    if (node && node->write) return node->write(node, offset, size, buffer);
    return 0;
}

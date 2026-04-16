#include "../../../include/sigma_fs.h"
#include <string.h>

/* =========================================================================
 * SIGMA OS: VIRTUAL FILE SYSTEM & STORAGE IMPLEMENTATION
 * Pure C implementation of physical disk structures and INode trees.
 * ========================================================================= */

#define MAX_INODES 1024

// In an actual kernel, these arrays reside dynamically on ATA/SATA disk sectors.
// For the pure C pre-alpha we simulate the disk sectors in high memory bounds.
static sigma_inode_t inode_table[MAX_INODES];
static uint32_t next_free_inode = 1;

void sigma_fs_init() {
    // Zero out the central inode tracking table
    for (int i = 0; i < MAX_INODES; i++) {
        inode_table[i].inode_id = 0;
        inode_table[i].size_bytes = 0;
        inode_table[i].data_block_head = 0;
    }

    // Initialize the absolute root directory ('/')
    sigma_inode_t* root = &inode_table[0];
    root->inode_id = 0;
    root->type = NODE_TYPE_DIRECTORY;
    root->permissions = 0x777; // RWX globally
    root->size_bytes = 0; 
    root->data_block_head = 0; // Empty directory block
}

int sigma_fs_create_file(const char* name, uint32_t parent_inode) {
    if (next_free_inode >= MAX_INODES) {
        return -1; // Out of memory/file handles
    }

    if (parent_inode >= MAX_INODES || inode_table[parent_inode].type != NODE_TYPE_DIRECTORY) {
        return -1; // Invalid parent routing
    }

    // Allocate physical tracking inode
    uint32_t new_id = next_free_inode++;
    sigma_inode_t* node = &inode_table[new_id];
    node->inode_id = new_id;
    node->type = NODE_TYPE_FILE;
    node->size_bytes = 0;
    node->permissions = 0x644; // RW for OS, R for userland layers
    
    // Abstract hook: The actual name is mapped into the parent's data block as a sigma_dir_entry_t.
    // Abstract hook: PMM allocates the raw pointer for node->data_block_head

    return new_id;
}

int sigma_fs_read_file(uint32_t inode_id, uint8_t* buffer, size_t length) {
    if (inode_id >= MAX_INODES || inode_table[inode_id].type != NODE_TYPE_FILE) {
        return -1;
    }

    sigma_inode_t* node = &inode_table[inode_id];
    
    // Simulated boundary check against memory allocation length
    size_t bytes_to_read = length < node->size_bytes ? length : node->size_bytes;
    
    // Abstract hook: Reads raw SATA/NVME disk blocks chained off node->data_block_head into volatile RAM buffer.

    return bytes_to_read;
}

int sigma_fs_write_file(uint32_t inode_id, const uint8_t* buffer, size_t length) {
    if (inode_id >= MAX_INODES || inode_table[inode_id].type != NODE_TYPE_FILE) {
        return -1;
    }

    sigma_inode_t* node = &inode_table[inode_id];

    // Abstract hook: Passes standard buffers back down into 4KB aligned sigma_disk_block_t structs writing back to NVME storage over PCIe.
    node->size_bytes += length;
    
    return length;
}

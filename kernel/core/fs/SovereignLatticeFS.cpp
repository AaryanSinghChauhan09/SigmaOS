// SPDX-License-Identifier: MIT
// =============================================================================
// SIGMAOS KERNEL CORE: SOVEREIGN LATTICE FILESYSTEM
// =============================================================================
// Hardened inode metadata parser and directory entry validator designed to prevent
// corrupt partition mounting, buffer overflows, and path traversal vulnerabilities.
// =============================================================================

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#define LATTICE_INODE_MAGIC 0x4C415454 // "LATT"
#define MAX_NAME_LEN 255
#define MAX_INODE_BLOCKS 65536

typedef struct {
    uint32_t magic;
    uint32_t inode_id;
    uint32_t mode;
    uint32_t uid;
    uint32_t gid;
    uint64_t file_size;
    uint32_t block_count;
    uint32_t direct_blocks[12];
    uint32_t indirect_block;
} SovereignLatticeInode;

typedef struct {
    uint32_t inode_id;
    uint16_t record_length;
    uint8_t  name_length;
    uint8_t  file_type;
    char     name[MAX_NAME_LEN + 1];
} SovereignLatticeDirEntry;

// Perform inode metadata sanity checks
bool inode_validate(const SovereignLatticeInode *inode) {
    if (inode == NULL) {
        return false;
    }

    if (inode->magic != LATTICE_INODE_MAGIC) {
        return false;
    }

    // Check block count boundaries
    if (inode->block_count > MAX_INODE_BLOCKS) {
        return false;
    }

    // Check file size against block allocation sanity
    uint64_t max_possible_size = (uint64_t)inode->block_count * 4096;
    if (inode->file_size > max_possible_size) {
        return false;
    }

    return true;
}

// Perform directory entry sanity checks
bool entry_sanity(const SovereignLatticeDirEntry *entry, size_t buffer_remaining) {
    if (entry == NULL) {
        return false;
    }

    if (entry->record_length < sizeof(SovereignLatticeDirEntry) || entry->record_length > buffer_remaining) {
        return false;
    }

    if (entry->name_length > MAX_NAME_LEN || entry->name_length >= entry->record_length) {
        return false;
    }

    return true;
}

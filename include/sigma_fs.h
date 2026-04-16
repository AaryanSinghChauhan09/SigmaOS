#ifndef SIGMA_FS_H
#define SIGMA_FS_H

#include <stdint.h>
#include <stddef.h>

/* =========================================================================
 * SIGMA OS: VIRTUAL FILE SYSTEM & STORAGE SHARD (SYSTEM-LEVEL HEADER)
 * Implements the base Ext2-inspired inode logical structures natively.
 * ========================================================================= */

#define SIGMA_FS_MAGIC 0x5168
#define MAX_FILES_PER_DIR 128
#define MAX_FILENAME_LEN 32

typedef enum {
    NODE_TYPE_FILE = 1,
    NODE_TYPE_DIRECTORY = 2,
    NODE_TYPE_SYMLINK = 3
} sigma_inode_type_t;

// The raw physical data block descriptor for the disk
typedef struct {
    uint32_t block_id;
    uint32_t next_block_id; // Pointer to next block if file exceeds block size
    uint8_t  data[4088];    // Assumes 4KB sector alignment
} __attribute__((packed)) sigma_disk_block_t;

// The logical File Node representation
typedef struct {
    uint32_t inode_id;
    sigma_inode_type_t type;
    uint32_t size_bytes;
    uint32_t permissions;
    uint32_t creation_time;
    uint32_t data_block_head; // Pointer to start of Sigma Disk Block Chain
} __attribute__((packed)) sigma_inode_t;

typedef struct {
    uint32_t inode_id;
    char name[MAX_FILENAME_LEN];
} __attribute__((packed)) sigma_dir_entry_t;

void sigma_fs_init();
int sigma_fs_create_file(const char* name, uint32_t parent_inode);
int sigma_fs_read_file(uint32_t inode_id, uint8_t* buffer, size_t length);
int sigma_fs_write_file(uint32_t inode_id, const uint8_t* buffer, size_t length);

#endif

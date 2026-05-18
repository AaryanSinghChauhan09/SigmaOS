#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Self-Optimising Filesystem
// Reorganises itself based on file usage patterns
// Hot files kept contiguous, cold files compressed
// ---------------------------------------------------------

#define MAX_FS_FILES     512
#define HOT_THRESHOLD    10   // accesses before file is "hot"
#define COLD_THRESHOLD   2    // accesses before file is "cold"

typedef enum {
    FILE_TIER_HOT,   // Frequently accessed, keep contiguous
    FILE_TIER_WARM,  // Moderate use
    FILE_TIER_COLD   // Rarely accessed, compress + move to back
} file_tier_t;

typedef struct {
    uint32_t    inode_id;
    uint32_t    start_block;
    uint32_t    block_count;
    uint32_t    access_count;
    uint64_t    last_access_tick;
    file_tier_t tier;
    uint8_t     compressed;
} fs_file_entry_t;

static fs_file_entry_t file_table[MAX_FS_FILES];
static uint32_t file_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Register a file in the self-optimizing FS tracker
uint32_t sofs_register(uint32_t inode, uint32_t start_block, uint32_t blocks) {
    if (file_count >= MAX_FS_FILES) return UINT32_MAX;
    fs_file_entry_t* f = &file_table[file_count];
    f->inode_id      = inode;
    f->start_block   = start_block;
    f->block_count   = blocks;
    f->access_count  = 0;
    f->last_access_tick = 0;
    f->tier          = FILE_TIER_WARM;
    f->compressed    = 0;
    return file_count++;
}

// Record a file access (called on every read/write)
void sofs_record_access(uint32_t inode_id, uint64_t current_tick) {
    for (uint32_t i = 0; i < file_count; i++) {
        if (file_table[i].inode_id == inode_id) {
            file_table[i].access_count++;
            file_table[i].last_access_tick = current_tick;
            return;
        }
    }
}

// Decompress a cold file before it is accessed
static void sofs_decompress(fs_file_entry_t* f) {
    if (!f->compressed) return;
    // In real impl: LZ4/zstd decompression of f->start_block..f->block_count
    f->compressed = 0;
}

// Compress a cold file to reclaim space
static void sofs_compress(fs_file_entry_t* f) {
    if (f->compressed) return;
    // In real impl: LZ4/zstd compression pass over blocks
    f->compressed = 1;
    audit_chain_append(0, 1, "SOFS_FILE_COMPRESSED");
}

// Defragment: move hot file to contiguous block region (stub)
static void sofs_defrag_hot(fs_file_entry_t* f) {
    // In real impl: allocate contiguous pages from NUMA allocator,
    // copy blocks, update block map, free old scattered blocks
    audit_chain_append(0, 1, "SOFS_FILE_DEFRAGGED");
}

// Periodic optimizer — called by kernel scheduler tick or timer
void sofs_optimize(uint64_t current_tick) {
    for (uint32_t i = 0; i < file_count; i++) {
        fs_file_entry_t* f = &file_table[i];

        // Classify into tiers
        uint64_t age = current_tick - f->last_access_tick;

        if (f->access_count >= HOT_THRESHOLD && age < 1000) {
            if (f->tier != FILE_TIER_HOT) {
                f->tier = FILE_TIER_HOT;
                if (f->compressed) sofs_decompress(f);
                sofs_defrag_hot(f);     // Move to contiguous region
            }
        } else if (f->access_count <= COLD_THRESHOLD || age > 100000) {
            if (f->tier != FILE_TIER_COLD) {
                f->tier = FILE_TIER_COLD;
                sofs_compress(f);       // Compress and move to back
            }
        } else {
            f->tier = FILE_TIER_WARM;
        }
    }
}

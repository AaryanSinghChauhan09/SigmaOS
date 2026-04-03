/* 
 Σ SIGMAOS ZENITH: SLAB ALLOCATOR HEADER (v1700.0)
 Mission: Industrial-Grade Kernel Object Management.
*/

#ifndef SIGMA_SLAB_H
#define SIGMA_SLAB_H

#include <stdint.h>
#include <stdbool.h>

#define SLAB_BLOCK_SIZE 4096
#define SLAB_MAX_BLOCKS 1024

// Σ SLAB BLOCK STRUCTURE
typedef struct {
    uint32_t id;
    bool is_free;
    void* addr;
} sigma_slab_block;

// Σ SLAB POOL STRUCTURE
typedef struct {
    sigma_slab_block blocks[SLAB_MAX_BLOCKS];
    uint32_t free_count;
} sigma_slab_pool;

// Σ ALLOCATOR API
void sigma_slab_init();
void* sigma_kmalloc(uint32_t size);
void sigma_kfree(void* ptr);

#endif

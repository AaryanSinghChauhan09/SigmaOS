/* 
 Σ SIGMAOS ZENITH: SOVEREIGN SLAB ALLOCATOR (v1700.0)
 Mission: Direct Silicon Memory Acquisition & Fragmentation Correction.
*/

#include "slab.h"

// Σ KERNEL MEMORY POOL (4MB Stack-Based Prototype)
static uint8_t g_SigmaSlabPool[SLAB_BLOCK_SIZE * SLAB_MAX_BLOCKS];
static sigma_slab_pool g_Pool;

// Σ INITIALIZE SLAB POOL
void sigma_slab_init() {
    g_Pool.free_count = SLAB_MAX_BLOCKS;
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        g_Pool.blocks[i].id = i;
        g_Pool.blocks[i].is_free = true;
        g_Pool.blocks[i].addr = (void*)&g_SigmaSlabPool[i * SLAB_BLOCK_SIZE];
    }
}

// Σ KERNEL ALLOCATE FUNCTION (v1700.0)
void* sigma_kmalloc(uint32_t size) {
    if (size > SLAB_BLOCK_SIZE) return (void*)0; // Only single block alloc supported
    
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_Pool.blocks[i].is_free) {
            g_Pool.blocks[i].is_free = false;
            g_Pool.free_count--;
            return g_Pool.blocks[i].addr;
        }
    }
    return (void*)0; // Out of memory
}

// Σ KERNEL FREE FUNCTION
void sigma_kfree(void* ptr) {
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_Pool.blocks[i].addr == ptr) {
            g_Pool.blocks[i].is_free = true;
            g_Pool.free_count++;
            return;
        }
    }
}

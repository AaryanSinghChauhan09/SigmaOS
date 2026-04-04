/* 
 Σ SIGMAOS ZENITH: SOVEREIGN SLAB ALLOCATOR (v1700.0)
 Mission: Direct Silicon Memory Acquisition & Fragmentation Correction.
*/

#include "slab.h"

// Σ KERNEL MEMORY POOL (4MB Stack-Based Prototype)
static uint8_t g_SigmaSlabPool[SLAB_BLOCK_SIZE * SLAB_MAX_BLOCKS];
static sigma_slab_pool g_Pool;

#include "../sigma_kernel_types.h"

static spinlock_t g_SlabLock;

// Σ INITIALIZE SLAB POOL
void sigma_slab_init() {
    spinlock_init(&g_SlabLock);
    g_Pool.free_count = SLAB_MAX_BLOCKS;
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        g_Pool.blocks[i].id = i;
        g_Pool.blocks[i].is_free = true;
        g_Pool.blocks[i].addr = (void*)&g_SigmaSlabPool[i * SLAB_BLOCK_SIZE];
    }
}

// Σ KERNEL ALLOCATE FUNCTION (v1700.0)
void* sigma_kmalloc(uint32_t size) {
    if (size > SLAB_BLOCK_SIZE) return (void*)0;
    
    spinlock_acquire(&g_SlabLock);
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_Pool.blocks[i].is_free) {
            g_Pool.blocks[i].is_free = false;
            g_Pool.free_count--;
            spinlock_release(&g_SlabLock);
            return g_Pool.blocks[i].addr;
        }
    }
    spinlock_release(&g_SlabLock);
    return (void*)0;
}

// Σ KERNEL FREE FUNCTION
void sigma_kfree(void* ptr) {
    if (!ptr) return;
    
    spinlock_acquire(&g_SlabLock);
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_Pool.blocks[i].addr == ptr) {
            if (!g_Pool.blocks[i].is_free) {
                g_Pool.blocks[i].is_free = true;
                g_Pool.free_count++;
            }
            spinlock_release(&g_SlabLock);
            return;
        }
    }
    spinlock_release(&g_SlabLock);
}

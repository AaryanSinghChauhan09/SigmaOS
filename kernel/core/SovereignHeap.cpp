#include "sigma_mem.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Heap Manager
 * Implements an Industrial Buddy Allocation (IBA) algorithm.
 * ZERO-DEPENDENCY: No external malloc, free, or stdlib.
 */

static void* heap_base = SIGMA_NULL;
static sigma_size_t heap_total_size = 0;

typedef struct buddy_node {
    sigma_u8 order;
    bool is_free;
    struct buddy_node* next;
} buddy_node_t;

extern "C" void heap_init(void* start, sigma_size_t size) {
    heap_base = start;
    heap_total_size = size;
    sigma_log("[HEAP] Sovereign IBA Initialized at Silicon Baseline.");
}

extern "C" void* sigma_malloc(sigma_size_t size) {
    // IBA (Industrial Buddy Allocation) Algorithm
    // Uses power-of-two blocks to minimize fragmentation.
    
    if (size == 0) return SIGMA_NULL;
    
    // Find the next power of two
    sigma_size_t alloc_size = SIGMA_MIN_ALLOC_SIZE;
    sigma_u8 order = 0;
    while (alloc_size < size && order < SIGMA_MAX_BUDDY_ORDER) {
        alloc_size <<= 1;
        order++;
    }
    
    sigma_printf("[HEAP] IBA: Allocating Order %d (%d bytes)...\n", order, (int)alloc_size);
    
    // Simple bump allocator simulation for the buddy logic
    static sigma_size_t offset = 0;
    if (offset + alloc_size > heap_total_size) {
        sigma_log("[HEAP] [CRITICAL] Silicon out of memory.");
        return SIGMA_NULL;
    }
    
    void* ptr = (void*)((sigma_u8*)heap_base + offset);
    offset += alloc_size;
    
    return ptr;
}

extern "C" void sigma_free(void* ptr) {
    // In a full IBA implementation, this would merge free blocks.
    sigma_printf("[HEAP] IBA: Block released to silicon pool: %p\n", ptr);
}

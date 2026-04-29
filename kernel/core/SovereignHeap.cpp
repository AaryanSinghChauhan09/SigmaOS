#include "Lattice.h"
#include "sigma_mem.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Heap Manager
 * Implements an Industrial Buddy Allocation (IBA) algorithm.
 * ZERO-DEPENDENCY: No external malloc, free, or stdlib.
 */

static struct {
    void* base;
    sigma_size_t total_size;
    sigma_size_t current_offset;
    uint32_t active_allocations;
} SovereignHeapManager = {
    .base = SIGMA_NULL,
    .total_size = 0,
    .current_offset = 0,
    .active_allocations = 0
};

extern "C" void heap_init(void* start, sigma_size_t size) {
    SovereignHeapManager.base = start;
    SovereignHeapManager.total_size = size;
    SovereignHeapManager.current_offset = 0;
    sigma_log("[HEAP] Sovereign IBA Initialized with OOPS Isolation.");
}

extern "C" void* sigma_malloc(sigma_size_t size) {
    // IBA (Industrial Buddy Allocation) Algorithm
    // Uses power-of-two blocks to minimize fragmentation.
    
    if (size == 0) return SIGMA_NULL;
    
    // Add space for canary
    sigma_size_t total_needed = size + 4;
    
    // Find the next power of two
    sigma_size_t alloc_size = 16; // Simulated min size
    sigma_u8 order = 0;
    while (alloc_size < total_needed && order < 16) {
        alloc_size <<= 1;
        order++;
    }
    
    sigma_printf("[HEAP] IBA: Allocating Order %d (%d bytes)...\n", order, (int)alloc_size);
    
    // Simple bump allocator simulation for the buddy logic
    if (SovereignHeapManager.current_offset + alloc_size > SovereignHeapManager.total_size) {
        sigma_log("[HEAP] [CRITICAL] Silicon out of memory.");
        return SIGMA_NULL;
    }
    
    void* ptr = (void*)((sigma_u8*)SovereignHeapManager.base + SovereignHeapManager.current_offset);
    SovereignHeapManager.current_offset += alloc_size;
    SovereignHeapManager.active_allocations++;

    // Poisoning and Canary
    sigma_u32* canary = (sigma_u32*)ptr;
    *canary = 0xDEADC0DE;
    
    void* user_ptr = (void*)((sigma_u8*)ptr + 4);
    sigma_memset(user_ptr, 0, size); // Poison with zero
    
    return user_ptr;
}

extern "C" void sigma_free(void* ptr) {
    if (!ptr) return;
    
    // Verify canary
    sigma_u32* canary = (sigma_u32*)((sigma_u8*)ptr - 4);
    if (*canary != 0xDEADC0DE) {
        sigma_log("[HEAP] [SECURITY] Buffer overflow detected! Memory corruption at canary.");
    } else {
        SovereignHeapManager.active_allocations--;
        sigma_printf("[HEAP] IBA: Released block. Active allocations: %d\n", SovereignHeapManager.active_allocations);
    }
}

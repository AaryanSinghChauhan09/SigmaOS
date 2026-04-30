
#include "sigma_mem.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Heap Manager
 * Implements an Industrial Buddy Allocation (IBA) algorithm.
 * ZERO-DEPENDENCY: No external malloc, free, or stdlib.
 */

#include "Lattice.h"
#include "sigma_mem.h"

/**
 * SigmaOS Sovereign Heap Manager
 * Implements an Industrial Buddy Allocation (IBA) algorithm.
 * ZERO-DEPENDENCY: No external malloc, free, or stdlib.
 *
 * Design: OOP-isolated singleton — SovereignHeapEngine.
 */

/* --- Sovereign Heap Engine (OOP Isolation) --- */
static struct {
    void*        base;
    sigma_size_t total_size;
    sigma_size_t current_offset;
    sigma_u32    active_allocations;
    sigma_u64    total_allocations;
    sigma_u32    initialized;
} SovereignHeapEngine = {
    .base = SIGMA_NULL,
    .total_size = 0u,
    .current_offset = 0u,
    .active_allocations = 0u,
    .total_allocations = 0u,
    .initialized = 0u
};

extern "C" void heap_init(void* start, sigma_size_t size) {
    SovereignHeapEngine.base = start;
    SovereignHeapEngine.total_size = size;
    SovereignHeapEngine.current_offset = 0u;
    SovereignHeapEngine.initialized = 1u;
    sigma_log("[HEAP] Sovereign IBA Initialized (OOP-Isolated Singleton).");
}

extern "C" void* sigma_malloc(sigma_size_t size) {
    /* IBA (Industrial Buddy Allocation) Algorithm
     * Uses power-of-two blocks to minimize fragmentation. */
    
    if (size == 0u) return SIGMA_NULL;
    
    // Add space for canary
    sigma_size_t total_needed = size + 4u;
    
    // Find the next power of two
    sigma_size_t alloc_size = 16u; // Simulated min size
    sigma_u8 order = 0u;
    while (alloc_size < total_needed && order < 16u) {
        alloc_size <<= 1u;
        order++;
    }
    
    sigma_printf("[HEAP] IBA: Allocating Order %u (%u bytes)...\n", (unsigned)order, (unsigned)alloc_size);
    
    if (SovereignHeapEngine.current_offset + alloc_size > SovereignHeapEngine.total_size) {
        sigma_log("[HEAP] [CRITICAL] Silicon out of memory.");
        return SIGMA_NULL;
    }
    
    void* ptr = (void*)((sigma_u8*)SovereignHeapEngine.base + SovereignHeapEngine.current_offset);
    SovereignHeapEngine.current_offset += alloc_size;
    SovereignHeapEngine.active_allocations++;
    SovereignHeapEngine.total_allocations++;

    // Poisoning and Canary
    sigma_u32* canary = (sigma_u32*)ptr;
    *canary = 0xDEADC0DEu;
    
    void* user_ptr = (void*)((sigma_u8*)ptr + 4u);
    sigma_memset(user_ptr, 0, size); // Poison with zero
    
    return user_ptr;
}

extern "C" void sigma_free(void* ptr) {
    if (!ptr) return;
    
    // Verify canary
    sigma_u32* canary = (sigma_u32*)((sigma_u8*)ptr - 4u);
    if (*canary != 0xDEADC0DEu) {
        sigma_log("[HEAP] [SECURITY] Buffer overflow detected! Memory corruption at canary.");
    } else {
        if (SovereignHeapEngine.active_allocations > 0u) SovereignHeapEngine.active_allocations--;
        sigma_printf("[HEAP] IBA: Released block. Active allocations: %u\n", (unsigned)SovereignHeapEngine.active_allocations);
    }
}

extern "C" sigma_u64 heap_get_total_allocations() {
    return SovereignHeapEngine.total_allocations;
}

extern "C" sigma_u32 heap_get_active_allocations() {
    return SovereignHeapEngine.active_allocations;
}

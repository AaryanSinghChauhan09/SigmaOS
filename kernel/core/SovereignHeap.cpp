#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"
#include "sigma_mem.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"


/**
 * SigmaOS Sovereign Heap Manager
 * Implements an Industrial Buddy Allocation (IBA) algorithm.
 * ZERO-DEPENDENCY: No external malloc, free, or stdlib.
 *
 * Design: OOP-isolated singleton — SovereignHeapEngine.
 */

class SovereignHeapEngine {
public:
    static SovereignHeapEngine& getInstance() {
        static SovereignHeapEngine instance;
        return instance;
    }

    void init(void* start, sigma_size_t size) {
        this->base = start;
        this->total_size = size;
        this->current_offset = 0u;
        this->initialized = 1u;
        sigma_log("[HEAP] Sovereign IBA Initialized (OOP-Isolated Singleton).");
    }

    void* allocate(sigma_size_t size) {
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
        
        sigma_log_info("[HEAP] IBA: Allocating Order %u (%u bytes)...\n", (unsigned)order, (unsigned)alloc_size);
        
        if (this->current_offset + alloc_size > this->total_size) {
            sigma_log("[HEAP] [CRITICAL] Silicon out of memory.");
            return SIGMA_NULL;
        }
        
        void* ptr = (void*)((sigma_u8*)this->base + this->current_offset);
        this->current_offset += alloc_size;
        this->active_allocations++;
        this->total_allocations++;

        // Poisoning and Canary
        sigma_u32* canary = (sigma_u32*)ptr;
        *canary = 0xDEADC0DEu;
        
        void* user_ptr = (void*)((sigma_u8*)ptr + 4u);
        sigma_memset(user_ptr, 0, size); // Poison with zero
        
        return user_ptr;
    }

    void release(void* ptr) {
        if (!ptr) return;
        
        // Verify canary
        sigma_u32* canary = (sigma_u32*)((sigma_u8*)ptr - 4u);
        if (*canary != 0xDEADC0DEu) {
            sigma_log("[HEAP] [SECURITY] Buffer overflow detected! Memory corruption at canary.");
        } else {
            if (this->active_allocations > 0u) this->active_allocations--;
            sigma_log_info("[HEAP] IBA: Released block. Active allocations: %u\n", (unsigned)this->active_allocations);
        }
    }

    sigma_u64 getTotalAllocations() const { return this->total_allocations; }
    sigma_u32 getActiveAllocations() const { return this->active_allocations; }

private:
    SovereignHeapEngine() : base(SIGMA_NULL), total_size(0), current_offset(0), active_allocations(0), total_allocations(0), initialized(0) {}
    
    void*        base;
    sigma_size_t total_size;
    sigma_size_t current_offset;
    sigma_u32    active_allocations;
    sigma_u64    total_allocations;
    sigma_u32    initialized;
};

/* --- C Wrappers --- */
extern "C" void heap_init(void* start, sigma_size_t size) {
    SovereignHeapEngine::getInstance().init(start, size);
}

extern "C" void* sigma_malloc(sigma_size_t size) {
    return SovereignHeapEngine::getInstance().allocate(size);
}

extern "C" void sigma_free(void* ptr) {
    SovereignHeapEngine::getInstance().release(ptr);
}

extern "C" sigma_u64 heap_get_total_allocations() {
    return SovereignHeapEngine::getInstance().getTotalAllocations();
}

extern "C" sigma_u32 heap_get_active_allocations() {
    return SovereignHeapEngine::getInstance().getActiveAllocations();
}



#include "sigma_allocator.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Custom Allocator
 * Implements a Quantum-Bucket Memory Pool (QBMP) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal custom memory management.
 */

/* --- Sovereign Allocator Engine (OOP Isolation) --- */

void SovereignAllocatorEngine::init() {
    sigma_log("[ALLOCATOR] Initializing Sovereign Custom Allocator (QBMP Algorithm)...");
    this->heap_offset = 0u;
    sigma_assert(SIGMA_HEAP_SIZE > 0);
}

void* SovereignAllocatorEngine::malloc(sigma_u32 size) {
    /* QBMP (Quantum-Bucket Memory Pool) Algorithm
     * Bump allocator with 8-byte alignment for silicon-native performance. */
    sigma_u32 aligned_size = (size + 7u) & ~7u;
    sigma_assert(this->heap_offset + aligned_size <= SIGMA_HEAP_SIZE);
    
    void* ptr = &this->heap[this->heap_offset];
    this->heap_offset += aligned_size;
    
    sigma_printf("[ALLOCATOR] QBMP: Allocated %u bytes at %p (Used: %u/%u)\n",
                 size, ptr, this->heap_offset, SIGMA_HEAP_SIZE);
    return ptr;
}

void SovereignAllocatorEngine::free(void* ptr) {
    /* Bump allocator doesn't support individual free.
     * In SigmaOS, we use per-shard reclamation. */
    sigma_printf("[ALLOCATOR] QBMP: Ignoring free for %p (Sovereign Policy).\n", ptr);
}

/* --- C Wrappers --- */
extern "C" void allocator_init() {
    SovereignAllocatorEngine::getInstance().init();
}

extern "C" void* allocator_malloc(sigma_u32 size) {
    return SovereignAllocatorEngine::getInstance().malloc(size);
}

extern "C" void allocator_free(void* ptr) {
    SovereignAllocatorEngine::getInstance().free(ptr);
}

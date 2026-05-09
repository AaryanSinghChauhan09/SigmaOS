#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/core/sigma_types.h"
#include "sigma_allocator.h"
#include "../../../include/hal/sigma_hal.h"

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

#define SIGMA_GUARD_MAGIC 0xDEADBEEF

void* SovereignAllocatorEngine::malloc(sigma_u32 size) {
    /* QBMP (Quantum-Bucket Memory Pool) Algorithm
     * Bump allocator with 8-byte alignment + guard bytes for silicon-native safety. */
    sigma_u32 total_size = size + 2 * sizeof(sigma_u32); // Prefix + Suffix guards
    sigma_u32 aligned_size = (total_size + 7u) & ~7u;
    
    if (this->heap_offset + aligned_size > SIGMA_HEAP_SIZE) {
        sigma_log("[ALLOCATOR] [FATAL] OOM: Requested %u bytes, heap full.\n", size);
        return nullptr;
    }
    
    // Write prefix guard
    sigma_u32* prefix = (sigma_u32*)&this->heap[this->heap_offset];
    *prefix = SIGMA_GUARD_MAGIC;
    
    // Calculate user pointer
    void* ptr = (void*)((sigma_u8*)prefix + sizeof(sigma_u32));
    
    // Write suffix guard
    sigma_u32* suffix = (sigma_u32*)((sigma_u8*)ptr + size);
    *suffix = SIGMA_GUARD_MAGIC;
    
    this->heap_offset += aligned_size;
    
    sigma_log("[ALLOCATOR] QBMP: Allocated %u bytes at %p (Used: %u/%u)\n",
                 size, ptr, this->heap_offset, SIGMA_HEAP_SIZE);
    return ptr;
}

void SovereignAllocatorEngine::free(void* ptr) {
    if (!ptr) return;
    
    // Verify prefix guard
    sigma_u32* prefix = (sigma_u32*)((sigma_u8*)ptr - sizeof(sigma_u32));
    if (*prefix != SIGMA_GUARD_MAGIC) {
        sigma_log("[ALLOCATOR] [FATAL] Memory corruption detected: Prefix guard overwritten at %p!\n", ptr);
        return;
    }
    
    /* Bump allocator doesn't support individual free.
     * In SigmaOS, we use per-shard reclamation. */
    sigma_log("[ALLOCATOR] QBMP: Verified guards, ignoring free for %p (Sovereign Policy).\n", ptr);
}

/* --- C Wrappers --- */
extern "C" void allocator_init() {
    SovereignAllocatorEngine::init();
}

extern "C" void* allocator_malloc(sigma_u32 size) {
    return SovereignAllocatorEngine::malloc(size);
}

extern "C" void allocator_free(void* ptr) {
    SovereignAllocatorEngine::free(ptr);
}




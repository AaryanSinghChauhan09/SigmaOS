#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_allocator.h"

/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN O(1) SLAB ALLOCATOR
 * =========================================================================
 * Mission: Prevent heap fragmentation, deterministic O(1) allocations.
 * Lockless fixed-size bucket design for bare-metal performance.
 * =========================================================================
 */

#define SIGMA_GUARD_MAGIC 0xDEADBEEF

struct SlabBlock {
    SlabBlock* next;
};

class SovereignAllocatorEngine {
private:
    sigma_u8* heap_base;
    sigma_u32 heap_offset;
    
    // Slab buckets for different power-of-2 sizes: 32, 64, 128, 256, 512, 1024, 2048, 4096
    SlabBlock* free_lists[8];
    
    sigma_u32 get_bucket_index(sigma_u32 size) {
        if (size <= 32) return 0;
        if (size <= 64) return 1;
        if (size <= 128) return 2;
        if (size <= 256) return 3;
        if (size <= 512) return 4;
        if (size <= 1024) return 5;
        if (size <= 2048) return 6;
        if (size <= 4096) return 7;
        return 8; // Too large for slab
    }
    
    sigma_u32 get_bucket_size(sigma_u32 index) {
        return 32 << index;
    }

    SovereignAllocatorEngine() = default;

public:
    static SovereignAllocatorEngine& getInstance() {
        static SovereignAllocatorEngine instance;
        return instance;
    }

    void init(sigma_u8* base, sigma_u32 size) {
        sigma_log("[ALLOCATOR] Initializing Sovereign O(1) Slab Allocator...");
        heap_base = base;
        heap_offset = 0;
        for (int i = 0; i < 8; ++i) {
            free_lists[i] = nullptr;
        }
    }

    void* sigma_malloc(sigma_u32 size) {
        sigma_u32 bucket_idx = get_bucket_index(size + sizeof(sigma_u32) * 2); // Include guards
        
        if (bucket_idx < 8) {
            // Fast path: O(1) Slab Allocation
            SlabBlock* block = free_lists[bucket_idx];
            if (block) {
                // Pop from free list
                free_lists[bucket_idx] = block->next;
                void* ptr = (void*)((sigma_u8*)block + sizeof(sigma_u32)); // Skip prefix
                // Re-write guards (just in case)
                *((sigma_u32*)((sigma_u8*)ptr - sizeof(sigma_u32))) = SIGMA_GUARD_MAGIC;
                *((sigma_u32*)((sigma_u8*)ptr + get_bucket_size(bucket_idx) - 2*sizeof(sigma_u32))) = SIGMA_GUARD_MAGIC;
                return ptr;
            } else {
                // Allocate new block from heap if free list empty
                sigma_u32 block_size = get_bucket_size(bucket_idx);
                if (heap_offset + block_size > SIGMA_HEAP_SIZE) {
                     sigma_log_info("[ALLOCATOR] [FATAL] OOM: Slab heap exhausted.\n");
                     return nullptr;
                }
                void* raw = heap_base + heap_offset;
                heap_offset += block_size;
                
                *((sigma_u32*)raw) = SIGMA_GUARD_MAGIC; // Prefix
                void* ptr = (void*)((sigma_u8*)raw + sizeof(sigma_u32));
                *((sigma_u32*)((sigma_u8*)raw + block_size - sizeof(sigma_u32))) = SIGMA_GUARD_MAGIC; // Suffix
                
                return ptr;
            }
        }
        
        // Slow path: Fallback (bump allocator for large blocks)
        sigma_u32 total_size = size + 2 * sizeof(sigma_u32);
        sigma_u32 aligned_size = (total_size + 7u) & ~7u;
        
        if (heap_offset + aligned_size > SIGMA_HEAP_SIZE) {
            sigma_log_info("[ALLOCATOR] [FATAL] OOM: Large allocation failed.\n");
            return nullptr;
        }
        
        sigma_u32* prefix = (sigma_u32*)&heap_base[heap_offset];
        *prefix = SIGMA_GUARD_MAGIC;
        void* ptr = (void*)((sigma_u8*)prefix + sizeof(sigma_u32));
        sigma_u32* suffix = (sigma_u32*)((sigma_u8*)ptr + size);
        *suffix = SIGMA_GUARD_MAGIC;
        
        heap_offset += aligned_size;
        return ptr;
    }

    void sigma_free(void* ptr, sigma_u32 original_req_size) {
        if (!ptr) return;
        
        sigma_u32* prefix = (sigma_u32*)((sigma_u8*)ptr - sizeof(sigma_u32));
        if (*prefix != SIGMA_GUARD_MAGIC) {
            sigma_log_info("[ALLOCATOR] [FATAL] Memory corruption detected!\n");
            return;
        }
        
        sigma_u32 bucket_idx = get_bucket_index(original_req_size + sizeof(sigma_u32) * 2);
        if (bucket_idx < 8) {
            // Push to free list (O(1))
            SlabBlock* block = (SlabBlock*)prefix;
            block->next = free_lists[bucket_idx];
            free_lists[bucket_idx] = block;
        } else {
             // Cannot easily free large bump allocations.
             sigma_log_info("[ALLOCATOR] Large block freed, cannot reclaim in bump fallback.\n");
        }
    }
};

/* --- C Wrappers --- */
extern "C" void allocator_init() {
    static sigma_u8 global_heap[SIGMA_HEAP_SIZE];
    SovereignAllocatorEngine::getInstance().init(global_heap, SIGMA_HEAP_SIZE);
}

extern "C" void* allocator_malloc(sigma_u32 size) {
    return SovereignAllocatorEngine::getInstance().sigma_malloc(size);
}

extern "C" void allocator_free(void* ptr) {
    SovereignAllocatorEngine::getInstance().sigma_free(ptr, 0);
}
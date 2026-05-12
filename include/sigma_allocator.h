/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CUSTOM ALLOCATOR (S-ALLOCATOR)
 * =========================================================================
 * Mission: A highly optimized, lightweight memory allocator tuned for 
 * modern silicon workloads, outperforming standard Linux malloc.
 * =========================================================================
 */

#ifndef SIGMA_ALLOCATOR_H
#define SIGMA_ALLOCATOR_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Allocator Primitives --- */
void allocator_init(void);
void* allocator_malloc(uint32_t size);
void allocator_free(void* ptr);
void allocator_defrag(void);

#ifdef __cplusplus
}

#define SIGMA_HEAP_SIZE (1024u * 1024u * 16u) /* 16MB Heap */

class SovereignAllocatorEngine {
public:
    static SovereignAllocatorEngine& getInstance() {
        static SovereignAllocatorEngine instance;
        return instance;
    }

    void init();
    void* malloc(sigma_u32 size);
    void free(void* ptr);

private:
    SovereignAllocatorEngine() : heap_offset(0) {}
    
    sigma_u8  heap[SIGMA_HEAP_SIZE];
    sigma_u32 heap_offset;
};
#endif

#endif /* SIGMA_ALLOCATOR_H */

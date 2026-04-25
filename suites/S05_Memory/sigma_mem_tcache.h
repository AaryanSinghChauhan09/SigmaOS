// SigmaOS — sigma-mem-jemalloc: Thread-Cache Slab (jemalloc-inspired)
// Inspired by: jemalloc, tcmalloc — advanced memory allocators
// Module: sigma-mem-tcache
// USP: Per-size-class free lists — O(1) alloc/free, zero fragmentation
// Size classes: 8, 16, 32, 64, 128, 256, 512, 1024 bytes

#ifndef SIGMA_MEM_TCACHE_H
#define SIGMA_MEM_TCACHE_H

#include "sigma_spinlock.h"

#define SIGMA_TC_NUM_CLASSES  8
#define SIGMA_TC_CLASS_DEPTH  64  // max items per free list

static const unsigned int sigma_tc_sizes[SIGMA_TC_NUM_CLASSES] = {
    8, 16, 32, 64, 128, 256, 512, 1024
};

#define SIGMA_TC_BACKING_BYTES (1024 * 1024 * 16) // 16 MB backing store

typedef struct SigmaTCacheFreeList {
    void*        ptrs[SIGMA_TC_CLASS_DEPTH];
    unsigned int depth;
} SigmaTCacheFreeList;

typedef struct SigmaTCache {
    unsigned char          backing[SIGMA_TC_BACKING_BYTES];
    unsigned long          backing_used;
    SigmaTCacheFreeList    classes[SIGMA_TC_NUM_CLASSES];
    SigmaSpinlock          lock;
    unsigned long          alloc_count;
    unsigned long          free_count;
    unsigned long          cache_hits;
} SigmaTCache;

static inline void tcache_init(SigmaTCache* tc) {
    tc->backing_used = 0;
    tc->alloc_count = tc->free_count = tc->cache_hits = 0;
    spinlock_init(&tc->lock);
    for (int i = 0; i < SIGMA_TC_NUM_CLASSES; i++) tc->classes[i].depth = 0;
}

// Find size class index for requested size
static inline int tcache_class(unsigned int size) {
    for (int i = 0; i < SIGMA_TC_NUM_CLASSES; i++)
        if (size <= sigma_tc_sizes[i]) return i;
    return -1; // too large for tcache
}

static inline void* tcache_alloc(SigmaTCache* tc, unsigned int size) {
    spinlock_acquire(&tc->lock);
    int cls = tcache_class(size);
    // Check free list first (cache hit)
    if (cls >= 0 && tc->classes[cls].depth > 0) {
        void* ptr = tc->classes[cls].ptrs[--tc->classes[cls].depth];
        tc->cache_hits++;
        tc->alloc_count++;
        spinlock_release(&tc->lock);
        return ptr;
    }
    // Allocate from backing store
    unsigned int alloc_size = (cls >= 0) ? sigma_tc_sizes[cls] : size;
    if (tc->backing_used + alloc_size > SIGMA_TC_BACKING_BYTES) {
        spinlock_release(&tc->lock);
        return (void*)0;
    }
    void* ptr = &tc->backing[tc->backing_used];
    tc->backing_used += alloc_size;
    tc->alloc_count++;
    spinlock_release(&tc->lock);
    return ptr;
}

static inline void tcache_free(SigmaTCache* tc, void* ptr, unsigned int size) {
    int cls = tcache_class(size);
    if (cls < 0) return; // oversized — no reclaim in this impl
    spinlock_acquire(&tc->lock);
    if (tc->classes[cls].depth < SIGMA_TC_CLASS_DEPTH)
        tc->classes[cls].ptrs[tc->classes[cls].depth++] = ptr;
    tc->free_count++;
    spinlock_release(&tc->lock);
}

static inline unsigned long tcache_hit_rate(SigmaTCache* tc) {
    if (!tc->alloc_count) return 0;
    return (tc->cache_hits * 100) / tc->alloc_count;
}

#endif /* SIGMA_MEM_TCACHE_H */

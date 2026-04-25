// SigmaOS — sigma-mem-pool: Sovereign Memory Pool
// Modularised from: SovereignMemoryZenith.c
// Single responsibility: manage a fixed static memory pool only

#ifndef SIGMA_MEM_POOL_H
#define SIGMA_MEM_POOL_H

#define SIGMA_MEM_POOL_MB    64
#define SIGMA_MEM_POOL_SIZE  (SIGMA_MEM_POOL_MB * 1024 * 1024)
#define SIGMA_MEM_MAX_SEGS   2048

typedef struct SigmaMemSeg {
    unsigned long addr;
    unsigned long size;
    unsigned char in_use;
} SigmaMemSeg;

typedef struct SigmaMemPool {
    unsigned char   pool[SIGMA_MEM_POOL_SIZE];
    unsigned long   used;
    SigmaMemSeg     segs[SIGMA_MEM_MAX_SEGS];
    unsigned int    seg_count;
} SigmaMemPool;

static inline void mem_pool_init(SigmaMemPool* mp) {
    mp->used = 0; mp->seg_count = 0;
}

static inline void* mem_pool_alloc(SigmaMemPool* mp, unsigned long size) {
    if (mp->used + size > SIGMA_MEM_POOL_SIZE) return (void*)0;
    if (mp->seg_count >= SIGMA_MEM_MAX_SEGS)   return (void*)0;
    void* ptr = &mp->pool[mp->used];
    mp->segs[mp->seg_count].addr   = (unsigned long)ptr;
    mp->segs[mp->seg_count].size   = size;
    mp->segs[mp->seg_count].in_use = 1;
    mp->seg_count++;
    mp->used += size;
    return ptr;
}

static inline void mem_pool_free(SigmaMemPool* mp, void* ptr) {
    for (unsigned int i = 0; i < mp->seg_count; i++) {
        if (mp->segs[i].addr == (unsigned long)ptr) {
            mp->segs[i].in_use = 0;
            return;
        }
    }
}

static inline unsigned long mem_pool_used_kb(SigmaMemPool* mp) {
    return mp->used / 1024;
}

static inline unsigned long mem_pool_free_kb(SigmaMemPool* mp) {
    return (SIGMA_MEM_POOL_SIZE - mp->used) / 1024;
}

#endif /* SIGMA_MEM_POOL_H */

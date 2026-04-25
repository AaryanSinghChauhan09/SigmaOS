// SigmaOS — sigma-perf-shadow: Shadow Memory Leak Detector
// Inspired by: Valgrind Memcheck, ASan (AddressSanitizer)
// Module: sigma-perf-shadow
// USP: No DWARF debug info needed — tracks alloc/free in a flat shadow table
// Each allocation gets a shadow entry — reports unfreed blocks on audit

#ifndef SIGMA_PERF_SHADOW_H
#define SIGMA_PERF_SHADOW_H

#define SIGMA_SHADOW_MAX_ALLOCS  1024
#define SIGMA_SHADOW_TAG_LEN      32

typedef enum SigmaShadowState {
    SHADOW_FREE  = 0,
    SHADOW_ALLOC = 1,
    SHADOW_FREED = 2
} SigmaShadowState;

typedef struct SigmaShadowEntry {
    void*             ptr;
    unsigned long     size;
    unsigned long     alloc_cycles;  // RDTSC at alloc
    unsigned long     free_cycles;   // RDTSC at free (0 if not freed)
    char              tag[SIGMA_SHADOW_TAG_LEN];  // caller label
    SigmaShadowState  state;
} SigmaShadowEntry;

typedef struct SigmaShadowMem {
    SigmaShadowEntry entries[SIGMA_SHADOW_MAX_ALLOCS];
    unsigned int     count;
    unsigned long    total_allocated;
    unsigned long    total_freed;
    unsigned long    peak_live;
    unsigned long    current_live;
} SigmaShadowMem;

static inline unsigned long shadow_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void shadow_init(SigmaShadowMem* s) {
    s->count = 0;
    s->total_allocated = s->total_freed = s->peak_live = s->current_live = 0;
}

// Record an allocation
static inline int shadow_record_alloc(SigmaShadowMem* s, void* ptr,
                                        unsigned long size, const char* tag) {
    if (s->count >= SIGMA_SHADOW_MAX_ALLOCS) return -1;
    SigmaShadowEntry* e = &s->entries[s->count++];
    e->ptr          = ptr;
    e->size         = size;
    e->alloc_cycles = shadow_rdtsc();
    e->free_cycles  = 0;
    e->state        = SHADOW_ALLOC;
    for (int i = 0; i < SIGMA_SHADOW_TAG_LEN - 1 && tag[i]; i++) e->tag[i] = tag[i];
    s->total_allocated += size;
    s->current_live    += size;
    if (s->current_live > s->peak_live) s->peak_live = s->current_live;
    return 0;
}

// Record a free
static inline int shadow_record_free(SigmaShadowMem* s, void* ptr) {
    for (unsigned int i = 0; i < s->count; i++) {
        if (s->entries[i].ptr == ptr && s->entries[i].state == SHADOW_ALLOC) {
            s->entries[i].state       = SHADOW_FREED;
            s->entries[i].free_cycles = shadow_rdtsc();
            s->total_freed   += s->entries[i].size;
            s->current_live  -= s->entries[i].size;
            return 0;
        }
    }
    return -1; // double-free or unknown ptr
}

// Count live (leaked) allocations
static inline unsigned int shadow_leak_count(SigmaShadowMem* s) {
    unsigned int n = 0;
    for (unsigned int i = 0; i < s->count; i++)
        if (s->entries[i].state == SHADOW_ALLOC) n++;
    return n;
}

// Total leaked bytes
static inline unsigned long shadow_leaked_bytes(SigmaShadowMem* s) {
    unsigned long n = 0;
    for (unsigned int i = 0; i < s->count; i++)
        if (s->entries[i].state == SHADOW_ALLOC) n += s->entries[i].size;
    return n;
}

#endif /* SIGMA_PERF_SHADOW_H */

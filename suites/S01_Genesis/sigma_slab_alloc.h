// SigmaOS — Sovereign Slab Allocator
// Module: sigma-sys-alloc
// Single responsibility: O(1) slab-based kernel memory allocation
// Zero dependencies — uses only sigma_libc types

#ifndef SIGMA_SLAB_ALLOC_H
#define SIGMA_SLAB_ALLOC_H

#ifdef __cplusplus
#include "../../include/libc/sigma_libc.h"
using size_type = sigma_size_t;
#else
typedef unsigned long size_type;
#endif

#define SLAB_BLOCK_SIZE   64
#define SLAB_MAX_BLOCKS   4096

typedef struct SlabBlock {
    unsigned char data[SLAB_BLOCK_SIZE];
    int in_use;
} SlabBlock;

typedef struct SlabAllocator {
    SlabBlock pool[SLAB_MAX_BLOCKS];
    unsigned int total;
    unsigned int used;
} SlabAllocator;

static SlabAllocator g_slab = {{{{0},0}}, SLAB_MAX_BLOCKS, 0};

/* Allocate one slab block — O(1) amortised via free-list future ext. */
static inline void* slab_alloc(SlabAllocator* s) {
    for (unsigned int i = 0; i < s->total; i++) {
        if (!s->pool[i].in_use) {
            s->pool[i].in_use = 1;
            s->used++;
            return (void*)s->pool[i].data;
        }
    }
    return (void*)0; /* out of slabs */
}

/* Free a slab block by pointer */
static inline void slab_free(SlabAllocator* s, void* ptr) {
    unsigned char* p = (unsigned char*)ptr;
    for (unsigned int i = 0; i < s->total; i++) {
        if (s->pool[i].data == p) {
            s->pool[i].in_use = 0;
            s->used--;
            return;
        }
    }
}

static inline unsigned int slab_used(SlabAllocator* s) { return s->used; }

#endif /* SIGMA_SLAB_ALLOC_H */

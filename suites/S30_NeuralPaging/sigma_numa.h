// SigmaOS — Sigma-NUMA: NUMA-Aware Memory Allocator
// Inspired by: Linux NUMA balancing, FreeBSD UMA
// Module: sigma-sys-numa
// USP: Per-node slab pools — memory allocated close to the CPU using it
// Zero libnuma dependency — direct node assignment via compile-time constants

#ifndef SIGMA_NUMA_H
#define SIGMA_NUMA_H

#include "../../include/sigma_slab_alloc.h"
#include "../../include/sigma_spinlock.h"

#define SIGMA_NUMA_MAX_NODES  8
#define SIGMA_NUMA_SLAB_BLOCKS 2048

typedef struct SigmaNumaNode {
    SlabAllocator slab;
    SigmaSpinlock lock;
    unsigned int  node_id;
    unsigned long total_alloc_bytes;
    unsigned long total_free_bytes;
} SigmaNumaNode;

typedef struct SigmaNumaAllocator {
    SigmaNumaNode nodes[SIGMA_NUMA_MAX_NODES];
    unsigned int  node_count;
} SigmaNumaAllocator;

static inline void numa_init(SigmaNumaAllocator* na, unsigned int nodes) {
    if (nodes > SIGMA_NUMA_MAX_NODES) nodes = SIGMA_NUMA_MAX_NODES;
    na->node_count = nodes;
    for (unsigned int i = 0; i < nodes; i++) {
        na->nodes[i].node_id          = i;
        na->nodes[i].total_alloc_bytes = 0;
        na->nodes[i].total_free_bytes  = 0;
        spinlock_init(&na->nodes[i].lock);
        // Slab will serve as the node-local memory pool
        for (unsigned int j = 0; j < SLAB_MAX_BLOCKS; j++) {
            na->nodes[i].slab.pool[j].in_use = 0;
        }
        na->nodes[i].slab.total = SLAB_MAX_BLOCKS;
        na->nodes[i].slab.used  = 0;
    }
}

// Allocate from a specific NUMA node (CPU-local allocation)
static inline void* numa_alloc_on(SigmaNumaAllocator* na, unsigned int node) {
    if (node >= na->node_count) return (void*)0;
    SigmaNumaNode* n = &na->nodes[node];
    spinlock_acquire(&n->lock);
    void* ptr = slab_alloc(&n->slab);
    if (ptr) n->total_alloc_bytes += SLAB_BLOCK_SIZE;
    spinlock_release(&n->lock);
    return ptr;
}

// Free to a specific NUMA node
static inline void numa_free_on(SigmaNumaAllocator* na, unsigned int node, void* ptr) {
    if (node >= na->node_count) return;
    SigmaNumaNode* n = &na->nodes[node];
    spinlock_acquire(&n->lock);
    slab_free(&n->slab, ptr);
    n->total_free_bytes += SLAB_BLOCK_SIZE;
    spinlock_release(&n->lock);
}

// NUMA-aware migration: move allocation to preferred node if misplaced
static inline void* numa_migrate(SigmaNumaAllocator* na,
                                   unsigned int from, unsigned int to,
                                   void* old_ptr, unsigned int size) {
    void* new_ptr = numa_alloc_on(na, to);
    if (!new_ptr) return old_ptr; // keep where it is
    // Copy data (manual loop — no memcpy dependency)
    unsigned char* s = (unsigned char*)old_ptr;
    unsigned char* d = (unsigned char*)new_ptr;
    unsigned int bytes = (size < SLAB_BLOCK_SIZE) ? size : SLAB_BLOCK_SIZE;
    for (unsigned int i = 0; i < bytes; i++) d[i] = s[i];
    numa_free_on(na, from, old_ptr);
    return new_ptr;
}

#endif /* SIGMA_NUMA_H */

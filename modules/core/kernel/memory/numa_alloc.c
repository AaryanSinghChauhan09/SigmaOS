#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS NUMA-Aware Memory Allocator Prototype
// ---------------------------------------------------------

#define MAX_NUMA_NODES 8
#define PAGES_PER_NODE 65536 // 256MB per node @ 4KB pages

typedef struct {
    uint32_t node_id;
    uint64_t base_physical_addr;
    uint32_t total_pages;
    uint32_t free_pages;
    uint8_t  bitmap[PAGES_PER_NODE / 8]; // 1 bit per page
} numa_node_t;

static numa_node_t numa_nodes[MAX_NUMA_NODES];
static uint32_t numa_node_count = 0;

void numa_register_node(uint32_t node_id, uint64_t base_addr, uint32_t pages) {
    if (numa_node_count >= MAX_NUMA_NODES) return;
    numa_node_t* n = &numa_nodes[numa_node_count++];
    n->node_id = node_id;
    n->base_physical_addr = base_addr;
    n->total_pages = pages;
    n->free_pages = pages;
    for (int i = 0; i < (int)(pages / 8); i++) n->bitmap[i] = 0;
}

// Allocate a page from the preferred NUMA node (minimize cross-socket latency)
void* numa_alloc_page(uint32_t preferred_node) {
    // Try preferred node first, then fall back to any available node
    for (int pass = 0; pass < 2; pass++) {
        for (uint32_t n = 0; n < numa_node_count; n++) {
            uint32_t target = (pass == 0) ? preferred_node : n;
            if (target >= numa_node_count) continue;
            numa_node_t* node = &numa_nodes[target];
            if (node->free_pages == 0) continue;
            // Find a free page in bitmap
            for (uint32_t i = 0; i < node->total_pages / 8; i++) {
                if (node->bitmap[i] != 0xFF) {
                    for (int b = 0; b < 8; b++) {
                        if (!(node->bitmap[i] & (1 << b))) {
                            node->bitmap[i] |= (1 << b);
                            node->free_pages--;
                            return (void*)(node->base_physical_addr + ((i * 8 + b) * 4096));
                        }
                    }
                }
            }
            if (pass == 0) break; // Only try preferred node on first pass
        }
    }
    return SIGMA_NULL; // Out of memory
}

void numa_free_page(void* addr) {
    uint64_t phys = (uint64_t)addr;
    for (uint32_t n = 0; n < numa_node_count; n++) {
        numa_node_t* node = &numa_nodes[n];
        if (phys < node->base_physical_addr) continue;
        uint64_t offset = phys - node->base_physical_addr;
        uint32_t page_idx = (uint32_t)(offset / 4096);
        if (page_idx >= node->total_pages) continue;
        node->bitmap[page_idx / 8] &= ~(1 << (page_idx % 8));
        node->free_pages++;
        return;
    }
}

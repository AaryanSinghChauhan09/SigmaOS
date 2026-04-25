#pragma once
#include <stdint.h>

namespace SigmaOS {
namespace Kernel {

// Track 1: Kernel Maturity - Memory Management
// Non-Uniform Memory Access (NUMA) topology awareness
#define MAX_NUMA_NODES 8

struct NUMANode {
    uint32_t node_id;
    uint64_t start_paddr;
    uint64_t end_paddr;
    uint64_t free_pages;
    uint64_t total_pages;
    bool is_active;
};

class SovereignNUMA {
private:
    NUMANode nodes[MAX_NUMA_NODES];
    uint32_t active_nodes;

public:
    SovereignNUMA();
    
    // Core Memory Management
    void discover_topology();
    void* alloc_page_local(uint32_t cpu_id);
    void* alloc_page_node(uint32_t node_id);
    void free_page(void* paddr);
    
    void print_topology();
};

} // namespace Kernel
} // namespace SigmaOS

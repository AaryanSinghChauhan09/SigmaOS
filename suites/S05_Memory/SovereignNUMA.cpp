#include "SovereignNUMA.hpp"
#include "../../include/libc/sigma_libc.h"

namespace SigmaOS {
namespace Kernel {

SovereignNUMA::SovereignNUMA() : active_nodes(0) {
    for (int i = 0; i < MAX_NUMA_NODES; i++) {
        nodes[i].is_active = false;
    }
    sigma_log("[MEMORY] Sovereign NUMA Manager Initialized.");
}

void SovereignNUMA::discover_topology() {
    // In a real bare-metal implementation, parse ACPI SRAT (System Resource Affinity Table)
    sigma_log("[MEMORY] Parsing ACPI SRAT for NUMA topology...");
    
    // Mocking 2 NUMA nodes for testing
    nodes[0] = {0, 0x00000000, 0x40000000, 1024, 1024, true};
    nodes[1] = {1, 0x40000000, 0x80000000, 1024, 1024, true};
    active_nodes = 2;
    
    sigma_print("[MEMORY] Discovered ");
    sigma_print_num(active_nodes);
    sigma_print(" NUMA nodes.\n");
}

void* SovereignNUMA::alloc_page_node(uint32_t node_id) {
    if (node_id >= MAX_NUMA_NODES || !nodes[node_id].is_active) {
        return nullptr; // Fallback to other nodes in a real implementation
    }
    
    if (nodes[node_id].free_pages > 0) {
        nodes[node_id].free_pages--;
        // Return calculated physical address (mocked)
        return (void*)(nodes[node_id].start_paddr + (nodes[node_id].total_pages - nodes[node_id].free_pages) * 4096);
    }
    return nullptr; // Out of memory on this node
}

void* SovereignNUMA::alloc_page_local(uint32_t cpu_id) {
    // Map CPU APIC ID to NUMA node ID
    uint32_t node_id = cpu_id % active_nodes; 
    return alloc_page_node(node_id);
}

void SovereignNUMA::free_page(void* paddr) {
    sigma_log("[MEMORY] Page freed to NUMA pool.");
}

void SovereignNUMA::print_topology() {
    sigma_print("\n--- Σ NUMA TOPOLOGY ---\n");
    for (uint32_t i = 0; i < active_nodes; i++) {
        sigma_print("| Node ID: "); sigma_print_num(nodes[i].node_id);
        sigma_print(" | Free: "); sigma_print_num(nodes[i].free_pages);
        sigma_print(" / "); sigma_print_num(nodes[i].total_pages);
        sigma_print("\n");
    }
}

} // namespace Kernel
} // namespace SigmaOS

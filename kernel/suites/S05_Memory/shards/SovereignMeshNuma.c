// =============================================================================
// SigmaOS — S05_Memory — SovereignMeshNuma.c
// Industrial-grade Distributed Shared Memory (DSM) Shard
// =============================================================================
// Market Leadership:
//   • Windows/Linux/macOS — Memory is strictly local to the physical silicon.
//   • SigmaOS Mesh NUMA — Treat the entire S12 Hive as a single NUMA pool. 
//     Page-faults are resolved via ultra-low-latency peer memory fetches.
// Architecture:
//   • Distributed Page Tables: Page directory spans multiple Hive nodes.
//   • RDMA-Lite: Direct memory packets over S07 Network with zero-copy.
//   • Coherency: S13 Sentience directs a "Relaxed Consistency" model to 
//     minimize mesh traffic while maintaining thread safety.
// =============================================================================

#include <sigma_types.h>


#define HIVE_PAGE_SIZE      4096
#define MESH_ADDR_SPACE_BITS 64

typedef struct {
    uint64_t virtual_addr;
    uint8_t  owner_node_id;
    uint64_t remote_physical_addr;
    uint8_t  protection; // RO, RW, NX
    bool     is_locally_cached;
} MeshPageDescriptor;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Mesh NUMA system
void mesh_numa_init(void);

// Register local RAM as available to the Hive Mesh (S12)
void mesh_numa_export_range(uintptr_t start, size_t size);

// Resolve a page fault by fetching from a Hive peer
bool mesh_numa_resolve_fault(uintptr_t fault_addr, uint8_t* out_page_data);

// Flush a page back to its owner node (Mesh coherency)
void mesh_numa_flush_page(uintptr_t addr);

// Audit mesh memory latency (S04 HAL path)
uint32_t mesh_numa_get_latency_ns(uint8_t node_id);

// Synchronize Mesh Page Tables with the S13 Neural Fabric (Predictive pre-fetch)
void mesh_numa_sentience_prefetch(uintptr_t predicted_addr);




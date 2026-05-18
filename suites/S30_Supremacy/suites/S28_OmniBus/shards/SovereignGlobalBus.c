// =============================================================================
// SigmaOS — S28_OmniBus — SovereignGlobalBus.c
// Industrial-grade Cross-Hardware PCIe & Memory Bridge
// =============================================================================
// Beyond the Leaders:
//   • Standard Cluster OSs — RDMA / InfiniBand (Network based).
//   • SigmaOS OmniBus — HARDWARE-MESH. Treats the entire Hive mesh as a 
//     single logical backplane. A GPU on Node A can write directly to the 
//     VRAM of Node B via simulated optical/ultra-high-speed lattice-links.
// Result: Distributed computing that feels like a single multi-socket board.
// =============================================================================

#include "core/sigma_types.h"


typedef struct {
    uint8_t  peer_node_id[16];
    uintptr_t remote_base_reg;
    uint32_t bus_velocity_gbps;
    bool     is_optical_sim_active;
} BusLink;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Omni-Bus backplane nexus
void omnibus_init(void);

// Bridge a local PCIe capability to a remote Hive node
bool omnibus_bridge_peripheral(uint32_t local_dev_id, uint8_t target_node_id);

// Synchronize global bus clock across the lattice (S04 HAL path)
void omnibus_sync_clocks(void);

// Map a remote device's BAR (Base Address Register) into local S05 MeshNuma
void* omnibus_map_remote_bar(uint8_t target_node_id, uint32_t remote_dev_id);

// Audit Bus-Coherence and Latency across the Fabric (S07 QSSS hook)
float omnibus_get_interconnect_health(void);

// Sync device state for 'Hot-Migration' of hardware tasks
void omnibus_swap_peripheral_host(uint32_t dev_id, uint8_t new_host_id);




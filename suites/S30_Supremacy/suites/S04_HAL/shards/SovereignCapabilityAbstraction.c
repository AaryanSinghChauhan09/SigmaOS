// =============================================================================
// SigmaOS — S04_HAL — SovereignCapabilityAbstraction.c
// Industrial-Grade Unified hardware Capability Nexus
// =============================================================================
// Beyond the Leaders:
//   • Windows/Linux/macOS — Driver-centric architecture (e.g. "Load Nvidia driver").
//   • SigmaOS Nexus — CAPABILITY-CENTRIC. The hardware is abstracted into 
//     universal primitives: "Compute", "Render", "Transmit", "Persist".
//   • OS automatically routes "Render" requests to the most efficient unit 
//     (iGPU, dGPU, NPU, or Peer Hive Node) without app-level visibility.
// =============================================================================

#include "core/sigma_types.h"


typedef enum {
    CAP_COMPUTE = 0,
    CAP_RENDER  = 1,
    CAP_STREAM  = 2,
    CAP_SENSE   = 3  // HID/Cam/Mic
} HardwareCapability;

typedef struct {
    uint32_t capability_id;
    HardwareCapability type;
    uint64_t performance_score;
    uint32_t latency_ns;
    bool     is_local; // True = On-die, False = Remote Hive Node
} CapabilityNode;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Capability Nexus
void capability_init(void);

// Register a piece of hardware as a "Capability Provider"
void capability_register_hw(CapabilityNode* node);

// Request a capability fulfillment (Zero-copy dispatch)
void* capability_request(HardwareCapability type, uint32_t required_perf);

// Autonomous Routing: Shift "Render" load from dGPU to Hive Peer to save power
void capability_auto_route(uint32_t session_id);

// Audit current Capability Stack for "Sovereign Purity" (S08)
bool capability_verify_stack_integrity(void);

// Sync available Capabilities with the Hive Mesh (Distributed Power)
void capability_mesh_sync(void);




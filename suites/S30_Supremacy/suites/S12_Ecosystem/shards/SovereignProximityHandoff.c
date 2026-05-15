// =============================================================================
// SigmaOS — S12_Ecosystem — SovereignProximityHandoff.c
// near-Field Proximity Aware Handoff Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Handoff / AirDrop — Seamless transfer based on closeness
//   • Google Nearby Share / Quick Share — Local discovery and transfer
//   • Microsoft Phone Link — Notification and clip sync
// SigmaOS Proximity:
//   • Ultra-Wideband (UWB) Logic: Precise <10cm distance tracking (via S04).
//   • Sentiment-Handoff: S13 predicts *where* you are going to hand off based on 
//     closeness and orientation (pointing your phone at your PC).
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


typedef struct {
    uint8_t  peer_uuid[16];
    float    distance_meters;
    float    orientation_vector[3]; // X, Y, Z pointing vector
} ProximityNode;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Proximity Handoff listener (Uses S04 HAL bluetooth/uwb)
void proximity_init(void);

// Heartbeat: Update physical distance/orientation for Hive peers
void proximity_update_node(uint8_t* peer_uuid, float dist, float* vector);

// Check if a handoff is "Likely" (Point-and-Handoff logic)
bool proximity_is_intent_detected(uint8_t* peer_uuid);

// Auto-trigger Continuity (S12) based on proximity
void proximity_trigger_handoff(uint8_t* peer_uuid, const char* app_state);

// Share files instantly via Near-Field Mesh (AirDrop parity)
void proximity_mesh_transfer(const char* sab_path, uint8_t* peer_uuid);

// Notify ZenithUI (S02) of "Nearby Device Found" with 3D spatial location
void proximity_report_to_ui(void);




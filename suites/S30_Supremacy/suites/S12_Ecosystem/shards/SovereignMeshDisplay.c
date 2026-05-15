// =============================================================================
// SigmaOS — S12_Ecosystem — SovereignMeshDisplay.c
// Zero-Latency High-Fidelity mesh Display Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Sidecar / AirPlay — Wireless display to iPad/Mac
//   • Windows Wireless Display  — Miracast protocol display extension
//   • Universal Control (Apple) — One HID for multiple devices
// SigmaOS Mesh Display:
//   • Zero-Latency: Direct VRAM-to-S07-Packet blitting (Zero-copy).
//   • Unified Buffer: The second device's VRAM becomes an extension of yours.
//   • Touch-Back: Multi-touch on external device flows back to host (S04).
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


#define DISPLAY_MAX_RESOLUTION_W 7680
#define DISPLAY_MAX_RESOLUTION_H 4320

typedef struct {
    uint8_t  peer_uuid[16];
    uint32_t current_fps;
    float    latency_ms;
    bool     is_streaming;
} DisplayPeer;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Mesh Display protocol
void mesh_display_init(void);

// Mirror the local screen to a Hive peer (S12)
bool mesh_display_mirror_to(uint8_t* peer_uuid);

// Extend the local desktop to a Hive peer (Sidecar parity)
bool mesh_display_extend_to(uint8_t* peer_uuid);

// Receive a display stream from a peer node
void mesh_display_receive_stream(void* frame_data, uint32_t len);

// Sync HID events (mouse/key) across a multi-display mesh (Universal Control)
void mesh_display_sync_hid(uint8_t* target_node_id);

// Handle touch-back events from the remote display (S04)
void mesh_display_process_remote_touch(void* touch_event);




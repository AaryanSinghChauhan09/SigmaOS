// =============================================================================
// SigmaOS — S12_Ecosystem — SovereignMeshUpdate.c
// P2P Purity-Verified Atomic In-Memory Updates
// =============================================================================
// Exceeding Competitors:
//   • Windows Update   — disruptive reboots, heavy background load.
//   • macOS/Linux      — large binary downloads, post-update re-indexing.
//   • Sigma Mesh Update— P2P decentralized updates: uses S12 Mesh to find 
//     verified delta-shards. Applies updates IN-MEMORY using S13 Hot-Patching
//     to avoid reboots entirely.
// =============================================================================

#include <sigma_types.h>


#define UPDATE_CHUNK_SIZE   65536
#define MAX_PEER_SOURCES    8

typedef struct {
    uint32_t suite_id;
    uintptr_t func_addr;
    uint32_t  delta_len;
    uint8_t   pqc_sig[64]; // S08 Quantum Signature verification
} DeltaShard;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Mesh Update engine (mDNS discovery via S07)
void mesh_update_init(void);

// Fetch a delta-shard from the Sovereign P2P mesh
bool mesh_update_fetch_delta(uint32_t suite_id, uint32_t version_target);

// Verify delta-shard integrity via Quantum-Safe Crypto (S08)
bool mesh_update_verify(DeltaShard* delta);

// Apply update IN-MEMORY (Hot-patch) without system reboot
void mesh_update_apply_hot(DeltaShard* delta);

// Sync update progress to ZenithUI Dashboard
void mesh_update_report_status(float progress);

// Broadcast update availability to Continuity peers (S12)
void mesh_update_advertise_shard(uint32_t suite_id);




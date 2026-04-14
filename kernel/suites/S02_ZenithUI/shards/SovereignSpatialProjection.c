// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignSpatialProjection.c
// Native AR/VR / Volumetric Output Shard
// =============================================================================
// Beyond the Leaders:
//   • visionOS / Quest — Separate OS for spatial computing.
//   • SigmaOS Spatial — NATIVE SPATIAL. The same S02 ZenithUI engine 
//     renders to 2D screens, AR glasses, and VR headsets simultaneously 
//     through volumetric projection shards.
// Result: Total immersion leader for all form-factors.
// =============================================================================

#include <sigma_types.h>


typedef struct {
    float projection_matrix[16];
    float eye_offset[3];
    uint32_t target_fov;
    bool     is_stereoscopic;
} SpatialView;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Spatial Projection engine
void spatial_project_init(void);

// Project the S02 Compositor into 3D Stereoscopic space (visionOS parity)
void spatial_project_render_stereo(SpatialView* left, SpatialView* right);

// Anchor a 2D window into a 3D volumetric "World Anchor" (S04)
void spatial_project_anchor_window(uint32_t window_id, float world_x, float world_y, float world_z);

// Sync spatial head-tracking with SovereignGestureCore (S04)
void spatial_project_sync_tracking(void);

// Handle volumetric UI occlusion using Z-Space Manager (S02)
void spatial_project_apply_occlusion(void);

// Sync spatial workspace across Hive mesh (Shared AR S12)
void spatial_project_mesh_collab(uint8_t* peer_uuid);



// =============================================================================
// SigmaOS — S02_ZenithUI — SovereignZSpaceManager.c
// Industrial-grade Z-axis Depth Compositor Shard
// =============================================================================
// Beyond the Leaders:
//   • Windows/macOS/Linux — 2D stacking (Windows behind windows).
//   • SigmaOS Z-Space — TRUE 3D VOLUMETRIC SPACE. Every "Window" is a 
//     Holographic Object (S02) with a real Z-coordinate, depth-blurring, 
//     and occlusion as it moves further away in the spatial desktop.
// =============================================================================

#include "sigma_types.h"


typedef struct {
    uint32_t object_id;
    float    x, y, z;
    float    depth_scale;
    float    blur_kernel_size;
    bool     is_occluded;
} SpatialObject;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Z-Space Compositor
void zspace_init(void);

// Push a window/object further into the volumetric workspace
void zspace_set_depth(uint32_t obj_id, float z_depth);

// Execute depth-aware blur and shadow blending (S04 GpuStack hook)
void zspace_composite_frame(void);

// Handle "Focus Pull" gesture (S04 GestureCore hook) to bring depth to front
void zspace_on_gesture_depth(float delta_z);

// Snapshot spatial arrangement for TimeVault (S06)
void zspace_cache_topology(void);

// Sync workspace depth with Hive mesh (MeshDisplay S12)
void zspace_sync_mesh(void);



